mod geohash;
mod pseudo_random;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a geospatial waypoint with latitude, longitude, a label, geohash, and connections.
#[derive(Debug, Clone)]
pub struct Waypoint {
    pub lat: f32,
    pub lon: f32,
    pub label: String,
    pub geohash: String,
    pub connections: Vec<Connection>,
}

/// Represents a connection between waypoints with a distance and a waypoint index.
#[derive(Debug, Clone)]
pub struct Connection {
    pub distance: f32,
    pub waypoint_index: usize,
}

/// Represents a Trie data structure for indexing waypoints based on geohash prefixes.
pub struct Trie {
    children: HashMap<char, Trie>,
    waypoint_index: Option<usize>,
}

/// Represents a node used in the A* algorithm for pathfinding, with an F score and waypoint index.
struct AStarNode {
    f_score: f32,
    waypoint_index: usize,
}

/// Represents a dataset of waypoints and geospatial data.
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
    /// Calculates and returns the great-circle distance between this waypoint
    /// and a target waypoint in kilometers using the Haversine formula.
    ///
    /// # Arguments
    ///
    /// * `target` - A reference to the target `Waypoint` to get the distance to.
    ///
    /// # Returns
    ///
    /// The great-circle distance in kilometers between this waypoint and the target waypoint.
    ///
    /// # Example
    ///
    /// ```
    /// let waypoint1 = Waypoint { lat: 37.7749, lon: -122.4194, label: String::from("A"), geohash: String::from("u4pruydq"), connections: Vec::new() };
    /// let waypoint2 = Waypoint { lat: 34.0522, lon: -118.2437, label: String::from("B"), geohash: String::from("9q5x9p6y"), connections: Vec::new() };
    ///
    /// let distance_km = waypoint1.get_distance_to(&waypoint2);
    ///
    /// println!("Distance between waypoints: {} km", distance_km);
    /// ```
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

    /// Generates sequential labels to act as unique identifiers based on an
    /// integer value and a total count. Will generate labels 'A' through 'Z'
    /// first, then 'AA', 'AB', etc. through 'ZZ', then 'AAA'...
    ///
    /// # Arguments
    ///
    /// * `n` - The integer value that represents the waypoint's index in the dataset
    /// * `total_amt` - The total amount of waypoints for which labels are being generated.
    ///
    /// # Returns
    ///
    /// A string representing the generated label.
    ///
    /// # Example
    ///
    /// ```
    /// let n1 = 25;
    /// let n2 = 27
    /// let total_amt = 32;
    ///
    /// let label_1 = generate_label(n1, total_amt);
    /// let label_2 = generate_label(n2, total_amt);
    ///
    /// println!("{}", label_1); // Example output: 'Z'
    /// println!("{}", label_2); // Example output: 'AB'
    /// ```
    pub fn generate_label(mut n: usize, mut total_amt: usize) -> String {
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

    /// Converts the latitude and longitude coordinates of a waypoint into a
    /// more readable Degree-Minute-Second (DMS) format string.
    ///
    /// # Returns
    ///
    /// A string representing the coordinates in DMS format.
    /// For example, "37°45'30.00\"N, 122°25'12.00\"W".
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
    /// Initializes a new Trie node with no associated waypoint index and an empty set of child nodes.
    ///
    /// # Returns
    ///
    /// - `Trie`: A new Trie node for geohash indexing.
    fn new() -> Self {
        Trie {
            waypoint_index: None,
            children: HashMap::new(),
        }
    }

    /// Inserts a geohash and the corresponding waypoint index into the Trie. It
    /// traverses the Trie structure, creating new nodes as needed to represent the geohash.
    ///
    /// # Parameters
    ///
    /// - `geohash`: A reference to the geohash to insert.
    /// - `waypoint_index`: The index of the waypoint associated with the geohash.
    fn insert(&mut self, geohash: &str, waypoint_index: usize) {
        let mut current_node = self;

        for c in geohash.chars() {
            current_node = current_node.children.entry(c).or_insert(Trie::new());
        }

        current_node.waypoint_index = Some(waypoint_index);
    }

    /// Searches the Trie for waypoint indices whose geohash prefixes match the specified
    /// prefix. It returns a vector of matching waypoint indices; empty if none.
    ///
    /// # Parameters
    ///
    /// - `prefix`: A reference to the geohash prefix to search for.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>`: A vector containing waypoint indices matching the geohash prefix.
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

    /// Recursively traverses the Trie nodes, collecting waypoint indices from
    /// nodes that have associated waypoints. It is used internally to implement
    /// `get_all_with_prefix`.
    ///
    /// # Parameters
    ///
    /// - `node`: A reference to the Trie node to start collecting from.
    /// - `waypoints`: A mutable reference to the vector where waypoint indices are collected.
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
    /// Initializes a new `Dataset` struct with empty waypoint and geohash index containers.
    /// Can store and manage geospatial data, such as waypoints and their connections.
    ///
    /// # Returns
    ///
    /// - `Dataset`: A new, empty dataset instance.
    pub fn new() -> Self {
        Dataset {
            waypoints: Vec::new(),
            geohash_index: Trie::new(),
        }
    }

    /// Randomly generates waypoints with random latitude and longitude values within the
    /// specified range and assigns unique labels to each waypoint. It also calculates the
    /// geohash for each waypoint and inserts it into a geohash index for quick spatial
    /// lookups.
    ///
    /// # Parameters
    ///
    /// - `amt`: The number of waypoints to generate and add to the dataset.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10);
    /// ```
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

    /// Creates a new waypoint with the specified latitude and longitude inserts it into the dataset.
    ///
    /// # Arguments
    ///
    /// * `lat` - The latitude coordinate of the new waypoint in degrees.
    /// * `lon` - The longitude coordinate of the new waypoint in degrees.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.add_new_waypoint(37.7749, -122.4194);
    /// ```
    pub fn add_new_waypoint(&mut self, lat: f32, lon: f32) -> usize {
        let geohash = geohash::encode(lat, lon, 8);
        let index = self.waypoints.len();

        let waypoint = Waypoint {
            label: Waypoint::generate_label(index, index + 1),
            lat,
            lon,
            geohash: geohash.clone(),
            connections: Vec::new(),
        };

        self.geohash_index.insert(&geohash, index);
        self.waypoints.push(waypoint);

        // If the dataset has already established connections, then assign some
        // connections to the new waypoint
        if self.waypoints[0].connections.len() > 0 {
            let new_connections =
                self.get_knn_geohash(&self.waypoints[index], self.waypoints[0].connections.len());

            for connection in &new_connections {
                self.waypoints[connection.waypoint_index]
                    .connections
                    .push(Connection {
                        waypoint_index: index,
                        distance: connection.distance,
                    })
            }

            self.waypoints[index].connections.extend(new_connections);
        }

        index
    }

    /// Searches for a waypoint with a matching label within the dataset and
    /// returns `Some(index)` if found.
    ///
    /// # Parameters
    ///
    /// - `waypoint`: A reference to the waypoint whose index should be retrieved.
    ///
    /// # Returns
    ///
    /// - `Option<usize>`: The index of the waypoint within the dataset if found.
    /// - `None`: If the waypoint isn't found, it returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10);
    /// let waypoint_a = dataset.waypoints[0];
    ///
    /// match dataset.get_waypoint_index(&waypoint_a) {
    ///     Some(index) => println!("Index of waypoint: {}", index),
    ///     None => println!("Waypoint not found in the dataset."),
    /// }
    /// ```
    fn get_waypoint_index(&self, waypoint: &Waypoint) -> Option<usize> {
        self.waypoints
            .iter()
            .position(|x| x.label == waypoint.label)
    }

    /// Queries the geohash index to retrieve all waypoint indices that share a common
    /// geohash prefix with the specified geohash. It is used to find waypoints within the same
    /// geohash cell or adjacent cells.
    ///
    /// # Parameters
    ///
    /// - `geohash`: A reference to the geohash prefix to search for.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>`: A vector containing the indices of waypoints matching the geohash prefix.
    /// Empty if none.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10);
    ///
    /// let geohash_prefix = "9t37";
    /// let matching_indices = dataset.search_geohash(geohash_prefix);
    ///
    /// for index in matching_indices {
    ///     println!("Matching Waypoint: {:?}", dataset.waypoints[index].label);
    /// }
    /// ```
    fn search_geohash(&self, geohash: &str) -> Vec<usize> {
        self.geohash_index.get_all_with_prefix(geohash)
    }

    /// Calculates the K-nearest neighbors to a specified waypoint within the dataset
    /// using a naive approach that iterates through all waypoints in the dataset,
    /// calculates its distance to all other waypoints in the dataset, sorts them,
    /// and returns the nearest K.
    ///
    /// # Parameters
    ///
    /// - `target`: A reference to the waypoint for which K-nearest neighbors are to be found.
    /// - `k`: The number of nearest neighbors to retrieve.
    ///
    /// # Returns
    ///
    /// - `Vec<Connection>`: A vector containing the K-nearest neighbor connections, sorted by distance.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10);
    ///
    /// let waypoint_a = &dataset.waypoints[0];
    /// let k = 3;
    /// let nearest_neighbors = dataset.get_knn_naive(waypoint_a, k);
    ///
    /// for neighbor in nearest_neighbors {
    ///     println!(
    ///         "Neighbor: {:?} - {:.2}km",
    ///         dataset.waypoints[neighbor.waypoint_index].label,
    ///         neighbor.distance
    ///     );
    /// }
    /// ```
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

    /// Calculates the K-nearest neighbors to a specified waypoint within the dataset based on
    /// geohash proximity. Uses a priority queue (binary heap) to efficiently find the nearest
    /// neighbors. It also considers neighboring geohash cells to handle edge cases.
    ///
    /// # Parameters
    ///
    /// - `waypoint`: A reference to the waypoint for which K-nearest neighbors are to be found.
    /// - `k`: The number of nearest neighbors to retrieve.
    ///
    /// # Returns
    ///
    /// - `Vec<Connection>`: A vector containing the K-nearest neighbor connections, sorted by distance.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10);
    ///
    /// let waypoint_a = &dataset.waypoints[0];
    /// let k = 3;
    /// let nearest_neighbors = dataset.get_knn_geohash(waypoint_a, k);
    ///
    /// for neighbor in nearest_neighbors {
    ///     println!(
    ///         "Neighbor: {:?} - {:.2}km",
    ///         dataset.waypoints[neighbor.waypoint_index].label,
    ///         neighbor.distance
    ///     );
    /// }
    /// ```
    pub fn get_knn_geohash(&self, waypoint: &Waypoint, k: usize) -> Vec<Connection> {
        let mut geohash_to_search = waypoint.geohash.clone();
        let mut min_heap: BinaryHeap<Connection> = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(self.get_waypoint_index(waypoint).unwrap());

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

    /// Iterates through each waypoint in the dataset and assigns connections to it based on
    /// K-nearest neighbors, calculated using the `get_knn_geohash` method. Populates the
    /// `connections` field of each waypoint with the calculated connections.
    ///
    /// # Parameters
    ///
    /// - `amt`: The number of nearest neighbors (K) to consider for each waypoint.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10);
    ///
    /// let k = 3;
    /// dataset.assign_connections(k);
    /// ```
    pub fn assign_all_connections_geohash(&mut self, amt: usize) {
        for i in 0..self.waypoints.len() {
            let connections = self.get_knn_geohash(&self.waypoints[i], amt);
            self.waypoints[i].connections.extend(connections);
        }
    }

    /// Iterates through each waypoint in the dataset and assigns connections to it based on
    /// K-nearest neighbors, calculated using the `get_knn_naive` method. Populates the
    /// `connections` field of each waypoint with the calculated connections.
    ///
    /// # Parameters
    ///
    /// - `amt`: The number of nearest neighbors (K) to consider for each waypoint.
    ///
    /// # Example
    ///
    /// ```
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10);
    ///
    /// let k = 3;
    /// dataset.assign_connections(k);
    /// ```
    pub fn assign_all_connections_naive(&mut self, amt: usize) {
        for i in 0..self.waypoints.len() {
            let connections = self.get_knn_naive(&self.waypoints[i], amt);
            self.waypoints[i].connections.extend(connections);
        }
    }

    /// Calculates the shortest route between a starting waypoint and a goal waypoint
    /// using the A* (A-star) algorithm.
    ///
    /// # Arguments
    ///
    /// - `start`: A reference to the starting waypoint.
    /// - `goal`: A reference to the goal waypoint.
    ///
    /// # Returns
    ///
    /// - `Some(Vec<usize>)`: If a valid route is found, it returns a vector of waypoint indices
    ///   representing the shortest path from the `start` waypoint to the `goal` waypoint. The
    ///   vector contains the indices of waypoints in the dataset's 'waypoints' field
    ///   in the order they should be visited.
    /// - `None`: If no valid route is found, it returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a dataset with 10,000 waypoints, 5 connections each
    /// let mut dataset = Dataset::new();
    /// dataset.generate_waypoints(10000);
    /// dataset.assign_connections(5);
    ///
    /// let start_waypoint = &dataset.waypoints[0];
    /// let goal_waypoint = &dataset.waypoints[3];
    ///
    /// match dataset.get_shortest_route(start_waypoint, goal_waypoint) {
    ///     Some(route) => {
    ///        for index in route {
    ///            print!("{}, ", dataset.waypoints[index].label);
    ///         }
    ///     }
    ///     None => {println!("No valid route found.")}
    /// }
    /// ```
    pub fn get_shortest_route(&self, start: &Waypoint, goal: &Waypoint) -> Option<Vec<usize>> {
        let mut open_set: BinaryHeap<AStarNode> = BinaryHeap::new();
        let mut came_from: HashMap<usize, usize> = HashMap::new();
        let mut g_scores: HashMap<usize, f32> = HashMap::new();
        let start_index = self.get_waypoint_index(&start).unwrap();

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
