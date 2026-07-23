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

// Le probleme ce que ca prends en entre des lettre majusucles, des lettres minuscule
// et des chiffres
// Les chiffres doivent etr ignorer et renvoyer null

/* impl Piece {
    fn creation_de_piece(a: char) -> Piece{
        match_lettre(&a);
    }

    fn match_lettre(a: char) -> Option<Piece::TypePiece> {
        match a {
            'K' | 'k' => TypePiece::Roi,
            'Q' | 'q' => TypePiece::Dame,
            'N' | 'n' => TypePiece::Cavalier,
            'T' | 't' => TypePiece::Tour,
            'B' | 'b' => TypePiece::Fou,
            'P' | 'p' => TypePiece::Pion,
            _ => Null,
        }
    }

    fn blanc_ou_noir(a: char) -> Option<Piece::Couleur>{
        if a.is_ascii_uppercase() == 
    }

}

*/

fn main() {
    let roi = Piece{couleur: Couleur::Blanc, type_piece:TypePiece::Roi};
    println!("{:?}", roi);
   
}
