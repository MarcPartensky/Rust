#!/usr/bin/env run-cargo-script

// use std::fs::File;

// extern crate rand;
// use tetra::graphics::{self, Color};
// use tetra::{Context, ContextBuilder, State};

// fn machin(i: i32) -> Option<String> {
//     if i < 10 {
//         Some("variable inférieure à 10".to_owned())
//     } else {
//         None
//     }
// }

// fn addition(nb1: i32, nb2: i32) -> i32 {
//     return nb1 + nb2;
// }

// fn get_bigger(nb1:  i32, nb2:  i32) ->  i32 {
//     if nb1 > nb2 {
//         nb1
//     } else {
//         nb2
//     }
// }


// fn fait_quelque_chose() {
//     println!("Je fais quelque chose !");
// }
// // ou bien :
// fn fait_quelque_chose_2() -> () {
//     println!("Je fais quelque chose !");
// }

fn test_expression(x: i32) -> i32 {
    if x < 0 {
        println!("{} < 0", x);
        -1
    } else if x == 0 {
        println!("{} == 0", x);
        0
    } else {
        println!("{} > 0", x);
        1
    }
}

fn main() {
    // ContextBuilder::new("Pong", 800, 600)
    //     .quit_on_escape(true)
    //     .build();
    // let i = 0;
    // println!("{}", i);
    // let mut i = 0;
    // i = 4i32;
    // println!("{}", i);
    // i += 1;
    // println!("{}", i);
    // let tab = [0, 1, 2];
    // let s = &tab;
    // println!("{:?}", s);
    // let s = &tab[1..];
    // println!("{:?}", s);


    // let mut v: Vec<u8> = Vec::new();
    // v.push(0);
    // v.push(1);
    // v.push(2);
    // let s = &v;
    // println!("{:?}", s);
    // let s = &v[1..];
    // println!("{:?}", s);
    //

    // let my_string = "hello";

    // match my_string {
    //     "bonjour" => {
    //         println!("français");
    //     }
    //     "ciao" => {
    //         println!("italien");
    //     }
    //     "hello" => {
    //         println!("anglais");
    //     }
    //     _ => {
    //         println!("je ne connais pas cette langue...");
    //     }
    // }
    // println!("{:?}", machin(5i32));
    // println!("test");

    // match machin(1) {
    //     Some(s) => println!("{}", &s),
    //     None => {}
    // }

    // if let Some(s) = machin(1) {
    //     println!("{}", &s)
    // }

    // let mut i : i32 = 0;
    // let mut v = vec!(1, 2, 3);
    // loop {
    //     match v.pop() {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    // }

    // println!("bidule");
    //
    // let mut end = false;
    // while end == false {
    //     println!("spam")

    // }

    // loop {
    //     println!("spam")
    // }


    // let mut i : i32 = 0;
    // loop {
    //     println!("{:?}", i);
    //     i += 1;
    //     if i > 10 {
    //         break;
    //     }
    // }

    // for i in 0..10 {
    //     println!("i vaut : {}", i);
    // }
    //

    // let v = vec!(1, 4, 5, 10, 6);

    // for value in v {
    //     println!("{}", value);
    // }
    //

    // for (i, j) in (5..10).enumerate() {
    //     println!("i = {} et j = {}", i, j);
    // }

    // let v = vec!("a", "b", "c", "d");

    // for (i, value) in v.iter().enumerate() {
    //     println!("i = {} et value = \"{}\"", i, value);
    // }

    // 'outer': for x in 0..10 {
    //     'inner': for y in 0..10 {
    //         if x % 2 == 0 { continue 'outer; }
    //         if y % 2 == 0 { continue 'inner; }
    //         println!("x: {}, y {}", x, y);
    //     }
    // }

    // 'outer: for x in 0..10 {
    //     'inner: for y in 0..10 {
    //         if x % 2 == 0 { continue 'outer; } // on continue la boucle sur x
    //         if y % 2 == 0 { continue 'inner; } // on continue la boucle sur y
    //         println!("x: {}, y: {}", x, y);
    //     }
    // }


    // 'global: for _ in 0..10 {
    //     'outer: for x in 0..10 {
    //         'inner: for y in 0..10 {
    //             if x > 3 { break 'global; } // on arrête la boucle qui s'appelle global
    //             if x % 2 == 0 { continue 'outer; } // on continue la boucle sur x
    //             if y % 2 == 0 { continue 'inner; } // on continue la boucle sur y
    //             println!("x: {}, y: {}", x, y);
    //         }
    //     }
    // }
    //
    // println!("1 + 2 = {}", addition(1, 2));

    // fait_quelque_chose();

    // let var = if true {
    //     1u32
    // } else {
    //     2u32
    // };
    // println!("{}", var);

    // let mut var = 0i32;
    // let var2 = (var = 1i32);

    // println!("{:?}", var2);


    // test_expression(5i32);
    //
    // let mut fichier = match File::open("readme.md") {
    //     Ok(f) => {
    //         println!("we're in");
    //         f
    //     },
    //     Err(e) => {
    //         println!("{}", e);
    //         return;
    //     }
    // };


    // let mut v = vec!(1, 2);

    // for i in 1..3 {
    //     println!("{:?}", v.pop());
    // }

    struct Vaisseau {
        // pleins de trucs
        salon: Option<Salon>
    }

    struct Salon {

    }

    impl Salon {
        pub fn new() -> Salon {
            Salon {

            }
        }
    }

    impl Vaisseau {
        pub fn new() -> Vaisseau {
            Vaisseau {
                // on initialise le reste
                salon: None // on n'a pas de salon
            }
        }
    }

    let mut vaisseau = Vaisseau::new();

    match vaisseau.salon {
        Some(s) => {
            println!("ce vaisseau a un salon");
        },
        None => {
            println!("ce vaisseau n'a pas de salon");
        }
    }

    vaisseau.salon = Some(Salon::new());

    match vaisseau.salon {
        Some(s) => {
            println!("ce vaisseau a un salon");
        },
        None => {
            println!("ce vaisseau n'a pas de salon");
        }
    }
}
