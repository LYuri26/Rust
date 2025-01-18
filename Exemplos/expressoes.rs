fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 //expressão não tem ponto e virgula.
    };
    println!("O valor de y é: {}", y);
}
