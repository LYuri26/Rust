fn cinco() -> i32 {
    5
}

fn main() {
    let x = cinco();
    let y = soma_um(5);
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
}

fn soma_um(y: i32) -> i32 {
    y + 1
}
