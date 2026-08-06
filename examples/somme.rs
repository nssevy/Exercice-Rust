/*

Dans main, en utilisant une boucle while, calcule la somme de tous les entiers de 1 à 5 inclus (donc 1 + 2 + 3 + 4 + 5), puis affiche le résultat une seule fois, à la fin.
Le résultat attendu à l'exécution :
15

*/

fn main() {
    let mut tour: i32 = 1;
    let mut total: i32 = 0;

    while tour <= 5 {
        total += tour;
        tour += 1;
    }

    println!("{}", total);
}
