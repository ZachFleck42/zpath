#![allow(dead_code, non_snake_case, unused_variables)]
use rand::Rng;

const EARTH_RADIUS: f32 = 6378.137; // Earth's radius in kilometers. Used in calculating distance between waypoints

struct Waypoint {
    x: f32, // Latitude; can range from -90 to 90
    y: f32, // Longitude; can range from -180 to 180
    label: String,
}

struct Dataset {
    name: String,
    waypoints: Vec<Waypoint>,
}

impl Waypoint {
    fn new(label: String) -> Self {
        let mut rng = rand::thread_rng();

        Waypoint {
            x: rng.gen_range(-90.0..=90.0),
            y: rng.gen_range(-180.0..=180.0),
            label,
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
}

impl Dataset {
    fn new(&self, name: String, size: usize) -> Self {
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
}
