/*

Consigne : reprends un struct Compte (titulaire: String, solde: u32). Ajoute deux méthodes : deposer qui augmente le solde d'un montant, et afficher qui affiche le solde. Dans main, dépose deux fois puis affiche.
Résultat attendu :

Solde de Bob : 250

*/

struct Compte {
	titulaire: String,
	solde: u32,
}

impl Compte {
    fn deposer(&self) -> u32 {
        
    }
}

fn main() {
    let Yves = Compte {
        titulaire: &str("Yves");
        solde: 
    }

    println!("Solde de {} : {} ", Yves.titulaire, Yves.solde);
}
