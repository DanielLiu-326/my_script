use ast;
use std::collections::HashMap;
use std::str::FromStr;
use frontend::*;
fn main() {
    println!("{:#?}", frontend::src_file("struct{var a : 1,};").unwrap());
}
