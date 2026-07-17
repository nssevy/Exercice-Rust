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

enum Feu {
    Rouge,
    Orange,
    Vert,
}

impl Feu {
    duree(&self) -> u32,
    peut_passer(&self) -> bool,
    suivant(&self) -> Feu
}

fn main() {
    let feu_rouge: Feu = Feu::Rouge
}
