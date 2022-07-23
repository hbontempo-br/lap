extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn time_it(attr: TokenStream, item: TokenStream) -> TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(item as syn::ItemFn);
    let stmts = &block.stmts;
    println!("{:#?}", sig);
    let fn_name =  &sig.ident.to_string();

    let callback = if attr.is_empty() {
        let ident = &Ident::new("default_callback", proc_macro2::Span::call_site());
        quote! (#ident)
    } else {
        let attr_vec = syn::parse_macro_input!(attr as syn::AttributeArgs);
        let first_attr = &attr_vec.first().unwrap();
        let ident = match first_attr {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => {
                    let syn::Path { segments, .. } = path;
                    &segments.first().unwrap().ident
                }
                _ => panic!("Unexpected attr"),
            },
            _ => panic!("Unexpected attr"),
        };
        quote! (#ident)
    };
    quote!(
        use std::time::Instant;

        #(#attrs)* #vis #sig {
            let start = Instant::now();
            let resp = #(#stmts)*;
            let duration = start.elapsed();
            #callback(#fn_name, duration);
            resp
        }
    )
    .into()
}
