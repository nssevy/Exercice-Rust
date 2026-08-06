/*
Exercice 1
L'énoncé :

Déclare une variable note de type i32 avec la valeur 13.
En utilisant un if / else qui renvoie une valeur, affecte à une
variable resultat la chaîne "admis" si la note est supérieure
ou égale à 10, sinon "recalé".
Affiche resultat.

Le résultat attendu à l'exécution :
admis
*/

fn main (){

    let note: i32 = 13;
    
    let statut = if note >= 10 {"admis"} else {"recalé"};

    println!("{}", statut);
}