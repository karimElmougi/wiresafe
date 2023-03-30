use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

#[proc_macro_derive(Wiresafe)]
pub fn whatever_you_want(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    let struct_name = ast.ident;

    let generics = ast.generics;

    let mut bound_generics = generics.clone();
    for param in &mut bound_generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Wiresafe));
        }
    }

    let checks = checks(&ast.data);
    let output = quote! {
        impl #bound_generics ::wiresafe::__private::Wiresafe for #struct_name #generics {
            fn check() {
                #checks
            }
        }
    };

    output.into()
}

fn checks(data: &syn::Data) -> TokenStream {
    let types = match &data {
        syn::Data::Struct(struct_data) => struct_field_types(struct_data),
        syn::Data::Enum(enum_data) => enum_field_types(enum_data),
        syn::Data::Union(union_data) => union_field_types(union_data),
    };

    quote! {
        #(<#types as ::wiresafe::__private::Wiresafe>::check();)*
    }
}

fn struct_field_types(data: &syn::DataStruct) -> Vec<&syn::Type> {
    field_types(&data.fields)
}

fn enum_field_types(data: &syn::DataEnum) -> Vec<&syn::Type> {
    data.variants
        .iter()
        .flat_map(|var| field_types(&var.fields))
        .collect()
}

fn union_field_types(data: &syn::DataUnion) -> Vec<&syn::Type> {
    data.fields.named.iter().map(|f| &f.ty).collect()
}

fn field_types(fields: &syn::Fields) -> Vec<&syn::Type> {
    match fields {
        syn::Fields::Unit => vec![],
        syn::Fields::Named(fields) => fields.named.iter().map(|f| &f.ty).collect(),
        syn::Fields::Unnamed(fields) => fields.unnamed.iter().map(|f| &f.ty).collect(),
    }
}
