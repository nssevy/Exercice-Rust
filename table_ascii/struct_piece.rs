/*
reprends tes enums Couleur (Blanc, Noir) et TypePiece (Roi, Dame, Tour, Fou, Cavalier, Pion). Définis une struct Piece avec deux champs : couleur de type Couleur, et type_piece de type TypePiece. Dans le main, crée à la main (instancie) une pièce représentant la dame blanche, puis affiche-la entièrement avec {:?}.
Résultat attendu :
Piece { couleur: Blanc, type_piece: Dame }

ajoute un bloc impl Piece contenant une méthode statique new qui prend deux paramètres, couleur: Couleur et type_piece: TypePiece, et renvoie la Piece correspondante (-> Piece). Dans le main, remplace ta création manuelle par un appel à Piece::new(...) pour fabriquer la dame blanche, puis affiche-la avec {:?}.
*/
#![allow(dead_code)]

#[derive(Debug)]
enum Couleur {
    Blanc, //majuscule
    Noir, //minuscule
}

#[derive(Debug)]
enum TypePiece {
    Roi, //k
    Dame, //q
    Tour, //r
    Fou,//b
    Cavalier,//n
    Pion,//p
}

#[derive(Debug)]
struct Piece {
    couleur: Couleur,
    type_piece: TypePiece,
}

impl Piece{
    fn new(couleur: Couleur,type_piece: TypePiece) -> Piece{
       Piece {couleur, type_piece} 
    }
}

fn main() {
    let dame = Piece::new(Couleur::Blanc, TypePiece::Dame );

    let roi = Piece::new(Couleur::Noir, TypePiece::Roi);

    println!("{:?}", dame);
    println!("{:?}", roi);
}
