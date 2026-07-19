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
#[allow(dead_code)]
enum PieceType {
    Roi,
    Dame,
    Cavalier,
    Tour,
    Fou,
    Pion,
    Vide,
}

impl PieceType {
    fn piece_en_lettre(&self) -> char {
        match self {
            PieceType::Roi => 'R',
            PieceType::Dame => 'D',
            PieceType::Cavalier => 'C',
            PieceType::Tour => 'T',
            PieceType::Fou => 'F',
            PieceType::Pion => 'P',
            PieceType::Vide => '.',
        }
    }
}

fn echequier(){
    let mut i: usize = 1;
    let vide = PieceType::Vide.piece_en_lettre();

    let mut ligne = vec![];
    let mut colonne = vec![];

    while i <= 8{
        ligne.push(vide);
        i+= 1;
    } 
    
    println!("{:?}", ligne);
    

    /* for p in ligne.iter(){
        print!("{p}");
    } */
}

fn main() {
    let _roi = PieceType::Roi.piece_en_lettre();
    // let vide = PieceType::Vide.piece_en_lettre();

    // let _echequier = vec![vide];

    println!("{:?}", echequier());
}
