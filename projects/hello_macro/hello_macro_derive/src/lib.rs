extern crate proc_macro;

// comes with rust, it is the compilers api that allows
// us to manipulate rust code from our own code.
// proc_macro crate as a whole.
use crate::proc_macro::TokenStream;
// The quote crate turns syn data struct back into Rust code.
use quote::quote;
use syn;


#[proc_macro_derive(HelloMacro)]
//this function will be called whenever someone uses the lib and
// does #[derive(HelloMacro)]
// the HelloMacro makes it possible as it matches the
// trait name
// this function first converts the input from a tokenstrem to a data
// struct that can be manipulatd, the syn method turns the tokenstream into a
// derive input struct that represents the parsed rust code (the type
// has all bunch of data regarding the type calling it as well as parts of the
// code)
// the function returns a token stream that will be added to the crate
// the user implementing this crate will use. 
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // consturcts a syntax tree of rust code that can be manipulated.
    // syn must panic as the only thing acceptable is a token stream in
    // case of failure and not a result type instance. rather than using
    // unwrap it would be best to panic! or use expect.
    let ast = syn::parse(input).unwrap();
    // implements the trait
    impl_hello_macro(&ast)
}


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // ident stands for identifier
    let name = &ast.ident;
    let gen = quote! {
        // #name is the name of the type # is the special syntax here
        impl HelloMacro for #name {
            fn hello_macro() {
                // stiringify takes the literal expression and does not 
                // evaluate it, it just turns it into a string
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    // into turns the code into a tokenstream again
    gen.into()

}
