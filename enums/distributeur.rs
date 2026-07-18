/*
Un distributeur de boissons. Déclare une enum Piece avec les variants Dix, Vingt, Cinquante, UnEuro, DeuxEuros, chacun sans donnée.

Déclare une enum Boisson avec Cafe, The, Chocolat.

Écris impl Piece avec une méthode valeur(&self) -> u32 renvoyant la valeur en centimes.
Écris impl Boisson avec une méthode prix(&self) -> u32 renvoyant le prix en centimes : café 80, thé 60, chocolat 90.

Écris une fonction libre acheter(boisson: &Boisson, pieces: &[Piece]) qui additionne la valeur des pièces insérées et affiche soit le rendu de monnaie, soit ce qu'il manque.

Dans main, tente deux achats : un café avec [Cinquante, Vingt, Dix, Dix], puis un chocolat avec [Cinquante, Vingt].

Résultat attendu :

Cafe : insere 90 centimes, prix 80, rendu 10 centimes
Chocolat : insere 70 centimes, prix 90, il manque 20 centimes

*/

enum Piece {
    // Centimes
    Dix,
    Vingt,
    Cinquante,
    // Euros
    UnEuro,
    DeuxEuros,
}

enum Boisson {
    Cafe,
    The,
    Chocolat,
}

impl Piece {

    fn valeur(&self) -> u32 {
        match self {
            // valeur en centimes
            Piece::Dix => 10,
            Piece::Vignt => 20,
            Piece::Cenquante => 50,
            Piece::UnEuro => 100,
            Piece::Deux => 200,
        }
    }

}

impl Boisson {

    fn prix(&self) -> u32 {
        match self {
            Boisson::Cafe => 80,
            Boisson::The => 60,
            Boisson::Chocolat => 90,
        }
    }

}

fn acheter(boisson: &Boisson, pieces: &[Piece]) {
    
}

fn main() {
    acheter(&Boisson::Cafe, 90)
}
