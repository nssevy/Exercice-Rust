/* #![allow(unused)] */

const PAS_TRIER: i32 = 0;
const CROISSANT: i32 = 1;
const DECROISSANT: i32 = 2;
const EGAL: i32 = 3;
const VIDE: i32 = 4;

fn vide(a: &[i32]) -> Option<i32> {

        if a.len() < 2 {
            
        } else {
            return None;
        }
    Some(VIDE)
}

fn croissant(a: &[i32]) -> Option<i32> {

    let mut i: usize = 0;

    /* is.none() renvoie true si l'option est None
    * Donc si le tableau n'est pas vide (egal à None) execute la boucle,
    * sinon retourne None directement */
    if vide(a).is_none() {

        while i < a.len()-1 {

            if a[i] <= a[i + 1] {
                i += 1;

            } else {
                return None;    
            }
        }

    } else {
        return None;    
    }

    Some(CROISSANT)
}

fn decroissant(a: &[i32]) -> Option<i32> {

    let mut i: usize = 0;

    if vide(a).is_none() {

    while i < a.len()-1 {


        if a[i] >= a[i + 1] {
            i += 1;
        } else {
            return None;
        }
        }

    } else {
        return None;

    }

    Some(DECROISSANT)
}

/* 
* Pour la fonction verify_tableau, elle prends en paramètre une slice (un tableau)
* et elle vient tester ce tableau avec 3 fonctions, qui sont des type options.
* Elles retournent donc soit None (null) soit une valeur défini en haut dans les CONST.
*
* A partir de cette valeur on peut determiner comment les valeurs du tableau sont trier
* (croissant decroissant etc...)
*
*
* Si le tableau est vide :
*
* On le test pas dans la fonction croissant et decroissant
* car sinon ca mettrait une erreur "vous essayez d'acceder à un index inxecistant" car
* pas d'éléments.
*
*
* Si le tableau est egal : 
*
* La fonction croissant et decroissant retourne toutes les deux
* leur valeur soit 1 et 2, et comme la variable valeur_retour_tab additionne le resultat
* des 3 fonctions, ca donne 0 + 1 + 2 = 3, et 3 est difinit dans les Const c'est donc egal.
*
*
* Si le tableau n'est pas trier : 
*
* Valeur_retour_tab est egal à 0 car les variables v, c et d sont egal à 0. 
* Car les 3 fonctions renvoient None soit 0, et 0 + 0 + 0 = 0, et la const VIDE est egal à 0
*
*/

fn verify_tableau (a: &[i32]) {
    
    let v = vide(a).unwrap_or(0); 
    let c = croissant(a).unwrap_or(0);
    let d = decroissant(a).unwrap_or(0);
    
    /* Pour debuger
    println!("{v}{c}{d}");
    */

    let valeur_retour_tab: i32 = c + d + v;

    /* println!("{valeur_retour_tab}"); */

    match valeur_retour_tab {
        CROISSANT => { println!("Le tableau est croissant"); }
        DECROISSANT => { println!("Le tableau est decroissant"); }
        EGAL => { println!("Toutes les valeurs du tableau sont egales"); }
        VIDE => { println!("Le tableau est vide ou bien ne contient pas assez d'éléments"); }
        PAS_TRIER => { println!("Le tableau n'est pas trier"); }
        _      => { println!("Ce tableau n'est pas dans nos données"); }
    }

}

fn main() {
    let tableau_croissant: Vec<i32> = vec![3, 5, 9, 10, 30];
    let tableau_decroissant: Vec<i32> = vec![30, 10, 9, 5, 3];
    let tableau_egal: Vec<i32> = vec![30, 30, 30, 30, 30];
    let tableau_croissant_egal: Vec<i32> = vec![30, 30, 30, 30, 60];
    let tableau_vide: Vec<i32> = vec![];
    let tableau_pas_trier: Vec<i32> = vec![1, 30, 7, 30, 60];

    verify_tableau(&tableau_croissant);
    verify_tableau(&tableau_decroissant);
    verify_tableau(&tableau_egal);
    verify_tableau(&tableau_croissant_egal); 
    verify_tableau(&tableau_vide);
    verify_tableau(&tableau_pas_trier);
}
