use anyhow::anyhow;
use anyhow::{bail, Result};
use aoc2021::stream_items_from_file;
use std::collections::HashMap;
use std::{path::Path, str::FromStr};

#[derive(Debug, Clone)]
enum RegisterOrConst {
    Register(usize),
    Const(isize),
}

#[derive(Debug, Clone)]
enum Instruction {
    Input(usize),
    Add(usize, RegisterOrConst),
    Mul(usize, RegisterOrConst),
    Div(usize, RegisterOrConst),
    Mod(usize, RegisterOrConst),
    Equal(usize, RegisterOrConst),
}

fn get_register<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<usize> {
    Ok(
        match parts.next().ok_or(anyhow!("Missing register operand"))? {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            u @ _ => bail!("Invalid register name {}", u),
        },
    )
}

fn get_register_or_const<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<RegisterOrConst> {
    use RegisterOrConst::*;
    Ok(
        match parts.next().ok_or(anyhow!("Missing register operand"))? {
            "w" => Register(0),
            "x" => Register(1),
            "y" => Register(2),
            "z" => Register(3),
            u @ _ => match u.parse::<isize>() {
                Ok(v) => Const(v),
                Err(_) => bail!("Invalid register or constant: {}", u),
            },
        },
    )
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let mut parts = s.split(' ');
        let opcode = parts.next().ok_or(anyhow!("Empty input"))?;
        Ok(match opcode {
            "inp" => Input(get_register(&mut parts)?),
            "add" => Add(
                get_register(&mut parts)?,
                get_register_or_const(&mut parts)?,
            ),
            "mul" => Mul(
                get_register(&mut parts)?,
                get_register_or_const(&mut parts)?,
            ),
            "div" => Div(
                get_register(&mut parts)?,
                get_register_or_const(&mut parts)?,
            ),
            "mod" => Mod(
                get_register(&mut parts)?,
                get_register_or_const(&mut parts)?,
            ),
            "eql" => Equal(
                get_register(&mut parts)?,
                get_register_or_const(&mut parts)?,
            ),
            _ => bail!("Invalid opcode {}", opcode),
        })
    }
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
struct MachineState {
    registers: [isize; 4],
    // This is a hack and limits my mini VM to programs with only a single input, 
    // BUT we can safe an allocation by not taking a Vec here and that safes about 1/5 of the total runtime.
    // It only works because we split the input program on every input anyway.
    input: isize,
}

impl RegisterOrConst {
    fn resolve(&self, state: &MachineState) -> isize {
        match self {
            RegisterOrConst::Register(reg) => state.registers[*reg],
            RegisterOrConst::Const(val) => *val,
        }
    }

    fn as_code(&self, register_vars: &[&str; 4]) -> String {
        match self {
            RegisterOrConst::Register(r) => format!("{}", register_vars[*r]),
            RegisterOrConst::Const(v) => format!("{}", v),
        }
    }
}

impl Instruction {
    fn execute(&self, mut state: MachineState) -> MachineState {
        match self {
            Instruction::Input(target) => {
                state.registers[*target] = state.input
            }
            Instruction::Add(target, operand) => {
                state.registers[*target] += operand.resolve(&state)
            }
            Instruction::Mul(target, operand) => {
                state.registers[*target] *= operand.resolve(&state)
            }
            Instruction::Div(target, operand) => {
                state.registers[*target] /= operand.resolve(&state)
            }
            Instruction::Mod(target, operand) => {
                state.registers[*target] %= operand.resolve(&state)
            }
            Instruction::Equal(target, operand) => {
                state.registers[*target] = if state.registers[*target] == operand.resolve(&state) {
                    1
                } else {
                    0
                }
            }
        }
        state
    }

    #[allow(dead_code)]
    fn code_gen(&self) -> String {
        let registers = ["register_w", "register_x", "register_y", "register_z"];
        match self {
            Instruction::Input(var) => format!("{} = inputs.pop();", registers[*var]),
            Instruction::Add(target, operand) => {
                format!("{} += {}", registers[*target], operand.as_code(&registers))
            }
            Instruction::Mul(target, operand) => {
                format!("{} *= {}", registers[*target], operand.as_code(&registers))
            }
            Instruction::Div(target, operand) => {
                format!("{} /= {}", registers[*target], operand.as_code(&registers))
            }
            Instruction::Mod(target, operand) => {
                format!("{} %= {}", registers[*target], operand.as_code(&registers))
            }
            Instruction::Equal(target, operand) => format!(
                "{} = if {} == {} {{ 1 }} else {{ 0 }}",
                registers[*target],
                registers[*target],
                operand.as_code(&registers)
            ),
        }
    }
}

fn run_program_from_state(program: &Vec<Instruction>, init_state: MachineState) -> MachineState {
    program
        .iter()
        .fold(init_state, |state, ins| ins.execute(state))
}

// The input programs has repeating parts that always start with an input instruction and very similar code after that.
// Every part will clear the w, x and y registers so only the z register gets carried over to the next part.
// We can use this to our advantage by splitting the program on input instructions and building 
// a map of possible states instead of brute-forcing every single input.
fn split_program(program: Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut cur = Vec::new();
    let mut res = Vec::new();
    for ins in program.into_iter() {
        match ins {
            Instruction::Input(_) => {
                if cur.len() > 0 {
                    res.push(cur);
                    cur = Vec::new();
                }
                cur.push(ins);
            }
            _ => cur.push(ins)
        }
    }

    if cur.len() > 0 {
        res.push(cur);
    }

    res
}

fn find_possible_states(input: isize, program: &Vec<Instruction>, state_inputs: &mut HashMap<isize, isize>, max: bool) {
    state_inputs.clear();
    for inp in 1..=9 {
        let state = MachineState { registers: [0,0,0,input], input: inp };
        let final_state = run_program_from_state(program, state);
        let entry = state_inputs.entry(final_state.registers[3]).or_default();
        *entry = if max {
            std::cmp::max(*entry, inp)
        } else {
            std::cmp::min(*entry, inp)
        };
    }
}

fn find_all_possible_states(program: Vec<Instruction>, max: bool) -> HashMap<isize, isize> {
    let mut current_known = HashMap::new();
    let mut local_scratchpad = HashMap::new();
    current_known.insert(0, 0);

    for (i,part) in split_program(program).into_iter().enumerate() {
        let mut next_known = HashMap::new();
        for (state, possible_input) in current_known {
            find_possible_states(state, &part, &mut local_scratchpad, max);
            for (&new_state, &input) in local_scratchpad.iter() {
                let new_input = possible_input * 10 + input;
                if max {
                    if new_input > *next_known.get(&new_state).unwrap_or(&0) {
                        next_known.insert(new_state, new_input);
                    }
                } else {
                    if new_input < *next_known.get(&new_state).unwrap_or(&100000000000000) {
                        next_known.insert(new_state, new_input);
                    }
                }
            }
        }
        current_known = next_known;
        println!("We currently know {} possible final states (After part {} with {} instructions)", current_known.len(), i, part.len());
    }

    current_known
}

fn part1<P: AsRef<Path>>(input: P) -> Result<isize> {
    let program: Vec<Instruction> = stream_items_from_file(input)?.collect();
    Ok(find_all_possible_states(program, true)[&0])
}

fn part2<P: AsRef<Path>>(input: P) -> Result<isize> {
    let program: Vec<Instruction> = stream_items_from_file(input)?.collect();
    Ok(find_all_possible_states(program, false)[&0])
}

const INPUT: &str = "input/day24.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}
