extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput, Type};

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Parser, attributes(arg))]
pub fn parser(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);
    impl_parse(&ast)
}

fn impl_parse(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Collect the field names and types
    let mut field_inits = Vec::new(); // To store field initializations
    let mut field_names = Vec::new();  // To store field names for `call` method

    if let syn::Data::Struct(ref data_struct) = ast.data {

        // Iterate over the fields of the struct
        for field in &data_struct.fields {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;

            // Store the field name for the `call` method
            field_names.push(field_name);

            // Generate code to initialize the field
            let init_value = match field_type {
                // If the field is `i32`, we initialize it to `10`
                Type::Path(type_path) if type_path.path.is_ident("i32") => quote! { 10 },
                // If the field is `String`, we initialize it to `String::from("hello")`
                Type::Path(type_path) if type_path.path.is_ident("String") => quote! {

                    String::from("hello")

                },

                // Handle other types if needed (you can extend this part)
                _ => quote! { Default::default() }, // Default initialization for other types
            };

            // Add the field initialization to the list
            field_inits.push(quote! { #field_name: #init_value });
        }
    }

    let mut gen = quote! {};

    gen.extend(quote! {
        impl #name {
            pub fn parse() -> Self {
                Self {
                    #(#field_inits),*
                }
            }
        }
    });

    gen.extend(quote! {
        impl #name {
            pub fn call(&self) {
                println!("---- test ");
            }
        }
    });

    gen.into()
}

