fn main() {
    println!("=== Os Doze Dias de Natal ===\n");

    let dias = [
        "primeiro",
        "segundo",
        "terceiro",
        "quarto",
        "quinto",
        "sexto",
        "sétimo",
        "oitavo",
        "nono",
        "décimo primeiro",
        "décimo segundo",
    ];

    let presentes = [
        "uma perdiz numa pereira",
        "duas rolas",
        "três galinhas francesas",
        "quatro pássaros cantando",
        "cinco anéis dourados",
        "seis gansos botando ovos",
        "sete cisnes nadando",
        "oito vacas ordenhando",
        "nove damas dançando",
        "dez lordes saltando",
        "onze gaiteiros tocando",
        "doze bateristas batendo",
    ];

    for dia in 0..12 {
        println!("No {} dia de Natal,", dias[dia]);
        println!("meu amor verdadeiro me deu:");

        // Imprime os presentes do dia atual até o primeiro
        for presente_idx in (0..=dia).rev() {
            if dia > 0 && presente_idx == 0 {
                println!("e ");
            }
            println!("{}", presentes[presente_idx]);
        }

        println!();
    }
}
