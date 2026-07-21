/*
Consigne : pour un caractère donné, affiche true si c'est une majuscule ASCII (A–Z), false sinon, en testant les bornes toi-même.

Résultat attendu (avec 'p') : false
*/

fn est_ce_maj(a: u8, z: u8, comparant: u8) -> bool {
    comparant >= a && comparant <= z
}

fn main() {
    let a_ascii: u8 = 'A' as u8;
    let z_ascii: u8 = 'Z' as u8;

    // variable test
    let test_ascii: u8 = 'V' as u8;

    println!("{:?}", est_ce_maj(a_ascii, z_ascii, test_ascii));
}
