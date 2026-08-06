/*
A. Un feu de circulation qui, à un instant donné, est rouge, orange ou vert rien d'autre à stocker.*/
#[allow(dead_code)]
#[derive(Debug)]
enum Couleur {
    Rouge,
    Orange,
    Vert
}

fn main() {
    let feu = Couleur::Rouge;
    println!("{:?}", feu);
// Vue qu'un feu rouge c'est juste un feu qui peut avoir 3 étâts, on aq juste a faire un enums.
}
