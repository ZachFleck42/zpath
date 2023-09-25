mod geohash;
mod pseudo_random;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Waypoint {
    pub lat: f32,
    pub lon: f32,
    pub label: String,
    pub geohash: String,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub distance: f32,
    pub waypoint_index: usize,
}

pub struct Trie {
    children: HashMap<char, Trie>,
    waypoint_index: Option<usize>,
}

struct AStarNode {
    f_score: f32,
    waypoint_index: usize,
}

pub struct Dataset {
    pub waypoints: Vec<Waypoint>,
    pub geohash_index: Trie,
}

impl PartialEq for Waypoint {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Waypoint {}

impl Waypoint {
    /// Implements the Haversine formula to find the distance between self and another waypoint (in km)
    pub fn get_distance_to(&self, target: &Waypoint) -> f32 {
        const EARTH_RADIUS: f32 = 6378.137;

        let lat1 = self.lat.to_radians();
        let lat2 = target.lat.to_radians();

        let dlat = lat2 - lat1;
        let dlon = target.lon.to_radians() - self.lon.to_radians();

        let a = (dlat / 2.0).sin().powi(2) + (dlon / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS * c
    }

    /// Generates a sequential character label based on waypoint's position 'n' in a set of size 'total_amt'
    fn generate_label(mut n: usize, mut total_amt: usize) -> String {
        let mut label = String::new();
        let mut label_len = 1;

        while total_amt > 26 {
            total_amt /= 26;
            label_len += 1;
        }

        for _ in 0..label_len {
            let remainder = n % 26;
            let char_value = (remainder as u8 + b'A') as char;
            label.push(char_value);
            n /= 26;
        }

        label.chars().rev().collect()
    }

    /// Returns a String of the Waypoint's coordinates in Degrees/Minutes/Seconds (DMS) format
    pub fn get_dms(&self) -> String {
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

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.waypoint_index == other.waypoint_index
    }
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

impl Trie {
    fn new() -> Self {
        Trie {
            waypoint_index: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, geohash: &str, waypoint_index: usize) {
        let mut current_node = self;

        for c in geohash.chars() {
            current_node = current_node.children.entry(c).or_insert(Trie::new());
        }

        current_node.waypoint_index = Some(waypoint_index);
    }

    fn get_all_with_prefix(&self, prefix: &str) -> Vec<usize> {
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

    fn collect_waypoints_recursive(&self, node: &Trie, waypoints: &mut Vec<usize>) {
        if let Some(waypoint) = node.waypoint_index {
            waypoints.push(waypoint);
        }

        for child in node.children.values() {
            self.collect_waypoints_recursive(child, waypoints);
        }
    }
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.waypoint_index == other.waypoint_index
    }
}

impl Eq for AStarNode {}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl Dataset {
    pub fn new() -> Self {
        Dataset {
            waypoints: Vec::new(),
            geohash_index: Trie::new(),
        }
    }

    pub fn generate_waypoints(&mut self, amt: usize) {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let seed = since_epoch.as_secs() ^ since_epoch.subsec_nanos() as u64;

        let mut rng = pseudo_random::XorShiftRng::new(seed);
        // let mut rng = pseudo_random::LcgRng::new(seed);

        for i in 0..amt {
            let label = Waypoint::generate_label(i, amt);
            let lat = rng.random_f32_in_range(-90.0, 90.0);
            let lon = rng.random_f32_in_range(-180.0, 180.0);
            let geohash = geohash::encode(lat, lon, 8);

            let waypoint = Waypoint {
                label,
                lat,
                lon,
                geohash: geohash.clone(),
                connections: Vec::new(),
            };

            self.geohash_index.insert(&geohash, i);
            self.waypoints.push(waypoint);
        }
    }

    fn get_waypoint_index(&self, waypoint: &Waypoint) -> usize {
        self.waypoints
            .iter()
            .position(|x| x.label == waypoint.label)
            .unwrap()
    }

    fn search_geohash(&self, geohash: &str) -> Vec<usize> {
        self.geohash_index.get_all_with_prefix(geohash)
    }

    pub fn get_knn_naive(&self, target: &Waypoint, k: usize) -> Vec<Connection> {
        let mut nearest_neighbors: Vec<Connection> = Vec::new();

        for (i, neighbor) in self.waypoints.iter().enumerate() {
            if target.label != neighbor.label {
                nearest_neighbors.push(Connection {
                    distance: target.get_distance_to(neighbor),
                    waypoint_index: i,
                })
            }
        }

        nearest_neighbors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        nearest_neighbors.truncate(k);
        nearest_neighbors
    }

    pub fn get_knn_geohash(&self, waypoint: &Waypoint, k: usize) -> Vec<Connection> {
        let mut geohash_to_search = waypoint.geohash.clone();
        let mut min_heap: BinaryHeap<Connection> = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(self.get_waypoint_index(waypoint));

        while min_heap.len() < k {
            // Remove a level of precision and search the larger geohash cell for neighbors
            geohash_to_search.pop();

            for neighbor_index in self.search_geohash(&geohash_to_search) {
                if visited.insert(neighbor_index) {
                    min_heap.push(Connection {
                        distance: waypoint.get_distance_to(&self.waypoints[neighbor_index]),
                        waypoint_index: neighbor_index,
                    })
                }
            }
        }

        // k neighbors have been found, but check surrounding cells for edge cases
        for adjacent_cell in geohash::get_surrounding_cells(&geohash_to_search) {
            for neighbor_index in self.search_geohash(&adjacent_cell) {
                if visited.insert(neighbor_index) {
                    min_heap.push(Connection {
                        distance: waypoint.get_distance_to(&self.waypoints[neighbor_index]),
                        waypoint_index: neighbor_index,
                    })
                }
            }
        }

        // Convert binary heap to vector, truncate to nearest k elements, and return
        let mut nearest_neighbors = min_heap.into_sorted_vec();
        nearest_neighbors.truncate(k);
        nearest_neighbors
    }

    pub fn assign_connections(&mut self, amt: usize) {
        for i in 0..self.waypoints.len() {
            let connections = self.get_knn_geohash(&self.waypoints[i], amt);
            self.waypoints[i].connections.extend(connections);
        }
    }

    /// Finds the shortest path between two waypoints using the A* algorithm.
    pub fn get_shortest_route(&self, start: &Waypoint, goal: &Waypoint) -> Option<Vec<usize>> {
        let mut open_set: BinaryHeap<AStarNode> = BinaryHeap::new();
        let mut came_from: HashMap<usize, usize> = HashMap::new();
        let mut g_scores: HashMap<usize, f32> = HashMap::new();
        let start_index = self.get_waypoint_index(&start);

        // Initialize the open set and g_scores map with the starting point
        g_scores.insert(start_index, 0.0);
        open_set.push(AStarNode {
            f_score: 0.0,
            waypoint_index: start_index,
        });

        // While there are still routes to explore in the open set...
        while let Some(node) = open_set.pop() {
            let current_index = node.waypoint_index;
            let current_waypoint = &self.waypoints[current_index];

            // If the current waypoint is the goal waypoint...
            if current_waypoint == goal {
                let mut path = vec![current_index];
                let mut current = current_index;

                // Reconstruct the route by following the 'came_from' map and return it
                while let Some(&previous_index) = came_from.get(&current) {
                    path.push(previous_index);
                    current = previous_index;
                }
                path.reverse();
                return Some(path);
            }

            // Explore neighbors of the current waypoint
            for neighbor in &current_waypoint.connections {
                let neighbor_index = neighbor.waypoint_index;
                let g_score = g_scores[&current_index] + neighbor.distance;

                // If the neighbor has not been visited or a shorter path is found...
                if !g_scores.contains_key(&neighbor_index) || g_score < g_scores[&neighbor_index] {
                    // This is a better path to the neighbor
                    came_from.insert(neighbor_index, current_index);
                    g_scores.insert(neighbor_index, g_score);

                    // Add the neighbor to the open set for further exploration
                    let h_score = &self.waypoints[neighbor_index].get_distance_to(goal);
                    open_set.push(AStarNode {
                        f_score: g_score + h_score,
                        waypoint_index: neighbor_index,
                    });
                }
            }
        }

        None
    }
}
