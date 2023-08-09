use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, Fields};


#[proc_macro_derive(AxionJson)]
pub fn axion_json_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_axion_json_derive(&ast)
}

fn impl_axion_json_derive(ast: &syn::DeriveInput) -> TokenStream {
    let (name, (impl_generics, ty_generics, where_clause)) = (&ast.ident, &ast.generics.split_for_impl());
    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("AxionJson macro only supports structs with named fields."),
        },
        _ => panic!("AxionJson macro can only be applied to structs")
    };

    let fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field name not found");
        let access_function = quote! {
            format!("{:?}:{}", stringify!(#field_name), self.#field_name.to_json())
        };
        access_function
    });

    (quote! {
        impl #impl_generics AxionJson for #name #ty_generics #where_clause {
            fn to_json(&self) -> String {
                let vec_fields = vec![
                    #(#fields),*
                ];
                format!("{{{}}}", vec_fields.join(","))
            }
        }
    }).into()
}