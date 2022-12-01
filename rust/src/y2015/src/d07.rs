use prelude::*;

pub struct Day07;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Var(String);

#[derive(Debug, Clone)]
pub enum Operand {
    Var(Var),
    Literal(u16),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Assign(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, u8),
    RShift(Operand, u8),
}

#[derive(Debug, Clone)]
pub struct Instruction(Operation, Var);

pub trait CapturesExtensions {
    fn get_var(&self, idx: usize) -> Var;
    fn get_operand(&self, idx: usize) -> Operand;
}

impl CapturesExtensions for inputs::Captures<'_> {
    fn get_var(&self, idx: usize) -> Var {
        Var(self.get_string(idx))
    }

    #[allow(clippy::option_if_let_else)]
    fn get_operand(&self, idx: usize) -> Operand {
        let val = self.get_string(idx);

        match val.parse::<u16>() {
            Ok(literal) => Operand::Literal(literal),
            Err(_) => Operand::Var(Var(val)),
        }
    }
}

impl AocSolution for Day07 {
    type Input = Vec<Instruction>;
    fn process_input(input: &str) -> Self::Input {
        inputs::regexes(
            input,
            vec![
                (
                    r#"(\w+) -> (\w+)"#,
                    Box::new(move |c| {
                        Instruction(Operation::Assign(c.get_operand(1)), c.get_var(2))
                    }),
                ),
                (
                    r#"NOT (\w+) -> (\w+)"#,
                    Box::new(move |c| {
                        Instruction(Operation::Not(c.get_operand(1)), Var(c.get_string(2)))
                    }),
                ),
                (
                    r#"(\w+) AND (\w+) -> (\w+)"#,
                    Box::new(move |c| {
                        Instruction(
                            Operation::And(c.get_operand(1), c.get_operand(2)),
                            c.get_var(3),
                        )
                    }),
                ),
                (
                    r#"(\w+) OR (\w+) -> (\w+)"#,
                    Box::new(move |c| {
                        Instruction(
                            Operation::Or(c.get_operand(1), c.get_operand(2)),
                            c.get_var(3),
                        )
                    }),
                ),
                (
                    r#"(\w+) LSHIFT (\d+) -> (\w+)"#,
                    Box::new(move |c| {
                        Instruction(
                            Operation::LShift(c.get_operand(1), c.get_u8(2)),
                            Var(c.get_string(3)),
                        )
                    }),
                ),
                (
                    r#"(\w+) RSHIFT (\d+) -> (\w+)"#,
                    Box::new(move |c| {
                        Instruction(
                            Operation::RShift(c.get_operand(1), c.get_u8(2)),
                            Var(c.get_string(3)),
                        )
                    }),
                ),
            ],
        )
    }

    const PART1_SOLUTION: Solution = solution(3176);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        run_instructions(input)[&Var("a".to_owned())]
    }

    const PART2_SOLUTION: Solution = solution(14710);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let a_state = run_instructions(input)[&Var("a".to_owned())];

        let mut instructions = input.clone();

        let var_b = Var("b".to_owned());

        // Replace anything assigning to 'b' with a literal assignment
        instructions = instructions
            .into_iter()
            .map(|i| match i {
                Instruction(_, var) if var == var_b => {
                    Instruction(Operation::Assign(Operand::Literal(a_state)), var_b.clone())
                }
                _ => i,
            })
            .collect();

        run_instructions(&instructions)[&Var("a".to_owned())]
    }
}

fn run_instructions(instructions: &[Instruction]) -> HashMap<Var, u16> {
    let mut vars: HashMap<Var, u16> = HashMap::new();
    let mut instructions = instructions.to_vec();

    while !instructions.is_empty() {
        instructions.retain(|i| run_instruction(i, &mut vars).is_err());
    }

    vars
}

fn run_instruction(instruction: &Instruction, vars: &mut HashMap<Var, u16>) -> Result<(), ()> {
    let Instruction(operation, dst) = instruction;
    *vars.entry(dst.clone()).or_insert(0) = match operation {
        Operation::Assign(src) => get(vars, src)?,
        Operation::Not(src) => !get(vars, src)?,
        Operation::And(s1, s2) => get(vars, s1)? & get(vars, s2)?,
        Operation::Or(s1, s2) => get(vars, s1)? | get(vars, s2)?,
        Operation::LShift(src, shift) => get(vars, src)? << shift,
        Operation::RShift(src, shift) => get(vars, src)? >> shift,
    };

    Ok(())
}

fn get(vars: &mut HashMap<Var, u16>, src: &Operand) -> Result<u16, ()> {
    match src {
        Operand::Var(s) => vars.get(s).ok_or(()).map(|r| *r),
        Operand::Literal(lit) => Ok(*lit),
    }
}
