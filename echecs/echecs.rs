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
    // Renvoie les lettres des pieces.
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

fn echequier() {
    // variable pour les boucles.
    let mut l: usize = 1;
    let mut c: usize = 1;
    let mut i: usize = 0;
    let mut j: usize = 0;

    // Pieces.
    let vide = PieceType::Vide.piece_en_lettre();

    // Initialisation des deux tableaux vides.
    let mut ligne = vec![];
    let mut colonne = vec![];

    // rempli la premiere ligne du tableau
    while l <= 8 {
        ligne.push(vide);
        l += 1;
    } 
    
    // Tableau tridimentionnel, rempli le tebleau à chaque index du tableau ligne.
    while c <= 8 {
        colonne.push(ligne.clone());
        c += 1;
    }

    /* Affiche le plateau (tableau)
    * i = index 0
    * j = les éléments à l'interrieur de i.
    */
    while i < 8 {
        while j < 8 {
            print!("{} ", colonne[i][j]);
            j += 1;
        }
        // Retour à la ligne
        println! {};
        // Reset j pour re-parcourir de l'index 0
        j = 0;
        i += 1;
    }
}

fn main() {
    let _roi = PieceType::Roi.piece_en_lettre();
    echequier();
}
