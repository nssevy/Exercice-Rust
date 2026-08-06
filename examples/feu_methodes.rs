/*
Reprends ton enum Feu (Rouge, Orange, Vert) du premier exercice.
Écris un bloc impl Feu contenant trois méthodes :

duree(&self) -> u32 — la durée du feu : 60, 5, 45
peut_passer(&self) -> bool — vrai uniquement si le feu est vert
suivant(&self) -> Feu — le feu qui vient après : rouge donne vert, vert donne orange, orange donne rouge

Dans main, crée un feu rouge, affiche sa durée et si on peut passer, puis fais la même chose pour son suivant.

Résultat attendu :

Feu actuel : 60 secondes, passage : false
Feu suivant : 45 secondes, passage : true
*/

/* #![allow(dead_code)] */
#[derive(Debug)]
enum Feu {
    Rouge,
    Orange,
    Vert,
}

impl Feu {
    fn duree(&self) -> u32 {
        match self {
            Feu::Rouge => 60,
            Feu::Vert => 45,
            Feu::Orange => 5,
        }
    }

    fn peut_passer(&self) -> bool {
        match self {
            Feu::Vert => true,
            _ => false,
        }
    }

    fn suivant(&self) -> Feu {
        match self {
            Feu::Rouge => Feu::Vert,
            Feu::Vert => Feu::Orange,
            Feu::Orange => Feu::Rouge,
        }
    }
}

fn main() {
    let feu = Feu::Rouge;

    println!(
        "Feu actuel : {} secondes, passage : {:?}",
        feu.duree(),
        feu.peut_passer()
    );
    let suivant = feu.suivant();
    // dbg!(&suivant);
    println!(
        "Feu actuel : {} secondes, passage : {:?}",
        suivant.duree(),
        suivant.peut_passer()
    );
}
