/*

Écris une fonction récursive nommée somme_jusqua qui prend un i32 nommé n et renvoie un i32 : la somme de tous les entiers de n jusqu'à 1 (donc n + (n-1) + (n-2) + ... + 1). Dans main, appelle-la avec la valeur 5 et affiche le résultat.
Le résultat attendu à l'exécution :
15

*/

fn somme_jusqua (n: i32) -> i32 {

	
	if n < 0 {
		return 0;
	}
	
	return n + somme_jusqua(n-1)
	

}

fn main () {
	println!("{}",somme_jusqua(5));
}
