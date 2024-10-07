
## Zhang Claude

# Journal 4


Quels sont les 4 thèmes abordés dans ce chapitre ?

Les 4 thèmes abordés dans ce chapitre sont :

* la possesion
* les références
* l'emprunt
* les tranches (slice)

Quelles sont les deux approches traditionnelles de gestion de la mémoire ?

Le **garbage collection** qui recherche régulièrement la mémoire inutilisée pendant l'exécution du programme et la deuxième approche est le fait que le **programmeur doit explicitement allouer et libérer la mémoire**.

Est-il a priori rapide de se familiariser avec le concept de possession ?

Non, vu que c'est un nouveau concept cela prend du temps à se familiariser avec.

Quelle structure de donnée est utilisé dans le chapitre pour exemplifier la possession ?

La structure de donnée utilisé dans le chapitre pour exemplifier la possession est le **Strings**. 

Connaîssez-vous un autre langage qui demande la maitrise de la différence pile/tas ?

**C et Java**

(Hors Livre) En gros, quelles instructions assembleurs permettent de manipuler la pile, et comment la pile est-elle gérée ? et par quoi ? processeur/système d'exploitation/bibliothèque ?

Instructions : push et pop

Lorsqu'on déclare une variable cela permet de manipuler la pile.

La pile est géré par le développeur à travers l'allocation de la mémoire du processeur.

(Hors Livre) En gros, quelles instructions assembleurs permettent de manipuler le tas, et comment le tas est-il géré ? et par quoi ? processeur/système d'exploitation/bibliothèque ?

Le tas est géré par l'utilisateur, elle est géré par l'allocateur de mémoire à travers le processus.

(Hors Livre) Pourquoi il est mieux de consulter des données qui sont proches les unes des autres pour un processeur moderne ?

C'est mieux car c'est une question de position (distance) le processeur effectuant le travail aura plus de facilité à faire son  travail si les données sont plus proches alors que plus les données sont loins plus le temps de calcul est long.

Quel genre de données sont stockées sur la pile ?

Pointeur ?
<!-- Le String type -->

Quel est le but principale de la possession ?

Le but principale de la possession est de garantir la sécurité des programmes Rust.

Quand est-ce qu'une valeur est supprimée ?

Lorsque le propriétaire sort du champ la variable est supprimé.

Vrai, faux ou autre ? "Une variable est "en vigueur" de sa déclaration jusqu'à la fin de l'exécution de programme."
En terme de pile et de tas, quelle est la différence entre les types du chapitre 3 et le type String ? Pourquoi cette différence est-elle nécessaire ?

ça dépend de la portée de la variable. Les types du chapitre 3 sont stockés sur la pile tandis que le type String est stocké sur le tas. Car pour les types du chapitre 3, ils ont plage de valeur fixe tandis que le type String peu changer de taille.

Donner une expression de type String.

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{s}"); // This will print `hello, world!`
```

Les littéraux comme ", world" sont-ils de type String ?

Faux le type String est présent seulement lors de son initialisation, c'est à dire que lorsqu'on va ajouter des éléments au sein de la variable, ils seront de type **str**.

Vrai, faux ou autre ? "Les chaînes de caractères littérales et celles saisies par l'utilisateur sont stockés dans la même zone mémoire."

Faux, les valeurs des inputs sont stockés en mémoire statique tandis que les chaînes de caractères littérales sont stockés dans la mémoire dynamique.

Quelle fonction est automatiquement appelée lorsque qu'un String sort de sa portée ?

**drop**

Une valeur de type String nécessite de la mémoire: uniquement dans le tas, uniquement sur la pile, ou sur les deux ?

Sur le tas.

Qu'est-ce qu'une "double libérations" (ou "double free") et son rapport avec la sécurité informatique ?

La double libération est une faille de gestion de la mémoire qui se produit lorsqu'un programme libère deux fois le même bloc de mémoire à l'aide de la fonction free() ou delete .

Étant donnée la façon dont Rust gère la mémoire, pourquoi serait-il un problème que deux pointeurs possèdent la même mémoire dans la tas en même temps ?

Car cela supprimerait les 2 variables et non celle dont, nous n'avons plus d'utilité.

Vrai, faux ou autre ? "Une instruction telle que let y = x; peut parfois représentée une copie profonde couteuse."
Quelle est la différence entre un déplacement et une copie superficielle ?

Un déplacement transfère la propriété des données sans duplication, ce qui est efficace pour les types complexes.

Une copie superficielle duplique les données pour les types qui implémentent Copy, mais ne concerne que des valeurs simples qui sont peu coûteuses à copier.

Vrai, faux ou autre: "Si on souhaite qu'un type fasse une copie il suffit simplement d'annoter ce type avec le trait Copy."

Faux, les types qui implémentent Copy. sont les **types simples (entier,décimal,booléen,...)** et peu coûteux à copier.

Vrai, faux ou autre: "Tout ce qui est alloué durant l'exécution d'une fonction est désallouée à la fin de l'exécution de la fonction."

Faux, les types simples sont eux désallouée à la fin de l'éxécution de la fonction cependant pour les structures dynamiques ce n'est pas le cas.

- Vrai, faux ou autre ? "Pour des raisons de sécurité et de cohérence, une fonction ne peut désallouer que ce qu'elle a alloué."

Faux, une fonction peut être responsable de la désallocation de ressources qu'elle n'a pas elle-même allouées, grâce au système de propriété (ownership). Lorsqu'une fonction reçoit une variable par valeur (et non par référence), elle prend possession de cette variable. Si elle ne transfère pas cette propriété ailleurs (par exemple, en la retournant), la variable sera désallouée lorsque la fonction se termine.

- Vrai, faux ou autre ? "Les fonctions n'introduisent aucunes subtilités sur la gestion de la possession: pour comprendre la possession il suffit comprendre ce qui se passe avec des variables."

Faux, Rust introduit des subtilités dans la gestion de la possession. Les fonctions peuvent modifier la manière dont la propriété est transférée ou empruntée

- Dans le même style que l'encart 4-5, écrire une fonction meme_longueur qui recoit deux String, et retourne un booléen indiquant s'ils ont la même taille, ainsi que les deux String pour rendre leur propriété. Écrire une fonction main similaire à celle de l'encart pour tester votre fonction.

```rust

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("helloz");
    let (is_same_len,b1, b2) = meme_longueur(s1,s2);
    println!("{} {} {}",is_same_len,b1, b2);
}
fn meme_longueur(s: String, s1 : String) -> (bool, String, String) {
    if s.len() == s1.len() {(true,s,s1)} else {(false,s,s1)}
}
```
