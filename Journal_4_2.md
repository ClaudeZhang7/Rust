
## Zhang Claude

# Journal 4.2

- Réécrire votre code précédent afin de profiter des références Rust.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("helloz");
    let (bol, s1, s2) = meme_longueur(&s1, &s2);
    println!("{} {} {}",bol, s1, s2);
}
fn meme_longueur(s: &String, s1 : &String) -> (bool, String, String) {
    if s.len() == s1.len() {(true,s.to_string(),s1.to_string())} else {(false,s.to_string(),s1.to_string())}
}
```

Vrai, faux ou autre ? "Lorsque d'une fonction à une référence vers un String, cette fonction possède ce String, et par les règles de la possession, cette String est désalloué à la fin de la fonction."

C'est faux, pour une fonction lorsqu'une référence est passé l'accès n'est que temporaire, la fonction ne peu par conséquent pas désalloué.

Vrai, faux ou autre ? "Lorsque d'une fonction à une référence vers un String peut également modifier le String".
Votre fonction meme_longueur précédente (version avec référence) peut-elle être appelé avec deux références identique, i.e. meme_longueur(&s, &s) ?

Vrai si le **mot clé mut est rajouté au sein des paramètres arguments et déclaration de la variable**.

Tentez d'écrire de mémoire les règles sur les emprunts.
(question volontairement flou) Donner différents exemples d'emprunts invalides et essayer d'imaginer, pour chacun d'eux, un contexte dans lequel cela provoquerait un bogue.

Les règles sur les emprunts :

* 2 variables ne peuvent pas emprunter en même temps une même variable sauf si celle-ci est immutable.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
// Erreur ! Deux références mutables simultanées
let r2 = &mut s;
```

```rust
let mut s = String::from("hello");
// Référence immuable
let r1 = &s;    
// Erreur ! Pas de coexistence de mutable et immuable   
let r2 = &mut s;   
```

Quels soucis pourraient survenir si on choisi de faire une fonction second_mot(&String) -> (usize,usize) qui se contente de retourner la taille du premier et laisse le programmeur utiliser le String d'origine et cette taille lorsqu'il veut travailler avec le premier mot ?

Cela crée une incohérence avec l'origine car la String pouvant pointer sur 2 adr (String[0] et String[1]) de par ce fait le système de pointeur est cassé.

Y aurait-il une façon de résoudre ces soucis sans utiliser le concept de slice ? Quel compromis faut-il faire ?

Il est en effet possible, il faudrait que au sein de la fonction, nous puissons avoir l'accès total permettant ainsi de libérer de la mémoire si nécessaire comme dans notre cas.

Si on a un variable s de type String, quel est le type de &s ? Vers quoi pointe le pointeur sous-jacent de &s ? quel est le type de &s[..] ? Vers quoi pointe le pointeur sous-jacent de &s[..] ?

&s sera de type String, elle va pointer vers la structure String avant de pouvoir accéder à la valeur.

Le type &s[..] est de type tableau. elle pointe vers un type de tableau.
