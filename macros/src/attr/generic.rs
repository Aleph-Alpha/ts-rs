use syn::{Attribute, Ident, Result};

use super::{parse_assign_str};
use crate::utils::{parse_attrs};

#[derive(Default)]
pub struct GenericAttr {
    pub concrete: Option<String>
}

impl GenericAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        Ok(result)
    }

    fn merge(
        &mut self,
        GenericAttr {
            concrete,
        }: GenericAttr,
    ) {
        self.concrete = self.concrete.take().or(concrete)
    }
}

impl_parse! {
    GenericAttr(input, out) {
        "concrete" => out.concrete = Some(parse_assign_str(input)?),
    }
}