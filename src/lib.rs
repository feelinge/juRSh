#![feature(no_panic_pow)]
mod parse;
mod exec;

#[cfg(test)]
mod tests {
    #[test]
    fn test_parser() {
        use super::parse::{parse, Instruction};
        let code = vec![Instruction::JumpIfZero(1), Instruction::JumpIfNotZero(0)];
        assert_eq!(parse("JoShjOsH"), Ok(code));
    }
}