use std::collections::HashMap;
use std::str::FromStr;
use ast;
fn main() {
    println!("{:#?}",frontend::program("struct{var a : 1,};").unwrap());
}
