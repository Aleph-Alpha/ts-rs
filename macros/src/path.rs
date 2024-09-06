use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    Error, Ident, Lit, LitStr, Result, Token,
};


const PATH_CONVENTION_MSG: &'static str = r###"
Remider on `TS` trait export path convention.

#[ts(export)]

    Generates a test which will export the type, by default to bindings/<name>.ts when running cargo test.
    The default base directory can be overridden with the `TS_RS_EXPORT_DIR` environment variable. Adding the variable to a project's config.toml can make it easier to manage.


# <project-root>/.cargo/config.toml
[env]
TS_RS_EXPORT_DIR = { value = "<OVERRIDE_DIR>", relative = true }


#[ts(export_to = "..")]

    Specifies where the type should be exported to. Defaults to <name>.ts.
    The path given to the export_to attribute is relative to the `TS_RS_EXPORT_DIR` environment variable, or, if `TS_RS_EXPORT_DIR` is not set, to ./bindings .
    If the provided path ends in a trailing /, it is interpreted as a directory.
    Note that you need to add the 'export' attribute as well, in order to generate a test which exports the type.
"###;

// If the crate MSRV is increased to 1.70.0 than the 'Note ' from point 4 should be removed !

const PARSING_ERROR_MSG: &'static str =

"`export_to` expects as arguments the following types:

1) string literal
    #[ts(export_to = \"my/path\")]

2) static or constant variable name

    #[ts(export_to = MY_STATIC_PATH)]
    #[ts(export_to = crate::MY_STATIC_PATH)]

Note: This option is available for Rust 1.70.0 and higher!

3) function name of a `Fn(&'static str) -> Option<&'static str>`

    #[ts(export_to = get_path)]
    #[ts(export_to = crate::get_path)]

Note: This option overrides the original `TS::output_path` logic`!

4) environment variable name

    #[ts(export_to = env(\"MY_ENV_VAR_PATH\"))]

Note: This option is for environment variables defined in the '.cargo/config.toml' file only, accessible through the `env!` macro!
";


#[derive(Clone, Debug)]
pub enum CustomPath {
    Str(String),
    Static(syn::Path),
    Fn(syn::Path),
    Env(syn::LitStr),
}

type FnOutputPathBody = (TokenStream, Option<TokenStream>);

impl CustomPath {
    pub fn get_path_and_some_decl(&self, ts_name: &String) -> FnOutputPathBody {
        match self {
            Self::Str(input) => Self::str_path(input, ts_name),

            Self::Static(input) => Self::static_path(input, ts_name),

            Self::Fn(input) => Self::fn_path(input, ts_name),

            Self::Env(input) => Self::env_path(input, ts_name),
        }
    }

    fn str_path(input: &String, ts_name: &String) -> FnOutputPathBody {
        let path = if input.ends_with('/') {
            format!("{}{}.ts", input, ts_name)
        } else {
            input.to_owned()
        };

        return (quote!(#path), None);
    }

    fn static_path(input: &syn::Path, ts_name: &String) -> FnOutputPathBody {
        let path_ident = format_ident!("path");
        let stat_path_ident = format_ident!("PATH");
        let path_decl = quote! {

            static #stat_path_ident: std::sync::OnceLock<String> = std::sync::OnceLock::new();

            let #path_ident = #stat_path_ident.get_or_init( ||
                {
                    if #input.ends_with('/')  {
                        format!("{}{}.ts", #input, #ts_name)
                    } else {
                        format!("{}",#input)
                    }
                }
            );
        };

        (quote!(#path_ident), Some(path_decl))
    }

    fn fn_path(input: &syn::Path, ts_name: &String) -> FnOutputPathBody {
        let path_ident = format_ident!("path");

        // check the type of the function pointer 
        let path_decl = quote! {
        let path : fn(&'static str) -> Option<&'static str> = #input;
        };

        (quote! {#path_ident(#ts_name)?}, Some(path_decl))
    }

    fn env_path(input: &LitStr, ts_name: &String) -> FnOutputPathBody {
        let path_ident = format_ident!("path");

        let path_decl = quote! {

            let #path_ident = if std::env!(#input).ends_with('/') {
                std::concat!(std::env!(#input),#ts_name,".ts")
            } else {
                std::env!(#input)
            };
        };

        (quote!(#path_ident), Some(path_decl))
    }
}

impl Parse for CustomPath {
    fn parse(input: ParseStream) -> Result<CustomPath> {
        input.parse::<Token![=]>()?;
        let span = input.span();
        let mut some_ident = None;

        let get_path = |pont_ident: Option<Ident>, input: ParseStream| -> Result<syn::Path> {
            let mut tokens = TokenStream::new();

            if let Some(ident) = pont_ident {
                tokens.extend(quote!(#ident))
            }

            if input.peek(Token![self]) {
                let token = input.parse::<Token![self]>()?;
                tokens.extend(quote!(#token));
            }
            if input.peek(Token![super]) {
                let token = input.parse::<Token![super]>()?;
                tokens.extend(quote!(#token));
            }
            if input.peek(Token![crate]) {
                let token = input.parse::<Token![crate]>()?;
                tokens.extend(quote!(#token));
            }
            if input.peek(Ident) {
                let ident = input.parse::<Ident>()?;
                tokens.extend(quote!(#ident));
            }

            while input.peek(Token![::]) {
                let token = input.parse::<Token![::]>()?;
                tokens.extend(quote!(#token));

                if input.peek(Ident) {
                    let ident = input.parse::<Ident>()?;
                    tokens.extend(quote!(#ident));
                } else {
                    return Err(Error::new(input.span(), "expected ident"));
                }
            }

            Ok(syn::parse2::<syn::Path>(tokens)?)
        };

        let get_str = |input: ParseStream| -> Result<LitStr> {
            match Lit::parse(input)? {
                Lit::Str(string) => Ok(string),
                _ => Err(Error::new(span, PARSING_ERROR_MSG)),
            }
        };

        // reminder
        if input.peek(Token![?]) {
            let msg = format!("{PATH_CONVENTION_MSG}\n{PARSING_ERROR_MSG}");
            return Err(Error::new(span, msg));
        }

        // string literal
        if input.peek(LitStr) {
            return Ok(CustomPath::Str(get_str(input)?.value()));
        }

        // environment variable
        if input.peek(Ident) {

            // needs a check for the ident
            let ident = input.parse::<Ident>()?;
            if ident == "env" {
                if input.peek(syn::token::Paren){

                    let content;
                    syn::parenthesized!(content in input);
                    let env_str = content.parse::<LitStr>()?;
                    return Ok(CustomPath::Env(env_str));

                } else {
                    return Err(Error::new(span, PARSING_ERROR_MSG));
                }
            } else {
                some_ident = Some(ident);
            }
        }

        // path  to a const | static | function
        if let Ok(path) = get_path(some_ident, input) {
            let last = &path.segments.last().unwrap().ident;

            // const | static
            if is_screaming_snake_case(&last.to_string()) {
                return Ok(CustomPath::Static(path));
            }

            // function
            if is_snake_case(&last.to_string()) {
                return Ok(CustomPath::Fn(path));
            }
        }

        Err(Error::new(span, PARSING_ERROR_MSG))
    }
}

// These functions mimic Rust's naming conventions for
// statics, constants, and function .
// To be replaced with proper, more robust validation.

fn is_screaming_snake_case(s: &str) -> bool {
    if s.is_empty() || s.starts_with('_') || s.ends_with('_') || s.contains("__") {
        return false;
    }

    for c in s.chars() {
        if !c.is_ascii_uppercase() && c != '_' {
            return false;
        }
    }
    true
}

fn is_snake_case(s: &str) -> bool {
    if s.is_empty() || s.starts_with('_') {
        return false;
    }

    for c in s.chars() {
        if !c.is_ascii_lowercase() && c != '_' {
            return false;
        }
    }
    true
}





#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_str_literal() {
        let input = r#"= "my/path""#;
        let parsed: CustomPath = parse_str(input).unwrap();

        if let CustomPath::Str(path) = parsed {
            assert_eq!(path, "my/path");
        } else {
            panic!("Expected CustomPath::Str variant");
        }
    }

    #[test]
    fn test_static_variable_single() {
        let input = "= MY_STATIC_PATH";
        let parsed: CustomPath = parse_str(input).unwrap();

        if let CustomPath::Static(path) = parsed {
            assert_eq!(path.segments.last().unwrap().ident, "MY_STATIC_PATH");
        } else {
            panic!("Expected CustomPath::Static variant");
        }
    }

    #[test]
    fn test_static_variable_full_path() {
        let input = "= crate::MY_STATIC_PATH";
        let parsed: CustomPath = parse_str(input).unwrap();

        if let CustomPath::Static(path) = parsed {
            assert_eq!(path.segments.len(), 2);
            assert_eq!(path.segments[0].ident, "crate");
            assert_eq!(path.segments[1].ident, "MY_STATIC_PATH");
        } else {
            panic!("Expected CustomPath::Static variant");
        }
    }

    #[test]
    fn test_function_name_single() {
        let input = "= my_func_get_path";
        let parsed: CustomPath = parse_str(input).unwrap();

        if let CustomPath::Fn(path) = parsed {
            assert_eq!(path.segments.last().unwrap().ident, "my_func_get_path");
        } else {
            panic!("Expected CustomPath::Fn variant");
        }
    }

    #[test]
    fn test_function_name_full_path() {
        let input = "= crate::my_func_get_path";
        let parsed: CustomPath = parse_str(input).unwrap();

        if let CustomPath::Fn(path) = parsed {
            assert_eq!(path.segments.len(), 2);
            assert_eq!(path.segments[0].ident, "crate");
            assert_eq!(path.segments[1].ident, "my_func_get_path");
        } else {
            panic!("Expected CustomPath::Fn variant");
        }
    }

    #[test]
    fn test_env_variable() {
        let input = r#"= env("MY_ENV_VAR_PATH")"#;
        let parsed: CustomPath = parse_str(input).unwrap();

        if let CustomPath::Env(lit) = parsed {
            assert_eq!(lit.value(), "MY_ENV_VAR_PATH");
        } else {
            panic!("Expected CustomPath::Env variant");
        }
    }
}
