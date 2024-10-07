use std::cmp::Ordering;
use std::io;
// Constantessee
const MAX_POINTS: u32 = 100_000;

fn main() {
    // Variables immuables et mutables
    let _x = 5;      // Immuable
    let mut y = 10; // Mutable
    println!("La valeur initiale de y: {}", y);
    y += 5;
    println!("La nouvelle valeur de y: {}", y);

    // Types de base
    let z: i32 = 42;
    let pi: f64 = 3.14;
    let is_active: bool = true;
    
    // Chaîne littérale, référence à une chaîne statique
    let _name: &str = "Alice";   
    let mut full_name: String = String::from("Alice");
    // Modifiable car `String` est propriétaire
    full_name.push_str(" Johnson"); 

    // Référence immuable à une chaîne littérale ou une sous-chaîne
    // Non modifiable, statique ou emprnut
    // & permet d'emprunter une valeur ce qui nous rend pas propriétaire
    // name emprunte la valeur Alice par exemple. 
    let name: &str = "Alice";
    println!("{}",name);

    let mut s = String::from("Alice");
    println!("{}",s);
    println!("z: {}, pi: {}, actif: {}, nom: {}", z, pi, is_active, name);
    s = String::from("ok");
    println!("{}",s);
    // Conditions
    if z % 2 == 0 {
        println!("z est pair");
    } else {
        println!("z est impair");
    }

    // Boucles
    for i in 0..5 {
        println!("Boucle for: i = {}", i);
    }

    let mut n = 0;
    while n < 3 {
        println!("Boucle while: n = {}", n);
        n += 1;
    }

    // Tableau
    let tableau = [10, 20, 30, 40, 50]; // Un tableau de 5 éléments

    for element in tableau.iter() {
        println!("Élément : {}", element);
    }

    for i in 0..tableau.len(){
        println!("élément : {}", tableau[i]);
    }


    // Appel de fonction
    let sum = addition(5, 10);
    println!("Somme: {}", sum);

    // Utilisation de structures
    let user1 = Utilisateur {
        nom: String::from("Alice"),
        age: 30,
        actif: true,
    };
    println!("Utilisateur: {}, âge: {}", user1.nom, user1.age);

    // Utilisation d'une énumération
    let couleur = Couleur::Rouge;
    afficher_couleur(couleur);

    // Processing game

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // Input grâce à la librairie
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);



    // Compaare

    let secret_number : i32 = 60;

    let change_guess : i32 = 12;
    let guess : &i32 = &change_guess;

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        } 
    }

    println!("{}", guess);

    let value :i32 = secret_number + guess;
    println!("{}",value);
    println!("{} {}",secret_number, guess);


}

// Fonction simple
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

// Structure
struct Utilisateur {
    nom: String,
    age: u8,
    actif: bool,
}

// Énumération
enum Couleur {
    Rouge,
    Vert,
    Bleu,
}

// Fonction qui prend une énumération comme argument
fn afficher_couleur(c: Couleur) {
    match c {
        Couleur::Rouge => println!("La couleur est rouge"),
        Couleur::Vert => println!("La couleur est verte"),
        Couleur::Bleu => println!("La couleur est bleue"),
    }
}
