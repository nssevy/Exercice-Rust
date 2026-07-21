/*
Définis un enum TypePiece avec une variante par type de pièce (roi, dame, tour, fou, cavalier, pion). Puis écris une fonction fn type_de_piece(a: char) -> Option<TypePiece> qui renvoie le bon type selon la lettre reçue, en traitant majuscule et minuscule de la même façon, et None pour tout ce qui n'est pas une lettre de pièce. Dans le main, teste-la sur 'Q', 'n', 'p' et '5', et affiche chaque résultat.

Résultat attendu :

Some(Dame)
Some(Cavalier)
Some(Pion)
None
*/
#![allow(dead_code)]
#[derive(Debug)]
enum TypePiece {
    Roi,
    Dame,
    Tour,
    Fou,
    Cavalier,
    Pion,
}

fn type_de_piece(a: char) -> Option<TypePiece> {
    match a {
        'K' | 'k' => Some(TypePiece::Roi),
        'Q' | 'q' => Some(TypePiece::Dame),
        'C' | 'c' => Some(TypePiece::Tour),
        'B' | 'b' => Some(TypePiece::Fou),
        'N' | 'n' => Some(TypePiece::Cavalier),
        'P' | 'p' => Some(TypePiece::Pion),
        _ => None,
    }
}

fn main() {
    let ligne_depart = vec!['r','n','b','q','k','b','n','r'];

    //println!("{:?}", ligne_depart);

    let mut i: usize = 0;

    while i < ligne_depart.len(){
        println!("{:?}",type_de_piece(ligne_depart[i]));
        i += 1;
    }
    // println!("{:?}", type_de_piece('Q')); */
}
