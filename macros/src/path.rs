use quote::{format_ident, quote};
use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Error, Lit, Ident,LitStr, Token, Result,
};


#[derive(Clone,Debug)]
pub enum CustomPath{
    Str(String),
    Static(syn::Path), 
    Fn(syn::Path),  
    Env(syn::LitStr),     
}

type FnOutputPathBody = ( TokenStream, Option<TokenStream> );

impl CustomPath {
    
    pub fn get_path_and_some_decl(&self, ts_name: &String ) -> FnOutputPathBody {

        match self {

            Self::Str(input) => { Self::str_path(input,ts_name) },

            Self::Static(input) => { Self::static_path(input,ts_name) },

            Self::Fn(input) => { Self::fn_path(input,ts_name) },

            Self::Env(input) => { Self::env_path(input,ts_name) },

        } 
    }

    fn str_path( input: &String, ts_name: &String ) -> FnOutputPathBody {

        let path = 
        if input.ends_with('/') {
            format!("{}{}.ts", input, ts_name)
        }  else {
            input.to_owned()
        };

        return (quote!(#path),None);
    }

    fn static_path( input: &syn::Path, ts_name: &String ) -> FnOutputPathBody {

        let path_ident      = format_ident!("path");
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

        ( quote!(#path_ident), Some(path_decl) )
    }

    fn fn_path( input: &syn::Path, ts_name: &String ) -> FnOutputPathBody { 
        
        ( quote!{#input (#ts_name)?}, None)
    }

    fn env_path( input: &LitStr, ts_name: &String ) -> FnOutputPathBody {

        let path_ident = format_ident!("path");

        let path_decl = quote!{

            let #path_ident = if std::env!(#input).ends_with('/') {
                std::concat!(std::env!(#input),#ts_name,".ts")
            } else {
                std::env!(#input)
            };
        };

        ( quote!(#path_ident), Some(path_decl) )
    }

}

impl Parse for CustomPath {

    fn parse(input: ParseStream) -> Result<CustomPath> {
        input.parse::<Token![=]>()?;
        let span = input.span();

        let msg = 
"expected arguments for 'export_to':

1) string literal 
    #[ts(export_to = \"my/path\")] 

2) static or constant variable name 

    #[ts(export_to = MY_STATIC_PATH)]
    #[ts(export_to = crate::MY_STATIC_PATH)]

Note: This option is available for Rust 1.7.0 and higher!

3) function name of a `Fn(&'static str) -> Option<&'static str>`

    #[ts(export_to = get_path)]
    #[ts(export_to = crate::get_path)]

Note: This option overrides the original `TS::output_path` logic`!

4) environment variable name  

    #[ts(export_to = env(\"MY_ENV_VAR_PATH\"))] 

Note: This option is for environment variables defined in the '.cargo/config.toml' file only, accessible through the `env!` macro!
";
        let get_path = |input: ParseStream| -> Result<(syn::Path,Option<LitStr>)>{ 
            let mut tokens = TokenStream::new();
            let mut env_var_str = None;

            if input.peek(Token![self])  {
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

                if input.peek(Ident){
                    let ident = input.parse::<Ident>()?;
                    tokens.extend(quote!(#ident));
                } else { return Err(Error::new(input.span(),"expected ident")) }
            }

            if input.peek(syn::token::Paren){
                let content;
                syn::parenthesized!(content in input);
                env_var_str = Some(content.parse::<LitStr>()?);
            }

            Ok((syn::parse2::<syn::Path>(tokens)?,env_var_str))
        };


        // string literal
        if input.peek(LitStr){
            if let Ok(lit) = Lit::parse(input){
                match lit {
                    Lit::Str(string) => { return Ok(CustomPath::Str(string.value())); },
                    _ => { return Err(Error::new(span, msg)); },
                }
            } 
        } 

        match get_path(input) {

            Ok((path,arg)) => {
    
                if !path.segments.is_empty(){
        
                    if let Some( env_var_str ) = arg {
            
                        if path.is_ident("env") {
                            return Ok(CustomPath::Env(env_var_str));
                        }
         
                    } else {
        
                        let last  = &path.segments.last().unwrap().ident;
        
                        // static or const 
                        if is_screaming_snake_case(&last.to_string()) {
                            return Ok(CustomPath::Static(path));
                        }
            
                        // function 
                        if is_snake_case(&last.to_string()) {
                            return Ok(CustomPath::Fn(path));
                        }    
                    }
                }
                return Err(Error::new(span, msg)); 
            },
            
            Err(e) => return Err(Error::new(e.span(), msg)),
        }
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