
## Zhang Claude

# Journal 3

Selon la page d'introduction du chapitre, combien de concepts y sont abordés ?

D'après la page d'introduction du chapitre, il y a 5 concepts :

* **les variables et la mutabilité**
* **les types de données**
* **les fonctions**
* **les commentaires**
* **le flux de contrôle**

Qu'est-ce qu'un mot clé ?

Un mot clé permet de définir la structure du code. Par exemple, les mots-clés **let, fn, et mut** permettent au compilateur de comprendre ce que nous souhaitons faire. Par exemple let est utilisé pour déclarer une variable, fn pour déclarer une fonction, et mut pour rendre une variable mutable.

Tous les mots clés sont-ils associés à des fonctionnalités ?

Non car d'après le chapitre 2 dans la section [Keyword]('https://doc.rust-lang.org/reference/keywords.html#strict-keywords') certains mot clés ne sont pas encore associés à des fonctionnalités mais, ils le seront dans le future.

Où peut-on trouver la liste de tous les mots clés ?

La liste de tous les mots clés se trouvent sur l'annexe de [Read The Book]('https://doc.rust-lang.org/stable/book/appendix-01-keywords.html') de Rust dans la partie **Keywords**.

Vrai, faux, ou autres ? "Une variable correspond à toujours une "mémoire" pouvant prendre différentes valeurs durant une seule et même exécution."

Faux, il est vrai qu'en Rust, nous pouvons gérer la mémoire. Cependant, celle-ci ne peut pas toujours prendre différentes valeurs. Par exemple, une variable immuable ne peut pas prendre différentes valeurs, à moins d'être déclarée comme mutable avec le **mot-clé mut**.

Comment déclarer une variable correspond à une "mémoire" pouvant prendre différentes valeurs durant une seule et même exécution ?

Il suffit de rendre mutable la variable pour cela, il suffit de rajouter le mot clé mut. 

```rust
let mut varMutable = 1;
varMutable = 5;
```

Vrai, faux, ou autres ? "Si un variable ne prends qu'une seule valeur au cours de l'exécution d'un programme, alors il est toujours possible de la déclarer comme une constante."

C'est faux, car le **mot clé pour une variable et une constante n'est pas la même** par conséquent, on ne peut pas passer d'une variable immuable à une constante.

Vrai, faux, ou autres ? "Comme en Java, deux variables Rust différentes ont forcément des noms différents au sein d'un seul et même bloc de portée."

C'est faux, c'est en effet un principe de bonne pratique de code qui nous a été enseigner cependant en Rust, nous pouvons déclarer une variable avec le même nom cela s'appel le shadowing.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        // 12
        println!("The value of x in the inner scope is: {x}");
    }
    // 6
    println!("The value of x is: {x}");
}
```

Vrai, faux, ou autres ? "Au sein d'un bloc de portée, un nom de variable correspond à un unique et même type tout au long du bloc de portée."

Faux, avec le shadowing, on peut utiliser un même nom de variable avec des types différents car le **shadowing permet de redéclarer la variable** et donc de pouvoir changer le type de celui-ci. Je pense également qu'il y a une autre manière tout comme en **Java** ou on peu **caster la variable** pour changer le type de donnée.

```rust
let mut entier = 5;
let mut 
```

Donner la liste des principaux types scalaires de Rust.

Les principaux types scalaires de Rust sont :

* **les entiers**
* **les nombres à virgules**
* **les booléens**
* **les caractères**

Combien y a-t-il de types de nombres entiers en Rust ?

Il en existe **12** répartis sur **6** chacune pour les **entiers signés (i8, i16, i32, i64, i128, isize)** et **non signés (u8, u16, u32, u64, u128, usize)**

Combien de types de nombres entiers ont un nombre de bit indépendant de l'architecture vers laquelle le programme est compilé ?

**10** tous les types de nombres entiers mis à part **isize et usize** ont un nombre de bit bit indépendant de l'architecture vers laquelle le programme est compilé.

Quelle est le type de nombres entiers par défaut ?

Le type de nombres entiers par défaut est **i32**.

Quelles types de nombres entiers utilise-t-on pour indexer un tableau en Rust ?

Les types de nombres entiers utiliser pour indexer un tableau en Rust sont : **isize et usize**

Quelle est la représentation binaire machine de la valeur -3 de type i16 ?

La représentation binaire machine de la valeur -3 de type i16 est **1111 1111 1111 1101**

Il suffit de convertir 3 en binaire puis d'inverser les signes des bits et enfin de rajouter 1. Lorsque la valeur est positif, sa représentation binaire est sa valeur en binaire directement.
```rust
// valeur binaire de 3 sur 16 bits
0000 0000 0000 0011
// inversion des bits/signes
1111 1111 1111 1100
// Rajout de la valeur 1
1111 1111 1111 1101
```

Donner le littéral correspond à -3 de type i16.

Le littéral correspondant à -3 de type i16 est : **-3i16**.

```rust
let litteral = -3i16;  
```

Comment écrire le littéral correspond à "cent millions" en Rust de sorte à rendre facilement lisible que le nombre de zéro est le bon ?

**100_000_000**

```rust
let centM = 100_000_000;
```

Vrai, faux, ou autres ? "En Rust, si on incrémente une variable de type u8 dans une boucle infini, alors toutes les valeurs de 0 à 255 s'affiche en boucle et le programme ne s'arrête jamais."

Ca depend des modes si nous sommes en release :

* cela est vrai car **la plage qui à été définit à été dépasser** par conséquent les valeurs suivantes revienent au point de départ comme par exemple 256 qui deviens 0 ou 257 qui deviens 1.

cependant lorsqu'on compile en mode débogage, on aura une erreur.

Donner la liste des types à virgule flottante.

Il existe deux types primitifs pour les nombres à virgule flottante : **f32 et f64**.

Vrai, faux, ou autres ? "Rust est un langage bas niveau, donc une variable à virgule flottante a une représentation différente en fonction de l'architecture vers laquelle on compile notre programme."

C'est faux car les virgules flottantes sont forcémment représentés conformément à la norme **IEEE-754**.

Vrai, faux, ou autres ? "Le type flottant par défaut a une taille de 32 bits."

C'est faux le type par défaut est **f64** qui à respectivement une taille de **64 bits**.

Où peut-on consulter la liste de tous les opérateurs numériques ?

Dans la partie Annexe de la section [Operators and Symbols]('https://doc.rust-lang.org/book/appendix-02-operators.html').

Vrai, faux, ou autres ? "Rust profite d'une panoplie de près de 200 opérateurs numériques."

Faux Rust n'a pas une aussi grande panoplie d'opérateurs numériques.

Vrai, faux, ou autres ? "En Rust, un test qui échoue (comme 10 > 30) renvoie 0 alors qu'un test qui réussi comme (10 < 30) renvoie 1."

C'est faux, cela est valable dans d'autres langages de programmation mais en Rust les booléens sont représentés seulement par **true ou false**.

Vrai, faux, ou autres ? "Le type char représente 8 octets et représente un caractère ASCII."

Faux, le type char représente **4 octets** et représente une **valeur scalaire Unicode**, ce qui signifie qu'il peut représenter bien plus que de l'ASCII.

Vrai, faux, ou autres ? "Il est possible d'ajouter des éléments dans un tableau Rust, modifiant ainsi sa taille, au cours de l'exécution d'un programme."

Faux, lors de son initialisation la taille du tableau est **fixe**, nous ne pouvons par conséquent pas modifier sa taille au cours de l'exécution d'un programme. Pour cela, il faudra utiliser des **vecteurs**.

Si on a écrit un programme avec une variable x de type (i32, i32, i32), décrire informellement comment le réécrire pour que son type soit maintenant [i32; 3] ? Le programme résultat est-il globalement plus court ? plus long ? de même longueur ?

J'utiliserais le **shadowing** pour redéclarer la même variable mais sous forme de tableau avec les données correspondant au tuple et le programme sera de **même longueur**.

```rust
let x: (i32, i32, i32) = (1, 2, 3);
let x : [i32;3] = [x.0, x.1, x.2];
```

Si on a écrit un programme avec une variable x de type [i32; 3], décrire informellement comment le réécrire pour que son type soit maintenant (i32, i32, i32) ? Le programme résultat est-il globalement plus court ? plus long ? de même longueur ?

La logique est la même que celle précedente et le programme sera de même longueur.

```rust
let x : [i32;3] = [1,2,3];
let x: (i32, i32, i32) = (x[0], x[1], x[2]);
```

Que se passe-t-il en Rust si on tente d'accéder à la case 10 d'un tableau de taille 5 ? En quoi cela diffère de ce qui se passe en C ?

On aura une **erreur (panique)** car les tableaux sont fixes de par ce fait, nous ne pouvons pas afficher les résultats supérieurs à la longueur du tableau. 

En C et comme tout autre langage **la taille n'est pas forcément fixe** comme en Rust ce qui signifie qu'après sa déclaration on peu rajouter des éléments dans le tableau jusqua une taille de tableau de 10. Cependant si on essaye d'afficher un l'index 10 d'un tableau mais que celle-ci n'a que 9 valeurs par exemple, nous aurons également une **erreur comme quoi l'index est en dehors de la longueur du tableau**.

Existe-t-il d'autres types Rust fonctionnant comme les tableaux ?

Oui, voici les différents types à ma connaissance qui fonctionne comme les tableaux :

* **les vecteurs**
* **les slices**

Vrai, faux, ou autres ? "Comme en Java, Rust a besoin d'allouer de la mémoire et d'utiliser des pointeurs en interne pour faire fonctionner ces types composés."

Cela est vrai mais **en Java la mémoire n'est pas gérer par le développeur contrairement à Rust**.

Proposer une expression à mettre à la place des points d'intérrogation afin que le programme suivant compile et affiche "Yes" à l'exécution.

```rust
fn main() {
    let x = ???;
   if x.2[x.1].0 
    {
    println!("Yes");
   } 
    else 
    {
    println!("No");
   }
  }
```

Proposision d'expression à mettre à la place des points d'intérrogation.

```rust
fn main() {
    // x.2 = [(true,false)]
    // x.2[0] = (true,false)
    // x.2[0].0 = true
    let x = (666, 0, [(true,false)]);
   if x.2[x.1].0 
    {
    println!("Yes");
   } 
    else 
    {
    println!("No");
   }
  }
```

Transformer le code précédent en une fonction recevant en paramètre la valeur de x.

```rust

fn main() {
    let x = (666, 0, [(true,false)]);

    let x = fct(x);
}

fn fct(x : (i32,usize, [(bool,bool);1]) ){
    if x.2[x.1].0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
```

Modifier la fonction pour qu'elle prenne un paramètre la valeur de x et retourne un booléen au lieu d'afficher un message. Faites le bien "à la Rust", en suivant le style indiqué dans le chapitre.

```rust
fn main() {
    let x = (666, 0, [(true,false)]);

    let x = fct(x);
    
    println!("{x}");
}
fn fct(x : (i32,usize, [(bool,bool);1])) -> bool {
    x.2[x.1].0
}
```

Quelle convention utilise-t-on en Rust pour nommer les variables et les fonctions ?

Rust utilise la convention **snake case** pour nommer les variables et les fonctions, **dans lequel toutes les lettres sont en minuscules et les mots séparés par des soulignements**.

```rust
let une_variable = 5;
fn une_fonction(){
    println!("20/20");
}
```

Quel est normalement la différence technique entre "paramètre" et "argument" ?

Le **paramètre correspond à une valeur inconnu donné** que je qualifie comme *"donné un type de valeur"* alors qu'un **argument correspond à la valeur qui est donné à la fonction** c'est à dire lorsqu'on **appel la fonction avec une valeur concrète**.

```rust
fn main(){
    println!("{}",test(32));
}
// paramètre
fn test(a : i32)-> i32{
    a 
}
```

Le bout de code let x = 10 est une expression ou une instruction ?

C'est une instruction car, **elle ne retourne pas de valeur.**

Le bout de code let x = 10 contient il une expression ?

Non, **une expression est représenté par le renvoie d'une valeur comme une fonction ou une encapsulation ({...})**.

Vrai, faux, ou autres ? "Lorsqu'on déclare une variable, il est optionnel d'indiquer le type."

Oui, il est en effet **optionnel d'indiquer le type**.
 
Vrai, faux, ou autres ? "En Rust, il n'existe qu'une seule façon de faire des commentaires."

C'est vrai, que le commentaire soit sur une ligne ou que le commentaire s'étend au-delà de plusieurs lignes, il faudra rajouter **les deux barres obliques (//)** sur chaque ligne. 

Dans le code suivant, que faut-il mettre à la place des parenthèses pour que le code de la fonction compile (on ignorera les warnings):

fn blabla(x: i32) -> ??? {
	x + x
}

Il suffit de spécifier le **type de retour de la fonction (i32)**.

PS : vous avez fait une erreur il ne devrait pas y avoir de point virgule à la fin de la fonction.

```rust
fn blabla(x: i32) -> i32 {
	x + x
}
```

Les if/else sont ils des instructions ou des expressions ?

Ce sont des expressions.

Remanier le code suivant afin de n'avoir qu'un seul appel à println! et aucune variable supplémentaire.

```rust
fn main() {
    let x = ???;
 if x.2[x.1].0 {
  println!("Yes");
 } else {
  println!("No");
 }
}
```

Proposition

```rust
fn main() {
    let x = (666, 0, [(true,false)]);
    println!("{}", if x.2[x.1].0 {"Yes"} else {"No"}  );
}
```

A quoi servent les étiquettes de boucle ?

C'est un nom qu'on donne à une boucle pour éviter l'ambiguïté entre plusieurs boucles (boucle imbriquée), les étiquettes de boucles commencent par une **apostrophe (')**.

Une boucle loop est une expression. Quels types peuvent être retournés par un loop ?

N'importe laquelle, il suffit de rajouter **break valeur_a_retourner** au sein de la boucle.

```rust
let result = loop 
{
    counter += 1;
    if counter == 10 
    {
        break counter * 2;
    }
};
```

Une boucle while est une expression. Quels types peuvent être retournés par un while ?

Aucune, une boucle while ne renvoie rien, le mot clé break ne marche d'ailleurs pas au sein d'une boucle while.

Les boucles for de Rust ressemble au boucle for de quels languages que vous connaissez ?

Pour moi les boucles for de Rust ressemble énormément aux languages :

* **Python**
* **JavaScript**
* **Java**
* **C#**

Comme suggéré à la fin du chapitre, réaliser les programmes suivants:
Convertir des températures entre les degrés Fahrenheit et Celsius.

```rust
fn main() {
    let number = fahrenheit_to_celsius(50.0, true);
    let nm = fahrenheit_to_celsius(10.0, false);
    println!(" 50 Fahrenheit équivaut à  {} degrés Celsius",number);
    println!("{nm}");
}
fn fahrenheit_to_celsius(f : f32, b : bool)->f32{
    if b {(f -32.0)/1.8} else {(f *1.8) + 32.0}
}
```

Générer le n-ième nombre de Fibonacci.

```rust
fn main() {
    let fib = fibonacci(10);
    println!("{fib}");
}
fn fibonacci(n : i32)->i32{
    if n <= 1 {n} else {fibonacci(n-1) + fibonacci(n-2)}
}
```
