#![allow(dead_code, non_snake_case, unused_variables)]
use rand::Rng; // Used to randomly generate coordinates for waypoints

// Waypoints have x and y components to represent latitude and longitude respectively
// Latitude can range from -90 to 90 and longitude can range from -180 to 180
struct Waypoint<'a> {
    x: f32,
    y: f32,
    label: String,
    neighbors: Vec<&'a Waypoint<'a>>,
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

        let long = self.y.abs();
        let long_degrees = long.floor();
        let long_minutes = (long - long_degrees) * 60.0;
        let long_seconds = (long_minutes - long_minutes.floor()) * 60.0;
        let long_direction = if self.y >= 0.0 { 'E' } else { 'W' };

        println!(
            "Waypoint {} is located at {}°{}'{:.2}\"{}, {}°{}'{:.2}\"{}",
            self.label,
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

    fn print_neighbors(&self) {
        print!("Neighbors of waypoint {}:", self.label);
        for neighbor in &self.neighbors {
            print!(" {}", neighbor.label);
        }
        println!();
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

    for i in 0..amt {
        let label = generate_label(i);
        let waypoint = Waypoint::new(label);
        waypoints.push(waypoint);
    }

    waypoints
}
