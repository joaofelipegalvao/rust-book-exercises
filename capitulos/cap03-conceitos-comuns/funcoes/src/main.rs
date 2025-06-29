fn main() {
    println!("Olá, mundo!");

    outra_funcao();
    funcao_com_parametro(5);
    imprime_medida(5, 'h');

    // Demonstrando statements vs expressions
    let y = {
        let x = 3;
        x + 1 // expressão (sem ponto e vírgula)
    };
    println!("O valor de y é: {y}");

    // Funções com retorno
    let x = cinco();
    println!("O valor de x é: {x}");

    let x = mais_um(5);
    println!("O valor de x é: {x}");
}

fn outra_funcao() {
    println!("Outra função.");
}

fn funcao_com_parametro(x: i32) {
    println!("O valor de x é: {x}");
}

fn imprime_medida(valor: i32, unidade_medida: char) {
    println!("A medida é: {valor}{unidade_medida}");
}

fn cinco() -> i32 {
    5 // expressão de retorno (sem ponto e vírgula)
}

fn mais_um(x: i32) -> i32 {
    x + 1
}

// Exemplos adicionais

fn calcular_area_retangulo(largura: u32, altura: u32) -> u32 {
    largura * altura
}

fn eh_par(numero: i32) -> bool {
    numero % 2 == 0
}

fn maior_numero(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

#[allow(dead_code)]
fn exemplos_adicionais() {
    let area = calcular_area_retangulo(10, 20);
    println!("Área do retângulo: {area}");

    let numero = 42;
    if eh_par(numero) {
        println!("{numero} é par");
    } else {
        println!("{numero} é impar");
    }

    let maior = maior_numero(10, 20);
    println!("Maior número: {maior}");
}
