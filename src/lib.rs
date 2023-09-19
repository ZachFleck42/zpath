#![allow(dead_code, non_snake_case, unused_variables)]
use rand::Rng; // Used to randomly generate coordinates for waypoints

// Waypoints have x and y components to represent latitude and longitude respectively
// Latitude can range from -90 to 90 and longitude can range from -180 to 180
struct Waypoint {
    x: f32,
    y: f32,
}

impl Waypoint {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        Waypoint {
            x: rng.gen_range(-90.0..=90.0),
            y: rng.gen_range(-180.0..=180.0),
        }
    }

    fn print_DD(&self) {
        println!("{:.6}, {:.6}", self.x, self.y)
    }

    fn print_DMS(&self) {
        let lat = self.x.abs(); // Convert (-) values to (+) for cleaner code; sign only relevant in determining direction
        let lat_degrees = lat.floor(); // The whole number portion of the value represents degrees
        let lat_minutes = (lat - lat_degrees) * 60.0; // The value to the right of the decimal, times 60, equals minutes
        let lat_seconds = (lat_minutes - lat_minutes.floor()) * 60.0; // The decimal portion of minutes, times 60, equals seconds
        let lat_direction: char = if self.x >= 0.0 { 'N' } else { 'S' }; // Assign the cardinal direction based on sign

        let long = self.y.abs();
        let long_degrees = long.floor();
        let long_minutes = (long - long_degrees) * 60.0;
        let long_seconds = (long_minutes - long_minutes.floor()) * 60.0;
        let long_direction: char = if self.y >= 0.0 { 'E' } else { 'W' };

        println! {"{}°{}'{:.2}\"{}, {}°{}'{:.2}\"{}", lat_degrees, lat_minutes.floor(), lat_seconds, lat_direction, long_degrees, long_minutes.floor(), long_seconds, long_direction}
    }
}

fn generate_waypoints(amt: isize) -> Vec<Waypoint> {
    return vec![];
}
