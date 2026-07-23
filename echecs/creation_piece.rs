#![allow(dead_code)]

#[derive(Debug)]
enum TypePiece {
    Roi, //K ing
    Dame, // Q uenne
    Cavalier, // N knignt
    Tour,// R ook
    Fou, // B ishop
    Pion // P awn
}

#[derive(Debug)]
enum Couleur {
    Blanc, // Majucule
    Noir, // Minuscule
}

#[derive(Debug)]
struct Piece {
    couleur: Couleur,
    type_piece : TypePiece,
}

fn couleur_piece(a: char) -> Option<Couleur> {
    if a.is_ascii_uppercase() {
        Some(Couleur::Blanc)
    } else if a.is_ascii_lowercase() {
        Some(Couleur::Noir)
    } else {
        None
    }
}

fn type_piece(a: char) -> Option<TypePiece>{

}

fn main() {
    let a = couleur_piece('7');
    println!("{:?}", a);
    let roi = Piece{couleur: couleur_piece('A').unwrap(), type_piece:TypePiece::Roi};
    println!("{:?}", roi);
   
}
