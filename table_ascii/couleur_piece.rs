/*
écris une fonction qui prend un char représentant une pièce et renvoie une &str (une chaîne) valant "blanc" si la pièce est en majuscule, "noir" si elle est en minuscule. Utilise un prédicat is_ascii_..., pas d'arithmétique. Dans le main, teste-la sur 'Q' (dame blanche) et sur 'q' (dame noire), et affiche les deux résultats.

Résultat attendu :

blanc
noir

*/
#[derive(Debug)]
enum Couleur {
    Blanc,
    Noir,
}

fn piece_noir_ou_blanche(a: char) -> Option<Couleur> {  
    if a.is_ascii_uppercase() {
        Some(Couleur::Blanc)
    } else if a.is_ascii_lowercase() {
        Some(Couleur::Noir)
    } else {
       None 
    }
}

fn verif_piece(a: &Option<Couleur>) {
    match a {
        Some(Couleur::Blanc) => println!("blanc"),
        Some(Couleur::Noir) => println!("noir"),
        _ => {}
    }
}

fn main(){

    let mut les_pieces: Vec<Option<Couleur>> = vec![];

    let piece_blanche = piece_noir_ou_blanche('Q');
    let piece_noir = piece_noir_ou_blanche('q');
    let chiffre = piece_noir_ou_blanche('5');

    les_pieces.push(piece_blanche);
    les_pieces.push(piece_noir);
    les_pieces.push(chiffre);

    //println!("{:?}", les_pieces);

    let mut i: usize = 0;

    while i < les_pieces.len() {
        verif_piece(&les_pieces[i]);
        i += 1;
    }
}
