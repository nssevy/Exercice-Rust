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
// Si la fonction renvoie None crée une case vide, de type Option<Piece>
// et les cases vides sont construite par des chiffres allant de 1 à 8.
impl Piece {
    fn construction_de_piece(a: char) -> Option<Piece> {
        let c = couleur_piece(a);
        let t = type_piece(a);

        match (c, t) {
            (Some(c),  Some(t)) => Some(Piece{couleur: c, type_piece: t}),
            None => 
            _ => None,
        }
    }

    /*fn case_vide(a: i32) -> Option<Piece> {
        
    }*/
}

//____________________________________

fn echequier() {
    // variable pour les boucles.
    let mut l: usize = 1;
    let mut c: usize = 1;
    let mut i: usize = 0;
    let mut j: usize = 0;

    //let vide Option<Piece> = 'v';

    let roi = Piece::construction_de_piece('K');
    let tour = Piece::construction_de_piece('R');

    let mut ligne = vec![];
    let mut colonne = vec![];
    
    while l <= 8 {
        ligne.push(vide);
        l += 1;
    } 
    
    while c <= 8 {
        colonne.push(ligne.clone());
        c += 1;
    }

    colonne[2][2] = tour;
    colonne[6][5] = roi;
 
    while i < 8 {
        while j < 8 {  
            print!("{} ", colonne[i][j]);
            j += 1;
        } 
        println!(); // Retour à la ligne
        i += 1;
        j = 0; // Reset j pour re-parcourir de l'index 0 
    }
}

fn main() {
    echequier();
    /*let a = couleur_piece('7');
    println!("{:?}", a); */
    //let roi = Piece{couleur: couleur_piece('A').unwrap(), type_piece: type_piece('Q').unwrap()};
    //let roi = Piece::construction_de_piece('K');
    //println!("{:?}", roi);
    //Je triche
   
}
