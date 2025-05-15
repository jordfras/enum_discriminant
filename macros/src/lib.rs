use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, parse_quote, AttrStyle};

macro_rules! compile_error_unless_ok {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => return error.to_compile_error().into(),
        }
    };
}

/// Adds a `discriminant()` function to get the numeric value of enum variants. Also adds
/// a `from_discriminant()` function to create unit type enum variants from discriminants.
///
/// The `discriminant()` function relies on casting, as described in the
/// [Rust language documentation](https://doc.rust-lang.org/std/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant).
/// The `from_discriminant()` function, on the other hand, is essentially a `match`
/// statement with all the unit type variants.
#[proc_macro_attribute]
pub fn discriminant(arguments: TokenStream, item: TokenStream) -> TokenStream {
    let enum_item = parse_macro_input!(item as syn::ItemEnum);
    let enum_name = &enum_item.ident;

    let arguments: TokenStream2 = arguments.into();

    let repr_type = compile_error_unless_ok!(get_repr_type(arguments.clone()));

    let from_discriminant_code = generate_from_discriminant_function(&repr_type, &enum_item);
    let discriminant_code = generate_discriminant_function(&repr_type);

    quote! {
        #[repr(#arguments)]
        #enum_item

        impl #enum_name {
            #from_discriminant_code

            #discriminant_code
        }
    }
    .into()
}

/// Derive macro generating an impl for the `IntoDiscriminant` trait for enums. The trait
/// adds a `discriminant()` function to get the numeric value of enum variants.
///
/// The `discriminant()` function relies on casting, as described in the
/// [Rust language documentation](https://doc.rust-lang.org/std/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant).
#[proc_macro_derive(IntoDiscriminant)]
pub fn derive_into_discriminant(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    let enum_name = &input.ident;

    let repr_args = compile_error_unless_ok!(get_repr_args("IntoDiscriminant", &input));
    let repr_type = compile_error_unless_ok!(get_repr_type(repr_args));

    let discriminant_code = generate_discriminant_function(&repr_type);

    quote! {
        impl IntoDiscriminant for #enum_name {
            type DiscriminantType = #repr_type;

            #discriminant_code
        }
    }
    .into()
}

/// Derive macro generating an impl for the `FromDiscriminant` trait for enums. The trait
/// adds a `from_discriminant()` function to create unit type enum variants from
/// discriminants.
///
/// The `from_discriminant()` function is essentially a `match` statement with all the
/// unit type variants.
#[proc_macro_derive(FromDiscriminant)]
pub fn derive_from_discriminant(item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();
    let input = parse_macro_input!(item as syn::DeriveInput);
    let enum_item = parse_macro_input!(cloned_item as syn::ItemEnum);
    let enum_name = &enum_item.ident;

    let repr_args = compile_error_unless_ok!(get_repr_args("FromDiscriminant", &input));
    let repr_type = compile_error_unless_ok!(get_repr_type(repr_args));

    let from_discriminant_code = generate_from_discriminant_function(&repr_type, &enum_item);

    quote! {
        impl FromDiscriminant for #enum_name {
            type DiscriminantType = #repr_type;

            #from_discriminant_code
        }
    }
    .into()
}

/// Returns the first valid representation type found in the arguments or a compile error
/// if none is found.
fn get_repr_type(arguments: TokenStream2) -> Result<syn::Path, syn::Error> {
    let allowed_types = [
        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
    ];

    arguments
        .clone()
        .into_iter()
        // Filter arguments that are allowed types and convert them to syn::Path
        .filter_map(|token_tree| {
            if let proc_macro2::TokenTree::Ident(ident) = token_tree {
                let ident_str = ident.to_string();
                if allowed_types.contains(&ident_str.as_str()) {
                    return Some(syn::parse_str::<syn::Path>(&ident_str).unwrap());
                }
            }
            None
        })
        .next()
        // On error, return a compile error as a TokenStream
        .ok_or_else(|| {
            syn::Error::new_spanned(
                arguments,
                "Valid enum representation type expected as argument to the discriminant \
                 macro, e.g., #[discriminant(u8)]",
            )
        })
}

// Finds the first `repr` or `discriminant` attribute in the input and returns its
// arguments. This is used to determine the representation type of the enum.
fn get_repr_args(macro_name: &str, input: &syn::DeriveInput) -> Result<TokenStream2, syn::Error> {
    let x = input
        .attrs
        .iter()
        .filter(|attr| matches!(attr.style, AttrStyle::Outer))
        .filter(|attr| {
            let path = attr.path();
            path.is_ident("repr") || path.is_ident("discriminant")
        })
        .filter_map(|attr| attr.meta.require_list().ok())
        .next()
        .ok_or_else(|| {
            syn::Error::new_spanned(
                input,
                format!(
                    "When deriving {} on an enum, you also need to specify \
                     representation type with #[repr()] or #[discriminant()]",
                    macro_name
                ),
            )
        })?;
    Ok(x.tokens.clone())
}

// Returns a tuple of the names and discriminants of the unit variants of an enum. The
// discriminants are returned as expressions, since explicit input discriminants can be
// constant expressions.
fn enum_unit_variants(enum_item: &syn::ItemEnum) -> (Vec<proc_macro2::Ident>, Vec<syn::Expr>) {
    let mut previous_expr: Option<syn::Expr> = None;
    enum_item
        .variants
        .iter()
        .filter(|variant| matches!(variant.fields, syn::Fields::Unit))
        .map(|variant| {
            let expr = if let Some(discriminant) = &variant.discriminant {
                discriminant.1.clone()
            } else if let Some(ref old_expr) = previous_expr {
                parse_quote!( 1 + #old_expr )
            } else {
                parse_quote!(0)
            };
            previous_expr = Some(expr.clone());
            (variant.ident.clone(), expr)
        })
        .unzip()
}

fn generate_from_discriminant_function(
    repr_type: &syn::Path,
    enum_item: &syn::ItemEnum,
) -> TokenStream2 {
    let (variant_names, discriminants) = enum_unit_variants(enum_item);
    let enum_name = &enum_item.ident;

    quote! {
        /// Creates an enum variant from a discriminant numeric value if there is a unit
        /// type variant with that value.
        fn from_discriminant(discriminant: #repr_type) -> Option<Self> {
            match discriminant {
                // Match arm guard needed in case discriminant is not a literal but
                // constant other expression
                #( discriminant if discriminant == #discriminants =>
                    Some(#enum_name::#variant_names), )*
                _ => None,
            }
        }
    }
}

fn generate_discriminant_function(repr_type: &syn::Path) -> TokenStream2 {
    quote! {
         /// Returns the discriminant numeric value of an enum variant.
         fn discriminant(&self) -> #repr_type {
            // See https://doc.rust-lang.org/core/mem/fn.discriminant.html
            unsafe {
                *<*const _>::from(self).cast::<#repr_type>()
            }
        }
    }
}
