use std::io;

fn main() {
    println!("Digite um número para ver a tabuada:");
    let mut number = String::new();
    
    io::stdin()
        .read_line(&mut number)
        .expect("Falha ao ler a linha");

    println!("Tabuada do {}:", number.trim());
}
