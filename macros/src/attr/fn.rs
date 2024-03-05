use syn::{Ident, Result};

use super::{parse_assign_inflection, parse_assign_str, Inflection};

#[derive(Default)]
pub struct FnAttr {
    pub args: Args,
    pub export_to: Option<String>,
    pub rename: Option<String>,
    pub rename_all: Option<Inflection>,
}

#[derive(Default)]
pub enum Args {
    #[default]
    Flattened,
    Inlined,
}

impl TryFrom<String> for Args {
    type Error = syn::Error;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "inlined" => Ok(Self::Inlined),
            "flattened" => Ok(Self::Flattened),
            x => syn_err!(r#"Expected "inlined" or "flattened", found "{x}""#),
        }
    }
}

impl_parse! {
    FnAttr(input, output) {
        "args" => output.args = parse_assign_str(input)?.try_into()?,
        "export_to" => output.export_to = Some(parse_assign_str(input)?),
        "rename" => output.rename = Some(parse_assign_str(input)?),
        "rename_all" => output.rename_all = Some(parse_assign_inflection(input)?),
    }
}
