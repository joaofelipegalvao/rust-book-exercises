fn main() {
    // Variaveis imutáveis por padrão
    let x = 5;
    println!("O valor de x é: {x}");

    // x = 6; // erro: cannot assign twice to immutable variable

    // Variávies imutáveis
    let mut y = 5;
    println!("O valor de y é: {y}");
    y = 6;
    println!("O valor de y agora é: {y}");

    // Shadowing (sombreamento)
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("O valor de z no espoco interno é: {z}"); // 12
    }
    println!("O valor de z é: {z}"); // 6

    // Shadowing com mudança de tipo
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Número de espaços: {spaces}");

    // Constantes
    const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;
    println!("Três horas em segundos: {TRES_HORAS_EM_SEGUNDOS}");
}
