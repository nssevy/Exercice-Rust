/*
Déclare une variable temperature de type i32 avec la valeur 28.
En utilisant un match avec des gardes, affecte à une variable
vetement la recommandation correspondante :

si la température est strictement supérieure à 30, renvoie
"t-shirt"
sinon si elle est strictement supérieure à 15, renvoie "pull"
sinon si elle est strictement supérieure à 0, renvoie "manteau"
dans tous les autres cas, renvoie "doudoune"


Affiche vetement.

Le résultat attendu à l'exécution (28 est supérieur à 15 mais
pas à 30) :
pull
*/

fn main (){
    let temperature: i32 = 28;
    let habits = match temperature{
        tp if tp > 30 => {"t-shirt"}
        tp if tp > 15 => {"pull"}
        tp if tp > 0 => {"manteau"}
        _ => {"doudoune"}
    };
    
    println!("{}", habits);
}