fn main() {
    // Tipos escalares

    // Inteiros
    let a: i32 = 42;
    let b: u64 = 100;
    let c = 98_222; // separadores visuais
    let d = 0xff; // hexadecimal
    let e = 0o77; // octal
    let f = 0b1111_0000; // binário
    let g = b'A'; // byte (apenas u8)

    println!("Inteiros: {a}, {b}, {c}, {d}, {e}, {f}, {g}");

    // Ponto flutuante
    let h = 2.0; // f64 por padrão
    let i: f32 = 3.0; // f32

    println!("Ponto fluautante: {h}, {i}");

    // Operações matemáticas
    let soma = 5 + 10;
    let diferenca = 95.5 - 4.3;
    let produto = 4 * 30;
    let quociente = 56.7 / 32.2;
    let quociente_inteiro = 2 / 3;
    let resto = 43 % 5;

    println!(
        "Operações: {soma}, {diferenca}, {produto}, {quociente}, {quociente_inteiro}, {resto}"
    );

    // Booleano
    let verdadeiro = true;
    let falso: bool = false;

    println!("Booleanos: {verdadeiro}, {falso}");

    // Caractere
    let j = 'z';
    let k = 'ℤ';
    let l = '😻';

    println!("Caracteres: {j}, {k}, {l}");

    // Tipos compostos

    // Tupla
    let tupla: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tupla; // desestruturação
    let primeiro = tupla.0;

    println!("Tupla: ({x}, {y}, {z}), primeiro elemento: {primeiro}");

    // Array
    let array = [1, 2, 3, 4, 5];
    let array_mesmo_valor = [3; 5]; // [3, 3, 3, 3, 3]
    let primeiro_elemento = array[0];
    let segundo_elemento = array[1];

    println!("Array: primeiro={primeiro_elemento}, segundo={segundo_elemento}");
    println!("Array mesmo valor: {:?}", array_mesmo_valor);

    // Demonstração de erro de íncide (comentado para não quebrar)
    // let indice = 10;
    // let elemento = array[indice]; // panic!
}
