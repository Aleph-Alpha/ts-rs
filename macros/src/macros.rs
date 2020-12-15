macro_rules! syn_err {
    ($l:literal $(, $a:expr)*) => {
        syn_err!(proc_macro2::Span::call_site(); $l $(, $a)*);
    };
    ($s:expr; $l:literal $(, $a:expr)*) => {
        return Err(syn::Error::new($s, format!($l $(, $a)*)));
    };
}

macro_rules! impl_parse {
    ($i:ident ($input:ident, $out:ident) { $($k:literal => $e:expr),* $(,)? }) => {
        impl std::convert::TryFrom<&syn::Attribute> for $i {
            type Error = syn::Error;

            fn try_from(attr: &syn::Attribute) -> syn::Result<Self> {
                attr.parse_args()
            }
        }

        impl syn::parse::Parse for $i {
            fn parse($input: syn::parse::ParseStream) -> syn::Result<Self> {
                let mut $out = $i::default();
                loop {
                    let key: Ident = $input.call(syn::ext::IdentExt::parse_any)?;
                    match &*key.to_string() {
                        $($k => $e,)*
                        other => syn_err!("unexpected attribute key `{}`", other)
                    }

                    match $input.is_empty() {
                        true => break,
                        false => {
                            $input.parse::<syn::Token![,]>()?;
                        }
                    }
                }

                Ok($out)
            }
        }
    };
}
