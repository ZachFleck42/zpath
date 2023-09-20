#![allow(dead_code, non_snake_case, unused_variables)]
use rand::Rng;

const EARTH_RADIUS: f32 = 6378.137; // Earth's radius in kilometers. Used in calculating distance between waypoints

#[derive(Debug, Clone)]
struct Connection {
    label: String,
    distance: f32,
}

#[derive(Debug, Clone)]
struct Waypoint {
    x: f32, // Latitude; can range from -90 to 90
    y: f32, // Longitude; can range from -180 to 180
    label: String,
    connections: Vec<Connection>,
}

struct Dataset {
    name: String,
    waypoints: Vec<Waypoint>,
}

#[derive(Debug, Clone)]
struct Node {
    waypoint: Waypoint,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct KDTree {
    root: Option<Box<Node>>,
}

impl Node {
    fn display(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!(
            "{}Waypoint ({:.6}, {:.6})",
            indent, self.waypoint.x, self.waypoint.y
        );

        if let Some(left) = &self.left {
            println!("{}Left:", indent);
            left.display(depth + 1);
        }

        if let Some(right) = &self.right {
            println!("{}Right:", indent);
            right.display(depth + 1);
        }
    }
}

impl KDTree {
    fn new(dataset: &Dataset) -> Self {
        let mut waypoints = dataset.waypoints.clone();
        let root = KDTree::build_kd_tree(&mut waypoints, 0);
        KDTree { root }
    }

    fn build_kd_tree(waypoints: &mut Vec<Waypoint>, depth: usize) -> Option<Box<Node>> {
        if waypoints.is_empty() {
            return None;
        }

        let dimension = depth % 2;

        waypoints.sort_by(|a, b| {
            if dimension == 0 {
                a.x.partial_cmp(&b.x).unwrap()
            } else {
                a.y.partial_cmp(&b.y).unwrap()
            }
        });

        let median_index = waypoints.len() / 2;

        let median = waypoints.remove(median_index);

        Some(Box::new(Node {
            waypoint: median,
            left: KDTree::build_kd_tree(&mut waypoints[..median_index].to_vec(), depth + 1),
            right: KDTree::build_kd_tree(&mut waypoints[median_index..].to_vec(), depth + 1),
        }))
    }

    fn display(&self) {
        if let Some(root) = &self.root {
            root.display(0);
        } else {
            println!("KD Tree is empty.");
        }
    }
}

impl Waypoint {
    fn new(label: String) -> Self {
        let mut rng = rand::thread_rng();

        Waypoint {
            x: rng.gen_range(-90.0..=90.0),
            y: rng.gen_range(-180.0..=180.0),
            label,
            connections: Vec::new(),
        }
    }

    // Use the Haversine formula to find the distance between self and another waypoint (in km)
    fn distance_to(&self, dest: &Waypoint) -> f32 {
        let lat1 = self.x.to_radians();
        let lat2 = dest.x.to_radians();

        let dlat = lat2 - lat1;
        let dlon = dest.y.to_radians() - self.y.to_radians();

        let a = (dlat / 2.0).sin().powi(2) + (dlon / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS * c
    }

    fn print_DD(&self) {
        println!(
            "Waypoint {} is located at {:.6}, {:.6}",
            self.label, self.x, self.y
        )
    }

    fn print_DMS(&self) {
        let lat = self.x.abs(); // Convert (-) values to (+) for cleaner code; sign only relevant in determining direction
        let lat_degrees = lat.floor(); // The whole number portion of the value equals degrees
        let lat_minutes = (lat - lat_degrees) * 60.0; // The decimal portion of the value, times 60, equals minutes
        let lat_seconds = (lat_minutes - lat_minutes.floor()) * 60.0; // The decimal portion of minutes, times 60, equals seconds
        let lat_direction = if self.x >= 0.0 { 'N' } else { 'S' }; // Assign the cardinal direction based on sign

        let lon = self.y.abs();
        let lon_degrees = lon.floor();
        let lon_minutes = (lon - lon_degrees) * 60.0;
        let lon_seconds = (lon_minutes - lon_minutes.floor()) * 60.0;
        let lon_direction = if self.y >= 0.0 { 'E' } else { 'W' };

        println!(
            "Waypoint {} is located at {}°{}'{:.2}\"{}, {}°{}'{:.2}\"{}",
            self.label,
            lat_degrees,
            lat_minutes.floor(),
            lat_seconds,
            lat_direction,
            lon_degrees,
            lon_minutes.floor(),
            lon_seconds,
            lon_direction
        )
    }

    fn print_connections(&self) {
        print!("Connections to waypoint {} are:", self.label);
        for connection in &self.connections {
            println!("{}: {:.2}km", connection.label, connection.distance);
        }
        println!();
    }
}

impl Dataset {
    fn new(name: String, size: usize) -> Self {
        let waypoints = Dataset::generate_waypoints(size);

        Dataset { name, waypoints }
    }

    // Generates sequential labels from AAA, AAB, AAC... to ZZZ
    fn generate_waypoint_label(n: usize) -> String {
        let mut label = String::new();
        let mut num = n - 1;

        for _ in 0..3 {
            let remainder = num % 26;
            let char_value = (remainder as u8 + b'A') as char;
            label.push(char_value);
            num /= 26;
        }

        label.chars().rev().collect()
    }

    fn generate_waypoints(amt: usize) -> Vec<Waypoint> {
        let mut waypoints = Vec::with_capacity(amt);

        for i in 1..=amt {
            let label = Dataset::generate_waypoint_label(i);
            let waypoint = Waypoint::new(label);
            waypoints.push(waypoint);
        }

        waypoints
    }

    // Builds a k-d tree from the dataset and then performs nearest-neighbor calculations using the tree.
    // Time complexity of O(N * log N) to build plus O(N * log N) to query. Overall O(N * log N).
    fn assign_connections_kdtree(&mut self, x: usize) {
        let tree = KDTree::new(&self);

        for i in 0..self.waypoints.len() {
            let waypoint = &self.waypoints[i];
        }
    }

    // Checks every waypoint's distance to every other waypoint in the dataset, sorts by distance,
    // and assigns the nearest x as connections. Time complexity of O(N^2 * log N).
    fn assign_connections_naive(&mut self, x: usize) {
        // For each waypoint in the dataset...
        for i in 0..self.waypoints.len() {
            let waypoint_1 = &self.waypoints[i];
            let mut all_connections: Vec<Connection> = Vec::with_capacity(self.waypoints.len() - 1);

            // Determine the distance to every other waypoint in the dataset and store in the 'all_connections' vec
            for j in 0..self.waypoints.len() {
                if i != j {
                    let waypoint_2 = &self.waypoints[j];
                    all_connections.push(Connection {
                        label: waypoint_2.label.clone(),
                        distance: waypoint_1.distance_to(waypoint_2),
                    });
                }
            }

            // Sort connections from nearest to farthest and assign the nearest x as connections
            all_connections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
            for j in 0..x {
                self.waypoints[i].connections.push(Connection {
                    label: all_connections[j].label.clone(),
                    distance: all_connections[j].distance,
                })
            }
        }
    }
}

fn main() {
    let dataset_size = 1000;
    let dataset_name: String = "Bob".to_string();
    let dataset = Dataset::new(dataset_name, dataset_size);
    let kdtree = KDTree::new(&dataset);
    kdtree.display();
}
