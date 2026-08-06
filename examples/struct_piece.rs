/*
reprends tes enums Couleur (Blanc, Noir) et TypePiece (Roi, Dame, Tour, Fou, Cavalier, Pion). Définis une struct Piece avec deux champs : couleur de type Couleur, et type_piece de type TypePiece. Dans le main, crée à la main (instancie) une pièce représentant la dame blanche, puis affiche-la entièrement avec {:?}.
Résultat attendu :
Piece { couleur: Blanc, type_piece: Dame }

ajoute un bloc impl Piece contenant une méthode statique new qui prend deux paramètres, couleur: Couleur et type_piece: TypePiece, et renvoie la Piece correspondante (-> Piece). Dans le main, remplace ta création manuelle par un appel à Piece::new(...) pour fabriquer la dame blanche, puis affiche-la avec {:?}.




Consigne : garde tes enums et ta struct. Tu vas ajouter deux fonctions libres que tu as déjà écrites (recopie-les de tes anciens fichiers), puis transformer le constructeur.

Une fonction fn couleur_depuis(a: char) -> Option<Couleur> : Some(Couleur::Blanc) si majuscule, Some(Couleur::Noir) si minuscule, None sinon.

Une fonction fn type_depuis(a: char) -> Option<TypePiece> : le match sur les lettres FEN ('K'|'k' => Roi, etc.), None pour le reste.

Dans impl Piece, remplace ton new par une méthode fn depuis_fen(a: char) -> Option<Piece> qui appelle les deux fonctions ci-dessus et les combine avec le match sur le couple.

Dans le main, appelle Piece::depuis_fen('Q'), Piece::depuis_fen('n') et Piece::depuis_fen('5'), et affiche chaque résultat avec {:?}.
Résultat attendu :
Some(Piece { couleur: Blanc, type_piece: Dame })
Some(Piece { couleur: Noir, type_piece: Cavalier })
None
*/
#![allow(dead_code)]

fn couleur_depuis(a: char) -> Option<Couleur> {  
    if a.is_ascii_uppercase() {
        Some(Couleur::Blanc)
    } else if a.is_ascii_lowercase() {
        Some(Couleur::Noir)
    } else {
       None 
    }
}
  
fn type_depuis(a: char) -> Option<TypePiece> {
    match a {
        'K' | 'k' => Some(TypePiece::Roi),
        'Q' | 'q' => Some(TypePiece::Dame),
        'R' | 'r' => Some(TypePiece::Tour),
        'B' | 'b' => Some(TypePiece::Fou),
        'N' | 'n' => Some(TypePiece::Cavalier),
        'P' | 'p' => Some(TypePiece::Pion),
        _ => None,
    }
}

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

   fn depuis_fen(a: char) -> Option<Piece> {
        let c = couleur_depuis(a);
        let t = type_depuis(a);

        match (c, t) {
            (Some(c), Some(t)) => Some(Piece { couleur: c, type_piece: t }),
            _ => None,
        }
    }

    /*fn new(couleur: Couleur,type_piece: TypePiece) -> Piece{
       Piece {couleur, type_piece} 
    }*/
}

fn main() {
    let dame = Piece::depuis_fen('Q');
    let cavalier = Piece::depuis_fen('n');
    let chiffre = Piece::depuis_fen('3');

    println!("{:?}", dame);
    println!("{:?}", cavalier);
    println!("{:?}", chiffre);
    
}
