use std::io;

fn saque(valor: f64, mut saldo: f64) {
    if valor > saldo {
        println!("Saldo insuficiente");
    } else {
        saldo -= valor;
        println!("Saque efetuado. Novo saldo: {}", saldo);
    }
}

fn main() {
    let saldo: f64 = 1000.00;
    let mut valor = String::new();

    println!("Digite o valor:");

    io::stdin()
        .read_line(&mut valor)
        .expect("Erro ao ler a linha");

    let valor: f64 = valor.trim().parse().expect("Número inválido");

    saque(valor, saldo); // Chama a função e exibe a mensagem sem retornar nada
}
