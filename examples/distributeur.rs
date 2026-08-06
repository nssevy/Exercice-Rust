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
#![allow(dead_code)]
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
            Piece::Vingt => 20,
            Piece::Cinquante => 50, 
            Piece::UnEuro => 100,
            Piece::DeuxEuros => 200,
        }
    }

}

impl Boisson {
    // Donne une valeur aux boissons.
    fn prix(&self) -> u32 {
        match self {
            Boisson::Cafe => 80,
            Boisson::The => 60,
            Boisson::Chocolat => 90,
        }
    }
    // Vient recuperer le nom de la boisson.
    fn nom(&self) -> &str {
        match self {
            Boisson::Cafe => "Café",
            Boisson::The => "Thé",
            Boisson::Chocolat => "Chocolat",
        }
    }

}

fn acheter(boisson: &Boisson, pieces: &[Piece]) {
    // Pour la boucle while
    let mut i: usize = 0;
    // Argent est la variable dans laquelle on va stocker la valeur des pieces saisis.
    let mut argent = 0;
    // Prix permet de recuperer le prix.
    let prix = boisson.prix();
    // Boisson permet de recuperer le nom (str).
    let nom = boisson.nom();
    
    //Additionne la valeur des pieces saissis.
    while i < pieces.len() {
        argent += pieces[i].valeur();
        i += 1;
    }

    if argent >= prix {
        let rendu = argent - prix;
    println!("{} : insere {} centimes, Prix {}, rendu {} centimes",nom, argent, prix, rendu);
    } else {
        let manque = prix - argent;
        println!("{} : insere {} centimes, Prix {}, il manque {} centimes",nom, argent, prix, manque);
    }
 
}

fn main() {
    let piece = [Piece::Cinquante, Piece::Vingt, Piece::Dix, Piece::Dix];
    acheter(&Boisson::Cafe, &piece);

    let piece2 = [Piece::Cinquante, Piece::Vingt];
    acheter(&Boisson::Chocolat, &piece2);
}
