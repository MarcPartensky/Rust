#!/usr/bin/env run-cargo-script

// extern crate rand;
// use tetra::graphics::{self, Color};
// use tetra::{Context, ContextBuilder, State};

fn machin(i: i32) -> Option<String> {
    if i < 10 {
        Some("variable inférieure à 10".to_owned())
    } else {
        None
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
}
