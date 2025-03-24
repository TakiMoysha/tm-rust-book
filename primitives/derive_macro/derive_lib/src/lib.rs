use proc_macro2::TokenStream;
use quote::quote;
use syn::Fields;

fn is_struct(input: &syn::DeriveInput) -> bool {
    match input.data {
        syn::Data::Struct(_) => true,
        _ => false,
    }
}

///
#[proc_macro_derive(FromRow)]
pub fn derive_from_row(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("Deriving FromRow for {}", input);
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let tokens = conf_tree(&ast).unwrap_or_else(|e| e.to_compile_error());
    tokens.into()
}

fn conf_tree(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    /// check if it is struct, and get fields
    let fields = if let syn::Data::Struct(ref data) = ast.data {
        &data.fields
    } else {
        return Err(syn::Error::new(ast.ident.span(), "Expected struct"));
    };

    // we have named, unnamed and unit structures

    match fields {
        Fields::Named(fields) => {
            let fields_vals = fields.named.iter().enumerate().map(|(i, field)| {
                let name = &field.ident;
                // transform to ast
                // `#name` - take name from &field.ident
                quote!(#name: row.try_get(#i)?)
            });

            return Ok(TokenStream::from(quote!(
                impl #impl_generics derive_macro::FromRow for #name #ty_generics #where_clause {
                fn from_row(row: derive_macro::Row) -> Result<Self, Box<dyn std::error::Error>> {
                    Ok(Self {
                        #(#fields_vals),*
                    })
                }
            })));
        }
        Fields::Unnamed(_) => return Err(syn::Error::new(ast.ident.span(), "work in progress")),
        Fields::Unit => return Err(syn::Error::new(ast.ident.span(), "work in progress")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExampleNamed {
        field: String,
    }

    struct ExampleUnnamed(String);

    struct ExampleUnit;

    #[test]
    fn should_macro_derive_to_named_structure() {}

    #[test]
    fn should_macro_derive_to_unnamed_structure() {}

    #[test]
    fn should_macro_derive_to_unit_structure() {}
}
