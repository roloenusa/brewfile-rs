extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput, Type, Data, Fields};

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(CmdParser, attributes(arg))]
pub fn parser(input: TokenStream) -> TokenStream {
    println!("--- parser");
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);
    impl_parse(&ast)
}

fn impl_parse(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    // Collect the field names and types
    let mut field_inits = Vec::new(); // To store field initializations
    let mut field_names = Vec::new();  // To store field names for `call` method
    let mut arg_fields = Vec::new();

    if let Data::Struct(ref data_struct) = ast.data {

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

            for attr in &field.attrs {
                if attr.path().is_ident("arg") {
                    arg_fields.push(field);
                    break;
                }

            }
        }
    }

    let mut gen = quote! {};

    gen.extend(quote! {
        impl #struct_name {
            pub fn init(fields: String) -> Self {
                Self {
                    #(#field_inits),*
                }
            }
        }
    });

    gen.extend(quote! {
        impl #struct_name {
            pub fn parse(input: &str) -> Self {
                let (_, value) = brewfile_parser::ast::get_command(input).unwrap();
                println!("Value: {:#?}", value.cmd);

                Self {
                    #(#field_inits),*
                }
            }
        }
    });

    let fields = if let Data::Struct(data) = &ast.data {
        if let Fields::Named(fields) = &data.fields {
            &fields.named
        } else {
            panic!("ParseFromMap only supports structs with named fields");
        }
    } else {
        panic!("ParseFromMap can only be used on structs");
    };

    // Generate parsing logic for each field
    let field_assignments = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        quote! {
            #field_name: value.get(stringify!(#field_name))
                .ok_or_else(|| format!("Missing field: {}", stringify!(#field_name)))?
                .parse::<#field_type>()
                .map_err(|_| format!("Failed to parse field: {} as {}", stringify!(#field_name), stringify!(#field_type)))?
        }
    });

    // Generate the `parse` function implementation
    let expanded = quote! {
        impl #struct_name {
            pub fn parse_t(value: std::collections::HashMap<String, String>) -> Result<Self, String> {
                Ok(Self {
                    #(#field_assignments),*
                })
            }
        }
    };
    gen.extend(expanded);

    let field_assignments = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();

        quote! {
            #field_name: {
                let value = values.get(index)
                    .ok_or_else(|| format!("Missing value for field `{}`", stringify!(#field_name)))?;
                index += 1;
                value.parse()
                    .map_err(|e| format!("Failed to parse field `{}`: {}", stringify!(#field_name), e))?
            }
        }
    });

    let expanded = quote! {
        use brewfile_parser::ast::ParseIdent; // Ensure the trait is in scope

        impl #struct_name {
            pub fn parse_me(values: Vec<brewfile_parser::ast::Ident>) -> Result<Self, String> {
                let mut index = 0;
                Ok(Self {
                    #(#field_assignments),*
                })
            }
        }
    };
    gen.extend(expanded);


    gen.extend(quote! {
        impl #struct_name {
            pub fn call(&self) {
                println!("---- test ");
            }
        }
    });

    // if let syn::Data::Struct(name) = &ast.data {
    //     if let Fields::Named(fields) = &name.fields {
    //         for field in &fields.named {
    //             for attr in &field.attrs {
    //                 if attr.path().is_ident("arg") {
    //                     arg_fields.push(field);
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }

    // Ensure at least one `#[arg]` field exists
    if arg_fields.is_empty() {
        return quote! {
            compile_error!("At least one field must be marked with #[arg].");
        }
        .into();
    }

    gen.into()
}

