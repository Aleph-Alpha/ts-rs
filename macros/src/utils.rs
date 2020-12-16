use std::fmt::Display;
use std::io::Write;

use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

macro_rules! syn_err {
    ($l:literal $(, $a:expr)*) => {
        syn_err!(proc_macro2::Span::call_site(); $l $(, $a)*);
    };
    ($s:expr; $l:literal $(, $a:expr)*) => {
        return Err(syn::Error::new($s, format!($l $(, $a)*)));
    };
}

macro_rules! impl_parse {
    ($i:ident ($input:ident, $out:ident) { $($k:pat => $e:expr),* $(,)? }) => {
        impl std::convert::TryFrom<&syn::Attribute> for $i {
            type Error = syn::Error;

            fn try_from(attr: &syn::Attribute) -> syn::Result<Self> { attr.parse_args() }
        }

        impl syn::parse::Parse for $i {
            fn parse($input: syn::parse::ParseStream) -> syn::Result<Self> {
                let mut $out = $i::default();
                loop {
                    let key: Ident = $input.call(syn::ext::IdentExt::parse_any)?;
                    match &*key.to_string() {
                        $($k => $e,)*
                        #[allow(unreachable_patterns)]
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

#[allow(unused)]
pub(crate) fn print_warning(
    title: impl Display,
    content: impl Display,
    note: impl Display,
) -> std::io::Result<()> {
    let make_color = |color: Color, bold: bool| {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(color)).set_bold(bold).set_intense(true);
        spec
    };

    let yellow_bold = make_color(Color::Yellow, true);
    let white_bold = make_color(Color::White, true);
    let white = make_color(Color::White, false);
    let blue = make_color(Color::Blue, true);

    let writer = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = writer.buffer();

    buffer.set_color(&yellow_bold)?;
    write!(&mut buffer, "warning")?;
    buffer.set_color(&white_bold)?;
    writeln!(&mut buffer, ": {}", title)?;

    buffer.set_color(&blue)?;
    writeln!(&mut buffer, "  | ")?;

    write!(&mut buffer, "  | ")?;
    buffer.set_color(&white)?;
    writeln!(&mut buffer, "{}", content)?;

    buffer.set_color(&blue)?;
    writeln!(&mut buffer, "  | ")?;

    write!(&mut buffer, "  = ")?;
    buffer.set_color(&white_bold)?;
    write!(&mut buffer, "note: ")?;
    buffer.set_color(&white)?;
    writeln!(&mut buffer, "{}", note)?;

    writer.print(&buffer)
}
