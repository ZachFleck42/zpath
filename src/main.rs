#![allow(dead_code, non_snake_case, unused_variables)]

use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const EARTH_RADIUS: f32 = 6378.137;

#[derive(Debug, Clone)]
struct Waypoint {
    lat: f32,
    lon: f32,
    label: String,
    connections: Vec<Connection>,
}

#[derive(Debug, Clone)]
struct Connection {
    waypoint: Rc<RefCell<Waypoint>>,
    distance: f32,
}

#[derive(Debug, Clone)]
struct KDTreeNode {
    waypoint: Rc<RefCell<Waypoint>>,
    left: Option<Box<KDTreeNode>>,
    right: Option<Box<KDTreeNode>>,
}

struct KDTree {
    root: Option<Box<KDTreeNode>>,
}

struct Dataset {
    name: String,
    waypoints: Vec<Rc<RefCell<Waypoint>>>,
    kd_tree: Option<KDTree>,
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

    fn generate_label(n: usize) -> String {
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
            indent,
            self.waypoint.borrow().lat,
            self.waypoint.borrow().lon
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
    fn new(waypoints: &mut Vec<Rc<RefCell<Waypoint>>>) -> Self {
        let root = KDTree::build_kd_tree(waypoints, 0);
        KDTree { root }
    }

    fn build_kd_tree(
        waypoints: &mut [Rc<RefCell<Waypoint>>],
        depth: usize,
    ) -> Option<Box<KDTreeNode>> {
        if waypoints.is_empty() {
            return None;
        }

        let dimension = depth % 2;

        waypoints.sort_by(|a, b| {
            if dimension == 0 {
                a.borrow().lat.partial_cmp(&b.borrow().lat).unwrap()
            } else {
                a.borrow().lon.partial_cmp(&b.borrow().lon).unwrap()
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
            println!("k-d tree is empty.");
        }
    }
}

impl Dataset {
    fn new(name: String, size: usize) -> Self {
        let waypoints = Dataset::generate_waypoints(size);

        Dataset {
            name,
            waypoints,
            kd_tree: None,
        }
    }

    fn generate_waypoints(amt: usize) -> Vec<Rc<RefCell<Waypoint>>> {
        let mut waypoints = Vec::with_capacity(amt);

        for i in 1..=amt {
            let label = Waypoint::generate_label(i);
            let waypoint = Rc::new(RefCell::new(Waypoint::new(label)));
            waypoints.push(waypoint);
        }

        waypoints
    }

    fn generate_kd_tree(&mut self) {
        let mut cloned_waypoint_refs: Vec<Rc<RefCell<Waypoint>>> =
            self.waypoints.iter().cloned().collect();
        self.kd_tree = Some(KDTree::new(&mut cloned_waypoint_refs));
    }

    fn print_kd_tree(&self) {
        if let Some(tree) = &self.kd_tree {
            tree.print();
        } else {
            println!("A k-d tree has not yet been generated for this dataset.");
        }
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let mut dataset = Dataset::new("Bob".to_string(), 100);
    let kd_tree = dataset.generate_kd_tree();
    let elapsed = now.elapsed();
    dataset.print_kd_tree();

    println!();
    println!("Elapsed time: {:.2?}", elapsed);
}
