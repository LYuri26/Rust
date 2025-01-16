use std::io;

// Função para realizar saque
fn realizar_saque(valor: f64, saldo_atual: &mut f64) {
    if valor > *saldo_atual {
        println!("Saldo insuficiente.");
    } else {
        *saldo_atual -= valor;
        println!("Saque efetuado. Novo saldo: R${:.2}", saldo_atual);
    }
}

// Função para realizar depósito
fn realizar_deposito(valor: f64, saldo_atual: &mut f64) {
    if valor < 0.0 {
        println!("Não é possível depositar valor negativo.");
    } else {
        *saldo_atual += valor;
        println!("Depósito efetuado. Novo saldo: R${:.2}", saldo_atual);
    }
}

// Função para exibir saldo
fn exibir_saldo(saldo_atual: f64) {
    println!("Saldo atual: R${:.2}", saldo_atual);
}

fn main() {
    let mut saldo: f64 = 1000.00;
    let mut deseja_continuar = String::new(); // Variável de controle para continuar o loop

    while deseja_continuar != "não" {
        // Escolher operação
        println!("Escolha a operação desejada (saque, saldo ou deposito):");
        let mut operacao = String::new(); // Variável para armazenar a operação escolhida pelo usuário
        io::stdin()
            .read_line(&mut operacao)
            .expect("Erro ao ler a entrada.");
        operacao = operacao.trim().to_lowercase(); // Normaliza a entrada

        if operacao == "saldo" {
            exibir_saldo(saldo);
            continue;
        }

        // Solicitar valor
        println!("Digite o valor:");
        let mut valor_digitado = String::new(); // Variável para armazenar o valor digitado pelo usuário
        io::stdin()
            .read_line(&mut valor_digitado)
            .expect("Erro ao ler a entrada.");

        // Converter o valor para f64
        let valor: f64 = valor_digitado
            .trim()
            .parse()
            .expect("Erro: Número inválido.");

        // Executar operação escolhida
        match operacao.as_str() {
            "saque" => realizar_saque(valor, &mut saldo),
            "deposito" => realizar_deposito(valor, &mut saldo),
            _ => println!("Opção inválida! Escolha entre 'saque', 'saldo' ou 'deposito'."),
        }

        // Perguntar se o usuário deseja continuar
        println!("Deseja realizar outra operação? Digite 'não' para encerrar ou qualquer outra tecla para continuar.");
        deseja_continuar.clear(); // Limpa o conteúdo da variável de controle
        io::stdin()
            .read_line(&mut deseja_continuar)
            .expect("Erro ao ler a entrada.");
        deseja_continuar = deseja_continuar.trim().to_lowercase(); // Normaliza a entrada
    }

    println!("Operações encerradas. Obrigado por utilizar nosso sistema!");
}
