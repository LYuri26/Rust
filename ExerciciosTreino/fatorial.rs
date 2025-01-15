use std::io;

fn fatorial(numero_fatorial: i32) {
    let mut resultado = 1;

    for i in 1..=numero_fatorial {
        resultado *= i;
    }

    println!("O fatorial de {} é {}", numero_fatorial, resultado);
}

fn main() {
    let mut numero_fatorial = String::new();

    println!("Digite o número:");

    io::stdin()
        .read_line(&mut numero_fatorial)
        .expect("Erro ao ler a linha");

    let numero_fatorial: i32 = numero_fatorial.trim().parse().expect("Número inválido");
    if numero_fatorial < 1 {
        println!("Número inválido!");
    } else {
        fatorial(numero_fatorial);
    }
}
