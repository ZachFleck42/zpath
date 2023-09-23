use crate::Direction;

const BASE_32GHS: &'static [u8; 32] = b"0123456789bcdefghjkmnpqrstuvwxyz";

const NEIGHBORS_A: [char; 32] = [
    'p', '0', 'r', '2', '1', '4', '3', '6', 'x', '8', 'z', 'b', '9', 'd', 'c', 'f', '5', 'h', '7',
    'k', 'j', 'n', 'm', 'q', 'e', 's', 'g', 'u', 't', 'w', 'v', 'y',
];
const NEIGHBORS_B: [char; 32] = [
    'b', 'c', '0', '1', 'f', 'g', '4', '5', '2', '3', '8', '9', '6', '7', 'd', 'e', 'u', 'v', 'h',
    'j', 'y', 'z', 'n', 'p', 'k', 'm', 's', 't', 'q', 'r', 'w', 'x',
];
const NEIGHBORS_C: [char; 32] = [
    '1', '4', '3', '6', '5', 'h', '7', 'k', '9', 'd', 'c', 'f', 'e', 's', 'g', 'u', 'j', 'n', 'm',
    'q', 'p', '0', 'r', '2', 't', 'w', 'v', 'y', 'x', '8', 'z', 'b',
];
const NEIGHBORS_D: [char; 32] = [
    '2', '3', '8', '9', '6', '7', 'd', 'e', 'b', 'c', '0', '1', 'f', 'g', '4', '5', 'k', 'm', 's',
    't', 'q', 'r', 'w', 'x', 'u', 'v', 'h', 'j', 'y', 'z', 'n', 'p',
];

const BORDERS_A: [char; 4] = ['p', 'r', 'x', 'z'];
const BORDERS_B: [char; 8] = ['b', 'c', 'f', 'g', 'u', 'v', 'y', 'z'];
const BORDERS_C: [char; 4] = ['0', '2', '8', 'b'];
const BORDERS_D: [char; 8] = ['0', '1', '4', '5', 'h', 'j', 'n', 'p'];

/// Builds a geohash String from the provided coordinates
pub fn encode(lat: f32, lon: f32, precision: usize) -> String {
    // We will be building the geohash character by character
    let mut geohash = Vec::with_capacity(precision);

    // Initialize latitude and longitude mins / maxes to the entire range of Earth
    // These values will change as we subdivide the Earth into smaller and smaller pieces
    let (mut lat_min, mut lat_max) = (-90.0, 90.0);
    let (mut lon_min, mut lon_max) = (-180.0, 180.0);

    let mut bits = 0; // The 5 binary bits used to determine which base32 char to append next; initially '00000'
    let mut bit = 0; // Which digit in 'bits' we are currently assigning (from least significant to most)
    let mut longitude_bit = true; // Alternates between true / false to switch between assigning lon and lat bits

    while geohash.len() < precision {
        let midpoint;

        // Determine whether or not the current digit in bits is a latitude or
        // longitude bit and whether it should be a '0' or '1'
        if longitude_bit {
            midpoint = (lon_min + lon_max) / 2.0;

            if lon > midpoint {
                bits |= 1 << (4 - bit);
                lon_min = midpoint;
            } else {
                lon_max = midpoint;
            }
        } else {
            midpoint = (lat_min + lat_max) / 2.0;

            if lat > midpoint {
                bits |= 1 << (4 - bit);
                lat_min = midpoint;
            } else {
                lat_max = midpoint;
            }
        }

        // Once 'bits' has all five bits populated, we can translate the constructed binary number into base32
        // Otherwise, move to the next bit in bits and repeat until full
        if bit == 4 {
            geohash.push(BASE_32GHS[bits]); // Push the appropriate 32ghs char to the end of the geohash
            bits = 0; // Reset bits back to 00000
            bit = 0; // Reset bit back to least significant digit in bits
        } else {
            bit += 1;
        }

        longitude_bit = !longitude_bit;
    }

    String::from_utf8(geohash).unwrap()
}

/// Returns the geohash string of the adjacent cell in the provided direction
pub fn get_adjacent_cell(geohash: &str, direction: Direction) -> String {
    let mut parent_geohash = String::from(&geohash[0..geohash.len() - 1]);
    let child_char = geohash.chars().last().unwrap();

    // Based on the current cell's type (4x8 or 8x4) and the direction of
    // the adjacent cell, determine which set of lookup tables to reference
    let (neighbor, border): (&[char; 32], &[char]) = match geohash.len() % 2 {
        0 => match direction {
            Direction::North => (&NEIGHBORS_A, &BORDERS_A),
            Direction::East => (&NEIGHBORS_B, &BORDERS_B),
            Direction::South => (&NEIGHBORS_C, &BORDERS_C),
            Direction::West => (&NEIGHBORS_D, &BORDERS_D),
        },
        _ => match direction {
            Direction::North => (&NEIGHBORS_B, &BORDERS_B),
            Direction::East => (&NEIGHBORS_A, &BORDERS_A),
            Direction::South => (&NEIGHBORS_D, &BORDERS_D),
            Direction::West => (&NEIGHBORS_C, &BORDERS_C),
        },
    };

    // In the case that the relevant adjacent cell is not contained within the
    // current cell's parent, we need to alter the parent_geohash to its adjacent
    // counterpart in the relevant direction.
    if border.contains(&child_char) && !parent_geohash.is_empty() {
        parent_geohash = get_adjacent_cell(&parent_geohash, direction)
    }

    // Use the neighbor lookup table to determine which child cell is in the relevant direction
    let index = neighbor.iter().position(|&c| c == child_char).unwrap();
    let adjacent_cell_char = BASE_32GHS[index] as char;

    format!("{}{}", parent_geohash, adjacent_cell_char)
}

/// Returns a vector of Strings of all adjacent cells to a geohash
pub fn get_surrounding_cells(geohash: &str) -> Vec<String> {
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut adjacent_cells = Vec::with_capacity(8);

    for direction in directions {
        let adjacent = get_adjacent_cell(geohash, direction.clone());

        if direction == Direction::North || direction == Direction::South {
            adjacent_cells.push(get_adjacent_cell(&adjacent, Direction::East));
            adjacent_cells.push(get_adjacent_cell(&adjacent, Direction::East));
        }

        adjacent_cells.push(adjacent);
    }

    adjacent_cells
}
