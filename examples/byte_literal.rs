/*
refais le passage majuscule → minuscule, mais en partant de b'P' (un u8 directement). Calcule en u8, convertis en char seulement pour l'affichage.
Résultat attendu : p */

fn main(){
    let majuscule: u8 = b'P'; // Transforme 'A' en valeur decimal directement 80
    let majuscule_to_minuscule: char = (majuscule + 32) as char;
    println!("{majuscule_to_minuscule}");
}
