use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(QuerunRG)]
pub fn querio_input_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_querio_input_macro(&ast)
}

fn impl_querio_input_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    quote!{
        impl #impl_generics QuerunRedisGraph for #name #ty_generics #where_clause {}
    }.into()
}