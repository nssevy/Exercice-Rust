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

impl Piece {
    fn construction_de_piece(a: char) -> Option<Piece> {
        let c = couleur_piece(a);
        let t = type_piece(a);

        match (c, t) {
            (Some(c),  Some(t)) => Some(Piece{couleur: c, type_piece: t}),
            _ => None,
        }
    }
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
 match a {
        'K' | 'k' => Some(TypePiece::Roi),
        'Q' | 'q' => Some(TypePiece::Dame),
        'N' | 'n' => Some(TypePiece::Cavalier),
        'R' | 'r' => Some(TypePiece::Tour),
        'B' | 'b' => Some(TypePiece::Fou),
        'P' | 'p' => Some(TypePiece::Pion),
        _ => None
    }
}

fn main() {
    /*let a = couleur_piece('7');
    println!("{:?}", a); */
    //let roi = Piece{couleur: couleur_piece('A').unwrap(), type_piece: type_piece('Q').unwrap()};
    let roi = Piece::construction_de_piece('K');
    println!("{:?}", roi);
   
}
