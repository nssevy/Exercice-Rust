/*
Déclare une enum Feu avec trois unit variants : Rouge, Orange, Vert.

Écris une fonction duree(feu: Feu) -> u32 qui renvoie la durée en secondes du feu : 60 pour rouge, 5 pour orange, 45 pour vert.

Dans main, crée les trois valeurs et affiche pour chacune sa durée.

Résultat attendu :

Rouge : 60 secondes
Orange : 5 secondes
Vert : 45 secondes
*/

enum Feu {
    Rouge,
    Orange,
    Vert,
}

fn duree(feu: Feu) -> u32 {

    match feu {
        Feu::Rouge => 60,
        Feu::Orange => 5,
        Feu::Vert => 45,
    }

}

fn main () {
    println!("Rouge : {} secondes", duree(Feu::Rouge));
    println!("Orange : {} secondes", duree(Feu::Orange));
    println!("Vert : {} secondes", duree(Feu::Vert));

}
