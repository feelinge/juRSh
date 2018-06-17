#![feature(no_panic_pow)]
mod parse;
mod exec;

fn main() {
    let code = parse::parse(include_str!("test.jsh")).unwrap();
    let adder = exec::as_callable(code);
    println!("1 + 2 is: {}", adder(vec![1, 2])[2]);
}