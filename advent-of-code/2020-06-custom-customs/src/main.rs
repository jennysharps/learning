use std::collections::HashSet;
use std::fs;
use std::str::Chars;

const INPUT_FILE: &str = "./input.txt";

struct PassengerGroup<'a>(&'a str);

impl<'a> PassengerGroup<'a> {
    fn declaration_forms(&self) -> Vec<DeclarationForm<'a>> {
        self.0.lines().map(|line| DeclarationForm(line)).collect()
    }

    fn unique_responses(&self) -> Vec<HashSet<char>> {
        let mut sets = Vec::new();

        for survey in self.declaration_forms() {
            sets.push(survey.unique_responses());
        }

        sets
    }
}

struct DeclarationForm<'a>(&'a str);

impl<'a> DeclarationForm<'a> {
    fn responses(&self) -> Chars {
        self.0.chars()
    }

    fn unique_responses(&self) -> HashSet<char> {
        let mut unique_chars = HashSet::new();

        for char in self.0.chars() {
            unique_chars.insert(char);
        }

        unique_chars
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .unwrap_or_else(|e| panic!("Could not read file at {}: {}", INPUT_FILE, e));
    let groups: Vec<PassengerGroup> = input
        .split("\n\n")
        .map(|pg| PassengerGroup(pg))
        .collect();

    println!(
        "Questions to which ANYONE answered yes: {}",
        anyone_yes_count(&groups)
    );
    println!(
        "Questions to which EVERYONE answered yes: {}",
        everyone_yes_count(&groups)
    );
}

fn anyone_yes_count(groups: &[PassengerGroup]) -> usize {
    let mut total_yes_answers = 0;

    for group in groups.iter() {
        let mut group_yes_answers = HashSet::new();

        for declaration_form in group.declaration_forms() {
            for responses in declaration_form.responses() {
                group_yes_answers.insert(responses);
            }
        }

        total_yes_answers += group_yes_answers.len();
    }

    total_yes_answers
}

fn everyone_yes_count(groups: &[PassengerGroup]) -> usize {
    let mut letters_in_every_line_count = 0;

    for group in groups.iter() {
        // Convert lines to hashsets or characters in each line
        let mut unique_responses = group.unique_responses();
        // Sort by number of characters so that the first item has the fewest characters
        unique_responses.sort_by(|a, b| a.len().cmp(&b.len()));
        let shortest_line_in_set = &unique_responses[0];
        let rest_lines_in_set = &unique_responses[1..];

        for needle_char in shortest_line_in_set {
            let mut in_every_line = true;

            for haystack_char in rest_lines_in_set {
                if !haystack_char.contains(needle_char) {
                    in_every_line = false;
                    break;
                }
            }

            if in_every_line {
                letters_in_every_line_count += 1;
            }
        }
    }

    letters_in_every_line_count
}
