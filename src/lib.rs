#![allow(dead_code, non_snake_case, unused_variables)]
use rand::Rng;

const EARTH_RADIUS: f32 = 6378.137; // Earth's radius in kilometers. Used in calculating distance between waypoints

#[derive(Debug, Clone)]
struct Waypoint<'a> {
    lat: f32,                           // Latitude; can range from -90 to 90
    lon: f32,                           // Longitude; can range from -180 to 180
    label: String,                      // Three-character label from AAA to ZZZ
    connections: Vec<&'a Waypoint<'a>>, // Vector of references to connected waypoints
}

struct Dataset<'a> {
    name: String,
    waypoints: Vec<Waypoint<'a>>,
}

#[derive(Debug, Clone)]
struct KDTreeNode<'a> {
    waypoint: &'a Waypoint<'a>,
    left: Option<Box<KDTreeNode<'a>>>,
    right: Option<Box<KDTreeNode<'a>>>,
}

struct KDTree<'a> {
    root: Option<Box<KDTreeNode<'a>>>,
}

impl<'a> KDTreeNode<'a> {
    fn display(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!(
            "{}Waypoint ({:.6}, {:.6})",
            indent, self.waypoint.lat, self.waypoint.lon
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

impl<'a> KDTree<'a> {
    fn new(dataset: &'a Dataset<'a>) -> Self {
        let mut waypoints: Vec<&'a Waypoint<'a>> = dataset.waypoints.iter().collect();
        let root = KDTree::build_kd_tree(&mut waypoints, 0);
        KDTree { root }
    }

    fn build_kd_tree(
        waypoints: &mut [&'a Waypoint<'a>],
        depth: usize,
    ) -> Option<Box<KDTreeNode<'a>>> {
        if waypoints.is_empty() {
            return None;
        }

        let dimension = depth % 2;

        waypoints.sort_by(|a, b| {
            if dimension == 0 {
                a.lat.partial_cmp(&b.lat).unwrap()
            } else {
                a.lon.partial_cmp(&b.lon).unwrap()
            }
        });

        let median_index = waypoints.len() / 2;

        let median = waypoints[median_index];

        Some(Box::new(KDTreeNode {
            waypoint: median,
            left: KDTree::build_kd_tree(&mut waypoints[..median_index], depth + 1),
            right: KDTree::build_kd_tree(&mut waypoints[median_index + 1..], depth + 1),
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

impl<'a> Waypoint<'a> {
    fn new(label: String) -> Self {
        let mut rng = rand::thread_rng();

        Waypoint {
            lat: rng.gen_range(-90.0..=90.0),
            lon: rng.gen_range(-180.0..=180.0),
            label,
            connections: Vec::new(),
        }
    }

    // Use the Haversine formula to find the distance between self and another waypoint (in km)
    fn distance_to(&self, dest: &Waypoint) -> f32 {
        let lat1 = self.lat.to_radians();
        let lat2 = dest.lat.to_radians();

        let dlat = lat2 - lat1;
        let dlon = dest.lon.to_radians() - self.lon.to_radians();

        let a = (dlat / 2.0).sin().powi(2) + (dlon / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS * c
    }

    fn print_DD(&self) {
        println!(
            "Waypoint {} is located at {:.6}, {:.6}",
            self.label, self.lat, self.lon
        )
    }

    fn print_DMS(&self) {
        let lat = self.lat.abs(); // Convert (-) values to (+) for cleaner code; sign only relevant in determining direction
        let lat_degrees = lat.floor(); // The whole number portion of the value equals degrees
        let lat_minutes = (lat - lat_degrees) * 60.0; // The decimal portion of the value, times 60, equals minutes
        let lat_seconds = (lat_minutes - lat_minutes.floor()) * 60.0; // The decimal portion of minutes, times 60, equals seconds
        let lat_direction = if self.lat >= 0.0 { 'N' } else { 'S' }; // Assign the cardinal direction based on sign

        let lon = self.lon.abs();
        let lon_degrees = lon.floor();
        let lon_minutes = (lon - lon_degrees) * 60.0;
        let lon_seconds = (lon_minutes - lon_minutes.floor()) * 60.0;
        let lon_direction = if self.lon >= 0.0 { 'E' } else { 'W' };

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
}

impl<'a> Dataset<'a> {
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

    fn generate_waypoints(amt: usize) -> Vec<Waypoint<'a>> {
        let mut waypoints = Vec::with_capacity(amt);

        for i in 1..=amt {
            let label = Dataset::generate_waypoint_label(i);
            let waypoint = Waypoint::new(label);
            waypoints.push(waypoint);
        }

        waypoints
    }

    // Builds a k-d tree from the dataset and then performs nearest-neighbor calculations using the tree.
    // Time complexity is O(N * log N) to build plus O(N * log N) to query all points. Overall O(N * log N).
    fn assign_connections_kdtree(&mut self, x: usize) {
        let tree = KDTree::new(&self);

        for i in 0..self.waypoints.len() {
            let waypoint = &self.waypoints[i];
        }
    }
}
