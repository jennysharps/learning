use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

static INPUT_FILE: &str = "./input.txt";

struct Innstructions<'a> {
    inner: &'a [Instruction],
    next_index: usize,
}

impl<'a> Innstructions<'a> {
    fn new_from(instructions: &'a [Instruction]) -> Self {
        Self {
            inner: instructions,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for Innstructions<'a> {
    type Item = &'a Instruction;

    fn next(&mut self) -> Option<&'a Instruction> {
        use std::convert::TryFrom;
        let current_index = self.next_index;
        let instruction = self.inner.get(current_index);

        if let Some(instruction) = instruction {
            match instruction.operation {
                Operation::Acc | Operation::NoOp => {
                    self.next_index = current_index + 1;
                }
                Operation::Jmp => {
                    let current_index =
                        i16::try_from(current_index).expect("usize did not fit i16");
                    self.next_index = usize::try_from(current_index + instruction.argument)
                        .expect("new i16 did not fit usize");
                }
            }
        };

        instruction
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Operation {
    Acc,
    Jmp,
    NoOp,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "jmp" => Ok(Self::Jmp),
            "acc" => Ok(Self::Acc),
            "nop" => Ok(Self::NoOp),
            _ => Err(format!("Unknown instruction kind: {}", input)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Instruction {
    id: usize,
    operation: Operation,
    argument: i16,
}

impl Instruction {
    fn try_new_from(id: usize, line: impl AsRef<str>) -> Result<Self, String> {
        match line.as_ref().split(' ').collect::<Vec<_>>().as_slice() {
            [operation_part, argument_part] => Ok(Instruction {
                id,
                operation: operation_part.parse::<Operation>()?,
                argument: argument_part.parse::<i16>().map_err(|e| {
                    format!("Could not parse i16 from \"{}\": {}", operation_part, e)
                })?,
            }),
            _ => Err("Invalid value; expected \"<kind> <i16>\"".into()),
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE)
        .unwrap_or_else(|e| panic!("Could not read file at {}: {}", INPUT_FILE, e));
    let instructions = input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, line)| Instruction::try_new_from(i, line).expect("Could not parse instruction"))
        .collect::<Vec<Instruction>>();

    let mut acc: i16 = 0;
    let handler = Innstructions::new_from(&instructions);
    let mut mem_cache: HashMap<usize, i16> = HashMap::new();

    for instruction in handler {
        if mem_cache.get(&instruction.id).is_some() {
            break;
        }
        if let Operation::Acc = instruction.operation {
            acc += instruction.argument;
        }
        mem_cache.insert(instruction.id, acc);
    }

    println!("What value is in the accumulator?: {:?}", &acc);
}
