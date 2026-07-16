#![allow(unused)]

const CROISSANT: i32 = 1;
const DECROISSANT: i32 = 2;
const EGAL: i32 = 3;
const VIDE: i32 = 4;

const _PAS_TRIER: i32 = 5;

fn vide(a: &[i32]) -> Option<i32> {

        if a.len() < 2 {
            VIDE;
        } else {
            return None;
        }
    Some(VIDE)
}

fn croissant(a: &[i32]) -> Option<i32> {

    let mut i: usize = 0;

    if vide(&a).is_none() == true {

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

    if vide(&a).is_none() == true {

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

fn verify_tableau (a: &[i32]) {
    
    let v = vide(a).unwrap_or(0); 
    let c = croissant(a).unwrap_or(0);
    let d = decroissant(a).unwrap_or(0);
    
    /* Pour debuger
    * println!("{v}{c}{d}");
    */
    

    let valeur_retour_tab: i32 = c + d + v;

    /* Le cas de egal est gerer par la somme de c + d, qui retourne tl2
*   une valeur si le tableau est egal, donc la somme des deux valeur est de 3.
*   si dans une des fonctions l'élement d'apres est plus sgrand ou plus petit 
*   la fonction en court echoue et revoie null, on recupere donc que la result
*   de la fonction qui a ete executer jsuqu'a la fin
*   */

    println!("{valeur_retour_tab}");

    match valeur_retour_tab {
        CROISSANT => { println!("Le tableau est croissant"); }
        DECROISSANT => { println!("Le tableau est decroissant"); }
        EGAL => { println!("Toutes les valeurs du tableau sont egales"); }
        VIDE => { println!("Le tableau est vide ou bien ne contient pas assez d'éléments"); }
        _      => { println!("Ce tableau n'est pas dans nos données"); }
    }

}

fn main() {
    let tableau_croissant: Vec<i32> = vec![3, 5, 9, 10, 30];
    let tableau_decroissant: Vec<i32> = vec![30, 10, 9, 5, 3];
    let tableau_egal: Vec<i32> = vec![30, 30, 30, 30, 30];
    let tableau_croissant_egal: Vec<i32> = vec![30, 30, 30, 30, 60];
    let tableau_vide: Vec<i32> = vec![];

    verify_tableau(&tableau_croissant);
    verify_tableau(&tableau_decroissant);
    verify_tableau(&tableau_egal);
    verify_tableau(&tableau_croissant_egal);
    verify_tableau(&tableau_vide);
}
