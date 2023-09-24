use std::time::Instant;
fn main() {
    let mut dataset = zpath::Dataset::new("Bob".to_string());
    dataset.generate_waypoints(10000);

    let aaa = dataset.waypoints[0].clone();
    let k = 7;

    println!(
        "Point {} at {:.2}, {:.2} - {}",
        aaa.borrow().label,
        aaa.borrow().lat,
        aaa.borrow().lon,
        aaa.borrow().geohash
    );

    println!();

    let start_time_1 = Instant::now();
    let nn_naive = dataset.find_knn_naive(aaa.clone(), k);
    let end_time_1 = Instant::now();
    println!("Nearest neighbors found by naive search");
    for connection in nn_naive {
        println!(
            "{} - {}, {:.2}km",
            connection.waypoint.borrow().label,
            connection.waypoint.borrow().geohash,
            connection.distance
        );
    }
    println!(
        "Search completed in {:?}",
        end_time_1.duration_since(start_time_1)
    );

    println!();

    let start_time_2 = Instant::now();
    let nn_geohash = dataset.find_knn_geohash(aaa.clone(), k);
    let end_time_2 = Instant::now();
    println!("Nearest neighbors found by geohash search:");
    for connection in nn_geohash {
        println!(
            "{} - {}, {:.2}km",
            connection.waypoint.borrow().label,
            connection.waypoint.borrow().geohash,
            connection.distance
        );
    }
    println!(
        "Search completed in {:?}",
        end_time_2.duration_since(start_time_2)
    );
}
