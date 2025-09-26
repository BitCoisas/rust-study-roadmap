use std::io;

fn main() {
    println!("Digite um número para ver a tabuada:");
    let mut number = String::new();
    
    io::stdin()
        .read_line(&mut number)
        .expect("Falha ao ler a linha");

    println!("Tabuada do {}:", number.trim());
    for i in 1..=10 {
        let result = number.trim().parse::<i32>().unwrap() * i;
        println!("{} x {} = {}", number.trim(), i, result);
    }
}
