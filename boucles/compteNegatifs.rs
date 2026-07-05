/*

Écris une fonction séparée nommée compte_negatifs qui prend une slice &[i32] et renvoie un i32 : le nombre d'éléments strictement négatifs dans la slice. Utilise une boucle while à l'intérieur de la fonction pour parcourir la slice. Dans main, crée un Vec<i32> contenant 4, -2, 7, -9, -1, 3, appelle la fonction en lui passant une slice de tout le Vec, et affiche le résultat.
Le résultat attendu à l'exécution :
3

*/

fn compte_negatifs (s: &[i32]) -> i32 {

	let mut i: usize = 0;
	let mut compte: i32 = 0;

	while i < s.len() {
		if s[i] < 0 {
			compte += 1;		
		}
	i += 1;
	}
	compte
}

fn main () {

	let v = vec![4, -2, 7, -9, -1, 3];

	println!("{}", compte_negatifs(&v));
}
