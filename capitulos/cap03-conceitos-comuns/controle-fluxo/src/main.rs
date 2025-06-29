fn main() {
    // Expressões if
    let numero = 3;

    if numero < 5 {
        println!("Condição era verdadeira");
    } else {
        println!("Condição era falsa");
    }

    // Multiplas condições
    let numero = 6;
    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("número é divisível por 2")
    } else {
        println!("número não é divisível por 4, 3, ou 2");
    }

    // Usando if em uma declaração let
    let condicao = true;
    let numero = if condicao { 5 } else { 6 };
    println!("O valor do número é: {numero}");

    // Loops

    // Loop infinito (comentado para não travar)

    /*
        loop {
        println!("de novo!");
        }
    */

    // Loop com break
    let mut contador = 0;
    let resultado = loop {
        contador += 1;

        if contador == 10 {
            break contador * 2;
        }
    };
    println!("O resultado é {resultado}");

    // Labels de loop
    let mut count = 0;
    'contando_para_cima: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'contando_para_cima;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");

    // While
    let mut numero = 3;

    while numero != 0 {
        println!("{numero}");

        numero -= 1;
    }

    println!("LIFTOFF");

    // Iterando sobre uma coleção com while (não recomendado)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("o valor é: {}", a[index]);

        index += 1;
    }

    // For (recomendado)
    let a = [10, 20, 30, 40, 50];

    for elemento in a {
        println!("o valor é: {elemento}");
    }

    // For com range
    for numero in (1..4).rev() {
        println!("{numero}!");
    }
    println!("LIFTOFF!!!");
}
