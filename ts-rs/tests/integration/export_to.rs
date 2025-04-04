use std::path::Path;
use ts_rs::TS;

#[derive(TS)]
#[ts(export, export_to = "export_to/with_str_to_file.ts")]
struct WithStrToFile;

#[derive(TS)]
#[ts(export, export_to = "export_to/")]
struct WithStrToDir;

// --

#[derive(TS)]
#[ts(export, export_to = &"export_to/with_str_ref_to_file.ts")]
struct WithStrRefToFile;

#[derive(TS)]
#[ts(export, export_to = &"export_to/")]
struct WithStrRefToDir;

// --

#[derive(TS)]
#[ts(export, export_to = format!("export_to/with_string_to_file.ts"))]
struct WithStringToFile;

#[derive(TS)]
#[ts(export, export_to = format!("export_to/"))]
struct WithStringToDir;

// --

#[derive(TS)]
#[ts(export, export_to = &format!("export_to/with_string_ref_to_file.ts"))]
struct WithStringRefToFile;

#[derive(TS)]
#[ts(export, export_to = &format!("export_to/"))]
struct WithStringRefToDir;

// --

#[test]
#[cfg(test)]
fn check_export_complete() {
    export_bindings_withstrtofile();
    export_bindings_withstrtodir();
    export_bindings_withstrreftofile();
    export_bindings_withstrreftodir();
    export_bindings_withstringtofile();
    export_bindings_withstringtodir();
    export_bindings_withstringreftofile();
    export_bindings_withstringreftodir();

    let files = [
        "with_str_to_file.ts",
        "WithStrToDir.ts",
        "with_str_ref_to_file.ts",
        "WithStrRefToDir.ts",
        "with_string_to_file.ts",
        "WithStringToDir.ts",
        "with_string_ref_to_file.ts",
        "WithStringRefToDir.ts",
    ];

    let dir = std::env::var("TS_RS_EXPORT_DIR").unwrap_or_else(|_| "./bindings".to_owned());
    let dir = Path::new(&dir).join("export_to");
    
    files.iter()
        .map(|file| dir.join(file))
        .for_each(|file| assert!(file.is_file(), "{file:?}"));
}
