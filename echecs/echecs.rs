/*
représente un échiquier de 8×8 cases, entièrement vide au départ (. partout), puis place quelques pièces 
la main (par exemple un R et un T). Enfin, affiche le plateau ligne par ligne dans le terminal.
Résultat attendu à l'exécution (l'emplacement exact des pièces t'appartient, c'est juste un exemple) :
. . . . . . . .
. . . . . . . .
. . T . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . R . .
. . . . . . . .
*/
// #[allow(dead_code)]
enum PieceType {
    Roi,
    Dame,
    Cavalier,
    Tour,
    Fou,
    Pion,
}

impl PieceType {
    fn piece_en_lettre(&self) -> &str {
        match self {
            PieceType::Roi => "R",
            PieceType::Dame => "D",
            PieceType::Cavalier => "C",
            PieceType::Tour => "T",
            PieceType::Fou => "F",
            PieceType::Pion => "P",
        }
    }
}

fn tableau(){

}

fn main() {
    let roi = PieceType::Roi.piece_en_lettre();
    println!("{roi}");
}
