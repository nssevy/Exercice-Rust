/*

Consigne : reprends un struct Compte (titulaire: String, solde: u32). Ajoute deux méthodes : deposer qui augmente le solde d'un montant, et afficher qui affiche le solde. Dans main, dépose deux fois puis affiche.
Résultat attendu :

Solde de Bob : 250

*/

struct Compte {
	titulaire: String,
	solde: u32,
}


fn deposer(s: &mut Compte, argent: u32) {
    let new_solde = s.solde + argent;

    s.solde = new_solde
}

fn retirer(s: &mut Compte, somme: u32) {
    let new_solde = s.solde - somme;

    s.solde = new_solde
}


fn main() {

    let mut yves = Compte {
        titulaire: String::from("Yves"),
        solde: 0,
    };

    deposer(&mut yves, 125);
    deposer(&mut yves, 125);
    retirer(&mut yves, 50);

    println!("Solde de {} : {} ", yves.titulaire, yves.solde);
}
