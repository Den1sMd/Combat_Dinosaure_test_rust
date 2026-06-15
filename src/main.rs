mod settings_dino;
use std::io;
use crate::settings_dino::{dinosaure, Degats};
use crate::settings_dino::Nourriture::{Cochon, Humain, Poulet};

fn main() {

    println!("Bienvenue dans mon jeu !");


    let mut forname: String = String::new();
    println!("Entre le nom de ton dinosaure : ");
    io::stdin().read_line(&mut forname).expect("Pas reussi a gardé le nom du dino (Joueur)");
    let forname = forname.trim().to_string();

    let mut crie_joueur: String = String::new();
    println!("Choisi un crie de dinosaure : ");
    io::stdin().read_line(&mut crie_joueur).expect("Pas réussi a gardé le crie du dino (Joueur)");
    let crie_joueur = crie_joueur.trim().to_string();

    let mut hauteur_joueur: String = String::new();
    println!("Choisi sa hauteur : ");
    io::stdin().read_line(&mut hauteur_joueur).expect("Pas réussi a gardé la hauteur.");
    let hauteur: i32 = hauteur_joueur.trim().parse().unwrap();

    let mut poids_joueur: String = String::new();
    println!("Choisi son poids : ");
    io::stdin().read_line(&mut poids_joueur).expect("Pas réussi a gardé le poids");
    let poids: i32 = poids_joueur.trim().parse().unwrap();




    //dino joueur ici
    let mut dino_joueur: dinosaure = dinosaure {
        type_de_dino: forname, crie: crie_joueur, hauteur: hauteur, poids: poids, vie: 100, degats: Degats::Faible, mange: Poulet
    };


    //dino de test ici
    let mut dino_bot: dinosaure = dinosaure {
        type_de_dino: String::from("T-rex"), crie: String::from("Rawrrrrr"), hauteur: 20, poids: 10, vie: 100, degats: Degats::Fort, mange: Humain
    };
    let mut descriptionDino2: dinosaure = dinosaure {
        type_de_dino: String::from("B-rex"), crie: String::from("grrrr"), hauteur: 60, poids: 30, vie: 100, degats: Degats::Moyen, mange: Cochon
    };
//

    dino_joueur.descriptionDino();


    while dino_bot.vie >= 0 && dino_joueur.vie >= 0 {
        bagarre(&mut dino_joueur, &mut dino_bot);
    }

    if dino_joueur.vie <= 0 && dino_bot.vie <= 0 {
        println!("Egalité !!")
    } else if dino_joueur.vie >= 0 && dino_bot.vie <= 0 {
        println!("Tu a gagné BRAVOOOO !!")
    }
    else {
        println!("Tu a perdu Nonnnnn !")
    }

    dino_joueur.descriptionDino();
}






fn bagarre(joueur1: &mut dinosaure, bot: &mut dinosaure) -> String {

    match bot.degats {
        Degats::Faible => {
            joueur1.vie -= 25;
        }
        Degats::Moyen => {
            joueur1.vie -= 35;
        }

        Degats::Fort => {
            joueur1.vie -= 45;
        }
    }

    match joueur1.degats {

        Degats::Faible => {
            bot.vie -= 25;
        }

        Degats::Moyen => {
            bot.vie -= 35;
        }
        Degats::Fort => {
            bot.vie -= 50;
        }
    }


    format!(
        "C'etait amusant !"
    )
}