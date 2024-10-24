use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::spanned::Spanned;

/// Mark a module with #[outer] to find all structs and functions marked with #[inner]
#[proc_macro_attribute]
pub fn outer(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_mod = parse_macro_input!(item as syn::ItemMod);
    let mut structs = Vec::new();
    let mut functions = Vec::new();
    println!("{:?}", &item_mod.content);

    if let Some((_, items)) = &item_mod.content {
        for item in items {
            match item {
                syn::Item::Struct(item_struct) => {
                    if has_inner_attribute(&item_struct.attrs) {
                        structs.push(item_struct);
                    }
                }
                syn::Item::Fn(item_fn) => {
                    if has_inner_attribute(&item_fn.attrs) {
                        functions.push(item_fn);
                    }
                }
                _ => {}
            }
        }
        // just print the structs and functions for now
        for item_struct in &structs {
            println!("struct: {:?}", item_struct.ident);
        }
        for item_fn in &functions {
            println!("function: {:?}", item_fn.sig.ident);
        }
    } else {
        // marking a module with #[outer] only makes sense if it has content
        // generate proper syn error with span
        return syn::Error::new(
            item_mod.span(),
            "module marked with #[outer] must have content",
        )
        .to_compile_error()
        .into();
    }
    let item_mod = quote::quote! {
        #item_mod
    };
    TokenStream::from(item_mod)
}

fn has_inner_attribute(attributes: &[syn::Attribute]) -> bool {
    for attribute in attributes {
        if attribute.path().is_ident("inner") {
            return true;
        }
    }
    false
}

/// Mark a struct or function with #[inner] to be found by the #[outer] macro
#[proc_macro_attribute]
pub fn inner(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
