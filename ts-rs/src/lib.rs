//! <h1 align="center" style="padding-top: 0; margin-top: 0;">
//! <img width="150px" src="https://raw.githubusercontent.com/Aleph-Alpha/ts-rs/main/logo.png" alt="logo">
//! <br/>
//! ts-rs
//! </h1>
//! <p align="center">
//! Generate typescript type declarations from rust types
//! </p>
//!
//! <div align="center">
//! <!-- Github Actions -->
//! <img src="https://img.shields.io/github/actions/workflow/status/Aleph-Alpha/ts-rs/test.yml?branch=main" alt="actions status" />
//! <a href="https://crates.io/crates/ts-rs">
//! <img src="https://img.shields.io/crates/v/ts-rs.svg?style=flat-square"
//! alt="Crates.io version" />
//! </a>
//! <a href="https://docs.rs/ts-rs">
//! <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//! alt="docs.rs docs" />
//! </a>
//! <a href="https://crates.io/crates/ts-rs">
//! <img src="https://img.shields.io/crates/d/ts-rs.svg?style=flat-square"
//! alt="Download" />
//! </a>
//! </div>
//!
//! ## Why?
//! When building a web application in rust, data structures have to be shared between backend and frontend.
//! Using this library, you can easily generate TypeScript bindings to your rust structs & enums so that you can keep your
//! types in one place.
//!
//! ts-rs might also come in handy when working with webassembly.
//!
//! ## How?
//! ts-rs exposes a single trait, `TS`. Using a derive macro, you can implement this interface for your types.
//! Then, you can use this trait to obtain the TypeScript bindings.
//! We recommend doing this in your tests.
//! [See the example](https://github.com/Aleph-Alpha/ts-rs/blob/main/example/src/lib.rs) and [the docs](https://docs.rs/ts-rs/latest/ts_rs/).
//!
//! ## Get started
//! ```toml
//! [dependencies]
//! ts-rs = "11.1"
//! ```
//!
//! ```rust
//! use ts_rs::TS;
//!
//! #[derive(TS)]
//! #[ts(export)]
//! struct User {
//!     user_id: i32,
//!     first_name: String,
//!     last_name: String,
//! }
//! ```
//!
//! When running `cargo test` or `cargo test export_bindings`, the TypeScript bindings will be exported to the file `bindings/User.ts`
//! and will contain the following code:
//!
//! ```ts
//! export type User = { user_id: number, first_name: string, last_name: string, };
//! ```
//!
//! ## Features
//! - generate type declarations from rust structs
//! - generate union declarations from rust enums
//! - inline types
//! - flatten structs/types
//! - generate necessary imports when exporting to multiple files
//! - serde compatibility
//! - generic types
//! - support for ESM imports
//!
//! If there's a type you're dealing with which doesn't implement `TS`, you can use either
//! `#[ts(as = "..")]` or `#[ts(type = "..")]`, enable the appropriate cargo feature, or open a PR.
//!
//! ## Cargo Features
//! | **Feature**        | **Description**                                                                                                                                     |
//! |:-------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
//! | serde-compat       | **Enabled by default** <br/>See the *"serde compatibility"* section below for more information.                                                     |
//! | format             | Enables formatting of the generated TypeScript bindings. <br/>Currently, this unfortunately adds quite a few dependencies.                          |
//! | no-serde-warnings  | By default, warnings are printed during build if unsupported serde attributes are encountered. <br/>Enabling this feature silences these warnings.  |
//! | serde-json-impl    | Implement `TS` for types from *serde_json*                                                                                                          |
//! | chrono-impl        | Implement `TS` for types from *chrono*                                                                                                              |
//! | bigdecimal-impl    | Implement `TS` for types from *bigdecimal*                                                                                                          |
//! | url-impl           | Implement `TS` for types from *url*                                                                                                                 |
//! | uuid-impl          | Implement `TS` for types from *uuid*                                                                                                                |
//! | bson-uuid-impl     | Implement `TS` for *bson::oid::ObjectId* and *bson::uuid*                                                                                           |
//! | bytes-impl         | Implement `TS` for types from *bytes*                                                                                                               |
//! | indexmap-impl      | Implement `TS` for types from *indexmap*                                                                                                            |
//! | ordered-float-impl | Implement `TS` for types from *ordered_float*                                                                                                       |
//! | heapless-impl      | Implement `TS` for types from *heapless*                                                                                                            |
//! | semver-impl        | Implement `TS` for types from *semver*                                                                                                              |
//! | smol_str-impl      | Implement `TS` for types from *smol_str*                                                                                                            |
//! | tokio-impl         | Implement `TS` for types from *tokio*                                                                                                               |
//! | jiff-impl          | Implement `TS` for types from *jiff*                                                                                                                |
//! | arrayvec-impl      | Implement `TS` for types from *arrayvec*                                                                                                            |
//!
//! ## Serde Compatibility
//! With the `serde-compat` feature (enabled by default), serde attributes are parsed for enums and structs.\
//! Supported serde attributes: `rename`, `rename-all`, `rename-all-fields`, `tag`, `content`, `untagged`, `skip`, `skip_serializing`, `skip_serializing_if`, `flatten`, `default`
//!
//! **Note**: `skip_serializing` and `skip_serializing_if` only have an effect when used together with
//! `#[serde(default)]`. This ensures that the generated type is correct for both serialization and deserialization.
//!
//! **Note**: `skip_deserializing` is ignored. If you wish to exclude a field
//! from the generated type, but cannot use `#[serde(skip)]`, use `#[ts(skip)]` instead.
//!
//! When ts-rs encounters an unsupported serde attribute, a warning is emitted, unless the feature `no-serde-warnings` is enabled.\
//! We are currently waiting for [#54140](https://github.com/rust-lang/rust/issues/54140), which will improve the ergonomics arund these diagnostics.
//!
//! ## Configuration
//! When using `#[ts(export)]` on a type, `ts-rs` generates a test which writes the bindings for it to disk.\
//! The following environment variables may be set to configure *how* and *where*:   
//! | Variable                 | Description                                                         | Default      |
//! |--------------------------|---------------------------------------------------------------------|--------------|
//! | `TS_RS_EXPORT_DIR`       | Base directory into which bindings will be exported                 | `./bindings` |
//! | `TS_RS_IMPORT_EXTENSION` | File extension used in `import` statements                          | *none*       |
//! | `TS_RS_LARGE_INT`        | Binding used for large integer types (`i64`, `u64`, `i128`, `u128`) | `bigint`     |
//!
//! To export bindings programmatically without the use of tests, `TS::export_all`, `TS::export`, and `TS::export_to_string` can be used instead.
//!
//! ## Contributing
//! Contributions are always welcome!
//! Feel free to open an issue, discuss using GitHub discussions or open a PR.
//! [See CONTRIBUTING.md](https://github.com/Aleph-Alpha/ts-rs/blob/main/CONTRIBUTING.md)
//!
//! ## MSRV
//! The Minimum Supported Rust Version for this crate is 1.88.0

use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
};

pub use ts_rs_macros::TS;

pub use crate::export::ExportError;

#[cfg(feature = "chrono-impl")]
mod chrono;
mod export;
#[cfg(feature = "jiff-impl")]
mod jiff;
#[cfg(feature = "serde-json-impl")]
mod serde_json;
#[cfg(feature = "tokio-impl")]
mod tokio;

/// A type which can be represented in TypeScript.
/// Most of the time, you'd want to derive this trait instead of implementing it manually.
/// ts-rs comes with implementations for all primitives, most collections, tuples,
/// arrays and containers.
///
/// ### exporting
/// Because Rusts procedural macros are evaluated before other compilation steps, TypeScript
/// bindings __cannot__ be exported during compile time.
///
/// Bindings can be exported within a test, which ts-rs generates for you by adding `#[ts(export)]`
/// to a type you wish to export to a file.
/// When `cargo test` is run, all types annotated with `#[ts(export)]` and all of their
/// dependencies will be written to `TS_RS_EXPORT_DIR`, or `./bindings` by default.
///
/// For each individual type, path and filename within the output directory can be changed using
/// `#[ts(export_to = "...")]`. By default, the filename will be derived from the name of the type.
///
/// If, for some reason, you need to do this during runtime or cannot use `#[ts(export)]`, bindings
/// can be exported manually using [`TS::export_all`], [`TS::export`], or [`TS::export_to_string`].
///
/// ### serde compatibility
/// With the `serde-compat` feature enabled (default), ts-rs parses serde attributes and adjusts the generated typescript bindings accordingly.  
/// Not all serde attributes are supported yet - if you use an unsupported attribute, you'll see a
/// warning. These warnings can be disabled by enabling the `no-serde-warnings` cargo feature.
///
/// ### container attributes
/// attributes applicable for both structs and enums
///
/// - **`#[ts(crate = "..")]`**
///   Generates code which references the module passed to it instead of defaulting to `::ts_rs`
///   This is useful for cases where you have to re-export the crate.
///
/// - **`#[ts(export)]`**
///   Generates a test which will export the type, by default to `bindings/<name>.ts` when running
///   `cargo test`. The default base directory can be overridden with the `TS_RS_EXPORT_DIR` environment variable.
///   Adding the variable to a project's [config.toml](https://doc.rust-lang.org/cargo/reference/config.html#env) can
///   make it easier to manage.
///   ```toml
///   # <project-root>/.cargo/config.toml
///   [env]
///   TS_RS_EXPORT_DIR = { value = "<OVERRIDE_DIR>", relative = true }
///   ```
///   <br/>
///
/// - **`#[ts(export_to = "..")]`**
///   Specifies where the type should be exported to. Defaults to `<name>.ts`.
///   The path given to the `export_to` attribute is relative to the `TS_RS_EXPORT_DIR` environment variable,
///   or, if `TS_RS_EXPORT_DIR` is not set, to `./bindings`
///   If the provided path ends in a trailing `/`, it is interpreted as a directory.
///   This attribute also accepts arbitrary expressions.
///   Note that you need to add the `export` attribute as well, in order to generate a test which exports the type.
///   <br/><br/>
///
/// - **`#[ts(as = "..")]`**
///   Overrides the type used in Typescript, using the provided Rust type instead.
///   This is useful when you have a custom serializer and deserializer and don't want to implement `TS` manually
///   <br/><br/>
///
/// - **`#[ts(type = "..")]`**
///   Overrides the type used in TypeScript.
///   This is useful when you have a custom serializer and deserializer and don't want to implement `TS` manually
///   <br/><br/>
///
/// - **`#[ts(rename = "..")]`**
///   Sets the typescript name of the generated type.
///   Also accepts expressions, e.g `#[ts(rename = module_path!().rsplit_once("::").unwrap().1)]`.
///   <br/><br/>
///
/// - **`#[ts(rename_all = "..")]`**
///   Rename all fields/variants of the type.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
///
/// - **`#[ts(concrete(..)]`**
///   Disables one ore more generic type parameters by specifying a concrete type for them.
///   The resulting TypeScript definition will not be generic over these parameters and will use the
///   provided type instead.
///   This is especially useful for generic types containing associated types. Since TypeScript does
///   not have an equivalent construct to associated types, we cannot generate a generic definition
///   for them. Using `#[ts(concrete(..)]`, we can however generate a non-generic definition.
///   Example:
///   ```
///   # use ts_rs::TS;
///   ##[derive(TS)]
///   ##[ts(concrete(I = std::vec::IntoIter<String>))]
///   struct SearchResult<I: Iterator>(Vec<I::Item>);
///   // will always generate `type SearchResult = Array<String>`.
///   ```
///   <br/><br/>
///
/// - **`#[ts(bound)]`**
///   Override the bounds generated on the `TS` implementation for this type. This is useful in
///   combination with `#[ts(concrete)]`, when the type's generic parameters aren't directly used
///   in a field or variant.
///
///   Example:
///   ```
///   # use ts_rs::TS;
///
///   trait Container {
///       type Value: TS;
///   }
///
///   struct MyContainer;
///
///   ##[derive(TS)]
///   struct MyValue;
///
///   impl Container for MyContainer {
///       type Value = MyValue;
///   }
///
///   ##[derive(TS)]
///   ##[ts(export, concrete(C = MyContainer))]
///   struct Inner<C: Container> {
///       value: C::Value,
///   }
///
///   ##[derive(TS)]
///   // Without `#[ts(bound)]`, `#[derive(TS)]` would generate an unnecessary
///   // `C: TS` bound
///   ##[ts(export, concrete(C = MyContainer), bound = "C::Value: TS")]
///   struct Outer<C: Container> {
///       inner: Inner<C>,
///   }
///   ```
///   <br/><br/>
///
/// ### struct attributes
/// - **`#[ts(tag = "..")]`**
///   Include the structs name (or value of `#[ts(rename = "..")]`) as a field with the given key.
///   <br/><br/>
///
/// - **`#[ts(optional_fields)]`**
///   Makes all `Option<T>` fields in a struct optional.
///   If `#[ts(optional_fields)]` is present, `t?: T` is generated for every `Option<T>` field of the struct.
///   If `#[ts(optional_fields = nullable)]` is present, `t?: T | null` is generated for every `Option<T>` field of the struct.
///   <br/><br/>
///
/// ### struct field attributes
///
/// - **`#[ts(type = "..")]`**
///   Overrides the type used in TypeScript.
///   This is useful when there's a type for which you cannot derive `TS`.
///   <br/><br/>
///
/// - **`#[ts(as = "..")]`**
///   Overrides the type of the annotated field, using the provided Rust type instead.
///   This is useful when there's a type for which you cannot derive `TS`.
///   `_` may be used to refer to the type of the field, e.g `#[ts(as = "Option<_>")]`.
///   <br/><br/>
///
/// - **`#[ts(rename = "..")]`**
///   Renames this field. To rename all fields of a struct, see the container attribute `#[ts(rename_all = "..")]`.
///   <br/><br/>
///
/// - **`#[ts(inline)]`**
///   Inlines the type of this field, replacing its name with its definition.
///   <br/><br/>
///
/// - **`#[ts(skip)]`**
///   Skips this field, omitting it from the generated *TypeScript* type.
///   <br/><br/>
///
/// - **`#[ts(optional)]`**
///   May be applied on a struct field of type `Option<T>`. By default, such a field would turn into `t: T | null`.
///   If `#[ts(optional)]` is present, `t?: T` is generated instead.
///   If `#[ts(optional = nullable)]` is present, `t?: T | null` is generated.
///   `#[ts(optional = false)]` can override the behaviour for this field if `#[ts(optional_fields)]`
///   is present on the struct itself.
///   <br/><br/>
///
/// - **`#[ts(flatten)]`**
///   Flatten this field, inlining all the keys of the field's type into its parent.
///   <br/><br/>
///
/// ### enum attributes
///
/// - **`#[ts(tag = "..")]`**
///   Changes the representation of the enum to store its tag in a separate field.
///   See [the serde docs](https://serde.rs/enum-representations.html) for more information.
///   <br/><br/>
///
/// - **`#[ts(content = "..")]`**
///   Changes the representation of the enum to store its content in a separate field.
///   See [the serde docs](https://serde.rs/enum-representations.html) for more information.
///   <br/><br/>
///
/// - **`#[ts(untagged)]`**
///   Changes the representation of the enum to not include its tag.
///   See [the serde docs](https://serde.rs/enum-representations.html) for more information.
///   <br/><br/>
///
/// - **`#[ts(rename_all = "..")]`**
///   Rename all variants of this enum.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
///
/// - **`#[ts(rename_all_fields = "..")]`**
///   Renames the fields of all the struct variants of this enum. This is equivalent to using
///   `#[ts(rename_all = "..")]` on all of the enum's variants.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
///
/// - **`#[ts(repr(enum))]`**
///   Exports the enum as a TypeScript enum instead of type union
///   Discriminants (`= {integer}`) are included in the exported enum's variants
///   If `#[ts(repr(enum = name))]` is used, all variants without a discriminant will be exported
///   as `VariantName = "VariantName"`
///   <br/><br/>
///
/// ### enum variant attributes
///
/// - **`#[ts(rename = "..")]`**
///   Renames this variant. To rename all variants of an enum, see the container attribute `#[ts(rename_all = "..")]`.
///   This attribute also accepts expressions, e.g `#[ts(rename = module_path!().rsplit_once("::").unwrap().1)]`.
///   <br/><br/>
///
/// - **`#[ts(skip)]`**
///   Skip this variant, omitting it from the generated *TypeScript* type.
///   <br/><br/>
///
/// - **`#[ts(untagged)]`**
///   Changes this variant to be treated as if the enum was untagged, regardless of the enum's tag
///   and content attributes
///   <br/><br/>
///
/// - **`#[ts(rename_all = "..")]`**
///   Renames all the fields of a struct variant.
///   Valid values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, "kebab-case" and "SCREAMING-KEBAB-CASE"
///   <br/><br/>
pub trait TS {
    /// If this type does not have generic parameters, then `WithoutGenerics` should just be `Self`.
    /// If the type does have generic parameters, then all generic parameters must be replaced with
    /// a dummy type, e.g `ts_rs::Dummy` or `()`.
    /// The only requirement for these dummy types is that `EXPORT_TO` must be `None`.
    ///
    /// # Example:
    /// ```
    /// use ts_rs::TS;
    /// struct GenericType<A, B>(A, B);
    /// impl<A, B> TS for GenericType<A, B> {
    ///     type WithoutGenerics = GenericType<ts_rs::Dummy, ts_rs::Dummy>;
    ///     type OptionInnerType = Self;
    ///     // ...
    ///     # fn name(_: &ts_rs::Config) -> String { todo!() }
    ///     # fn inline(_: &ts_rs::Config) -> String { todo!() }
    /// }
    /// ```
    type WithoutGenerics: TS + ?Sized;

    /// If the implementing type is `std::option::Option<T>`, then this associated type is set to `T`.
    /// All other implementations of `TS` should set this type to `Self` instead.
    type OptionInnerType: ?Sized;

    #[doc(hidden)]
    const IS_OPTION: bool = false;

    #[doc(hidden)]
    const IS_ENUM: bool = false;

    /// JSDoc comment to describe this type in TypeScript - when `TS` is derived, docs are
    /// automatically read from your doc comments or `#[doc = ".."]` attributes
    fn docs() -> Option<String> {
        None
    }

    /// Identifier of this type, excluding generic parameters.
    fn ident(cfg: &Config) -> String {
        // by default, fall back to `TS::name()`.
        let name = <Self as crate::TS>::name(cfg);

        match name.find('<') {
            Some(i) => name[..i].to_owned(),
            None => name,
        }
    }

    /// Declaration of this type, e.g. `type User = { user_id: number, ... }`.
    /// This function will panic if the type has no declaration.
    ///
    /// If this type is generic, then all provided generic parameters will be swapped for
    /// placeholders, resulting in a generic typescript definition.
    /// Both `SomeType::<i32>::decl()` and `SomeType::<String>::decl()` will therefore result in
    /// the same TypeScript declaration `type SomeType<A> = ...`.
    fn decl(cfg: &Config) -> String {
        panic!("{} cannot be declared", Self::name(cfg))
    }

    /// Declaration of this type using the supplied generic arguments.
    /// The resulting TypeScript definition will not be generic. For that, see `TS::decl()`.
    /// If this type is not generic, then this function is equivalent to `TS::decl()`.
    fn decl_concrete(cfg: &Config) -> String {
        panic!("{} cannot be declared", Self::name(cfg))
    }

    /// Name of this type in TypeScript, including generic parameters
    fn name(cfg: &Config) -> String;

    /// Formats this types definition in TypeScript, e.g `{ user_id: number }`.
    /// This function will panic if the type cannot be inlined.
    fn inline(cfg: &Config) -> String;

    /// Flatten a type declaration.
    /// This function will panic if the type cannot be flattened.
    fn inline_flattened(cfg: &Config) -> String {
        panic!("{} cannot be flattened", Self::name(cfg))
    }

    /// Iterates over all dependency of this type.
    fn visit_dependencies(_: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
    }

    /// Iterates over all type parameters of this type.
    fn visit_generics(_: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
    }

    /// Resolves all dependencies of this type recursively.
    fn dependencies(cfg: &Config) -> Vec<Dependency>
    where
        Self: 'static,
    {
        struct Visit<'a>(&'a Config, &'a mut Vec<Dependency>);
        impl TypeVisitor for Visit<'_> {
            fn visit<T: TS + 'static + ?Sized>(&mut self) {
                let Visit(cfg, deps) = self;
                if let Some(dep) = Dependency::from_ty::<T>(cfg) {
                    deps.push(dep);
                }
            }
        }

        let mut deps: Vec<Dependency> = vec![];
        Self::visit_dependencies(&mut Visit(cfg, &mut deps));
        deps
    }

    /// Manually export this type to the filesystem.
    /// To export this type together with all of its dependencies, use [`TS::export_all`].
    ///
    /// # Automatic Exporting
    /// Types annotated with `#[ts(export)]`, together with all of their dependencies, will be
    /// exported automatically whenever `cargo test` is run.
    /// In that case, there is no need to manually call this function.
    ///
    /// To alter the filename or path of the type within the target directory,
    /// use `#[ts(export_to = "...")]`.
    fn export(cfg: &Config) -> Result<(), ExportError>
    where
        Self: 'static,
    {
        let relative_path = Self::output_path()
            .ok_or_else(std::any::type_name::<Self>)
            .map_err(ExportError::CannotBeExported)?;
        let path = cfg.export_dir.join(relative_path);

        export::export_to::<Self, _>(cfg, path)
    }

    /// Manually export this type to the filesystem, together with all of its dependencies.
    /// To export only this type, without its dependencies, use [`TS::export`].
    ///
    /// # Automatic Exporting
    /// Types annotated with `#[ts(export)]`, together with all of their dependencies, will be
    /// exported automatically whenever `cargo test` is run.
    /// In that case, there is no need to manually call this function.
    ///
    /// To alter the filenames or paths of the types within the target directory,
    /// use `#[ts(export_to = "...")]`.
    fn export_all(cfg: &Config) -> Result<(), ExportError>
    where
        Self: 'static,
    {
        export::export_all_into::<Self>(cfg)
    }

    /// Manually generate bindings for this type, returning a [`String`].
    /// This function does not format the output, even if the `format` feature is enabled.
    ///
    /// # Automatic Exporting
    /// Types annotated with `#[ts(export)]`, together with all of their dependencies, will be
    /// exported automatically whenever `cargo test` is run.
    /// In that case, there is no need to manually call this function.
    fn export_to_string(cfg: &Config) -> Result<String, ExportError>
    where
        Self: 'static,
    {
        export::export_to_string::<Self>(cfg)
    }

    /// Returns the output path to where `T` should be exported, relative to the output directory.
    /// The returned path does _not_ include any base directory.
    ///
    /// When deriving `TS`, the output path can be altered using `#[ts(export_to = "...")]`.
    /// See the documentation of [`TS`] for more details.
    ///
    /// If `T` cannot be exported (e.g because it's a primitive type), this function will return
    /// `None`.
    fn output_path() -> Option<PathBuf> {
        None
    }
}

/// A visitor used to iterate over all dependencies or generics of a type.
/// When an instance of [`TypeVisitor`] is passed to [`TS::visit_dependencies`] or
/// [`TS::visit_generics`], the [`TypeVisitor::visit`] method will be invoked for every dependency
/// or generic parameter respectively.
pub trait TypeVisitor: Sized {
    fn visit<T: TS + 'static + ?Sized>(&mut self);
}

/// A typescript type which is depended upon by other types.
/// This information is required for generating the correct import statements.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dependency {
    /// Type ID of the rust type
    pub type_id: TypeId,
    /// Name of the type in TypeScript
    pub ts_name: String,
    /// Path to where the type would be exported. By default, a filename is derived from the types
    /// name, which can be customized with `#[ts(export_to = "..")]`.
    /// This path does _not_ include a base directory.
    pub output_path: PathBuf,
}

impl Dependency {
    /// Constructs a [`Dependency`] from the given type `T`.
    /// If `T` is not exportable (meaning `T::EXPORT_TO` is `None`), this function will return
    /// `None`
    pub fn from_ty<T: TS + 'static + ?Sized>(cfg: &Config) -> Option<Self> {
        let output_path = <T as crate::TS>::output_path()?;
        Some(Dependency {
            type_id: TypeId::of::<T>(),
            ts_name: <T as crate::TS>::ident(cfg),
            output_path,
        })
    }
}

/// Configuration that affects the generation of TypeScript bindings and how they are exported.  
pub struct Config {
    // TS_RS_LARGE_INT
    large_int_type: String,
    // TS_RS_USE_V11_HASHMAP
    use_v11_hashmap: bool,
    // TS_RS_EXPORT_DIR
    export_dir: PathBuf,
    // TS_RS_IMPORT_EXTENSION
    import_extension: Option<String>,
    array_tuple_limit: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            large_int_type: "bigint".to_owned(),
            use_v11_hashmap: false,
            export_dir: "./bindings".into(),
            import_extension: None,
            array_tuple_limit: 64,
        }
    }
}

impl Config {
    /// Creates a new `Config` with default values.  
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Config` with values read from environment variables.
    ///
    /// | Variable                 | Description                                                         | Default      |
    /// |--------------------------|---------------------------------------------------------------------|--------------|
    /// | `TS_RS_EXPORT_DIR`       | Base directory into which bindings will be exported                 | `./bindings` |
    /// | `TS_RS_IMPORT_EXTENSION` | File extension used in `import` statements                          | *none*       |
    /// | `TS_RS_LARGE_INT`        | Binding used for large integer types (`i64`, `u64`, `i128`, `u128`) | `bigint`     |
    pub fn from_env() -> Self {
        let mut cfg = Self::default();

        if let Ok(ty) = std::env::var("TS_RS_LARGE_INT") {
            cfg = cfg.with_large_int(ty);
        }

        if let Ok(dir) = std::env::var("TS_RS_EXPORT_DIR") {
            cfg = cfg.with_out_dir(dir);
        }

        if let Ok(ext) = std::env::var("TS_RS_IMPORT_EXTENSION") {
            if !ext.trim().is_empty() {
                cfg = cfg.with_import_extension(Some(ext));
            }
        }

        #[allow(deprecated)]
        if let Ok("1" | "true" | "on" | "yes") = std::env::var("TS_RS_USE_V11_HASHMAP").as_deref() {
            cfg = cfg.with_v11_hashmap();
        }

        cfg
    }

    /// Sets the TypeScript type used to represent large integers.
    /// Here, "large" refers to integers that can not be losslessly stored using the 64-bit "binary64" IEEE 754 float format used by JavaScript.  
    /// Those include `u64`, `i64, `u128`, and `i128`.
    ///
    /// Default: `"bigint"`
    pub fn with_large_int(mut self, ty: impl Into<String>) -> Self {
        self.large_int_type = ty.into();
        self
    }

    /// Returns the TypeScript type used to represent large integers.
    pub fn large_int(&self) -> &str {
        &self.large_int_type
    }

    /// When enabled, `HashMap<K, V>` and similar types will always be translated to `{ [key in K]?: V }`.  
    /// Normally, with this option disabled, `{ [key in K]: V }` is generated instead, unless the key `K` is an enum.  
    /// This option is only intended to aid migration and will be removed in a future release.
    ///
    /// Default: disabled
    #[deprecated = "this option is merely meant to aid migration to v12 and will be removed in a future release"]
    pub fn with_v11_hashmap(mut self) -> Self {
        self.use_v11_hashmap = true;
        self
    }

    /// Sets the output directory into which bindings will be exported.  
    /// This affects `TS::export`, `TS::export_all`, and the automatic export of types annotated with `#[ts(export)]` when `cargo test` is run.
    ///
    /// Default: `./bindings`
    pub fn with_out_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.export_dir = dir.into();
        self
    }

    /// Returns the output directory into which bindings will be exported.
    pub fn out_dir(&self) -> &Path {
        &self.export_dir
    }

    /// Sets the file extension used for `import` statements in generated TypeScript files.  
    ///
    /// Default: `None`
    pub fn with_import_extension(mut self, ext: Option<impl Into<String>>) -> Self {
        self.import_extension = ext.map(Into::into);
        self
    }

    /// Returns the file extension used for `import` statements in generated TypeScript files.  
    pub fn import_extension(&self) -> Option<&str> {
        self.import_extension.as_deref()
    }

    /// Sets the maximum size of arrays (`[T; N]`) up to which they are treated as TypeScript tuples (`[T, T, ...]`).  
    /// Arrays beyond this size will instead result in a TypeScript array (`Array<T>`).
    ///
    /// Default: `64`
    pub fn with_array_tuple_limit(mut self, limit: usize) -> Self {
        self.array_tuple_limit = limit;
        self
    }

    /// Returns the maximum size of arrays (`[T; N]`) up to which they are treated as TypeScript tuples (`[T, T, ...]`).  
    pub fn array_tuple_limit(&self) -> usize {
        self.array_tuple_limit
    }
}

#[doc(hidden)]
#[diagnostic::on_unimplemented(
    message = "`#[ts(optional)]` can only be used on fields of type `Option`",
    note = "`#[ts(optional)]` was used on a field of type {Self}, which is not permitted",
    label = "`#[ts(optional)]` is not allowed on field of type {Self}"
)]
pub trait IsOption {
    type Inner;
}

impl<T> IsOption for Option<T> {
    type Inner = T;
}

// generate impls for primitive types
macro_rules! impl_primitives {
    ($($($ty:ty),* => $l:expr),*) => { $($(
        impl TS for $ty {
            type WithoutGenerics = Self;
            type OptionInnerType = Self;
            fn name(_: &$crate::Config) -> String { String::from($l) }
            fn inline(cfg: &$crate::Config) -> String { <Self as $crate::TS>::name(cfg) }
        }
    )*)* };
}

// generate impls for big integers
macro_rules! impl_large_integers {
    ($($ty:ty),*) => { $(
        impl TS for $ty {
            type WithoutGenerics = Self;
            type OptionInnerType = Self;
            fn name(cfg: &$crate::Config) -> String { cfg.large_int_type.clone() }
            fn inline(cfg: &$crate::Config) -> String { <Self as $crate::TS>::name(cfg) }
        }
    )* };
}

// generate impls for tuples
macro_rules! impl_tuples {
    ( impl $($i:ident),* ) => {
        impl<$($i: TS),*> TS for ($($i,)*) {
            type WithoutGenerics = (Dummy, );
            type OptionInnerType = Self;
            fn name(cfg: &$crate::Config) -> String {
                format!("[{}]", [$(<$i as $crate::TS>::name(cfg)),*].join(", "))
            }
            fn inline(_: &$crate::Config) -> String {
                panic!("tuple cannot be inlined!");
            }
            fn visit_generics(v: &mut impl TypeVisitor)
            where
                Self: 'static
            {
                $(
                    v.visit::<$i>();
                    <$i as $crate::TS>::visit_generics(v);
                )*
            }
            fn inline_flattened(_: &$crate::Config) -> String { panic!("tuple cannot be flattened") }
            fn decl(_: &$crate::Config) -> String { panic!("tuple cannot be declared") }
            fn decl_concrete(_: &$crate::Config) -> String { panic!("tuple cannot be declared") }
        }
    };
    ( $i2:ident $(, $i:ident)* ) => {
        impl_tuples!(impl $i2 $(, $i)* );
        impl_tuples!($($i),*);
    };
    () => {};
}

// generate impls for wrapper types
macro_rules! impl_wrapper {
    ($($t:tt)*) => {
        $($t)* {
            type WithoutGenerics = Self;
            type OptionInnerType = Self;
            fn name(cfg: &$crate::Config) -> String { <T as $crate::TS>::name(cfg) }
            fn inline(cfg: &$crate::Config) -> String { <T as $crate::TS>::inline(cfg) }
            fn inline_flattened(cfg: &$crate::Config) -> String { <T as $crate::TS>::inline_flattened(cfg) }
            fn visit_dependencies(v: &mut impl TypeVisitor)
            where
                Self: 'static,
            {
                <T as $crate::TS>::visit_dependencies(v);
            }

            fn visit_generics(v: &mut impl TypeVisitor)
            where
                Self: 'static,
            {
                <T as $crate::TS>::visit_generics(v);
                v.visit::<T>();
            }
            fn decl(_: &$crate::Config) -> String { panic!("wrapper type cannot be declared") }
            fn decl_concrete(_: &$crate::Config) -> String { panic!("wrapper type cannot be declared") }
        }
    };
}

// implement TS for the $shadow, deferring to the impl $s
macro_rules! impl_shadow {
    (as $s:ty: $($impl:tt)*) => {
        $($impl)* {
            type WithoutGenerics = <$s as $crate::TS>::WithoutGenerics;
            type OptionInnerType = <$s as $crate::TS>::OptionInnerType;
            fn ident(cfg: &$crate::Config) -> String { <$s as $crate::TS>::ident(cfg) }
            fn name(cfg: &$crate::Config) -> String { <$s as $crate::TS>::name(cfg) }
            fn inline(cfg: &$crate::Config) -> String { <$s as $crate::TS>::inline(cfg) }
            fn inline_flattened(cfg: &$crate::Config) -> String { <$s as $crate::TS>::inline_flattened(cfg) }
            fn visit_dependencies(v: &mut impl $crate::TypeVisitor)
            where
                Self: 'static,
            {
                <$s as $crate::TS>::visit_dependencies(v);
            }
            fn visit_generics(v: &mut impl $crate::TypeVisitor)
            where
                Self: 'static,
            {
                <$s as $crate::TS>::visit_generics(v);
            }
            fn decl(cfg: &$crate::Config) -> String { <$s as $crate::TS>::decl(cfg) }
            fn decl_concrete(cfg: &$crate::Config) -> String { <$s as $crate::TS>::decl_concrete(cfg) }
            fn output_path() -> Option<std::path::PathBuf> { <$s as $crate::TS>::output_path() }
        }
    };
}

impl<T: TS> TS for Option<T> {
    type WithoutGenerics = Self;
    type OptionInnerType = T;
    const IS_OPTION: bool = true;

    fn name(cfg: &Config) -> String {
        format!("{} | null", T::name(cfg))
    }

    fn inline(cfg: &Config) -> String {
        format!("{} | null", T::inline(cfg))
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_generics(v);
        v.visit::<T>();
    }
}

impl<T: TS, E: TS> TS for Result<T, E> {
    type WithoutGenerics = Result<Dummy, Dummy>;
    type OptionInnerType = Self;

    fn name(cfg: &Config) -> String {
        format!("{{ Ok : {} }} | {{ Err : {} }}", T::name(cfg), E::name(cfg))
    }

    fn inline(cfg: &Config) -> String {
        format!(
            "{{ Ok : {} }} | {{ Err : {} }}",
            T::inline(cfg),
            E::inline(cfg)
        )
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_dependencies(v);
        <E as crate::TS>::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_generics(v);
        v.visit::<T>();
        <E as crate::TS>::visit_generics(v);
        v.visit::<E>();
    }
}

impl<T: TS> TS for Vec<T> {
    type WithoutGenerics = Vec<Dummy>;
    type OptionInnerType = Self;

    fn ident(_: &Config) -> String {
        "Array".to_owned()
    }

    fn name(cfg: &Config) -> String {
        format!("Array<{}>", T::name(cfg))
    }

    fn inline(cfg: &Config) -> String {
        format!("Array<{}>", T::inline(cfg))
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_generics(v);
        v.visit::<T>();
    }
}

impl<T: TS, const N: usize> TS for [T; N] {
    type WithoutGenerics = [Dummy; N];
    type OptionInnerType = Self;

    fn name(cfg: &Config) -> String {
        if N > cfg.array_tuple_limit() {
            return <Vec<T> as crate::TS>::name(cfg);
        }

        format!(
            "[{}]",
            (0..N)
                .map(|_| T::name(cfg))
                .collect::<Box<[_]>>()
                .join(", ")
        )
    }

    fn inline(cfg: &Config) -> String {
        if N > cfg.array_tuple_limit() {
            return <Vec<T> as crate::TS>::inline(cfg);
        }

        format!(
            "[{}]",
            (0..N)
                .map(|_| T::inline(cfg))
                .collect::<Box<[_]>>()
                .join(", ")
        )
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        <T as crate::TS>::visit_generics(v);
        v.visit::<T>();
    }
}

impl<K: TS, V: TS, H> TS for HashMap<K, V, H> {
    type WithoutGenerics = HashMap<Dummy, Dummy>;
    type OptionInnerType = Self;

    fn ident(_: &Config) -> String {
        panic!()
    }

    fn name(cfg: &Config) -> String {
        let optional = K::IS_ENUM || cfg.use_v11_hashmap;
        format!(
            "{{ [key in {}]{}: {} }}",
            K::name(cfg),
            if optional { "?" } else { "" },
            V::name(cfg),
        )
    }

    fn inline(cfg: &Config) -> String {
        let optional = K::IS_ENUM || cfg.use_v11_hashmap;
        format!(
            "{{ [key in {}]{}: {} }}",
            K::inline(cfg),
            if optional { "?" } else { "" },
            V::inline(cfg),
        )
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        K::visit_dependencies(v);
        V::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        K::visit_generics(v);
        v.visit::<K>();
        V::visit_generics(v);
        v.visit::<V>();
    }

    fn inline_flattened(cfg: &Config) -> String {
        format!("({})", Self::inline(cfg))
    }
}

// TODO: replace manual impl with dummy struct & `impl_shadow` (like for `JsonValue`)
impl<I: TS> TS for Range<I> {
    type WithoutGenerics = Range<Dummy>;
    type OptionInnerType = Self;

    fn name(cfg: &Config) -> String {
        let name = I::name(cfg);
        format!("{{ start: {name}, end: {name}, }}")
    }

    fn visit_dependencies(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        I::visit_dependencies(v);
    }

    fn visit_generics(v: &mut impl TypeVisitor)
    where
        Self: 'static,
    {
        I::visit_generics(v);
        v.visit::<I>();
    }

    fn inline(cfg: &Config) -> String {
        panic!("{} cannot be inlined", Self::name(cfg))
    }
}

impl_shadow!(as Range<I>: impl<I: TS> TS for RangeInclusive<I>);
impl_shadow!(as Vec<T>: impl<T: TS, H> TS for HashSet<T, H>);
impl_shadow!(as Vec<T>: impl<T: TS> TS for BTreeSet<T>);
impl_shadow!(as HashMap<K, V>: impl<K: TS, V: TS> TS for BTreeMap<K, V>);
impl_shadow!(as Vec<T>: impl<T: TS> TS for [T]);

impl_wrapper!(impl<T: TS + ?Sized> TS for &T);
impl_wrapper!(impl<T: TS + ?Sized> TS for Box<T>);
impl_wrapper!(impl<T: TS + ?Sized> TS for std::sync::Arc<T>);
impl_wrapper!(impl<T: TS + ?Sized> TS for std::rc::Rc<T>);
impl_wrapper!(impl<'a, T: TS + ToOwned + ?Sized> TS for std::borrow::Cow<'a, T>);
impl_wrapper!(impl<T: TS> TS for std::cell::Cell<T>);
impl_wrapper!(impl<T: TS> TS for std::cell::RefCell<T>);
impl_wrapper!(impl<T: TS> TS for std::sync::Mutex<T>);
impl_wrapper!(impl<T: TS> TS for std::sync::RwLock<T>);
impl_wrapper!(impl<T: TS + ?Sized> TS for std::sync::Weak<T>);
impl_wrapper!(impl<T: TS> TS for std::marker::PhantomData<T>);

impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

#[cfg(feature = "bigdecimal-impl")]
impl_primitives! { bigdecimal::BigDecimal => "string" }

#[cfg(feature = "smol_str-impl")]
impl_primitives! { smol_str::SmolStr => "string" }

#[cfg(feature = "uuid-impl")]
impl_primitives! { uuid::Uuid => "string" }

#[cfg(feature = "url-impl")]
impl_primitives! { url::Url => "string" }

#[cfg(feature = "ordered-float-impl")]
impl_primitives! { ordered_float::OrderedFloat<f32> => "number" }

#[cfg(feature = "ordered-float-impl")]
impl_primitives! { ordered_float::OrderedFloat<f64> => "number" }

#[cfg(feature = "bson-uuid-impl")]
impl_primitives! { bson::oid::ObjectId => "string" }

#[cfg(feature = "bson-uuid-impl")]
impl_primitives! { bson::Uuid => "string" }

#[cfg(feature = "indexmap-impl")]
impl_shadow!(as Vec<T>: impl<T: TS> TS for indexmap::IndexSet<T>);

#[cfg(feature = "indexmap-impl")]
impl_shadow!(as HashMap<K, V>: impl<K: TS, V: TS> TS for indexmap::IndexMap<K, V>);

#[cfg(feature = "heapless-impl")]
impl_shadow!(as Vec<T>: impl<T: TS, const N: usize> TS for heapless::Vec<T, N>);

#[cfg(feature = "arrayvec-impl")]
impl_shadow!(as Vec<T>: impl<T: TS, const N: usize> TS for arrayvec::ArrayVec<T, N>);

#[cfg(feature = "arrayvec-impl")]
impl_shadow!(as String: impl<const N: usize> TS for arrayvec::ArrayString<N>);

#[cfg(feature = "semver-impl")]
impl_primitives! { semver::Version => "string" }

#[cfg(feature = "bytes-impl")]
mod bytes {
    use super::TS;

    impl_shadow!(as Vec<u8>: impl TS for bytes::Bytes);
    impl_shadow!(as Vec<u8>: impl TS for bytes::BytesMut);
}

impl_primitives! {
    u8, i8, NonZeroU8, NonZeroI8,
    u16, i16, NonZeroU16, NonZeroI16,
    u32, i32, NonZeroU32, NonZeroI32,
    usize, isize, NonZeroUsize, NonZeroIsize, f32, f64 => "number",
    bool => "boolean",
    char, Path, PathBuf, String, str,
    Ipv4Addr, Ipv6Addr, IpAddr, SocketAddrV4, SocketAddrV6, SocketAddr => "string",
    () => "null"
}

impl_large_integers! {
    u64, i64, NonZeroU64, NonZeroI64,
    u128, i128, NonZeroU128, NonZeroI128
}

#[rustfmt::skip]
pub(crate) use impl_primitives;
#[rustfmt::skip]
pub(crate) use impl_shadow;
#[rustfmt::skip]
pub(crate) use impl_wrapper;

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dummy;

impl std::fmt::Display for Dummy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TS for Dummy {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name(_: &Config) -> String {
        "Dummy".to_owned()
    }

    fn inline(cfg: &Config) -> String {
        panic!("{} cannot be inlined", Self::name(cfg))
    }
}

/// Formats rust doc comments, turning them into a JSDoc comments.
/// Expects a `&[&str]` where each element corresponds to the value of one `#[doc]` attribute.
/// This work is deferred to runtime, allowing expressions in `#[doc]`, e.g `#[doc = file!()]`.
#[doc(hidden)]
pub fn format_docs(docs: &[&str]) -> String {
    match docs {
        // No docs
        [] => String::new(),

        // Multi-line block doc comment (/** ... */)
        [doc] if doc.contains('\n') => format!("/**{doc}*/\n"),

        // Regular doc comment(s) (///) or single line block doc comment
        _ => {
            let mut buffer = String::from("/**\n");
            let mut lines = docs.iter().peekable();

            while let Some(line) = lines.next() {
                buffer.push_str(" *");
                buffer.push_str(line);

                if lines.peek().is_some() {
                    buffer.push('\n');
                }
            }
            buffer.push_str("\n */\n");
            buffer
        }
    }
}
