#![allow(dead_code, non_snake_case, unused_variables)]

mod geohash;

use rand::Rng;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Waypoint {
    lat: f32,
    lon: f32,
    label: String,
    geohash: String,
    connections: Vec<Connection>,
}

#[derive(Debug, Clone)]
struct Connection {
    waypoint: Rc<RefCell<Waypoint>>,
    distance: f32,
}

struct Dataset {
    name: String,
    waypoints: Vec<Rc<RefCell<Waypoint>>>,
    geohash_index: HashMap<String, Rc<RefCell<Waypoint>>>,
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

impl Dataset {
    fn new(name: String) -> Self {
        Dataset {
            name,
            waypoints: Vec::new(),
            geohash_index: HashMap::new(),
        }
    }

    fn generate_waypoints(&mut self, amt: usize) {
        let mut rng = rand::thread_rng();

        for i in 0..amt {
            let label = Waypoint::generate_label(i);
            let lat = rng.gen_range(-90.0..=90.0);
            let lon = rng.gen_range(-180.0..=180.0);

            let waypoint = Rc::new(RefCell::new(Waypoint {
                label,
                lat,
                lon,
                geohash: geohash::encode(lat, lon, 8), // Default precision is '8' (+/- ~0.02km)
                connections: Vec::new(),
            }));

            self.geohash_index
                .insert(waypoint.borrow().geohash.clone(), waypoint.clone());
            self.waypoints.push(waypoint);
        }
    }

    /// Naive method of assigning connections to waypoints. Cycles through every waypoint in the dataset, checks distance
    /// to every other waypoint, sorts distances, and assigns closest 'amt' as connections. O(N^2 * Log N) time complexity.
    fn assign_connections_naive(&self, amt: usize) {
        // For each waypoint in the dataset...
        for i in 0..self.waypoints.len() {
            let mut source_waypoint = self.waypoints[i].borrow_mut();
            let mut connections: Vec<Connection> = Vec::with_capacity(self.waypoints.len() - 1);

            // Determine the distance to every other waypoint in the dataset and store in the 'all_connections' vec
            for j in 0..self.waypoints.len() {
                if i != j {
                    let target_waypoint = &self.waypoints[j];
                    connections.push(Connection {
                        waypoint: target_waypoint.clone(),
                        distance: source_waypoint.distance_to(target_waypoint.clone()),
                    })
                }
            }

            // Sort connections from nearest to furthest and assign the nearest 'amt' as connections
            connections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
            for j in 0..amt {
                source_waypoint.connections.push(connections[j].clone())
            }
        }
    }

    /// Finds and assigns connections to waypoints more intelligently using geohashes. Cycles through every point in the
    /// dataset and uses the dataset's geohash_index HashMap to find waypoints in parent / neighboring geohash partitions.
    fn assign_connections_geohash(&self) {
        // For each waypoint in the dataset...
        for i in 0..self.waypoints.len() {}
    }
}

fn main() {
    // use std::time::Instant;
    // let now = Instant::now();
    // let elapsed = now.elapsed();

    let mut dataset = Dataset::new("Bob".to_string());
    dataset.generate_waypoints(10000);

    let mut keys: Vec<&String> = dataset.geohash_index.keys().collect();
    keys.sort();

    for key in keys {
        if key.starts_with("zz") {
            println!("{}", key);
        }
    }
}
