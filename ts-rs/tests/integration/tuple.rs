#![allow(unused)]

use ts_rs::{Config, TS};

#[test]
fn test_tuple() {
    type Tuple = (String, i32, (i32, i32));
    let cfg = Config::from_env();
    assert_eq!("[string, number, [number, number]]", Tuple::name(&cfg));
}

#[test]
#[should_panic]
fn test_decl() {
    type Tuple = (String, i32, (i32, i32));
    let cfg = Config::from_env();
    let _ = Tuple::decl(&cfg);
}

#[test]
fn test_newtype() {
    #[derive(TS)]
    struct NewType(String);

    let cfg = Config::from_env();
    assert_eq!("type NewType = string;", NewType::decl(&cfg));
}

#[derive(TS)]
#[ts(export, export_to = "tuple/", rename_all = "camelCase")] // rename_all should compile, but it's a noop
struct TupleNewType(String, i32, (i32, i32));

#[test]
fn test_tuple_newtype() {
    let cfg = Config::from_env();
    assert_eq!(
        "type TupleNewType = [string, number, [number, number]];",
        TupleNewType::decl(&cfg)
    )
}

#[derive(TS)]
#[ts(export, export_to = "tuple/", rename_all = "kebab-case")]
struct Dep1;

#[derive(TS)]
#[ts(export, export_to = "tuple/")]
struct Dep2;

#[derive(TS)]
#[ts(export, export_to = "tuple/")]
struct Dep3;

#[derive(TS)]
#[ts(export, export_to = "tuple/")]
struct Dep4<T> {
    a: (T, T),
    b: (T, T),
}

#[derive(TS)]
#[ts(export, export_to = "tuple/")]
struct TupleWithDependencies(Dep1, Dep2, Dep4<Dep3>);

#[test]
fn tuple_with_dependencies() {
    let cfg = Config::from_env();
    assert_eq!(
        "type TupleWithDependencies = [Dep1, Dep2, Dep4<Dep3>];",
        TupleWithDependencies::decl(&cfg)
    );
}

#[derive(TS)]
#[ts(export, export_to = "tuple/")]
struct StructWithTuples {
    a: (Dep1, Dep1),
    b: (Dep2, Dep2),
    c: (Dep4<Dep3>, Dep4<Dep3>),
}

#[test]
fn struct_with_tuples() {
    let cfg = Config::from_env();
    assert_eq!(
        "type StructWithTuples = { \
            a: [Dep1, Dep1], \
            b: [Dep2, Dep2], \
            c: [Dep4<Dep3>, Dep4<Dep3>], \
        };",
        StructWithTuples::decl(&cfg)
    );
}
