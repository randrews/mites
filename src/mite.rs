use crate::error::MiteError;

pub type Point = (u32, u32);

#[derive(Copy, Clone)]
pub enum Dir {
    North,
    South,
    East,
    West
}

#[derive(Copy, Clone)]
pub enum Instruction {
    Noop,
    Forward,
    Left,
    Right,
    Flip,
    Query,
    Clone,
    Mutate
}

pub type Program = Vec<Instruction>;

pub struct Mite {
    loc: Point,
    dir: Dir,
    energy: u32,
    pc: usize,
    program: Program
}

impl Mite {
    pub fn new(program: &str) -> Result<Self, MiteError> {
        Ok(Self {
            loc: (0, 0),
            dir: Dir::North,
            energy: 100,
            pc: 0,
            program: <Program as FromStr>::from(program)?,
        })
    }

    pub fn instruction(&self) -> Instruction {
        self.program[self.pc % self.program.len()]
    }
}

pub trait FromStr: Sized {
    fn from(s: &str) -> Result<Self, MiteError>;
}

impl FromStr for Program {
    fn from(s: &str) -> Result<Self, MiteError> {
        let mut v = Program::new();
        for c in s.chars() {
            v.push(c.try_into()?)
        }
        Ok(v)
    }
}

impl TryFrom<char> for Instruction {
    type Error = MiteError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Instruction::*;
        match value {
            '<' => Ok(Left),
            '>' => Ok(Right),
            '^' => Ok(Forward),
            '.' | ' ' => Ok(Noop),
            '!' => Ok(Flip),
            '?' => Ok(Query),
            '&' => Ok(Clone),
            '/' => Ok(Mutate),
            _ => Err(MiteError::InstructionError(value))
        }
    }
}