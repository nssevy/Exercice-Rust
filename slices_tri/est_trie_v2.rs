const CROISSANT: i32 = 1;
const DECROISSANT: i32 = 2;

const _EGAL: i32 = 3;
const _VIDE: i32 = 4;
const _PAS_TRIER: i32 = 5;

fn croissant(a: &[i32]) -> Option<i32> {

    let mut i: usize = 0;

    while i < a.len()-1 {

        if a[i] <= a[i + 1] {
            i += 1;
        } else {
            return None;
        }

    }

    Some(CROISSANT)
}

fn decroissant(a: &[i32]) -> Option<i32> {

    let mut i: usize = 0;

    while i < a.len()-1 {

        if a[i] >= a[i + 1] {
            i += 1;
        } else {
            return None;
        }

    }

    Some(DECROISSANT)
}

fn verify_tableau (a: &[i32]) {
    let c = croissant(a).unwrap_or(0);
    let d = decroissant(a).unwrap_or(0);

    let valeur_retour_tab: i32 = c + d;

    println!("{valeur_retour_tab}");

    match valeur_retour_tab {
        1 => { println!("Le tableau est croissant"); }
        2 => { println!("Le tableau est decroissant"); }
        _      => { println!("Ce tableau n'est pas dans nos données"); }
    }

}

fn main() {
    let tableau_croissant: Vec<i32> = vec![3, 5, 9, 10, 30];
    let tableau_decroissant: Vec<i32> = vec![30, 10, 9, 5, 3];

    verify_tableau(&tableau_croissant);
    verify_tableau(&tableau_decroissant);

}
