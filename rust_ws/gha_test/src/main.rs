use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    let ans = mul_add(args.num1, args.num2, args.num3);
    println!("hello, world!");
    println!("{} * {} + {} = {}", args.num1, args.num2, args.num3, ans);
}

fn mul_add(a: u64, b: u64, c: u64) -> u64 {
    gha_test_lib::add(a * b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_add() {
        let result = mul_add(2, 2, 2);
        assert_eq!(result, 6);
    }
}

#[derive(Debug, Parser)]
#[clap(about = "calculate: <NUM1> * <NUM2> + <NUM3>")]
struct CliArgs {
    num1: u64,
    num2: u64,
    num3: u64,
}
