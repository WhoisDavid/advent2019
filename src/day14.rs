use advent2019::{get_raw_input, AdventError, AdventResult};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> AdventResult<()> {
    let input = get_raw_input(14)?;
    let reactions = &format_input(input)?;

    solve_part1(reactions);
    solve_part2(reactions);
    Ok(())
}

type Reactions = HashMap<String, Reaction>;

struct Reaction {
    input: Vec<Chemical>,
    output: Chemical,
}

#[derive(Clone)]
struct Chemical {
    name: String,
    quantity: usize,
}

fn format_input(input: String) -> AdventResult<Reactions> {
    let re = Regex::new(r"(?P<quantity>\d+) (?P<chemical>[A-Z]+)")
        .map_err(|_| AdventError::InvalidValue)?;
    let mut reactions = Vec::with_capacity(input.lines().count());
    for formula_str in input.lines() {
        let mut chemicals: Vec<Chemical> = Vec::new();
        for term in re.captures_iter(formula_str) {
            let chemical = term["chemical"].to_string();
            let quantity = term["quantity"].parse::<usize>()?;
            chemicals.push(Chemical {
                name: chemical,
                quantity,
            })
        }
        let output = chemicals.pop().unwrap();
        let reaction = Reaction {
            input: chemicals,
            output,
        };
        reactions.push(reaction);
    }
    Ok(reactions
        .into_iter()
        .map(|r| (r.output.name.to_string(), r))
        .collect())
}

fn solve_part1(reactions: &Reactions) -> usize {
    let res = ore_needed_for_n_fuel(1, reactions);
    println!("Ore needed: {}", res);
    res
}

fn solve_part2(reactions: &Reactions) -> usize {
    let trillion = 1_000_000_000_000;
    let res = ore_to_fuel(trillion, reactions);
    println!("Fuel from a trillion ore: {}", res);
    res
}

fn ore_needed_for_n_fuel(n: usize, reactions: &Reactions) -> usize {
    let fuel = &Chemical {
        name: "FUEL".to_string(),
        quantity: n,
    };
    let extra = &mut HashMap::new();
    ore_needed(fuel, reactions, extra)
}

fn ore_needed(
    chemical: &Chemical,
    reactions: &Reactions,
    available: &mut HashMap<String, usize>,
) -> usize {
    if chemical.name == "ORE" {
        return chemical.quantity;
    }

    let mut chemical_available = available.remove(&chemical.name).unwrap_or_default();
    let reaction = &reactions[&chemical.name];
    let quantity_produced = reaction.output.quantity;
    let mut chemical_needed = chemical.quantity;

    let chemical_used = chemical_needed.min(chemical_available);
    chemical_needed -= chemical_used;
    chemical_available -= chemical_used;
    available.insert(chemical.name.to_string(), chemical_available);

    if chemical_needed == 0 {
        return 0;
    }

    let reactions_needed = (chemical_needed - 1) / quantity_produced + 1;
    let leftover = reactions_needed * quantity_produced - chemical_needed;

    available.insert(chemical.name.to_string(), chemical_available + leftover);

    let mut ore = 0;
    for reaction_chemical in reaction.input.iter() {
        let chem = &mut reaction_chemical.clone();
        chem.quantity *= reactions_needed;
        ore += ore_needed(chem, reactions, available);
    }
    ore
}

fn ore_to_fuel(ore_quantity: usize, reactions: &Reactions) -> usize {
    let min_ore_per_fuel = ore_needed_for_n_fuel(1, reactions);
    let min_fuel = ore_quantity / min_ore_per_fuel;

    let mut lower_fuel = min_fuel;
    let mut upper_fuel = 2 * min_fuel;
    while ore_needed_for_n_fuel(upper_fuel, reactions) < ore_quantity {
        upper_fuel += lower_fuel
    }

    println!("Upperbound: x{}", upper_fuel / lower_fuel);
    while lower_fuel < upper_fuel {
        let mid = (lower_fuel + upper_fuel + 1) / 2;
        match ore_needed_for_n_fuel(mid, reactions).cmp(&ore_quantity) {
            Ordering::Less => lower_fuel = mid,
            Ordering::Equal => return mid,
            Ordering::Greater => upper_fuel = (lower_fuel + upper_fuel) / 2,
        }
    }
    lower_fuel
}

#[allow(dead_code)]
fn test_part1(s: &str, ore: usize) {
    let reactions = &format_input(s.to_string()).expect("input");
    assert_eq!(ore_needed_for_n_fuel(1, reactions), ore)
}

#[allow(dead_code)]
fn test_part2(s: &str, fuel: usize) {
    let reactions = &format_input(s.to_string()).expect("input");
    assert_eq!(solve_part2(reactions), fuel)
}

#[test]
fn test_day14_case1() {
    let input = "10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL";
    test_part1(input, 31)
}

#[test]
fn test_day14_case2() {
    let input = "9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL";
    test_part1(input, 165)
}

#[test]
fn test_day14_case3() {
    let input = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    test_part1(input, 13312);
    test_part2(input, 82_892_753)
}

#[test]
fn test_day14_case4() {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";
    test_part1(input, 180_697);
    test_part2(input, 5_586_022)
}

#[test]
fn test_day14_case5() {
    let input = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";
    test_part1(input, 2_210_736);
    test_part2(input, 460_664)
}
