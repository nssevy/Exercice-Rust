/*

Dans main, en utilisant une boucle while, compte combien il y a de multiples de 3 entre 1 et 20 inclus (les multiples de 3 sont 3, 6, 9, 12, 15, 18, il y en a donc 6). Affiche ce nombre une seule fois à la fin.
Le résultat attendu à l'exécution :
6


*/

fn main () {
	let mut i: i32 = 1;
	let mut nombre_de_multiples: i32 = 0;

	while i <=20 {

		if i  % 3 == 0 {
		nombre_de_multiples += 1;
		}
	i += 1;
}

println!("{}", nombre_de_multiples);
}
