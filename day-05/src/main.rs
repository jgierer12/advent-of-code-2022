use lazy_static::lazy_static;
use regex::Regex;
use utils::read_input_file;

fn parse_input<'a>(input: &'a String) -> (Vec<&'a str>, Vec<&'a str>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut parts = lines.split(|l| l.is_empty());
    let diagram = parts.next().unwrap().to_vec();
    let instructions = parts.next().unwrap().to_vec();

    (diagram, instructions)
}

fn parse_diagram<'a>(diagram: Vec<&str>) -> Vec<Vec<char>> {
    let (count_line, diagram) = diagram.split_last().unwrap();
    let stack_count: usize = {
        let re = Regex::new(r"\d+$").unwrap();
        re.find(count_line).unwrap().as_str().parse().unwrap()
    };

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..stack_count {
        stacks.push(vec![]);
    }

    for line in diagram.iter().rev() {
        let mut chars = line.chars();
        for i in 0..stack_count {
            match chars.nth(1) {
                Some(' ') => {}
                Some(char) => {
                    stacks[i].push(char);
                }
                None => break,
            };
            chars.nth(1); // advance to next group
        }
    }

    stacks
}

struct CraneInstruction {
    count: usize,
    from: usize,
    to: usize,
}
impl CraneInstruction {
    fn from_str(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let cap = RE.captures(s).unwrap();

        let count = (&cap[1]).parse::<usize>().unwrap();
        let from = (&cap[2]).parse::<usize>().unwrap() - 1;
        let to = (&cap[3]).parse::<usize>().unwrap() - 1;

        Self { count, from, to }
    }
}

enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

struct Crane {
    stacks: Vec<Vec<char>>,
    model: CraneModel,
}
impl Crane {
    fn exec_instr(&mut self, instr: &CraneInstruction) {
        let from_new_len = self.stacks[instr.from].len().saturating_sub(instr.count);

        let payload = self.stacks[instr.from].split_off(from_new_len);

        match self.model {
            CraneModel::CrateMover9000 => {
                self.stacks[instr.to].extend(payload.iter().rev());
            }
            CraneModel::CrateMover9001 => {
                self.stacks[instr.to].extend(payload.iter());
            }
        }
    }

    fn result(&self) -> String {
        let mut result = String::from("");

        for stack in &self.stacks {
            result.push(*stack.last().unwrap());
        }

        result
    }
}

fn main() {
    let input = read_input_file();

    let (diagram, instructions) = parse_input(&input);
    let stacks = parse_diagram(diagram);

    let mut crane_one = Crane {
        stacks: stacks.clone(),
        model: CraneModel::CrateMover9000,
    };
    let mut crane_two = Crane {
        stacks: stacks.clone(),
        model: CraneModel::CrateMover9001,
    };

    for s in instructions {
        let instr = CraneInstruction::from_str(s);
        crane_one.exec_instr(&instr);
        crane_two.exec_instr(&instr);
    }

    println!("Part one result: {:?}", crane_one.result());
    println!("Part two result: {:?}", crane_two.result());
}
