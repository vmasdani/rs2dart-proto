extern crate proc_macro;
use std::{any::type_name, fs};

use proc_macro::{Ident, TokenStream};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};
// use syn::{DeriveInput, Field, ItemEnum, Token, braced, parse::{ParseStream, Parse}, parse_macro_input, punctuated::Punctuated, token};

#[proc_macro_attribute]
pub fn gen_dart(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Attr: {:?}", attr);
    println!("Itme: {:?}", item);

    let item_clone = item.clone();
    let input = parse_macro_input!(item_clone as DeriveInput);

    let struct_ident = &input.ident.to_string();
    println!("\n\niDENTITY: {:?}\n\n", struct_ident);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Expected struct with named fields."),
    };

    #[derive(Debug)]
    struct DartTypeAttr {
        field_name: String,
        type_name: String,
    }

    let mut dart_types_arr: Vec<DartTypeAttr> = vec![]; 

    for field in fields {
        let mut field_name_str = 
        match &field.ident {
            Some(ident) => {
                ident.to_string()
            },
            None => {
                String::from("")
            }
        };
        let mut type_name_str = "";

        
        
        match &field.ty {
            syn::Type::Array(a) => {
                println!("arr ");
            },
            syn::Type::BareFn(a) => {
                println!("bare fn");
                // println!("{:?}", a);
            }
            syn::Type::Group(a) => {
                println!("group");
                // println!("{:?}", a);
            }
            syn::Type::ImplTrait(a) => {
                println!("impl trait");
                // println!("{:?}", a.);
            }
            syn::Type::Infer(a) => {
                println!("infer");
                // println!("{:?}", a);
            }
            syn::Type::Macro(a) => {
                println!("macro");
                // println!("{:?}", a);
            }
            syn::Type::Never(a) => {
                println!("never");
                // println!("{:?}", a);
            }
            syn::Type::Paren(a) => {
                println!("paren");
                // println!("{:?}", a);
            }
            syn::Type::Path(a) => {
                println!("path");

                match a.path.get_ident() {
                    Some(typ) => {
                        let type_string =  typ.to_string();

                        match type_string.as_str() {
                            "i32" => type_name_str = "int",
                            "u32" => type_name_str = "int",
                            "i64" => type_name_str = "int",
                            "u64" => type_name_str = "int",
                            "f32" => type_name_str = "double",
                            "f64" => type_name_str = "double",
                            "String" => type_name_str = "String",
                            _ => {
                                     
                            }
                        }

                        println!("Type {:?}", typ.to_string());    
                    },
                    _ => {
                        println!("Type unknown");
                    }
                       
                }
            }
            syn::Type::Ptr(a) => {
                println!("ptr");
                // println!("{:?}", a);
            }
            syn::Type::Reference(a) => {
                println!("ref");
                // println!("{:?}", a);
            }
            syn::Type::Slice(a) => {
                println!("slice");
                // println!("{:?}", a);
            }
            syn::Type::TraitObject(a) => {
                println!("trait obj");
                // println!("{:?}", a);
            }
            syn::Type::Tuple(a) => {
                println!("tuple");
                // println!("{:?}", a);
            }
            syn::Type::Verbatim(a) => {
                println!("vebatin");
                println!("{:?}", a);
            }
            syn::Type::__Nonexhaustive => {
                // println!("{:?}", &field.ty);
                println!("Non exhaustive");
            }
        }

        if field_name_str != "" && type_name_str != "" {
            dart_types_arr.push(DartTypeAttr {
                field_name: field_name_str,
                type_name: type_name_str.to_string()
            });
        }
    }

    println!("Fields final:");
    println!("{:?}", dart_types_arr);

    let types_vec: Vec<String> = dart_types_arr.into_iter().map(|dart_type_attr| {
        format!("  {} {};", dart_type_attr.type_name, dart_type_attr.field_name)
    }).collect();

    let types_str = types_vec.join("\n");

    fs::write(
        "model.dart",
        format!(r#"class {} {{
{}
}}"#,
            struct_ident,
            types_str
        ),
    );

    // println!("Test");
    // println!("Tokenstream: {:?}", _item);

    // let item_str = format!("{:?}", _item);

    // format!(r#"
    //     fn answer() -> u32 {{
    //         let item_str = "test";

    //         println!("Helloworld");
    //         println!("");
    //         42
    //     }}
    // "#).parse().unwrap()

    item
}
