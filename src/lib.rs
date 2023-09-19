#![allow(dead_code, non_snake_case, unused_variables)]
use rand::Rng; // Used to randomly generate coordinates for waypoints

const EARTH_RADIUS: f32 = 6378.137; // Earth's radius in kilometers. Used in calculating distance between waypoints

struct Waypoint<'a> {
    x: f32, // Latitude; can range from -90 to 90
    y: f32, // Longitude; can range from -180 to 180
    label: String,
    neighbors: Vec<&'a Waypoint<'a>>,
}

struct Dataset<'a> {
    name: String,
    waypoints: Vec<Waypoint<'a>>,
}

impl<'a> Waypoint<'a> {
    fn new(label: String) -> Self {
        let mut rng = rand::thread_rng();

        Waypoint {
            x: rng.gen_range(-90.0..=90.0),
            y: rng.gen_range(-180.0..=180.0),
            label,
            neighbors: Vec::new(),
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

    fn find_neighbors(&mut self, dataset: &'a Dataset<'a>, amt: usize) {
        let mut distances: Vec<(usize, f32)> = Vec::with_capacity(dataset.waypoints.len() - 1);

        for (i, waypoint) in dataset.waypoints.iter().enumerate() {
            if !std::ptr::eq(self, waypoint) {
                let distance = self.distance_to(waypoint);
                distances.push((i, distance));
            }
        }

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut nearest_neighbors = Vec::with_capacity(amt);
        for (index, _) in distances.iter().take(amt) {
            nearest_neighbors.push(&dataset.waypoints[*index]);
        }

        self.neighbors.extend(nearest_neighbors);
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

    fn print_neighbors(&self) {
        print!("Neighbors of waypoint {}:", self.label);
        for neighbor in &self.neighbors {
            print!(" {}", neighbor.label);
        }
        println!();
    }
}

impl<'a> Dataset<'a> {
    fn new(name: String, size: usize) -> Self {
        let waypoints = generate_waypoints(size);

        Dataset { name, waypoints }
    }
}

fn generate_label(n: usize) -> String {
    let mut label = String::new();
    let mut num = n;

    while num > 0 {
        let remainder = (num - 1) % 26;
        let char_value = (remainder as u8 + b'A') as char;
        label.insert(0, char_value);
        num = (num - 1) / 26;
    }

    label
}

fn generate_waypoints<'a>(amt: usize) -> Vec<Waypoint<'a>> {
    let mut waypoints = Vec::with_capacity(amt);

    for i in 1..=amt {
        let label = generate_label(i);
        let waypoint = Waypoint::new(label);
        waypoints.push(waypoint);
    }

    waypoints
}
