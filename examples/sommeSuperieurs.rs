/*

Écris une fonction séparée nommée somme_superieurs qui prend deux paramètres : une slice &[i32] et un i32 nommé seuil. La fonction renvoie un i32 : la somme des éléments de la slice qui sont strictement supérieurs au seuil. Utilise une boucle while avec un index pour parcourir la slice. Dans main, crée un Vec<i32> contenant 5, 12, 3, 20, 8, 15, appelle la fonction avec un seuil de 10, et affiche le résultat.
Le résultat attendu à l'exécution :
47

*/

fn somme_superieurs (a: &[i32], seuil: i32) -> i32 {
	
	let mut total: i32 = 0;
	let mut i: usize = 0;
	
	while i < a.len() {
		if a[i] > seuil {
			total += a[i];
		}
		i += 1;
	}
	total
} 

fn main () {
	let v: Vec<i32> = vec![5, 12, 3, 20, 8, 15];
	println!("{}", somme_superieurs(&v, 10));
}
