/*

Dans main, en utilisant une boucle while, calcule la somme des chiffres du nombre 2531. Autrement dit, additionne 2 + 5 + 3 + 1. Affiche le résultat une seule fois à la fin.
Le résultat attendu à l'exécution :
11

*/

fn main () {
	let mut i: i32 = 2531;
	let mut total: i32 = 0;

	while i > 0 {
		total += i % 10;
		i /= 10;
	}

	println!("{}", total);
}
