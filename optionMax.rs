/* 

Écris une fonction séparée nommée maximum qui prend une slice &[i32] et renvoie directement l'Option produite par .max(). Autrement dit, cette fois tu ne mets pas de .unwrap() dans la fonction : son type de retour sera Option<&i32>, et son corps sera simplement s.iter().max().
Dans main, crée un Vec<i32> contenant 7, 2, 9, 4, 6.
Appelle maximum en lui passant une slice du Vec, et récupère le résultat dans une variable.
Utilise un match sur ce résultat pour traiter les deux cas :

Some(m) : affiche "le maximum est m" (où m est la valeur capturée)
None : affiche "la liste est vide"



Le résultat attendu à l'exécution :
le maximum est 9 */

fn maximum (s: &[i32]) -> &i32 {
	s.iter().max().expect("REASON")

}

fn main (){

let v = vec![7, 2, 9, 4, 6];
let result = maximum(&v);

match result {
	Some(m) => println!("le maximum est m"),
		none => println!("la liste estv vide"), 

}

}


