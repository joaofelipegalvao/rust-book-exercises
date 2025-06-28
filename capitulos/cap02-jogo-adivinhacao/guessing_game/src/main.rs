// Importa módulos necessários da biblioteca padrão do Rust
use std::{cmp::Ordering, io}; // cmp::Ordering para comparações, io para entrada/saída
//

// Importa o trait Rng da crate externa 'rand' para gerar números aleatórios
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Gera um número aleatório entre 1 e 100 (inclusive)
    // rand::rng() cria um gerador de números aleatórios
    // random_range(1..=100) gera um número no intervalo [1, 100]
    let secret_number = rand::rng().random_range(1..=100);

    // Loop infinito - continuará até encontrarmos um 'break'
    loop {
        println!("Please input your guess.");

        // Cria uma String mutável vazia para armazenar a entrada do usuário
        // 'mut' torna a variável mutável (pode ser modificada)
        let mut guess = String::new();

        // Lê uma linha da entrada padrão (teclado)
        io::stdin()
            .read_line(&mut guess) // &mut = referência mutável para a string
            .expect("Failed to read line"); // Trata erro - programa para se falhar

        // Converte a string para número u32 (inteiro sem sinal de 32 bits)
        // guess.trim() remove espaços e quebras de linha
        // parse() tenta converter string para o tipo especificado
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // Se conversão foi bem-sucedida, usa o número
            Err(_) => continue, // Se falhou, ignora o erro e continua o loop
        };

        println!("Your guessed: {}", guess);

        // Compara o palpite com o número secreto usando pattern matching
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // Palpite menor que o número
            Ordering::Greater => println!("Too big!"), // Palpite maior que o número
            Ordering::Equal => {
                // Palpite igual ao número
                println!("You win!");
                break; // Sai do loop - fim do jogo
            }
        }
    } // Fim do loop - volta para o início se não houve 'break'
}
