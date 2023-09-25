fn main() {
    // Create a new, empty dataset
    let mut dataset = zpath::Dataset::new();

    // Generate random waypoints and insert them into the dataset
    let number_of_waypoints = 1000;
    dataset.generate_waypoints(number_of_waypoints);

    // Assign connections to all waypoints in the dataset.
    // Can use either naive method or using geohashes and the datset's geohash index.
    let number_of_connections = 5;
    dataset.assign_all_connections_geohash(number_of_connections);
    // dataset.assign_all_connections_naive(number_of_neighbors);

    // Add custom waypoints to the dataset whenever you'd like
    let custom_waypoint_index = dataset.add_new_waypoint(37.7749, -122.4194);

    // Find the shortest route between two waypoints
    let waypoint_a = &dataset.waypoints[0];
    let custom_waypoint = &dataset.waypoints[custom_waypoint_index];
    let route = dataset.get_shortest_route(waypoint_a, custom_waypoint);

    // Print details about the route (if one was discovered)
    dataset.print_route_details(route);
}
