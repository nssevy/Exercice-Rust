/*Un distributeur de boissons qui contient un stock de canettes et on veut pouvoir
en distribuer une, sachant qu'il peut être vide.*/
#![allow(dead_code)]
#[derive(Debug)]
enum Boissons {
    Coca,
    Pepsi,
    Orangina,
    Fanta,
    Oasis
}
#[derive(Debug)]
struct Distributeur {
    boissons: Boissons,
    stock: i32
}
fn main() {

}