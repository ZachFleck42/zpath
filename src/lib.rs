// Waypoints have x and y components to represent latitude and longitude respectively
// Latitude can range from -90 to 90 and longitude can range from -180 to 180
struct Waypoint {
    x: f32,
    y: f32,
}

impl Waypoint {
    fn print_DD(&self) {}
    fn print_DMM(&self) {}
    fn print_DMS(&self) {}
}

fn generate_waypoints(amt: isize) -> Vec<Waypoint> {
    return vec![];
}
