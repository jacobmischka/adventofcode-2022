use std::str::FromStr;

pub fn main(input: &str) -> (String, String) {
    let mut cpu = CPU::new(input);

    let mut sig_strength = 0;
    let mut screen = String::new();

    while cpu.pc < cpu.program.len() || cpu.pending_instruction.is_some() {
        if cpu.pc < cpu.program.len() && cpu.pending_instruction.is_none() {
            cpu.load_instruction();
        }

        let x_pos = cpu.cycle % 40;

        if x_pos == 0 {
            screen.push_str("\n");
        }

        if (x_pos as i32).abs_diff(cpu.x) <= 1 {
            screen.push_str("#");
        } else {
            screen.push_str(".");
        }

        if cpu.cycle == 20 || (cpu.cycle > 20 && (cpu.cycle - 20) % 40 == 0) {
            sig_strength += cpu.cycle as i32 * cpu.x;
        }

        cpu.complete_cycle();

        cpu.cycle += 1;
    }

    (sig_strength.to_string(), screen)
}

#[derive(Debug)]
struct CPU {
    cycle: u32,

    x: i32,

    pc: usize,
    program: Vec<Instruction>,
    pending_instruction: Option<(Instruction, u32)>,
}

impl CPU {
    fn new(program: &str) -> Self {
        CPU {
            cycle: 0,
            x: 1,
            pc: 0,
            program: program
                .lines()
                .map(|s| Instruction::from_str(s).unwrap())
                .collect(),
            pending_instruction: None,
        }
    }

    fn complete_cycle(&mut self) {
        if let Some(mut inst) = self.pending_instruction {
            inst.1 -= 1;
            if inst.1 == 0 {
                self.do_instruction(inst.0);
                self.pending_instruction = None;
            } else {
                self.pending_instruction = Some(inst)
            }
        }
    }

    fn load_instruction(&mut self) {
        let inst = self.program[self.pc];
        self.pc += 1;
        self.pending_instruction = Some((inst, inst.cycles()));
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(op) => {
                self.x += op as i32;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split_whitespace();
        match pieces.next().ok_or("missing operation".to_string())? {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let operand = pieces.next().ok_or("missing operand".to_string())?;
                let operand: i32 = operand
                    .parse()
                    .map_err(|_| format!("invalid operand {operand}"))?;
                Ok(Instruction::Addx(operand))
            }
            x => Err(format!("invalid operation {x}")),
        }
    }
}
