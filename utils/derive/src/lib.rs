extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn;
use syn::Data;

#[proc_macro_derive(OpCodeTable)]
pub fn opcode_table_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_opcode_table_macro(&ast)
}

fn enum_case_assoc_stream(len: usize) -> TokenStream2 {
    match len {
        0 => TokenStream2::new(),
        _ => {
            let mut assoc = Vec::new();
            for i in 0..len {
                assoc.push(quote! { _ });
                if i + 1 != len {
                    assoc.push(quote! { , });
                }
            }
            let f = assoc.iter().fold(quote! {}, |acc, new| quote! {#acc #new});
            quote! { (#f) }
        }
    }
}

fn enum_case_token_stream(ident: &syn::Ident, variant: &syn::Variant) -> TokenStream2 {
    let name = &variant.ident;
    let len = variant.fields.len();
    let offset = len + 1;
    let assoc = enum_case_assoc_stream(len);
    let case = quote! {
        #ident::#name#assoc => #offset,
    };  
    case
}

fn impl_opcode_table_macro(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let mut cases = Vec::new();
    match ast.data {
        Data::Enum(ref data) => {
            for variant in &data.variants {
                let case = enum_case_token_stream(ident, variant);
                cases.push(case);
            }
        },
        _ => panic!("Вerive must be applied to enum only"),
    }

    let content = cases.iter().fold(quote! {}, |acc, new| quote! {#acc #new});
    let content = proc_macro2::TokenStream::from(content);
    let gen = quote! {
        impl OpCodeTable for #ident {
            fn offset(&self) -> usize {
                match self {
                    #content
                    _ => panic!("Op {:?} has missing offset", self),
                }
            }
        }
    };
    gen.into()
}