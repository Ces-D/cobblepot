use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, Type, TypePath, parse_macro_input,
    punctuated::Punctuated, token::Comma,
};

#[proc_macro_derive(CliDocs, attributes(cli_docs))]
pub fn cli_docs_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named) => named.named,
            _ => panic!("CliDocs only supports named‐field structs"),
        },
        _ => panic!("CliDocs only supports structs"),
    };

    // We'll collect a Vec of (ident, json_ty, optional?, description?, default?)
    let mut specs = Vec::new();
    for f in fields.iter() {
        let ident = f.ident.as_ref().unwrap().to_string();
        // detect Option<T>
        let is_optional = matches!(
            &f.ty,
            Type::Path(TypePath { path, .. })
                if path.segments.iter().any(|seg| seg.ident == "Option")
        );
        // map Rust type -> JSON type label
        let json_ty = {
            let ts = quote!(#f.ty).to_string();
            if ts.contains("String") || ts.contains("str") {
                "string"
            } else if ts.contains("DateTime") {
                "string"
            } else if ts.contains("f32")
                || ts.contains("f64")
                || ts.contains("i8")
                || ts.contains("i16")
                || ts.contains("i32")
                || ts.contains("i64")
                || ts.contains("u8")
                || ts.contains("u16")
                || ts.contains("u32")
                || ts.contains("u64")
            {
                "number"
            } else if ts == "bool" {
                "boolean"
            } else {
                "object" // fallback
            }
        };

        // parse #[cli_docs(...)] attributes
        let mut description = None;
        let mut default = None;
        for attr in &f.attrs {
            if !attr.path().is_ident("cli_docs") {
                continue;
            }

            if let Meta::List(list) = &attr.meta {
                let nested: Punctuated<Meta, Comma> =
                    list.parse_args_with(Punctuated::parse_terminated).unwrap_or_default();
                for meta in nested.iter() {
                    if let Meta::NameValue(nv) = meta {
                        if nv.path.is_ident("description") {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(s),
                                ..
                            }) = &nv.value
                            {
                                description = Some(s.value());
                            }
                        }
                        if nv.path.is_ident("default") {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(s),
                                ..
                            }) = &nv.value
                            {
                                default = Some(s.value());
                            }
                        }
                    }
                }
            }
        }

        specs.push((ident, json_ty, is_optional, description, default));
    }

    // build lines for help
    let help_lines = specs.iter().map(|(name, jty, opt, desc, def)| {
        // bullet, name padded to 12 chars, type & optionality, 4 spaces, dash, desc, default
        let opt_str = if *opt {
            "optional"
        } else {
            "required"
        };
        let type_optionality = format!("({}, {})", jty, opt_str);
        let mut line = format!("• {:<13}{:<25}", name, type_optionality);
        if let Some(d) = desc {
            // align d after enough spaces
            line.push_str(&format!("– {}", d));
        }
        if let Some(dft) = def {
            // append default at end
            line.push_str(&format!(" (default: {})", dft));
        }
        line
    });

    // build pieces of example
    let example_parts = specs.iter().filter_map(|(name, jty, opt, _desc, def)| {
        if *opt {
            def.as_ref().map(|dft| format!("\"{}\": {}", name, dft))
        } else {
            Some(format!("\"{}\": <{}>", name, jty))
        }
    });

    let help_block = help_lines.collect::<Vec<_>>().join("\n");
    let example_block = example_parts.collect::<Vec<_>>().join(", ");

    let expanded = quote! {
        impl #name {
            /// Returns the help block
            pub fn help() -> &'static str {
                concat!(
                    "A JSON object with the following fields:\n",
                    #help_block
                )
            }

            /// Returns a minimal JSON example
            pub fn example() -> String {
                let example = format!("\x1b[4mExample:\x1b[0m");
                format!("{}\n{{ {} }}",example, #example_block)
            }
        }
    };
    TokenStream::from(expanded)
}
