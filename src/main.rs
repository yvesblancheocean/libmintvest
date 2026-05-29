const SVC: &str = "config-parser-9ef0c3";
fn fibonacci(n: u64) -> u64 { match n { 0 => 0, 1 => 1, _ => { let (mut a, mut b) = (0u64, 1u64); for _ in 2..=n { let tmp = a + b; a = b; b = tmp; } b } } }
fn main() { println!("[{}] Computing fibonacci sequence...", SVC); for i in [5, 10, 20, 30, 40] { println!("  fib({}) = {}", i, fibonacci(i)); } }
