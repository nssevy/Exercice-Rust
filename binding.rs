/*
Déclare une variable age de type i32 avec la valeur 25.
En utilisant un match avec un binding @, écris les bras suivants :

si l'âge est entre 0 et 17 inclus, capture la valeur dans une
variable et affiche "mineur, tu as X ans" (où X est l'âge réel,
récupéré grâce au binding)

si l'âge est entre 18 et 64 inclus, capture la valeur et affiche
"adulte, tu as X ans"

dans tous les autres cas, capture la valeur et affiche "senior,
tu as X ans"


Le match ne renvoie rien cette fois : chaque bras se contente
d'afficher directement avec println!.

Le résultat attendu à l'exécution (25 est entre 18 et 64) :
adulte, tu as 25 ans
*/

fn main (){
    let age: i32 = 25;
    
    match age {
        v @ 0..=17 => println!("mineur, tu as {:?} ans", v),
        v @ 18..=64 => println!("adulten, tu as {:?} ans", v),
        v => println!{"senior, tu as {} ans", v},
    }
}