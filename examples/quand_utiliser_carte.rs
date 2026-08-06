/*
Une carte à jouer, qui a une valeur (de 2 à l'As) et une enseigne (cœur, carreau, trèfle ou pique).
 */
#[allow(dead_code)]
#[derive(Debug)]
enum Valeur {
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Valet,
    Dame,
    Roi,
    As
}
#[allow(dead_code)]
#[derive(Debug)]
enum Enseigne {
    Coeur,
    Carreau,
    Trefle,
    Pique
}
#[derive(Debug)]
#[allow(dead_code)]
struct Carte {
    valeur: Valeur,
    enseigne: Enseigne
}

impl Carte {
    fn new(v: Valeur, e: Enseigne) -> Carte {
        Carte {valeur: v, enseigne: e}
    }
}
fn main() {
    let carte = Carte::new(Valeur::Deux, Enseigne::Coeur);
    //let valet = Carte {valeur: Valeur::As, enseigne: Enseigne::Coeur};
    println!("{:?}", carte);
}