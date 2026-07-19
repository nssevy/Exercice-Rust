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
    // variable pour les boucles.
    let mut l: usize = 1;
    let mut c: usize = 1;
    let mut i: usize = 0;
    let mut j: usize = 0;

    // Variables des pices.
    let vide = PieceType::Vide.piece_en_lettre();

    // Variables des tableaux pour les lignes et colonne.
    let mut ligne = vec![];
    let mut colonne = vec![]; 

    while l <= 8{
        ligne.push(vide);
        l+= 1;
    } 

    // println!("{:?}", ligne); 

    while c <= 8 {
        colonne.push(ligne.clone());
        c += 1;
    }

    // println!("{:?}", colonne);
     
    /* while i <= 8 {
        println!("{:?}", colonne[i].iter());
        i += 1;
    } */

   

        /* while i < 8 {
            print!("{}", colonne[i][j]);  
                   
            i+= 1; 
        } */

        while i < 8 {
            while j < 8 {
                print!("{}", colonne[i][j]);
                j += 1;
            }
            println!();
            i+= 1;
        }
     
   
}

fn main() {
    let _roi = PieceType::Roi.piece_en_lettre();
    // let vide = PieceType::Vide.piece_en_lettre();

    // let _echequier = vec![vide];

    echequier();
}
