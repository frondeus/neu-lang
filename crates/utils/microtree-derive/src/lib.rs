extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use darling::*;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(token_kind), supports(enum_any))]
struct TokenKindOpts {
    ident: syn::Ident,
    //variants: Vec<Variant>
    data: ast::Data<Variant, ()>,

    #[darling(default)]
    extras: Option<syn::Path>,

    #[darling(default)]
    mergeable: Option<syn::Path>,

    #[darling(default)]
    debug: Option<()>
}

#[derive(Debug, FromVariant)]
#[darling(attributes(token_kind))]
struct Variant {
    ident: syn::Ident,

    #[darling(default)]
    token: Option<String>,

    #[darling(default)]
    regex: Option<String>,

    #[darling(default)]
    display: Option<String>,

    #[darling(default)]
    error: Option<()>,

    #[darling(default)]
    callback: Option<syn::Path>
}

#[proc_macro_derive(TokenKind, attributes(token_kind))]
pub fn token_kind_derive(args: TokenStream) -> TokenStream {
    let opts = syn::parse_macro_input!(args as syn::DeriveInput);
    let opts = match TokenKindOpts::from_derive_input(&opts) {
        Ok(opts) => opts,
        Err(e) => return e.write_errors().into()
    };
    //dbg!(&opts);
    let name = opts.ident;
    let extras = match opts.extras {
        Some(extras) => {
            quote!(#extras)
        }
        None => quote!(())
    };

    let mut regexes = vec![];
    let mut displays = vec![];
    let mut variants = vec![];
    let mut error: Option<syn::Ident> = None;

    for variant in opts.data
        .take_enum()
        .expect("Should never be struct")
    {
        let name = variant.ident;
        let callback = match variant.callback {
            Some(ref callback) => quote!(
                return CallbackResult::result(#callback(bumped, source, extras), |()| Self::#name);
            ),
            _ => quote!(return Self::#name;)
        };

        if let Some(regex) = variant.regex {
            let regex_name = quote::format_ident!("{}_REG", name.to_string().to_uppercase());
            let regex = format!("^{}", regex);

            regexes.push(quote!(
                static ref #regex_name: Regex = Regex::new(#regex).unwrap();
            ));

            variants.push(quote!(
                if let Some(m) = #regex_name.find(source.as_ref()) {
                    let count = m.as_str().chars().count();
                    let bumped = source.bump(count);
                    #callback
                }
            ));

            if variant.display.is_none() {
                let display_name = name.to_string().to_lowercase();
                displays.push(quote!(Self::#name => write!(f, #display_name)));
                continue;
            }
        }
        else if let Some(token) = variant.token {
            let count = token.chars().count();
            variants.push(quote!(
                if source.as_ref().starts_with(#token) {
                    let bumped = source.bump(#count);
                    #callback
                }
            ));
            if variant.display.is_none() {
                let display_name = format!("`{}`", token);
                displays.push(quote!(Self::#name => write!(f, #display_name)));
                continue;
            }
        }
        else if variant.error.is_some() {
            if error.is_none() {
                error = Some(name.clone());
            }
            else {
                panic!("There must be only one error variant");
            }
            if variant.callback.is_some() {
                panic!("There cannot be a callback for error variant");
            }
            if variant.display.is_none() {
                displays.push(quote!(Self::#name => write!(f, "error")));
                continue;
            }
        }
        else {
            panic!("Expected token, regex or error attribute");
        }

        let custom_display = variant.display.unwrap();
        displays.push(quote!(
            Self::#name => write!(f, #custom_display)
        ));
    }

    let error_variant = error.expect("Expected one error variant");

    let regexes = if !regexes.is_empty() {
        quote!(
            use regex::Regex;
            use lazy_static::lazy_static;
            lazy_static! {
                #(#regexes)*
            }
        )
    } else { quote!() };

    let mergeable = if let Some(mergeable) = opts.mergeable {
        quote! (
            fn mergeable(self, other: Self) -> bool {
                #mergeable(self, other)
            }
        )
    } else { quote!() };

    let debug = if opts.debug.is_some() {
        quote!(dbg!(&source.as_ref());)
    } else {quote!()};

    let quoted = quote!{
        impl TokenKind for #name {
            type Extras = #extras;
            const ERROR: Self = Self::#error_variant;
            fn lex(source: &mut Source<'_>, extras: &mut Self::Extras) -> Self {
                //dbg!(&source.as_ref());
                #debug
                #regexes
                #(#variants)*
                source.bump(1);
                Self::ERROR
            }
            #mergeable
        }

        impl std::fmt::Display for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#displays),*
                }
            }
        }
    };
    if opts.debug.is_some() {
        eprintln!("{}", &quoted.to_string());
    }
    quoted.into()
}
