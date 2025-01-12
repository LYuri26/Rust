fn main() {
    let numero = 5;
    let mut anterior = 0;
    let mut atual = 1;

    for i in 0..numero {
        let resultado = anterior + atual;
        anterior = atual;
        atual = resultado;
        println!("{}º Número:{}", i + 1, resultado);
    }
}
