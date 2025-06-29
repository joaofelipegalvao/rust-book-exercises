use std::io;

fn main() {
    println!("=== Gerador de Fibonacci ===");

    loop {
        println!("\nDigite qual posição da sequência Fibonacci você quer (0 para sair):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        let n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número válido!");
                continue;
            }
        };

        if n == 0 {
            println!("Saindo...");
            break;
        }

        let resultado = fibonacci(n);
        println!("O {n}º número de Fibonacci é: {resultado}");

        // Mostra a sequência até n
        println!("Sequência até a posição {n}:");
        for i in 1..=n {
            print!("{} ", fibonacci(i));
        }
        println!();
    }
}

fn fibonacci(n: u32) -> u64 {
    if n <= 2 {
        1
    } else {
        let mut a = 1;
        let mut b = 1;

        for _ in 3..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }

        b
    }
}
