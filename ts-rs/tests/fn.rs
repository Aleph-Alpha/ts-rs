#![allow(dead_code, unused)]
use ts_rs::{ts_rs_fn, TS};

#[ts_rs_fn(args = "inlined", export_to = "tests-out/fn/")]
fn my_void_function() {}

#[ts_rs_fn(args = "inlined", export_to = "tests-out/fn/")]
fn my_non_void_function() -> String {
    String::from("Hello world")
}

#[ts_rs_fn(args = "inlined", export_to = "tests-out/fn/")]
fn my_void_function_with_inlined_args(str_arg: &str, int_arg: u32) {}

#[ts_rs_fn(args = "flattened", export_to = "tests-out/fn/")]
fn my_void_function_with_flattened_args(str_arg: &str, int_arg: u32) {}

#[ts_rs_fn(args = "inlined", export_to = "tests-out/fn/")]
fn my_non_void_function_with_inlined_args(str_arg: &str, int_arg: u32) -> String {
    String::from("Hello world")
}

#[ts_rs_fn(args = "flattened", export_to = "tests-out/fn/")]
fn my_non_void_function_with_flattened_args(str_arg: &str, int_arg: u32) -> String {
    String::from("Hello world")
}


#[derive(TS)]
#[ts(export, export_to = "tests-out/fn/")]
struct Foo {
    foo: u32,
}

#[ts_rs_fn(export_to = "tests-out/fn/")]
fn function_with_imported_return() -> Foo {
    Foo { foo: 0 }
}

#[ts_rs_fn(export_to = "tests-out/fn/")]
fn function_with_imported_flattened_args(foo: Foo) {}

#[ts_rs_fn(args = "inlined", export_to = "tests-out/fn/")]
fn function_with_imported_inlined_args(foo: Foo) {}

#[test]
fn void_fn() {
    assert_eq!(MyVoidFunctionFn::inline(), "() => void")
}

#[test]
fn non_void_fn() {
    assert_eq!(MyNonVoidFunctionFn::inline(), "() => string")
}

#[test]
fn void_fn_inlined_args() {
    assert_eq!(MyVoidFunctionWithInlinedArgsFn::inline(), "(args: { str_arg: string, int_arg: number, }) => void")
}

#[test]
fn void_fn_flattened_args() {
    assert_eq!(MyVoidFunctionWithFlattenedArgsFn::inline(), "(str_arg: string, int_arg: number,) => void")
}

#[test]
fn non_void_fn_inlined_args() {
    assert_eq!(MyNonVoidFunctionWithInlinedArgsFn::inline(), "(args: { str_arg: string, int_arg: number, }) => string")
}

#[test]
fn non_void_fn_flattened_args() {
    assert_eq!(MyNonVoidFunctionWithFlattenedArgsFn::inline(), "(str_arg: string, int_arg: number,) => string")
}

#[test]
fn fn_with_imported_return() {
    assert_eq!(FunctionWithImportedReturnFn::inline(), "() => Foo")
}

#[test]
fn fn_with_imported_inlined_args() {
    assert_eq!(FunctionWithImportedInlinedArgsFn::inline(), "(args: { foo: Foo, }) => void")
}

#[test]
fn fn_with_imported_flattened_args() {
    assert_eq!(FunctionWithImportedFlattenedArgsFn::inline(), "(foo: Foo,) => void")
}
