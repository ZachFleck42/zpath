fn main() {
    // Create a new, empty dataset
    let mut dataset = zpath::Dataset::new();

    // Define the number of waypoints to generate and how many connections each waypoint should have
    let number_of_waypoints = 1000;
    let number_of_connections = 5;

    // Generate the waypoints and insert them into the dataset
    dataset.generate_waypoints(number_of_waypoints);

    // Assign connections to all waypoints in the dataset.
    // Can use either naive method or using geohashes and the datset's geohash index.
    dataset.assign_all_connections_geohash(number_of_connections);
    // dataset.assign_all_connections_naive(number_of_neighbors);

    // Add custom waypoints to the dataset whenever you'd like
    let custom_waypoint_index = dataset.add_new_waypoint(37.7749, -122.4194);

    // Find the shortest route between two waypoints
    let waypoint_a = &dataset.waypoints[0];
    let custom_waypoint = &dataset.waypoints[custom_waypoint_index];
    let route = dataset.get_shortest_route(waypoint_a, custom_waypoint);

    // Print the resulting route (if one was found)
    if route.is_some() {
        print!(
            "The shortest route from {} to {} is:",
            waypoint_a.label, custom_waypoint.label
        );
        let waypoints = &dataset.waypoints;
        let route = route.unwrap();

        for (i, &waypoint_index) in route.iter().enumerate() {
            print!(" {}", waypoints[waypoint_index].label);

            if i < route.len() - 1 {
                print!(" ->");
            }
        }
    } else {
        print!("No route found was found.");
    }
}
