#![allow(dead_code)]
use outer_macro_pattern::outer;

#[outer]
mod outer {

    use outer_macro_pattern::inner;

    #[inner]
    struct InnerStruct {
        field: u32,
    }

    struct NormalStruct {
        field: u32,
    }

    #[inner]
    fn inner_function() {}

    fn normal_function() {}
}

fn main() {
    println!("The outer macro should have found the marked inner macros")
}
