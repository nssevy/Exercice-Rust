/*

Déclare une variable jour de type i32 valant 3. Avec un match, affiche le nom du jour : 1 donne "lundi", 2 donne "mardi", 3 donne "mercredi", et tout autre nombre donne "jour inconnu".
Résultat attendu : mercredi

*/

fn main () {
let jour: i32 = 3;

let a = match jour {
	1 => {"lundi"}
	2 => {"mardi"}
	3 => {"mercredi"}
	_ => {"jour inconnu"}
};

println!("{}", a);
}
