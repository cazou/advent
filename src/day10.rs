use crate::traits::AdventOfCode;
use anyhow::{bail, Result};
use std::str::FromStr;

enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr, value) = match s.split_once(" ") {
            Some((i, v)) => (i, v),
            None => (s, ""),
        };

        if instr == "addx" {
            Ok(Instruction::Addx(value.parse()?))
        } else if instr == "noop" {
            Ok(Instruction::Noop)
        } else {
            bail!("Cannot parse {}", s)
        }
    }
}

struct CRT {
    display: Vec<String>,
    line: u8,
}

impl CRT {
    pub fn new() -> CRT {
        CRT {
            display: vec![],
            line: 0,
        }
    }

    pub fn update_at(&mut self, clk: usize, reg: isize) {
        if clk % 40 == 0 {
            self.display.push(String::new());
            self.line = self.display.len() as u8 - 1;
        }

        if (reg - 1..=reg + 1).contains(&(clk as isize % 40)) {
            self.display[self.line as usize].push('#');
        } else {
            self.display[self.line as usize].push(' ');
        }
    }
}

impl ToString for CRT {
    fn to_string(&self) -> String {
        self.display.join("\n")
    }
}

struct CPU {
    code: Vec<Instruction>,
    reg: isize,
    pc: usize,
    load: Option<isize>,
    crt: CRT,
}

impl CPU {
    pub fn init() -> CPU {
        CPU {
            reg: 1,
            code: vec![],
            pc: 0,
            load: None,
            crt: CRT::new(),
        }
    }
    pub fn load_program(&mut self, program: &str) -> Result<()> {
        for line in program.lines() {
            self.code.push(line.parse()?);
        }

        Ok(())
    }

    fn clock(&mut self) {
        match (&self.code[self.pc], self.load) {
            (Instruction::Noop, _) => self.pc += 1,
            (Instruction::Addx(v), None) => self.load = Some(*v),
            (Instruction::Addx(_), Some(l)) => {
                self.load = None;
                self.reg += l;
                self.pc += 1;
            }
        }
    }

    pub fn run(&mut self) -> isize {
        let mut clock_count = 0;
        let mut ret = 0;
        while self.pc < self.code.len() {
            self.crt.update_at(clock_count, self.reg);
            clock_count += 1;

            if clock_count >= 20 && (clock_count - 20) % 40 == 0 {
                ret += clock_count as isize * self.reg;
            }
            self.clock();
        }
        ret
    }

    pub fn display(&self) -> String {
        self.crt.to_string()
    }
}

pub struct Day10;

impl AdventOfCode for Day10 {
    fn day(&self) -> u8 {
        10
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut cpu = CPU::init();
        cpu.load_program(&input.unwrap())?;
        Ok(cpu.run().to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut cpu = CPU::init();
        cpu.load_program(&input.unwrap())?;
        cpu.run();
        Ok(cpu.display())
    }
}
