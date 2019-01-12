//gcd, gcd test, and a good portion of main are from Programming Rust: Fast, Safe, System Development

use std::env;
use std::io::Write;
use std::str::FromStr;
fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn sum(input: &Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for num in input.iter() {
        sum += num;
    }
    sum
}

#[test]
fn test_sum() {
    let s: Vec<u64> = [10, 13, 24].to_vec();
    assert_eq!(sum(&s), 47);
}

fn product(input: &Vec<u64>) -> u64 {
    let mut prod: u64 = 1;
    for num in input.iter() {
        prod *= num;
    }
    prod
}

#[test]
fn test_product() {
    let s: Vec<u64> = [1, 2, 3].to_vec();
    assert_eq!(product(&s), 6);
}
//Help computing LCM came from //www.geeksforgeeks.org/lcm-of-given-array-elements/
fn lcm(input: &Vec<u64>) -> u64 {
    let mut i = 1;
    let mut ans = input[0];
    for _num in &input[1..] {
        ans = ans * input[i] / gcd(ans, input[i]);
        i += 1;
    }
    ans
}

#[test]
fn test_lcm() {
    let s: Vec<u64> = [10, 14, 12].to_vec();
    assert_eq!(lcm(&s), 420);
}

fn main() {
    let mut numbers = Vec::new();
    let args: Vec<String> = env::args().collect();
    let command = args[1].to_string();
    let mut answer: u64 = 0;

    for arg in std::env::args().skip(2) {
        numbers.push(u64::from_str(&arg).expect("error parsing args"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER...").unwrap();
        std::process::exit(1);
    }
    //super ugly not 0 check. Couldn't figure our another way to check &u64 to &u64 equality.
    for num in numbers.iter() {
        assert!(num != &answer);
		assert!(num > &answer);
    }

    if command == "sum".to_string() {
        answer = sum(&numbers);
    } else if command == "product".to_string() {
        answer = product(&numbers);
    } else if command == "gcd".to_string() {
        let d = numbers[0];

        for m in &numbers[1..] {
            answer = gcd(d, *m)
        }
    } else if command == "lcm".to_string() {
        answer = lcm(&numbers);
    } else {
        println!(
            "Error parsing args. Please use name of the function you want to run (gcd, lcm..ect)"
        );
        std::process::exit(1);
    }

    println!("The {} of {:?} is {}", command, numbers, answer);
}
