#[derive(Debug, Clone)]
pub struct Input {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
struct Operand(u8);

impl Operand {
    fn as_literal(self) -> u8 {
        self.0
    }

    fn as_combo(self, registers: &Registers) -> usize {
        match self.0 {
            0..=3 => self.0 as usize,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            7 => panic!("reserved operand"),
            _ => panic!("unknown operand"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("invalid opcode"),
        }
    }
}

pub fn parse(input: &str) -> Input {
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let registers: Vec<&str> = parts[0].lines().collect();
    assert_eq!(registers.len(), 3);

    fn parse_register(name: &'static str, line: &str) -> usize {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "Register");
        assert_eq!(parts[1], format!("{name}:"));
        parts[2].parse().unwrap()
    }

    Input {
        register_a: parse_register("A", registers[0]),
        register_b: parse_register("B", registers[1]),
        register_c: parse_register("C", registers[2]),
        program: parts[1]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|c| c.parse::<u8>().unwrap())
            .collect(),
    }
}

#[derive(Debug, Clone)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct State {
    program: Vec<u8>,
    isp: usize,
    registers: Registers,
}

impl State {
    fn run(mut self) -> Vec<usize> {
        let mut output = Vec::new();
        loop {
            if let Some(instruction) = self.program.get(self.isp..=self.isp + 1) {
                let opcode: Opcode = instruction[0].into();
                let operand = Operand(instruction[1]);

                fn div(registers: &Registers, operand: Operand) -> usize {
                    let numerator = registers.a;
                    let denominator = 2_usize.pow(operand.as_combo(registers) as u32);
                    numerator / denominator
                }

                match opcode {
                    Opcode::Adv => {
                        self.registers.a = div(&self.registers, operand);
                        self.isp += 2;
                    }
                    Opcode::Bxl => {
                        self.registers.b = self.registers.b ^ operand.as_literal() as usize;
                        self.isp += 2;
                    }

                    Opcode::Bst => {
                        self.registers.b = operand.as_combo(&self.registers) % 8;
                        self.isp += 2;
                    }
                    Opcode::Jnz => match self.registers.a {
                        0 => {
                            self.isp += 2;
                        }
                        _ => self.isp = operand.as_literal() as usize,
                    },
                    Opcode::Bxc => {
                        self.registers.b = self.registers.b ^ self.registers.c;
                        self.isp += 2;
                    }
                    Opcode::Out => {
                        let o = operand.as_combo(&self.registers) % 8;
                        output.push(o);
                        self.isp += 2;
                    }
                    Opcode::Bdv => {
                        self.registers.b = div(&self.registers, operand);
                        self.isp += 2;
                    }
                    Opcode::Cdv => {
                        self.registers.c = div(&self.registers, operand);
                        self.isp += 2;
                    }
                }
            } else {
                break;
            }
        }
        output
    }
}

impl From<Input> for State {
    fn from(value: Input) -> Self {
        Self {
            program: value.program,
            isp: 0,
            registers: Registers {
                a: value.register_a,
                b: value.register_b,
                c: value.register_c,
            },
        }
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    let state: State = input.clone().into();
    let output = state.run();
    output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
        .into()
}

pub fn part_2(input: &Input) -> crate::Output {
    let state: State = input.clone().into();

    let goal: Vec<usize> = state.program.iter().map(|s| *s as usize).collect();

    for i in 0.. {
        if i % 1_000_000 == 0 {
            println!("{i}");
        }
        let mut state = state.clone();
        state.registers.a = i;
        let output = state.run();
        if output == goal {
            return i.into();
        }
    }
    panic!("no valid value found")
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<&[crate::OutputConst]>; 2] = [
    Some(&[crate::OutputConst::Static("4,6,3,5,6,3,5,2,1,0")]),
    Some(&[crate::OutputConst::Int(117440)]),
];
