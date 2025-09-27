pub fn reverse(input: &str) -> String {
    let output: String = input.chars().rev().collect();
    output
}

pub fn reverse_robot(input: &str) -> String {
    let input: String = "robot".to_string();
    let output: String = input.chars().rev().collect();
    output
}

pub fn reverse_ramen(input: &str) -> String {
    let input: String = "Ramen".to_string();
    let output: String = input.chars().rev().collect();
    output
}