use syn::{Ident, Result};

use super::{parse_assign_str, Inflection, parse_assign_inflection};

#[derive(Default)]
pub struct FnAttr {
    pub args: Args,
    pub rename: Option<String>,
    pub rename_all: Option<Inflection>,
}

#[derive(Default)]
pub enum Args {
    #[default]
    Positional,
    Named,
}

impl TryFrom<String> for Args {
    type Error = syn::Error;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "named" => Ok(Self::Named),
            "positional" => Ok(Self::Positional),
            x => syn_err!(r#"Expected "named" or "positional", found "{}""#, x)
        }
    }
}

impl_parse! {
    FnAttr(input, output) {
        "args" => output.args = parse_assign_str(input)?.try_into()?,
        "rename" => output.rename = Some(parse_assign_str(input)?),
        "rename_all" => output.rename_all = Some(parse_assign_inflection(input)?),
    }
}
