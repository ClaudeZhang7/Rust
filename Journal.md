
# Compte Rendu

### Les différentes commandes se trouvent dans le fichier commandes.md !

Page d'accueil :

* Lire les 2 premières extraits et écrire dans le journal les questions que ça vous pose.  

Page Apprendre :

* Jeter un coup d'oeil sur les trois sources qui y sont mentionnées (télécharger Rustlings, regarder la table des matières du Rust Book et de Rust By Example).

Rust Book :

* Commencer à le lire depuis le début, en écrivant une phrase résumé et une question pour chaque paragraphe.

__________________________________________________________________

Page d'accueil :

**Question :** Rust est-il plus rapide et économe en mémoire ?

Rust Book

**1. Getting Started** : Initialisation basic à rust avec son installation et ses dépendances **(rust-analyzer, compilateur C++)**.

**Cargo** est le gestionnaire de paquets de Rust Similaire au package NuGet pour .Net

**Question :** 
* quels sont les dépendances de rust ?
* qu'est ce que Cargo ?

**2. Programming a Guessing Game**

**Question :** 
* c'est quoi le type par défaut d'une variable? 
* pourquoi les types sont représenté par des valeurs exemple : i32 ?

**Rust** ressemble beaucoup au **Java** cependant son typage semble différent par exemple les types mutables et immuable sont présents.

L'expression **match** permet de comparer une valeur avec une série de motifs et d'exécuter du code en fonction du motif qui correspond. 

L'expression **let** permet d'instancier une variable tous comme dans les autres langages de programmations.

Les dépendances se trouvent dans le fichier **Cargo.toml**.

Exemple de fichier Cargo.toml :
```rust 
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies] 
```

Syntaxe pour un **input** :
```rust
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```
Syntaxe pour **appeler une variable** :
```rust
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
```
Syntaxe d'un **appel à une fonction depuis une librairie** :
```rust
// NomLibrairie::fonction

   match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
        }
```
**3. Common Programming Concepts**

Les variables sont par défauts **immuable** de par ce fait il faut rajouter le type **mut**. Même si la mention mut est rajouté on ne peut pas changer le type de la même variable. Lorsqu'une variable à été assigné à une autre celle assigné disparaît. Il est en effet possible d'avoir plusieurs fois le même nom de variable cependant celle-ci est **"masqué"**.

Exemple : 
```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        //12
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // 6 
    println!("The value of x is: {x}");
}
```


Les types **uNombre** ou **iNombre** correspondent à des types signé ou non signé par exemple i32 correspond à la longueur que peut prendre une variable représenté par 32 **bits** (2^32). Ces types **existent** car dans Rust **chaque valeur est assigné à un type (langage typé)** cela permet d'indiquer à Rust quel type de données est spécifié.

   Length |   Signed |   Unsigned |  
|--- |:-:     |:-:      |
|   8-bit     |   i8    |   u8    |
|   16-bit    |   i16   |   u16   |
|   32-bit    |   i32   |   u32   |
|   64-bit    |   i64   |   u64   |
|   128-bit   |   i128  |   u128  |
|   arch      |   isize |   usize |

Syntaxe pour **préciser le type** d'une variable :
```rust
    let c: char = "T";
    let d: i32 = 64;
```
Syntaxe pour **regrouper plusieurs types** :
```rust
    // correspond à un vecteur/ligne
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    // Pareil mais pour un tableau
    let a: [i32; 5] = [1, 2, 3, 4, 5];
```
Syntaxe pour une **fonction** :
```rust
    fn greaterThan(a : i32, b : i32) -> bool {
        a > b
    }
```
Syntaxe **particulière pour une condition** :
```rust
    // On assigne la valeur directement selon la condition
    let condition = true;
    let number = if condition { 5 } else { 6 };
```

Syntaxe pour une **boucle** :
```rust
    let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
```
**4. Understanding Ownership**

**Question :**

* à quoi sert d'allouer ?
* comment faire une réference (pointe vers) ?
* quels sont les règles établi par Rust ?

**Ownership** correspond à **une sucession de règle** établi par Rust pour un programme.

Règles :

* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

Chaque variable à une **portée** qui est dite encapsuler selon la manière dont elle est faite c'est à dire que chaque variable que nous allons créer à une espérance de vie correspondant à son encapsulation.

Exemple :

```rust
    {   // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }   // this scope is now over, and s is no longer valid
```

Une **référence** est un terme désigné pour parler du fait qu'on va **pointer** sur l'adresse de la variable au lieu de pointer sur sa valeur comme dans d'autre langages de programmations sa référence se fait à travers l'élément **&**.

Syntaxe pour **pointer vers l'adresse** d'une variable :
```rust
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");
```

Les **tranches (slices)** sont des références spéciales permettant d'emprunter une séquence d'éléments d'une collection(tableau) plutôt que la collection complète.

Syntaxe pour une **tranche** :
```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

Il est également possible de renvoyer une slice (une partie d'un tableau). La logique est similaire à ce qui se trouve en python ou on peu a

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

**5. Using Structs to Structure Related Data**

**Question :**
* à quoi sert une structure ?
* peut on typer une variable avec une structure ?

Une structure est un **type de données personnalisé** qui permet de regrouper et de nommer plusieurs valeurs liées qui constituent un groupe significatif. A titre d'exemple cela ressemble à du json. Pour pouvoir **utiliser** une structure, il faut que celle-ci soit implémenté avec le terme **Debug**. Chaque structure **peu avoir un format différent** selon le code par exemple on peu retrouver une structure au format tableau ou bien mêmes des tuples.

Syntaxe pour une **structure**:

```rust
    #[derive(Debug)] // obligatoire
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    } 

    fn main(){
    // Son utilisation
    let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    // on peu remplir plus tards la structure
    sign_in_count:,
    };
    // L'affichage d'une structure est différente
    println!("{:?}", user1);

}
```
Syntaxe structure **format tuple**

```rust
fn main() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

```

Il est également possible de définir des structures qui n'ont **aucun champ (structure de type unitaire)**

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```





