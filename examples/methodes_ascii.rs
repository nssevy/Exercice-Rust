/*
Ecris une fonction qui prend un char en paramètre et renvoie (-> char) sa version minuscule, en utilisant la méthode toute faite to_ascii_lowercase(). Aucune arithmétique, aucun as u8. Dans le main, appelle-la sur 'P', puis sur '5', et affiche les deux résultats.

Résultat attendu à l'exécution :
p
5
*/

fn majuscule_to_minuscule(a: char) -> char {
    a.to_ascii_lowercase()
}

fn main() {
    println!("{}", majuscule_to_minuscule('P'));
    println!("{}", majuscule_to_minuscule('5'));
}
