use std::time::Instant;
fn main() {
    let mut dataset = zpath::Dataset::new();
    dataset.generate_waypoints(10000);
    let aaa = &dataset.waypoints[0];

    println!(
        "Point {} at {:.2}, {:.2} - {}",
        aaa.label, aaa.lat, aaa.lon, aaa.geohash
    );

    let start_time_1 = Instant::now();
    let thing = dataset.get_knn_geohash(&aaa, 7);
    let end_time_1 = Instant::now();

    for connection in thing {
        let neighbor = &dataset.waypoints[connection.waypoint_index];
        println!(
            "{} - {}, {:.2}km",
            neighbor.label, neighbor.geohash, connection.distance
        );
    }
    println!(
        "Search completed in {:?}",
        end_time_1.duration_since(start_time_1)
    );
}
