use std::io;

fn primos(numero_primo: i32) {
    let mut verificador = true;
    for i in 2..numero_primo {
        if numero_primo % i != 0 {
            verificador = true;
        } else {
            verificador = false;
            break;
        }
    }

    if verificador == true {
        println!("Número {} é primo!", numero_primo);
    } else {
        println!("Número {} não é primo!", numero_primo);
    }
}

fn main() {
    let mut numero_primo = String::new();

    println!("Digite o número:");

    io::stdin()
        .read_line(&mut numero_primo)
        .expect("Erro ao ler a linha");

    let numero_primo: i32 = numero_primo.trim().parse().expect("Número inválido");
    if numero_primo < 2 {
        println!("Número não é primo!");
    } else {
        primos(numero_primo);
    }
}
