/*
reprends tes enums Couleur (Blanc, Noir) et TypePiece (Roi, Dame, Tour, Fou, Cavalier, Pion). Définis une struct Piece avec deux champs : couleur de type Couleur, et type_piece de type TypePiece. Dans le main, crée à la main (instancie) une pièce représentant la dame blanche, puis affiche-la entièrement avec {:?}.
Résultat attendu :
Piece { couleur: Blanc, type_piece: Dame }
*/
#![allow(dead_code)]

#[derive(Debug)]
enum Couleur {
    Blanc,
    Noir,
}

#[derive(Debug)]
enum TypePiece {
    Roi,
    Dame,
    Tour,
    Fou,
    Cavalier,
    Pion,
}

#[derive(Debug)]
struct Piece {
    couleur: Couleur,
    type_piece: TypePiece,
}

fn main() {
    let dame_blanche = Piece { couleur: Couleur::Blanc, type_piece: TypePiece::Dame };
    println!("{:?}", dame_blanche);
}
