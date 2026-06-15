use Nourriture::{Cochon, Humain, Rien};

pub enum Nourriture {
    Poulet,
    Cochon,
    Humain,
    Rien,
}

pub enum Degats {
    Faible,    // 25
    Moyen,     // 35
    Fort,      // 50
}

pub struct dinosaure {
    type_de_dino: String,
    crie: String,
    hauteur: i32,
    poids: i32,
    vie: i32,
    degats: Degats,
    mange: Nourriture,
}

impl dinosaure {

    pub fn descriptionDino(&self) {
        println!("Le crie du dino est : {} \n La hauteur du dino est : {} \n Le poids du dino est de {} \n Le dino a {} de vie \n", self.crie, self.hauteur, self.poids, self.vie);
    }

    pub fn attaquer(&mut self, adverssaire: &mut dinosaure) {
        match self.degats {
            Degats::Faible => {
                adverssaire.vie -= 25;
            }

            Degats::Moyen => {
                adverssaire.vie -= 35;
            }

            Degats::Fort => {
                adverssaire.vie -= 50;
            }
        }
    }

    pub fn typedenourriture(&mut self, aliment: Nourriture) {
        match aliment {
            Nourriture::Poulet => {
                if self.vie >= 100 {
                    println!("On ne rajoute rien");
                }
                else {
                    self.vie += 5
                }
            }
            Nourriture::Cochon => {
                if self.vie >= 100 {
                    println!("On ne rajoute rien");
                }
                else {
                    self.vie += 10
                }
            }
            Nourriture::Humain => {
                if self.vie >= 100 {
                    println!("On ne rajoute rien");
                }
                else {
                    self.vie += 20
                }
            }

            Nourriture::Rien => {
                println!("Tu n'a rien a mangé");
            }
        }
    }

    pub fn mange(&mut self) {
        self.poids += 2;
        self.hauteur += 20;
    }
}