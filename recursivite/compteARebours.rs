/*

Écris une fonction récursive nommée compte_a_rebours qui prend un i32 nommé n et qui affiche tous les nombres de n jusqu'à 0 inclus, un par ligne, en se rappelant elle-même. Dans main, appelle-la avec la valeur 5.

Le résultat attendu à l'exécution :
5
4
3
2
1
0

*/

fn compte_a_rebours (n: i32) -> () {
	if n < 0 {
		return;
	}
	println!("{n}");
	compte_a_rebours(n-1);
}

fn main () {
	compte_a_rebours(5);
}
