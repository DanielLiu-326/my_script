use ast;
use std::collections::HashMap;
use std::str::FromStr;
fn main() {
    println!("{:#?}", frontend::program("struct{var a : 1,};").unwrap());
}