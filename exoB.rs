/*

Écris une fonction double qui prend un i32 en paramètre et renvoie ce nombre multiplié par 2. Dans main, appelle-la avec la valeur 21 et affiche le résultat.
Résultat attendu : 42

*/

fn double (a: i32) -> i32 {
	a * 2
}

fn main () {
println!("{:?}", double(21));
}

