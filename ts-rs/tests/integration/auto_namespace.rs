use std::path::PathBuf;

use ts_rs::{Config, ExportError, TypeVisitor, TS};

// -- Manual TS impls for testing cross-crate behavior --

struct NsTypeA;
impl TS for NsTypeA {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name(_: &Config) -> String {
        "NsColliding".to_owned()
    }
    fn inline(_: &Config) -> String {
        "{ a: string }".to_owned()
    }
    fn decl(_: &Config) -> String {
        "type NsColliding = { a: string };".to_owned()
    }
    fn decl_concrete(cfg: &Config) -> String {
        Self::decl(cfg)
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsColliding.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("crate_a")
    }
}

struct NsTypeB;
impl TS for NsTypeB {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name(_: &Config) -> String {
        "NsColliding".to_owned()
    }
    fn inline(_: &Config) -> String {
        "{ b: number }".to_owned()
    }
    fn decl(_: &Config) -> String {
        "type NsColliding = { b: number };".to_owned()
    }
    fn decl_concrete(cfg: &Config) -> String {
        Self::decl(cfg)
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsColliding.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("crate_b")
    }
}

struct NsDep;
impl TS for NsDep {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name(_: &Config) -> String {
        "NsDep".to_owned()
    }
    fn inline(_: &Config) -> String {
        "{ val: string }".to_owned()
    }
    fn decl(_: &Config) -> String {
        "type NsDep = { val: string };".to_owned()
    }
    fn decl_concrete(cfg: &Config) -> String {
        Self::decl(cfg)
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsDep.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("dep_crate")
    }
}

struct NsParent;
impl TS for NsParent {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name(_: &Config) -> String {
        "NsParent".to_owned()
    }
    fn inline(cfg: &Config) -> String {
        format!("{{ dep: {} }}", NsDep::inline(cfg))
    }
    fn decl(cfg: &Config) -> String {
        format!("type NsParent = {};", Self::inline(cfg))
    }
    fn decl_concrete(cfg: &Config) -> String {
        Self::decl(cfg)
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsParent.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("parent_crate")
    }
    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        v.visit::<NsDep>();
    }
}

struct NsHyphenType;
impl TS for NsHyphenType {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
    fn name(_: &Config) -> String {
        "NsHyphenType".to_owned()
    }
    fn inline(_: &Config) -> String {
        "{ x: number }".to_owned()
    }
    fn decl(_: &Config) -> String {
        "type NsHyphenType = { x: number };".to_owned()
    }
    fn decl_concrete(cfg: &Config) -> String {
        Self::decl(cfg)
    }
    fn output_path() -> Option<PathBuf> {
        Some(PathBuf::from("NsHyphenType.ts"))
    }
    fn crate_name() -> Option<&'static str> {
        Some("my-crate")
    }
}

fn unique_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join("ts_rs_auto_ns_tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn auto_namespace_separates_crates() {
    let dir = unique_dir("separates_crates");
    let cfg = Config::new()
        .with_out_dir(&dir)
        .with_auto_namespace(true);

    NsTypeA::export_all(&cfg).unwrap();
    NsTypeB::export_all(&cfg).unwrap();

    let path_a = dir.join("crate_a/NsColliding.ts");
    let path_b = dir.join("crate_b/NsColliding.ts");

    assert!(path_a.exists(), "crate_a/NsColliding.ts should exist");
    assert!(path_b.exists(), "crate_b/NsColliding.ts should exist");

    let content_a = std::fs::read_to_string(&path_a).unwrap();
    let content_b = std::fs::read_to_string(&path_b).unwrap();

    assert!(content_a.contains("a: string"), "crate_a should have NsTypeA's content");
    assert!(content_b.contains("b: number"), "crate_b should have NsTypeB's content");
}

#[test]
fn auto_namespace_import_paths() {
    let dir = unique_dir("import_paths");
    let cfg = Config::new()
        .with_out_dir(&dir)
        .with_auto_namespace(true);

    NsParent::export_all(&cfg).unwrap();

    let parent_path = dir.join("parent_crate/NsParent.ts");
    assert!(parent_path.exists(), "parent_crate/NsParent.ts should exist");

    let content = std::fs::read_to_string(&parent_path).unwrap();
    assert!(
        content.contains("../dep_crate/NsDep"),
        "import path should reference ../dep_crate/NsDep, got:\n{content}"
    );
}

#[test]
fn auto_namespace_off_still_errors() {
    let dir = unique_dir("off_still_errors");
    let cfg = Config::new()
        .with_out_dir(&dir)
        .with_auto_namespace(false);

    NsTypeA::export_all(&cfg).unwrap();
    let result = NsTypeB::export_all(&cfg);

    assert!(
        matches!(result, Err(ExportError::Collision { .. })),
        "expected Collision error without auto_namespace, got: {result:?}"
    );
}

#[test]
fn auto_namespace_hyphen_to_underscore() {
    let dir = unique_dir("hyphen_to_underscore");
    let cfg = Config::new()
        .with_out_dir(&dir)
        .with_auto_namespace(true);

    NsHyphenType::export_all(&cfg).unwrap();

    let path = dir.join("my_crate/NsHyphenType.ts");
    assert!(
        path.exists(),
        "my_crate/NsHyphenType.ts should exist (hyphen converted to underscore)"
    );
}
