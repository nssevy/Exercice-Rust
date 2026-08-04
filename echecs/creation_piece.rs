#![allow(dead_code)]

#[derive(Debug)]
enum TypePiece {
    Roi, //K ing
    Dame, // Q uenne
    Cavalier, // N knignt
    Tour,// R ook
    Fou, // B ishop
    Pion, // P awn
    Vide
}

#[derive(Debug)]
enum Couleur {
    Blanc, // Majucule
    Noir, // Minuscule
    Vide, // Vide
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
 match a {
        'K' | 'k' => Some(TypePiece::Roi),
        'Q' | 'q' => Some(TypePiece::Dame),
        'N' | 'n' => Some(TypePiece::Cavalier),
        'R' | 'r' => Some(TypePiece::Tour),
        'B' | 'b' => Some(TypePiece::Fou),
        'P' | 'p' => Some(TypePiece::Pion),
        //1..=8 => Some(TypePiece::Vide),
        _ => None
    }
}
// Si la fonction renvoie None crée une case vide, de type Option<Piece>
// et les cases vides sont construite par des chiffres allant de 1 à 8.
impl Piece {
    fn construction_de_piece(a: char) -> Option<Piece> {
        let c = couleur_piece(a);
        let t = type_piece(a);

        match (c, t) {
            (Some(c),  Some(t)) => Some(Piece{couleur: c, type_piece: t}),
            //(Some(c), Some()) => 
            _ => None,
        }
    }
 
}

fn fen(a: &str) -> Vec<char> {
    //crée un tableau à partir d'une chaine de str
    a.chars().collect()
}

struct Plateau {
    fen: &str,
    echequier: Vec<Vec<char>>,
}

fn echequier(a: &str) -> Vec<Vec<char>> {

    let s: Vec<_> = a.split('/').collect(); // ["rnbqkbn", "RNBQKBN"]

    let mut i: usize = 0;
    let mut un = vec![];
    let mut deux = vec![];

    un = fen(s[i]);
    i += 1;
    deux = fen(s[i]);

    let mut echequier = vec![];
    echequier.push(un);
    echequier.push(deux);
    echequier
}

fn print_echequier(a: Vec<Vec<char>>) {
    for p in a.iter(){
        println!("{:?}", p);
    }
}

fn main(){
    //rnbqkbnr 
    let base_fen: &str = "rnbqkbn/RNBQKBN";

    let plateau = echequier(base_fen);

    print_echequier(plateau);

    let fen = fen(base_fen); // ['r', 'n', 'b', 'q', 'k', 'b', 'n', '/', 'R', 'N', 'B', 'Q', 'K', 'B', 'N']    

    // Permet de reconnaitre le type de piece dans le tableau (echequier)
    for f in fen.iter(){
        let p = Piece::construction_de_piece(*f);
        println!("{:?}", p);
    }

    //println!("{:?}", fen);
}
