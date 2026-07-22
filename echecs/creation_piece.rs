#[derive(Debug)]
enum TypePiece {
    Roi, //K ing
    Dame, // Q uenne
    Cavalier, // N knignt
    Tour,// R ook
    Fou, // B ishop
    Pion // P awn
}

enum Couleur {
    Blanc, // Majucule
    Noir, // Minuscule
}

struct Piece {
    couleur: Couleur,
    type_piece : TypePiece,
}

/*impl Piece {
    
}*/

fn blanc_ou_noir(a: char) -> Option<Couleur>{
    a.is_ascii_uppercase() == true
}

fn match_lettre(a: char) -> TypePiece {
    match a {
        'K' | 'k' => TypePiece::Roi,
        'Q' | 'q' => TypePiece::Dame,
        'N' | 'n' => TypePiece::Cavalier,
        'T' | 't' => TypePiece::Tour,
        'B' | 'b' => TypePiece::Fou,
        'P' | 'p' => TypePiece::Pion,
        _ => todo!(),
    }
}


/* fn color_piece(a: char) -> Couleur::Piece {
    if 
} */

fn main() {
    let 
    let piece_roi = match_lettre('b');
    dbg!(piece_roi);
    //println!("{:?}", std::any::type_name_of_val(&roi));
}
