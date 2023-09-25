use std::time::Instant;

fn main() {
    let number_of_points = 10000;
    let number_of_neighbors = 5;
    let mut dataset = zpath::Dataset::new();

    let start_time_0 = Instant::now();
    dataset.generate_waypoints(number_of_points);
    let end_time_0 = Instant::now();

    let start_time_1 = Instant::now();
    dataset.assign_connections(number_of_neighbors);
    let end_time_1 = Instant::now();

    let aaa = &dataset.waypoints[0];
    let aab = &dataset.waypoints[1];

    let start_time_2 = Instant::now();
    let route = dataset.get_shortest_route(aaa, aab);
    let end_time_2 = Instant::now();

    println!();

    if route.is_some() {
        print!("The shortest route from AAA to AAB is:");
        let waypoints = &dataset.waypoints;
        let route = route.unwrap();

        for (i, &waypoint_index) in route.iter().enumerate() {
            print!(" {}", waypoints[waypoint_index].label);

            if i < route.len() - 1 {
                print!(" ->");
            }
        }
    } else {
        print!("No route found");
    }

    println!();
    println!();

    println!(
        "{} points generated in {:?}",
        number_of_points,
        end_time_0.duration_since(start_time_0)
    );

    println!(
        "{} nearest-neighbors for all {} points found in {:?}",
        number_of_neighbors,
        number_of_points,
        end_time_1.duration_since(start_time_1)
    );

    println!(
        "Route found in {:?}",
        end_time_2.duration_since(start_time_2)
    );

    println!();
}
