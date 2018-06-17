pub fn parse(source: &str) -> Result<Program, String> {
    let mut feed = source.chars();
    let mut position = (1usize, 0usize); // Line, column
    let mut program = Program::new();

    'parser: loop {
        let position = &mut position;
        let feed = &mut feed;
        if let Some(ch) = feed.next() {
            match ch {
                '(' => 'parentheses: loop {
                    match feed.next() {
                        Some(')') | None => break 'parentheses,
                        Some(_) => continue,
                    }
                }
                'j' | 'J' => {
                    let ins = feed
                        .take(3)
                        .fold(
                            {
                                let mut v = String::new();
                                v.push(ch);
                                v
                            },
                            |mut accumulator, character| {
                                accumulator.push(character);
                                accumulator
                            });
                    match Instruction::from(ins.as_str())
                    {
                        Instruction::Invalid => return Err(format!("Unsupported instruction {}", ins.clone())),
                        x => program.push(x)
                    }
                },
                '\r' | '\n' | '\t' | ' ' => continue 'parser,
                _ => return Err(format!("{}", ch))
            }
        } else {
            break 'parser
        }
    }

    // Time to validate program and replace jumps with smarter JMP shit
    let mut loops = Vec::<(usize, usize)>::new();
    let mut stack = Vec::<usize>::new();  // Holds loop starts
    for (idx, item) in program.iter().enumerate() {
        match *item {
            Instruction::LoopStart => {
                stack.push(idx);
            }
            Instruction::LoopEnd => {
                if let Some(x) = stack.pop() {
                    loops.push((x, idx));
                } else {
                    return Err("Extra loop end instruction".into());
                }
            }
            _ => ()
        }
    }

    if !stack.is_empty() {
        // Invalid script-- unmatched loop tags
        return Err("Unbalanced loop end instructions!".into());
    }

    for (start, end) in loops.iter() {
        program[*start] = Instruction::JumpIfZero(*end);
        program[*end] = Instruction::JumpIfNotZero(*start);
    }

    Ok(program)
}

pub struct ParseError((String, usize, usize));

pub type Program = Vec<Instruction>;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Increment,
    Decrement,
    Multiply,
    Divide,
    IntInput,
    CharInput,
    Clear,
    Square,
    IntOutput,
    CharOutput,
    RRot,
    LRot,
    LoopStart,
    LoopEnd,
    JumpIfZero(usize),
    JumpIfNotZero(usize),
    Invalid,
}

impl<'a> From<&'a str> for Instruction {
    fn from(token: &'a str) -> Self {
        use self::Instruction::*;
        match token {
            "JOSH" => Increment,
            "josh" => Decrement,
            "Josh" => Multiply,
            "josH" => Divide,
            "JOsh" => CharOutput,
            "JoSH" => IntInput,
            "JOsH" => CharInput,
            "jOsh" => Clear,
            "JOSh" => Square,
            "JosH" => IntOutput,
            "joSH" => RRot,
            "joSh" => LRot,
            "JoSh" => LoopStart,
            "jOsH" => LoopEnd,
            _ => Invalid
        }
    }
}