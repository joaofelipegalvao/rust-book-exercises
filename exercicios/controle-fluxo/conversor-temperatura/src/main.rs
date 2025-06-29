use std::io;

fn main() {
    println!("=== Conversor de Temperatura ===");
    println!("1. Celsius para Fahrenheit");
    println!("2. Fahrenheit para Celsius");

    loop {
        println!("\nEscolha uma opção (1 ou 2, 0 para sair):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        let opcao: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número válido!");
                continue;
            }
        };

        if opcao == 0 {
            println!("Saindo...");
            break;
        }

        if opcao == 1 {
            celsius_para_fahrenheit();
        } else if opcao == 2 {
            fahrenheit_para_celsius();
        } else {
            println!("Opção inválida! Digite 1, 2 ou 0");
        }
    }
}

fn celsius_para_fahrenheit() {
    println!("Digite a temperatura em Celsius:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");

    let celsius: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, digite um número válido");
            return;
        }
    };

    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{celsius}°C = {fahrenheit:.2}°F");
}

fn fahrenheit_para_celsius() {
    println!("Digite a temperatura em Fahrenheit:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");

    let fahrenheit: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, digite um número válido!");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F = {celsius:.2}°C");
}
