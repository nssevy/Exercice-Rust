/*
Tu as une grille 2D d'entiers. Cherche une valeur cible et affiche la ligne et la colonne de sa
première occurrence (parcours de haut en bas, puis de gauche à droite). Dès que tu la trouves,
arrête immédiatement toutes les boucles. Si la cible est absente de la grille, affiche un message
le signalant.

Utilise cette grille et cette cible pour tester :

let grid: [[u8; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];
let target: u8 = 6;

Résultat attendu à l'exécution :

Trouvé à la ligne 1, colonne 2

Et si tu remplaces target par une valeur absente (par exemple 42) :

Valeur absente de la grille
 */
fn main() {
    let grid: [[u8; 3]; 3] = [
        [1,2,3],
        [4,5,6],
        [7,8,9],
    ];

    let target: u8 = 6;
    let mut verify: bool = false;

    'search: for (mut index_l, row) in grid.iter().enumerate(){
        for (mut index_col, value) in row.iter().enumerate()  {
            if *value == target {
                index_l += 1; //+1 pour que ce soit lisible pour une personne qui ne fait pas de prog
                index_col += 1;
                println!("Trouvé à la ligne {index_l}, colonne {index_col}");
                verify = true;
                break 'search;
            }
        }
    }
    if !verify {
        println!("Valeur absente de la grid.");}
}