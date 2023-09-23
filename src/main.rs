#![allow(dead_code, non_snake_case, unused_variables)]

mod geohash;

use rand::Rng;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq)]
struct Waypoint {
    lat: f32,
    lon: f32,
    label: String,
    geohash: String,
    connections: Vec<Connection>,
}

#[derive(Debug, Clone, PartialEq)]
struct Connection {
    distance: f32,
    waypoint: Rc<RefCell<Waypoint>>,
}

struct Trie {
    waypoint: Option<Rc<RefCell<Waypoint>>>,
    children: HashMap<char, Trie>,
}

struct Dataset {
    name: String,
    waypoints: Vec<Rc<RefCell<Waypoint>>>,
    geohash_index: Trie,
}

impl Eq for Connection {}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}

impl Waypoint {
    /// Implements the Haversine formula to find the distance between self and another waypoint (in km)
    fn distance_to(&self, target: Rc<RefCell<Waypoint>>) -> f32 {
        const EARTH_RADIUS: f32 = 6378.137;

        let target_waypoint = target.borrow();

        let lat1 = self.lat.to_radians();
        let lat2 = target_waypoint.lat.to_radians();

        let dlat = lat2 - lat1;
        let dlon = target_waypoint.lon.to_radians() - self.lon.to_radians();

        let a = (dlat / 2.0).sin().powi(2) + (dlon / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS * c
    }

    /// Generates a sequential three-character label, from 'AAA' to 'AAB' to 'ZZZ', based on the passed-in value 'n'.
    fn generate_label(mut n: usize) -> String {
        let mut label = String::new();

        for _ in 0..3 {
            let remainder = n % 26;
            let char_value = (remainder as u8 + b'A') as char;
            label.push(char_value);
            n /= 26;
        }

        label.chars().rev().collect()
    }

    /// Returns a String of the Waypoint's coordinates in Degrees/Minutes/Seconds (DMS) format
    fn get_DMS(&self) -> String {
        let lat = self.lat.abs(); // Convert (-) values to (+) for cleaner code; sign only relevant in determining direction
        let lat_degrees = lat.floor(); // The whole number portion of the value equals degrees
        let lat_minutes = (lat - lat_degrees) * 60.0; // The decimal portion of the value, times 60, equals minutes
        let lat_seconds = (lat_minutes - lat_minutes.floor()) * 60.0; // The decimal portion of minutes, times 60, equals seconds
        let lat_direction = if self.lat >= 0.0 { 'N' } else { 'S' }; // Assign the cardinal direction based on sign

        let long = self.lon.abs();
        let long_degrees = long.floor();
        let long_minutes = (long - long_degrees) * 60.0;
        let long_seconds = (long_minutes - long_minutes.floor()) * 60.0;
        let long_direction = if self.lon >= 0.0 { 'E' } else { 'W' };

        format!(
            "{}°{}'{:.2}\"{}, {}°{}'{:.2}\"{}",
            lat_degrees,
            lat_minutes.floor(),
            lat_seconds,
            lat_direction,
            long_degrees,
            long_minutes.floor(),
            long_seconds,
            long_direction
        )
    }
}

impl Trie {
    fn new() -> Self {
        Trie {
            waypoint: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, geohash: &str, waypoint: Option<Rc<RefCell<Waypoint>>>) {
        let mut current_node = self;

        for c in geohash.chars() {
            current_node = current_node.children.entry(c).or_insert(Trie::new());
        }

        current_node.waypoint = waypoint;
    }

    fn get_all_with_prefix(&self, prefix: &str) -> Vec<Rc<RefCell<Waypoint>>> {
        let mut current = self;
        let mut found_waypoints = Vec::new();

        for c in prefix.chars() {
            if let Some(child) = current.children.get(&c) {
                current = child;
            } else {
                return Vec::new();
            }
        }

        self.collect_waypoints_recursive(&current, &mut found_waypoints);
        found_waypoints
    }

    fn collect_waypoints_recursive(&self, node: &Trie, waypoints: &mut Vec<Rc<RefCell<Waypoint>>>) {
        if let Some(waypoint) = &node.waypoint {
            waypoints.push(waypoint.clone());
        }

        for child in node.children.values() {
            self.collect_waypoints_recursive(child, waypoints);
        }
    }
}

impl Dataset {
    fn new(name: String) -> Self {
        Dataset {
            name,
            waypoints: Vec::new(),
            geohash_index: Trie::new(),
        }
    }

    fn generate_waypoints(&mut self, amt: usize) {
        let mut rng = rand::thread_rng();

        for i in 0..amt {
            let label = Waypoint::generate_label(i);
            let lat = rng.gen_range(-90.0..=90.0);
            let lon = rng.gen_range(-180.0..=180.0);
            let geohash = geohash::encode(lat, lon, 8);

            // while self.geohash_index.contains_key(&geohash) {
            //     lat = rng.gen_range(-90.0..=90.0);
            //     lon = rng.gen_range(-180.0..=180.0);
            //     geohash = geohash::encode(lat, lon, 8);
            // }

            let waypoint = Rc::new(RefCell::new(Waypoint {
                label,
                lat,
                lon,
                geohash,
                connections: Vec::new(),
            }));

            self.geohash_index
                .insert(&waypoint.borrow().geohash, Some(waypoint.clone()));

            self.waypoints.push(waypoint);
        }
    }

    fn search_geohash(&self, geohash: &str) -> Vec<Rc<RefCell<Waypoint>>> {
        self.geohash_index.get_all_with_prefix(geohash)
    }

    fn find_knn_naive(&self, target: Rc<RefCell<Waypoint>>, k: usize) -> Vec<Connection> {
        let mut nearest_neighbors: Vec<Connection> = Vec::new();

        for neighbor in &self.waypoints {
            if target != neighbor.clone() {
                nearest_neighbors.push(Connection {
                    distance: target.borrow().distance_to(neighbor.clone()),
                    waypoint: neighbor.clone(),
                })
            }
        }

        nearest_neighbors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        nearest_neighbors.truncate(k);
        nearest_neighbors
    }

    fn find_knn_geohash(&self, target: Rc<RefCell<Waypoint>>, k: usize) -> Vec<Connection> {
        let mut min_heap: BinaryHeap<Connection> = BinaryHeap::new();
        let mut geohash_to_search = target.borrow().geohash.clone();
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(target.borrow().label.clone());

        while min_heap.len() < k {
            // Search the current geohash cell for points
            for neighbor in self.search_geohash(&geohash_to_search) {
                // println!("Searching {}", &geohash_to_search);
                if visited.insert(neighbor.borrow().label.clone()) {
                    min_heap.push(Connection {
                        distance: target.borrow().distance_to(neighbor.clone()),
                        waypoint: neighbor,
                    })
                }
            }

            // Search the adjacent geohash cells for points that might be closer
            for adjacent_cell in geohash::get_surrounding_cells(&geohash_to_search) {
                // println!("Searching {}", adjacent_cell);
                for neighbor in self.search_geohash(&adjacent_cell) {
                    if visited.insert(neighbor.borrow().label.clone()) {
                        min_heap.push(Connection {
                            distance: target.borrow().distance_to(neighbor.clone()),
                            waypoint: neighbor,
                        })
                    }
                }
            }

            // If we still don't have k neighbors, remove a level of precision and repeat
            geohash_to_search.pop();
        }

        // Convert binary heap to vector, truncate to nearest k elements, and return
        let mut nearest_neighbors = min_heap.into_sorted_vec();
        nearest_neighbors.dedup();
        nearest_neighbors.truncate(k);
        nearest_neighbors
    }

    fn assign_connections(&self, amt: usize) {
        for waypoint in &self.waypoints {
            let connections = self.find_knn_geohash(waypoint.clone(), amt);
            waypoint.borrow_mut().connections.extend(connections);
        }
    }
}

fn main() {
    let mut dataset = Dataset::new("Bob".to_string());
    dataset.generate_waypoints(10000);

    let aaa = dataset.waypoints[0].clone();
    let k = 7;

    println!(
        "Point {} at {:.2}, {:.2} - {}",
        aaa.borrow().label,
        aaa.borrow().lat,
        aaa.borrow().lon,
        aaa.borrow().geohash
    );

    println!();

    let start_time_1 = Instant::now();
    let nn_naive = dataset.find_knn_naive(aaa.clone(), k);
    let end_time_1 = Instant::now();
    println!("Nearest neighbors found by naive search");
    for connection in nn_naive {
        println!(
            "{} - {}, {:.2}km",
            connection.waypoint.borrow().label,
            connection.waypoint.borrow().geohash,
            connection.distance
        );
    }
    println!(
        "Search completed in {:?}",
        end_time_1.duration_since(start_time_1)
    );

    println!();

    let start_time_2 = Instant::now();
    let nn_geohash = dataset.find_knn_geohash(aaa.clone(), k);
    let end_time_2 = Instant::now();
    println!("Nearest neighbors found by geohash search:");
    for connection in nn_geohash {
        println!(
            "{} - {}, {:.2}km",
            connection.waypoint.borrow().label,
            connection.waypoint.borrow().geohash,
            connection.distance
        );
    }
    println!(
        "Search completed in {:?}",
        end_time_2.duration_since(start_time_2)
    );
}
