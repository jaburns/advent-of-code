use std::collections::HashMap;

fn path_to_center(orbit_map: &HashMap<String, String>, item: &str) -> Vec<String> {
    let mut result = Vec::<String>::new();

    let mut cur_item: &str = item;
    loop {
        result.push(String::from(cur_item));
        if orbit_map.contains_key(cur_item) {
            cur_item = &orbit_map[cur_item];
        } else {
            break;
        }
    }

    result
}

fn count_total_orbits(orbit_map: &HashMap<String, String>) -> i32 {
    let mut sum = 0i32;
    for (k, _) in orbit_map {
        sum += path_to_center(orbit_map, k).len() as i32 - 1;
    }
    sum
}

fn find_shortest_transfer(orbit_map: &HashMap<String, String>, from: &str, to: &str) -> i32 {
    let mut path_from = path_to_center(orbit_map, from);
    let mut path_to = path_to_center(orbit_map, to);

    path_from.remove(0);
    path_to.remove(0);

    path_from.reverse();
    path_to.reverse();

    while path_from[0] == path_to[0] {
        path_from.remove(0);
        path_to.remove(0);
    }

    (path_from.len() + path_to.len()) as i32
}

pub fn main() {
    let mut orbit_map = HashMap::<String, String>::new();

    std::fs::read_to_string("data/day6.txt")
        .unwrap()
        .lines()
        .for_each(|x| {
            let parts: Vec<&str> = x.split(")").collect();
            orbit_map.insert(String::from(parts[1]), String::from(parts[0]));
        });

    let result0 = count_total_orbits(&orbit_map);
    let result1 = find_shortest_transfer(&orbit_map, "YOU", "SAN");

    println!("{} {}", result0, result1);
}
