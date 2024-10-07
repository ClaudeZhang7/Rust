## Zhang Claude

# Journal 5

Les structures sont similaires à une famille de types vue au chapitre 3, laquelle ?

Les tuples

Quelle est la différence entre cette famille de types du chapitre 3 et les structures ?

Les structures permettents de définir plus ou moins un objet (ex struct Zoo) tandis que les tuples n'ont pas de nom ce qui fais que nous pouvons plus ou moins tout faire avec.

Pour accéder aux valeurs d'une structure on passe par leur champs tandis que sur un tuple on passe par leur position (indice). Les structures sont similaires aux dictionnaires que nous avons en Python.

Comment appelle-t-on les différentes parties qui composent une structure ?

Ils sont appelés des champs mais pour ma part, je vois les structures comme une définition d'un objet par conséquent je préfére me dire que ces champs sont des propriétés.

Deviner si le code suivant compile ou non (en ignorant les warnings):

```rust
  struct A { n: i32, m: f64 }
  fn f (x: A) { println!("{} {}", x.n, x.m); }
  
  struct B { n: i32, m: f64 }
  fn main() {
    let x = B { n: 0, m: 0.0 };
    println!("{} {}", x.n, x.m);
    f(x);
  }
```

Je ne pense pas que le code compilera car d'après moi les structures permettent de définir quelque chose, dans le code ci-dessus en effet les paramètres données à la fonction correspondent cependant son type (struct B) n'est pas celle attendu. A mes yeux si cela marcherait, je ne verrais pas l'utilité d'avoir plusieurs structures pour des noms différents.

Vrai, faux ou autre ? "Lorsqu'on instancie une structure, il faut donner les champs dans le même ordre que lors de la déclaration de la structure.

C'est totalement faux, l'ordre n'a pas d'importance cependant, il faut impérativement que toutes les valeurs soient donnés à la structure.

```rust
let x = B { 
    m: 0.0, 
    n: 5 
};
```

Vrai, faux ou autre ? "Dans le code suivant, les deux instances x et y des structures unités X et Y sont égales, car les instances de structures unités sont toutes égales."

```rust
struct X;
struct Y;
fn main() {
    let x = X;
    let y = Y;
}
```

Faux peut-être que leur valeurs sont égales mais leur types n'est pas le même. Pour vérifier cela, il suffirait de tester la condition avec **==** et **===**. 

Déclarer une structure décrivant un livre par son titre, sa description et son nombre de chapitre.

```rust
#[derive(Debug)]
struct Livre {
    titre: String,
    description: String,
    nbr_chapitre: i8,
}
fn main() {
    let test = Livre {
        titre: String::from("teste"),
        description: String::from("ça marche"),
        nbr_chapitre: 6,
    };
    println!("{:?}", test);
}

```

Donner l'expression (et uniquement elle sans rien autour) créant une instance de votre structure pour le Rust Book.

```rust
let rust = Livre {
    titre : String::from("Rust Book"),
    nbr_chapitre : 21,
};
```

Comment indiquer que le champ description de votre structure est mutable mais pas le titre ni le nombre de chapitre ?

Il suffit de rajouter le mot clé mut mais lors de la déclaration de la variable.

```rust
let mut test = Livre {
    titre: String::from("teste"),
    description: String::from("ça marche"),
    nbr_chapitre: 6,
};
```

Donner le code d'une fonction creer_livre qui créer un livre avec un titre et un nombre de chapitre reçu en paramètre et retourne ce livre avec une description vide. Donner deux versions de votre code: l'une sans raccourci d'initialisation de champ et l'autre avec.

```rust 
#[derive(Debug)]
struct Livre {
    titre: String,
    description: String,
    nbr_chapitre: i8,
}
fn creer_livre(title: String, nbr: i8) -> Livre {
    Livre {
        titre: title,
        description: String::from(""),
        nbr_chapitre: nbr,
    }
}
```

En combinant la syntaxe de mise à jour de structure et un raccourci d'initialisation de champ, créer une fonction nouvelle_description qui reçoit un livre et une description et retourne un livre identique à celui en premier paramètre hormis pour sa description est celle reçue en second paramètre. ⚠ Ne pas utiliser de référence ! ⚠

```rust
fn nouvelle_description(livre: Livre, description: String) -> Livre {
    Livre {
        description,
        ..livre
    }
}
```

Écrire une fonction main qui instance un livre l1 via la fonction creer_livre, demande à l'utilisateur une description que l'on va stocker dans une variable description_utilisateur et instancie un autre livre l2 via la fonction nouvelle_description appelée avec l1 et description_utilisateur. Votre fonction main doit afficher le contenu de toutes les variables encore en vigueur juste avant de se terminer, en utilisant la macro dbg!.

```rust
fn main() {
    let l1 = creer_livre(String::from("The Rust Book"), 20);
    print!("Entrez une description pour le livre : ");
    io::stdout().flush().unwrap(); 
    let mut description_utilisateur = String::new();
    io::stdin()
        .read_line(&mut description_utilisateur)
        .expect("fail");
    let description_utilisateur = description_utilisateur.trim().to_string();
    let l2 = nouvelle_description(l1, description_utilisateur);
    dbg!(l2);
}
```

En utilisant la méthode clone vue au chapitre 3, écrire une fonction dupliquer_livre qui reçoit une référence vers un livre et retourne un nouveau livre avec le même titre, la même description et le même nombre de chapitre.

```rust
fn dupliquer_livre(livre: &Livre) -> Livre {
    livre.clone()
}
```

Écrire une nouvelle version de nouvelle_description appelée changer_description qui reçoit en paramètre une référence mutable vers un livre ainsi qu'une description et modifie le champ description du livre pour y mettre la nouvelle description.

```rust

fn changer_description(livre : &mut Livre, descript : String){
    livre.description = descript;
}
```

On souhaiterait éviter de cloner inutilement des chaînes de caractères qui vont en réalité rester identiques. Le type &str permet justement à plusieurs variable de pointé vers la même zone mémoire de chaîne de caractère, mais pas le type String. Quelle fonctionnalité de Rust faudrait-il utiliser pour remplacer les String de votre structure livre par des &str ?

Il suffirait d'utiliser le mot clé self.

Remanier votre code de sorte à ce que creer_livre, nouvelle_description et changer_description soient des fonctions associées à votre structure. Mettre à jour votre fonction main en conséquence et utiliser la syntaxe d'appel de méthode lorsque c'est possible.

```rust
fn nouvelle_description(&self, description: &str) -> Livre {
    Livre {
        description: description.to_string(),
        ..self
    }
}

fn changer_description(&mut self, description: &str) {
    self.description = description.to_string();
}

```

Donner un exemple de votre nouveau main où référencement et déréférencement automatiques de Rust lors rend l'appel de méthodes plus simple.

```rust
fn main() {
    let mut l1 = Livre::creer_livre("The Rust Book", 66);
    let l2 = l1.nouvelle_description("ok");
    let mut l3 = l2.nouvelle_description("ok2");
    l3.changer_description("ok3");
}
```

Qu'est-ce qu'un bloc impl ? Est-il possible d'en avoir plusieurs pour une seule structure ?

C'est un bloc qui permet de définir des fonctions pour la structure spécifié. Oui il est tout à fait possible d'en avoir plusieurs

La notation &self en premier paramètre d'un déclaration de méthode est un raccourci. Quelle est la forme complète ?

La forme complète est fn(&self){...}

Vrai ou faux ? "Comme les méthodes et les champs d'une structure s'invoquent tous les deux avec la notation pointé x.nom, une méthode ne peut pas porter le même nom d'un champ."

Faux.

Donner un exemple d'attribut externe.

```rust
#[derive(Debug)]
```

## Chapitre 6

Définir une énumération permettant de décrire une couleur qui peut soit être en RGB ou en CMYK, et faire une fonction main de teste qui les instancie et affiche une instance de chaque variante en utilisant dbg!

```rust
#[derive(Debug)]
enum Couleur {
    RGB(u8, u8, u8),
    CMYK(u8, u8, u8, u8),
}
fn main() {
    let rgb = Couleur::RGB(120, 0, 0);
    let cmyk = Couleur::CMYK(1, 50, 60, 2); 
    dbg!(rgb);
    dbg!(cmyk);
}
```

Définir une structure permettant de décrire un carte par sa "couleur" (qui est une énumération dont les variantes sont cœur, trèfle, carreau et pique) et sa "valeur" (qui est une énumération dans la première variante est muni d'un u8 précisant la valeur en 1 et 10 et trois variantes supplémentaires correspondent à valet, dame et roi).

```rust
#[derive(Debug)]
enum Couleur {
    Coeur,
    Trefle,
    Carreau,
    Pique,
}
#[derive(Debug)]
enum Valeur {
    Nbr(u8),  
    Valet,
    Dame,
    Roi,
}
```

```rust

Faire un programme qui instancie toutes les cartes possibles et les affiche avec dbg!.

fn main() {
    let couleurs = [Couleur::Coeur, Couleur::Trefle, Couleur::Carreau, Couleur::Pique];
    for couleur in &couleurs {
        for valeur in 1..10 {
            let carte = Carte {
                couleur: couleur,
                valeur: Valeur::Num(valeur),
            };
            dbg!(carte);
        }

        dbg!(Carte {
            couleur: Couleur::Carreau,
            valeur: Valeur::Valet,
        });
        dbg!(Carte {
            couleur: Couleur::Carreau,
            valeur: Valeur::Dame,
        });
        dbg!(Carte {
            couleur: Couleur::Carreau,
            valeur: Valeur::Roi,
        });
    }
}
```

Vrai ou faux ? "Dans un langage utilisé le concept de valeur nulle, n'importe quelle variable peut potentiellement contenir null et c'est au programmeur de savoir si cette possibilité est effectivement ou non, ce qui pose un problème si le programmeur pense que ce n'est pas possible alors qu'en fait la valeur nulle est possible"

Je pense que c'est vrai.

Vrai ou faux ? "En Rust, une valeur pouvant être invalide ou non-disponible à pour type Option<T> alors qu'une valeur qui est toujours valide a pour type T directement, donc le programmeur ne peut pas se tromper et doit travailler différent dans un cas et dans l'autre."

C'est vrai.

Donner le code d'une fonction seulement_positif qui reçoit une valeur i32 et retourne un Option<i32> qui est None si la valeur est strictement négative, et Some(v) sinon.

```rust
fn seulement_positif(valeur: i32) -> Option<i32> {
    if valeur < 0 {None} else {Some(v)} 
}
```

En utilisant match, faire une fonction valeur_non_atout qui reçoit une carte (voir question précédente) et retourne sa valeur à la belote (https://fr.wikipedia.org/wiki/Belote#Valeur_des_cartes) en supposant que les cartes de couleur cœur sont atout et les autres couleurs non.

```rust
fn valeur_non_atout(carte: Carte) -> u8 {
    match carte.couleur {
        Couleur::Coeur => 0, 
        _ => match carte.valeur {
            Valeur::Roi => 4,
            Valeur::Dame => 3,
            Valeur::Valet => 2,
            Valeur::Num(10) => 10,
            Valeur::As => 11,
            => 0
        },
    }
}
```

```rust
fn somme(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(val_a), Some(val_b)) => Some(val_a + val_b),
        _ => None,
    }
}

fn somme2(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    if  (Some(val_a), Some(val_b)) = (a, b) {Some(val_a + val_b)} else {None}
}
```