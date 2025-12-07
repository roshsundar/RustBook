use proc_macro::TokenStream;
use quote::quote;

// The hello_macro_derive() function will be called when a user of the library
// calls #[derive(HelloMacro)] on a type.

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a reperesentation of Rust code as a syntax tree that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait impl
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // Get the name of the struct.
    let generated = quote! { // quote! lets us define the code we want to return.
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("This type's name is {}", stringify!(#name)); // stringify! converts the struct name into a string literal.
            }
        }
    };
    generated.into() // Return the code as a TokenStream.
}