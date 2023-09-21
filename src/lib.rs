#![allow(dead_code, non_snake_case, unused_variables)]
use rand::Rng;
use std::rc::Rc;

const EARTH_RADIUS: f32 = 6378.137; // Earth's radius in kilometers. Used in calculating distance between waypoints

#[derive(Debug, Clone)]
struct Waypoint {
    lat: f32,      // Latitude; can range from -90 to 90
    lon: f32,      // Longitude; can range from -180 to 180
    label: String, // Three-character label from AAA to ZZZ
    connections: Vec<Connection>,
}

#[derive(Debug, Clone)]
struct Connection {
    waypoint: Rc<Waypoint>,
    distance: f32,
}

struct Dataset {
    name: String,
    waypoints: Vec<Rc<Waypoint>>,
}

#[derive(Debug, Clone)]
struct KDTreeNode {
    waypoint: Rc<Waypoint>,
    left: Option<Box<KDTreeNode>>,
    right: Option<Box<KDTreeNode>>,
}

struct KDTree {
    root: Option<Box<KDTreeNode>>,
}

impl Waypoint {
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

    fn convert_to_DMS(coord: f32, direction: char) -> String {
        let abs_coord = coord.abs();
        let degrees = abs_coord.floor();
        let minutes = (abs_coord - degrees) * 60.0;
        let seconds = (minutes - minutes.floor()) * 60.0;

        format!(
            "{}°{}'{:.2}\"{}",
            degrees,
            minutes.floor(),
            seconds,
            direction
        )
    }

    fn print_DMS(&self) {
        let lat_direction = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_direction = if self.lon >= 0.0 { 'E' } else { 'W' };

        println!(
            "Waypoint {} is located at {}, {}",
            self.label,
            Waypoint::convert_to_DMS(self.lat, lat_direction),
            Waypoint::convert_to_DMS(self.lon, lon_direction)
        );
    }

    fn print(&self) {
        println!("Waypoint {}: {:.6}, {:.6}", self.label, self.lat, self.lon)
    }
}

impl KDTreeNode {
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

impl KDTree {
    fn new(dataset: &Dataset) -> Self {
        let mut waypoints: Vec<Rc<Waypoint>> = dataset.waypoints.iter().cloned().collect();
        let root = KDTree::build_kd_tree(&mut waypoints, 0);
        KDTree { root }
    }

    fn build_kd_tree(waypoints: &mut [Rc<Waypoint>], depth: usize) -> Option<Box<KDTreeNode>> {
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
        let median = waypoints[median_index].clone();

        Some(Box::new(KDTreeNode {
            waypoint: median,
            left: KDTree::build_kd_tree(&mut waypoints[..median_index], depth + 1),
            right: KDTree::build_kd_tree(&mut waypoints[median_index + 1..], depth + 1),
        }))
    }

    fn print(&self) {
        if let Some(root) = &self.root {
            root.display(0);
        } else {
            println!("KD Tree is empty.");
        }
    }
}

impl Dataset {
    fn new(name: String, size: usize) -> Self {
        let waypoints = Dataset::generate_waypoints(size);

        Dataset { name, waypoints }
    }

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
    fn generate_waypoints(amt: usize) -> Vec<Rc<Waypoint>> {
        let mut waypoints = Vec::with_capacity(amt);

        for i in 1..=amt {
            let label = Dataset::generate_waypoint_label(i);
            let waypoint = Rc::new(Waypoint::new(label));
            waypoints.push(waypoint);
        }

        waypoints
    }
}
