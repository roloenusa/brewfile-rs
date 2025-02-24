extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput, Data, Fields};

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
    let mut field_names = Vec::new();  // To store field names for `call` method
    let mut arg_fields = Vec::new();

    if let Data::Struct(ref data_struct) = ast.data {
        // Iterate over the fields of the struct
        for field in &data_struct.fields {
            let field_name = field.ident.as_ref().unwrap();

            // Store the field name for the `call` method
            field_names.push(field_name);

            for attr in &field.attrs {
                if attr.path().is_ident("arg") {
                    arg_fields.push(field);
                    break;
                }

            }
        }
    }

    let mut gen = quote! {};

    // Check only structs and named fields are allowed
    let fields = if let Data::Struct(data) = &ast.data {
        if let Fields::Named(fields) = &data.fields {
            &fields.named
        } else {
            panic!("ParseFromMap only supports structs with named fields");
        }
    } else {
        panic!("ParseFromMap can only be used on structs");
    };

    // Iterate over each value of the vector and attempt to parse the field
    let field_assignments = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let is_vec = match field_type {
            syn::Type::Path(type_path) => {
                let path = &type_path.path;
                path.segments.len() == 1 && path.segments[0].ident == "Vec"
            }
            _ => false,
        };

        if is_vec {
            quote! {
                #field_name: {
                    let value = values.get(index)
                        .ok_or_else(|| format!("Missing value for field `{}`", stringify!(#field_name)))?;
                    index += 1;

                    value.parse_vector()
                        .map_err(|errs| format!("Failed to parse list field `{}`: {:?}", stringify!(#field_name), errs))?
                }
            }
        } else {
            quote! {
                #field_name: {
                    let value = values.get(index)
                        .ok_or_else(|| format!("Missing value for field `{}`", stringify!(#field_name)))?;
                    index += 1;

                    value.parse()
                        .map_err(|e| format!("Failed to parse field `{}`: {}", stringify!(#field_name), e))?
                }
            }
        }


        // quote! {
            // #field_name: {
            //     let value = values.get(index)
            //         .ok_or_else(|| format!("Missing value for field `{}`", stringify!(#field_name)))?;
            //     index += 1;
            //
            //     // value.parse()
            //     //     .map_err(|e| format!("Failed to parse field `{}`: {}", stringify!(#field_name), e))?
            //
            //     // match value {
            //     //     brewfile_parser::ast::Ident::Str(_) => {
            //     //         value.parse()
            //     //         .map_err(|e| format!("Failed to parse field `{}`: {}", stringify!(#field_name), e))?
            //     //     },
            //     //     // brewfile_parser::ast::Ident::List(_) => {
            //     //     //     value.parse_vector()
            //     //     //     .map_err(|e| format!("Failed to parse field `{}`: {}", stringify!(#field_name), e))?
            //     //     // }
            //     //     _ => panic!("Unable to parse value"),
            //     // }
            //
            //     if let brewfile_parser::ast::Ident::List(_) = value {
            //         value.parse_vector()
            //             .map_err(|errs| format!("Failed to parse list field `{}`: {:?}", stringify!(#field_name), errs))?
            //     } else {
            //         value.parse()
            //             .map_err(|e| format!("Failed to parse field `{}`: {}", stringify!(#field_name), e))?
            //     }
            // }
            // #field_name: {
            //     let value = values.get(index)
            //         .ok_or_else(|| format!("Missing value for field `{}`", stringify!(#field_name)))?;
            //     index += 1;
            //
            //     if <#field_type as std::any::Any>::type_id() == <Vec<String> as std::any::Any>::type_id() {
            //         value.parse_vector()
            //             .map_err(|errs| format!("Failed to parse list field `{}`: {:?}", stringify!(#field_name), errs))?
            //     } else {
            //         value.parse()
            //             .map_err(|e| format!("Failed to parse field `{}`: {}", stringify!(#field_name), e))?
            //     }
            // }
        // }
    });

    // Initialize the fields in the parse call.
    let expanded = quote! {
        use brewfile_parser::ast::ParseIdent; // Ensure the trait is in scope
        use brewfile_parser::ast::ParseIdentVec; // Ensure the trait is in scope

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

    // Ensure at least one `#[arg]` field exists
    if arg_fields.is_empty() {
        return quote! {
            compile_error!("At least one field must be marked with #[arg].");
        }
        .into();
    }

    gen.into()
}

