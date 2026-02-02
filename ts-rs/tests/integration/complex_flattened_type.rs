use ts_rs::{Config, TS};

/// Defines the type of input and its intial fields
#[derive(TS)]
#[ts(tag = "input_type")]
pub enum InputType {
    Text,
    Expression,
    Number {
        min: Option<isize>,
        max: Option<isize>,
    },
    Dropdown {
        options: Vec<(String, String)>,
    },
}

#[derive(TS)]
#[ts(tag = "type")]
pub enum InputFieldElement {
    Label {
        text: String,
    },
    Input {
        #[ts(flatten)]
        input: InputType,
        name: Option<String>,
        placeholder: Option<String>,
        default: Option<String>,
    },
}

#[derive(TS)]
#[ts(export, export_to = "complex_flattened_type/")]
pub struct InputField {
    #[ts(flatten)]
    r#type: InputFieldElement,
}

#[test]
fn complex_flattened_type() {
    let cfg = Config::from_env();
    assert_eq!(
        InputFieldElement::decl(&cfg),
        r#"type InputFieldElement = { "type": "Label", text: string, } | { "type": "Input", name: string | null, placeholder: string | null, default: string | null, } & ({ "input_type": "Text" } | { "input_type": "Expression" } | { "input_type": "Number", min: number | null, max: number | null, } | { "input_type": "Dropdown", options: Array<[string, string]>, });"#
    );
    assert_eq!(
        InputField::decl(&cfg),
        r#"type InputField = { "type": "Label", text: string, } | { "type": "Input", name: string | null, placeholder: string | null, default: string | null, } & ({ "input_type": "Text" } | { "input_type": "Expression" } | { "input_type": "Number", min: number | null, max: number | null, } | { "input_type": "Dropdown", options: Array<[string, string]>, });"#
    )
}
