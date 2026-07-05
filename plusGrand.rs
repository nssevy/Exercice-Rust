/*

Écris une fonction séparée nommée plus_grand qui prend une slice &[i32] et renvoie un i32 : le plus grand élément de la slice. Utilise une boucle while avec un index pour parcourir la slice (pas de .iter().max() cette fois, on veut le faire à la main). Dans main, crée un Vec<i32> contenant 3, 17, 8, 42, 15, 4, appelle la fonction en lui passant une slice de tout le Vec, et affiche le résultat.
Le résultat attendu à l'exécution :
42

*/

fn plus_grand(a: &[i32]) -> i32 {
	let mut i: usize = 0;
	let mut grand: i32 = a[0];
	
	while i < a.len(){

		if a[i] > grand {
			grand = a[i];
		}
		i += 1;
	}
	grand 
}


fn main () {
	let v: Vec<i32> = vec![3,17,8,42,15,4];
	println!("{}", plus_grand(&v));
}
