/*

Dans main, en utilisant une boucle while, calcule la somme des nombres pairs de 1 à 10 inclus (donc 2 + 4 + 6 + 8 + 10), puis affiche le résultat une seule fois à la fin.
Le résultat attendu à l'exécution :
30

*/

fn main () {
	let mut compteur: i32 = 1;
	let mut total: i32 = 0;

	while compteur <= 10 {
		if compteur % 2 == 0 {
			total += compteur;
	}
		compteur += 1;
	}

	println!("{}", total);
}
