use super::parse::{Instruction, Program};
use std::io::{self, Write, Read};
use std::num::Wrapping;
use std::error;

pub fn evaluate(prog: &Program, bank: &mut [u8; 256]) -> Result<(), Box<error::Error>> {
    let mut pc = 0;
    let mut tape_pointer = 0u8;
    'run: loop {
        let pc = &mut pc;
        let t = tape_pointer as usize;
        let tape_pointer = &mut tape_pointer;
        if let Some(x) = prog.get(*pc) {
            use self::Instruction::*;
            match x {
                Increment => bank[t] = bank[t].wrapping_add(1),
                Decrement => bank[t] = bank[t].wrapping_sub(1),
                Multiply => bank[t] = bank[t].wrapping_mul(2),
                Divide => bank[t] = bank[t].wrapping_div(2),
                IntInput => {
                        let mut buf = String::new();
                        io::stdin().read_line(&mut buf)?;
                        match buf.trim().parse::<u8>() {
                            Ok(num) => {
                                println!("{}", num);
                                bank[t] = num
                            }
                            Err(_) => bank[t] = 0,
                        };
                }
                CharInput => {
                    bank[t] = io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .unwrap_or(0);
                }
                Clear => bank[t] = 0,
                Square => bank[t] = bank[t].wrapping_pow(2),
                IntOutput => {print!("{}", bank[t]);},
                CharOutput => {
                    io::stdout().write(&[bank[t]]);
                },
                RRot => *tape_pointer = (*tape_pointer).wrapping_add(1),
                LRot => *tape_pointer = (*tape_pointer).wrapping_sub(1),
                JumpIfNotZero(target) => {
                    if bank[t] != 0 {
                        *pc = *target;
                    }
                }
                JumpIfZero(target) => {
                    if bank[t] == 0 {
                        *pc = *target;
                    }
                }
                _ => ()
            }
            *pc += 1;
        } else {
            break 'run
        }
    }
    Ok(())
}

pub fn as_callable<'a, T>(prog: Program) -> Box<Fn(T) -> Result<[u8; 256], Box<error::Error>>>
    where T: IntoIterator<Item = u8>
{
    let prog = Box::new(prog);
    Box::new(move |b| {
        let mut bank = [0; 256];
        for (idx, byte) in b.into_iter().enumerate() {
            bank[idx] = byte.clone();
        }
        evaluate(&*prog, &mut bank)?;
        Ok(bank)
    })
}