#![feature(ptr_metadata)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(arbitrary_enum_discriminant)]
#![feature(rustc_attrs)]
#![feature(concat_idents)]
extern crate core;

mod opcode;

mod types;
mod mem_collection;
mod util;
mod vm;
mod stack;

fn main() {
    println!("Hello, world!");
}
