fn main() {
    let presentes = [
        "uma perdiz num pé de peras",
        "dois pombos da paz",
        "três galinhas francesas",
        "quatro pássaros cantores",
        "cinco anéis de ouro",
        "seis gansos pondo ovos",
        "sete cisnes nadando",
        "oito criadas ordenhando",
        "nove damas dançando",
        "dez senhores saltando",
        "onze flautistas tocando",
        "doze bateristas tocando",
    ];

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
        "décimo",
        "décimo primeiro",
        "décimo segundo",
    ];

    for indice_dias in 0..dias.len() {
        println!("{} Dia", dias[indice_dias]);
        for i in 0..=indice_dias {
            println!(" {}", presentes[i]);
        }
    }
}
