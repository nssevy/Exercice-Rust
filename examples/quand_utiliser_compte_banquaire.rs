/*
Un compte bancaire caractérisé par un numéro, un solde et le nom du titulaire et on veut pouvoir
y déposer et retirer de l'argent.
 */
#[derive(Debug)]
#[allow(unused)]
struct Compte {
    numero: u32,
    solde: i32,
    nom_titulaire: String
}

impl Compte{
    fn deposer(&mut self, a: &i32) {
        self.solde += a;
    }

    fn retirer(&mut self, a: &i32) {
        self.solde -= a;
    }
}

fn main() {
    let mut sevy = Compte{
        numero: 9234,
        solde: 0,
        nom_titulaire: String::from("sevy")
    };

    (&mut sevy).deposer(&10);
    (&mut sevy).retirer(&5);

    println!("{:?}", sevy);
}