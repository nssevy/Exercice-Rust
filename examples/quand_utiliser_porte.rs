/*
Une porte qui, à un instant donné, est ouverte, fermée ou verrouillée et on veut pouvoir
la faire passer à l'état suivant selon le cycle ouverte → fermée → verrouillée → ouverte.*/
#[derive(Debug)]
#[allow(unused)]
enum State {
    Ouvert,
    Ferme,
    Verrouille
}
#[derive(Debug)]
#[allow(unused)]
struct Porte {
    state: State
}
//#[derive(Debug)]
#[allow(unused)]
impl Porte {
    fn new(a: State) -> Porte {
        Porte {state: a}
    }
    fn swap(&mut self) {
        self.state = match self.state {
            State::Ouvert => State::Ferme,
            State::Ferme => State::Verrouille ,
            State::Verrouille => State::Ouvert,
        };
    }
}
fn main() {
    let mut porte = Porte::new(State::Ouvert);
    porte.swap();
    porte.swap();
    println!("{:?}", porte);
}
