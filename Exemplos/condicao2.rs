fn main() {
    let numero = 6;
    if numero % 4 == 0 {
        println!("Número divisivel por quatro.");
    } else if numero % 3 == 0 {
        println!("Número divisivel por três.");
    } else if numero % 2 == 0 {
        println!("Número divisivel por dois.")
    } else {
        println!("Número não é divisivel por dois, três ou quatro.")
    }
}
