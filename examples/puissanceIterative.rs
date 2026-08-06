/*

Écris une fonction séparée nommée puissance qui prend deux paramètres : un i32 nommé nb (la base) et un i32 nommé exposant. La fonction renvoie un i32 : le résultat de nb élevé à la puissance exposant, calculé avec une boucle while. Dans main, appelle puissance(3, 4) et affiche le résultat.
Le résultat attendu à l'exécution :
81
(car 3 × 3 × 3 × 3 = 81)

*/

fn puissance (nb: i32, exposant: i32) -> i32 {
	
	let mut i: i32 = 1;
	let mut total: i32 = nb;	
	
	if exposant == 0 {
		total = 1;
	}
	
	while i < exposant {
		total = total * nb;
		i += 1; 
	}
	total
}

fn main () {

	println!("{puissance(3,4)}");

}
