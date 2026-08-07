/*
Tu as une grille où chaque ligne représente une série de mesures. Une ligne est considérée
« invalide » si elle contient au moins un zéro. Parcours la grille et affiche la somme de la
première ligne entièrement valide (sans aucun zéro). Dès qu'une ligne contient un zéro, tu dois
abandonner cette ligne et passer directement à la suivante, sans finir de l'additionner.

Grille de test :

let grid: [[u8; 3]; 3] = [
    [4, 0, 7],
    [2, 5, 3],
    [9, 1, 8],
];

Résultat attendu :

Première ligne valide, somme = 10

(La ligne 0 est rejetée à cause du zéro, la ligne 1 [2,5,3] est la première sans zéro,
sa somme est 10.)
 */
fn main() {
    let grid: [[u8; 3]; 3] = [
        [4, 0, 7],
        [2, 5, 3],
        [9, 1, 8],
    ];

    let mut somme: u8 = 0;

    'search: for row in grid {
        for value in row {
            if value == 0 {
                somme = 0;
                continue 'search; // Passe à la ligne suivante
            }
           somme += value;
        }
        println!("Première ligne valide, somme = {somme}");
        break 'search;
    }
}