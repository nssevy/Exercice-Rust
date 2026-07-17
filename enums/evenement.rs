/*

Déclare une enum Evenement qui mélange les trois formes :

Connexion { utilisateur: String, ip: String } — un struct variant
Message(String, String) — un tuple variant : l'expéditeur, puis le texte
Deconnexion — un unit variant

Écris une fonction afficher(ev: &Evenement) qui ne renvoie rien et affiche une ligne de journal selon l'événement.

Dans main, crée les trois événements et appelle afficher sur chacun.

Résultat attendu :

[CONNEXION] alice depuis 192.168.1.10
[MESSAGE] alice : bonjour tout le monde
[DECONNEXION] session terminee

*/

enum Evenement {
    Connexion { utilisateur: String, ip: String},
    Message(String, String),
    Deconnexion,
}

fn afficher(ev: &Evenement) {
    match ev {
        Evenement::Connexion { utilisateur: u, ip: i} => println!("[CONNEXION] {u} depuis {i}"),
        Evenement::Message (prenom, message) => println!("[MESSAGE] {prenom} : {message} "),
        Evenement::Deconnexion => println!("[DECONNEXION] session terminée"), 
    }
}

fn main() {
    let ev1 = Evenement::Connexion { utilisateur: String::from("Alice"), ip: String::from("192.168.1.10") };
    let ev2 = Evenement::Message (String::from("Alice"), String::from("Bonjour tout le monde"));
    let ev3 = Evenement::Deconnexion;
    afficher(&ev1);
    afficher(&ev2);
    afficher(&ev3);
}
