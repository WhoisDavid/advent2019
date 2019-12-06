use crate::{get_input_with_params, AdventError, AdventResult};
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_part1() -> AdventResult<u64> {
    let orbit_map = get_input_with_params::<String>(6, false, ')')?.data;
    let orbit_hashmap = map_to_hasmap(&orbit_map);
    let root = "COM";
    let res = orbits_count(&orbit_hashmap, root, 0);
    println!("Number of orbits: {}", res);
    Ok(res)
}

pub fn solve_part2() -> AdventResult<u64> {
    let orbit_map = get_input_with_params::<String>(6, false, ')')?.data;
    let orbit_hashmap = direct_orbits_hashmap(&orbit_map);
    let res =
        distance_to_common_root(&orbit_hashmap, "YOU", "SAN").ok_or(AdventError::InvalidValue)?;
    println!("Number of orbits: {}", res);
    Ok(res)
}

fn map_to_hasmap<T: AsRef<str>>(orbit_map: &[Vec<T>]) -> HashMap<&str, Vec<&str>> {
    orbit_map
        .iter()
        .map(|v| (v[0].as_ref(), v[1].as_ref()))
        .into_group_map()
}

fn direct_orbits_hashmap<T: AsRef<str>>(orbit_map: &[Vec<T>]) -> HashMap<&str, &str> {
    // The graph being a tree this has a one to one match
    // Every node has only one parent node
    orbit_map
        .iter()
        .map(|v| (v[1].as_ref(), v[0].as_ref()))
        .collect()
}

fn orbits_count(orbit_map: &HashMap<&str, Vec<&str>>, node: &str, depth: u64) -> u64 {
    // calculates the number of orbits from the root
    let mut res = 0;
    if let Some(orbits) = orbit_map.get(node) {
        let depth = depth + 1;
        // Add all the orbits at depth
        res = depth * orbits.len() as u64;
        for &object in orbits {
            // Add subsequent orbits
            res += orbits_count(orbit_map, object, depth);
        }
    }
    res
}

fn distance_to_common_root(
    direct_orbit_map: &HashMap<&str, &str>,
    node1: &str,
    node2: &str,
) -> Option<u64> {
    // Build map of node1's ancestors to their distance to node1
    let mut node1_ancestors = HashMap::new();
    let mut node = node1;
    let mut distance_to_node1 = 0u64;
    while let Some(parent) = direct_orbit_map.get(node) {
        node1_ancestors.insert(parent, distance_to_node1);
        node = parent;
        distance_to_node1 += 1;
    }

    // Go through node2's ancestors maintaining distances
    // stop when hitting a common ancestor with node 1
    let mut common_ancestor = None;
    node = node2;
    let mut distance_to_node2 = 0;
    while let Some(parent) = direct_orbit_map.get(node) {
        if node1_ancestors.contains_key(parent) {
            common_ancestor = Some(parent);
            break;
        } else {
            node = parent;
            distance_to_node2 += 1;
        }
    }

    let distance_to_node1 = node1_ancestors.get(common_ancestor?)?;
    Some(distance_to_node1 + distance_to_node2)
}

#[test]
fn test_day6_case_part1() {
    let orbits = vec![
        vec!["COM", "B"],
        vec!["B", "C"],
        vec!["C", "D"],
        vec!["D", "E"],
        vec!["E", "F"],
        vec!["B", "G"],
        vec!["G", "H"],
        vec!["D", "I"],
        vec!["E", "J"],
        vec!["J", "K"],
        vec!["K", "L"],
    ];
    let orbit_hashmap = map_to_hasmap(&orbits);
    assert_eq!(orbits_count(&orbit_hashmap, "COM", 0), 42);
}

#[test]
fn test_day6_case_part2() {
    let orbits = vec![
        vec!["COM", "B"],
        vec!["B", "C"],
        vec!["C", "D"],
        vec!["D", "E"],
        vec!["E", "F"],
        vec!["B", "G"],
        vec!["G", "H"],
        vec!["D", "I"],
        vec!["E", "J"],
        vec!["J", "K"],
        vec!["K", "L"],
        vec!["K", "YOU"],
        vec!["I", "SAN"],
    ];
    let orbit_hashmap = direct_orbits_hashmap(&orbits);
    assert_eq!(
        distance_to_common_root(&orbit_hashmap, "YOU", "SAN").unwrap(),
        4
    );
}
