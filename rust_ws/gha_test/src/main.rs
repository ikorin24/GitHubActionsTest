use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <num1> <num2> <num3>",
            Path::new(&args[0]).file_name().unwrap().to_string_lossy()
        );
        std::process::exit(1);
    }

    let a = args[1].parse().unwrap();
    let b = args[2].parse().unwrap();
    let c = args[3].parse().unwrap();
    let ans = mul_add(a, b, c);

    println!("{} * {} + {} = {}", a, b, c, ans);
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
