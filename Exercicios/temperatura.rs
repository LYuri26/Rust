fn main() {
    let fahrenheit = 32;
    let mut celsius = 0;
    println!("A temperatura em fahrenheit é: {}", fahrenheit);
    celsius = (fahrenheit - 32) * 5 / 9;
    println!("A temperatura em celsius é: {}", celsius);
}
