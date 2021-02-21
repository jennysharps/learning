use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const NEEDLE_BAG_COLOR: &str = "shiny gold";

struct BagRules(HashSet<(String, u32)>);

impl BagRules {
    fn new() -> Self {
        Self(HashSet::<(String, u32)>::new())
    }

    fn iter(&self) -> std::collections::hash_set::Iter<(String, u32)> {
        self.0.iter()
    }

    fn add_rule(&mut self, color: String, count: u32) -> bool {
        self.0.insert((color, count))
    }
}

fn main() {
    let bag_colors_map = parse_input("./input.txt");
    run_part_1(&bag_colors_map);
    run_part_2(&bag_colors_map);
}

fn parse_input(input: &'_ str) -> HashMap<String, BagRules> {
    let file = File::open(input).expect("Error reading file");
    let reader = BufReader::new(file);
    let mut bag_colors_map: HashMap<String, BagRules> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("could not read line");
        let color_and_rules = line.split(" bags contain ").collect::<Vec<_>>();
        if color_and_rules.len() < 2 {
            panic!("Invalid input");
        }
        let root_color = color_and_rules[0];
        let mut bag_rules = BagRules::new();
        // ex: `3 muted white bags, 3 striped magenta bags.`
        let re = Regex::new(r"(?P<count>[0-9]+) (?P<color>.+?) bag").unwrap();

        for color_rules_substr in &color_and_rules[1..] {
            for captures in re.captures_iter(color_rules_substr) {
                bag_rules.add_rule(
                    (&captures["color"]).to_string(),
                    (&captures["count"])
                        .parse()
                        .expect("could not parse number"),
                );
            }
        }

        bag_colors_map.insert(root_color.to_owned(), bag_rules);
    }

    bag_colors_map
}

fn run_part_1(bag_rules_map: &HashMap<String, BagRules>) {
    let count = possible_bags_count(&bag_rules_map);
    println!(
        "Part 1: How many bag colors can eventually contain at least one *{}* bag?: {}",
        NEEDLE_BAG_COLOR, count
    );
}

fn run_part_2(bag_rules_map: &HashMap<String, BagRules>) {
    let count = individual_bags_count(NEEDLE_BAG_COLOR, &bag_rules_map);
    println!(
        "Part 2: How many individual bags are required inside your *{}* bag?: {}",
        NEEDLE_BAG_COLOR, count
    );
}

fn contains_bag(
    bag_rules_map: &HashMap<String, BagRules>,
    contains_bag_map: &mut HashMap<String, bool>,
    color: &str,
) -> bool {
    if let Some(&can_hold) = contains_bag_map.get(color) {
        return can_hold;
    };

    let bag_rules = bag_rules_map.get(color).expect("color not found");
    for (nested_color, _) in bag_rules.iter() {
        if nested_color == NEEDLE_BAG_COLOR
            || contains_bag(&bag_rules_map, contains_bag_map, nested_color)
        {
            contains_bag_map.insert(color.to_string(), true);
            return true;
        }
    }

    contains_bag_map.insert(color.to_owned(), false);
    false
}

fn possible_bags_count(bag_rules_map: &HashMap<String, BagRules>) -> u32 {
    let mut count = 0;
    let mut contains_bag_map: HashMap<String, bool> = HashMap::new();
    for (color, _) in bag_rules_map.iter() {
        if contains_bag(&bag_rules_map, &mut contains_bag_map, color) {
            count += 1;
        }
    }
    count
}

fn individual_bags_count(color: &'_ str, bag_rules_map: &HashMap<String, BagRules>) -> u32 {
    let mut total_count = 0;
    let bag_rules = bag_rules_map.get(color).expect("color not found ");
    for (nested_color, count) in bag_rules.iter() {
        let nested_count = individual_bags_count(nested_color, bag_rules_map);
        total_count += count + count * nested_count;
    }
    total_count
}
