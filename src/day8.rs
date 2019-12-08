use crate::{get_input, AdventError, AdventResult};
use std::char;

pub fn solve_part1() -> AdventResult<usize> {
    let image = get_input::<String>(8)?.first_element();
    let width = 25;
    let height = 6;
    let layers = get_layers(image, width, height);
    let res = get_min_layer_product(layers).ok_or(AdventError::InvalidValue)?;
    println!("Output: {}", res);
    Ok(res)
}

pub fn solve_part2() -> AdventResult<usize> {
    let image = get_input::<String>(8)?.first_element();
    let width = 25;
    let height = 6;
    let layers = get_layers(image, width, height);
    let res = merge_layers(layers);
    print_image(res, width);
    Ok(1)
}

pub fn get_layers(image: String, width: usize, height: usize) -> Vec<Vec<u8>> {
    image
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Expected 0, 1, 2"),
        })
        .collect::<Vec<u8>>()
        .chunks_exact(width * height)
        .map(|l| l.to_vec())
        .collect()
}

pub fn get_min_layer_product(layers: Vec<Vec<u8>>) -> Option<usize> {
    let min_layer = layers
        .iter()
        .map(|l| (l.iter().filter(|p| **p == 0).count(), l))
        .min()
        .map(|(_, l)| l)?;
    let ones = min_layer.iter().filter(|c| **c == 1).count();
    let twos = min_layer.iter().filter(|c| **c == 2).count();
    Some(ones * twos)
}

pub fn merge_layers(layers: Vec<Vec<u8>>) -> Vec<u8> {
    layers
        .iter()
        .fold(Vec::new(), |acc, v| {
            if acc.is_empty() {
                return v.to_vec();
            }
            acc.iter()
                .zip(v)
                .map(|(merge, l)| match merge {
                    2 => *l,
                    _ => *merge,
                })
                .collect()
        })
}

pub fn print_image(merged_layer: Vec<u8>, width: usize) {
    merged_layer
        .iter()
        .map(|c| match c {
            1 => '*',
            _ => ' ',
        })
        .collect::<Vec<char>>()
        .chunks_exact(width)
        .map(|c| c.iter().collect::<String>())
        .for_each(|s| println!("{}", s))
}
