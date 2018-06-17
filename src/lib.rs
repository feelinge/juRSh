#![feature(no_panic_pow)]
pub mod parse;
pub mod exec;

#[cfg(test)]
mod tests {
    #[test]
    fn test_parser() {
        use super::parse::{parse, Instruction};
        let code = vec![Instruction::JumpIfZero(1), Instruction::JumpIfNotZero(0)];
        assert_eq!(parse("JoShjOsH"), Ok(code));
    }

    #[test]
    fn test_embedding() {
        use super::{exec, parse};
        let program = parse::parse(include_str!("../resource/adder.jsh")).unwrap();
        let callable = exec::as_callable::<Vec<u8>>(program);
        assert_eq!(callable(vec![1, 2]).unwrap()[2], 3)
    }
}