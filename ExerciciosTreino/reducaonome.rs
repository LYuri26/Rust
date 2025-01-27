use std::io;

fn redutor_frase(frase: &str, redutor: &str) {
    let palavras_reduzidas: Vec<&str> = redutor.split_whitespace().collect();
    let frase_reduzida: Vec<&str> = frase.split_whitespace().collect();
    let mut palavras_filtradas: Vec<&str> = Vec::new();

    for palavra in frase_reduzida {
        if !palavras_reduzidas.contains(&palavra) {
            palavras_filtradas.push(palavra);
        }
    }

    println!("{}", palavras_filtradas.join(" "));
}

fn main() {
    let mut frase = String::new();
    let mut redutor = String::new();

    println!("Digite a frase:");
    io::stdin()
        .read_line(&mut frase)
        .expect("Erro ao ler a entrada.");
    frase = frase.trim().to_lowercase(); // Normaliza a entrada

    println!("Digite as palavras para remover:");
    io::stdin()
        .read_line(&mut redutor)
        .expect("Erro ao ler a entrada.");
    redutor = redutor.trim().to_lowercase(); // Normaliza a entrada

    redutor_frase(&frase, &redutor);
}
