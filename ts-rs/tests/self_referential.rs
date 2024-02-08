#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use ts_rs::TS;

#[test]
fn named() {
    #[derive(TS)]
    struct HasT {
        t: &'static T<'static>,
    }

    #[derive(TS)]
    struct T<'a> {
        t_box: Box<T<'a>>,
        self_box: Box<Self>,

        t_ref: &'a T<'a>,
        self_ref: &'a Self,

        t_arc: Arc<T<'a>>,
        self_arc: Arc<Self>,

        #[ts(inline)]
        has_t: HasT,
    }

    assert_eq!(
        T::decl(),
        "type T = { \
            t_box: T, \
            self_box: T, \
            t_ref: T, \
            self_ref: T, \
            t_arc: T, \
            self_arc: T, \
            has_t: { t: T, }, \
         }"
    );
}

#[test]
fn enum_externally_tagged() {
    #[derive(TS)]
    #[ts(rename = "T")]
    enum ExternallyTagged {
        A(Box<ExternallyTagged>),
        B(&'static ExternallyTagged),
        C(Box<Self>),
        D(&'static Self),
        E(
            Box<ExternallyTagged>,
            Box<Self>,
            &'static ExternallyTagged,
            &'static Self,
        ),
        F {
            a: Box<Self>,
            b: &'static ExternallyTagged,
            c: HashMap<String, ExternallyTagged>,
            d: Option<Arc<ExternallyTagged>>,
            #[ts(optional = nullable)]
            e: Option<Arc<ExternallyTagged>>,
            #[ts(optional)]
            f: Option<Arc<ExternallyTagged>>,
        },

        G(
            Vec<ExternallyTagged>,
            [&'static ExternallyTagged; 1024],
            HashMap<String, ExternallyTagged>,
        ),
    }

    assert_eq!(
        ExternallyTagged::decl(),
       "type T = { \"A\": T } | \
                 { \"B\": T } | \
                 { \"C\": T } | \
                 { \"D\": T } | \
                 { \"E\": [T, T, T, T] } | \
                 { \"F\": { a: T, b: T, c: Record<string, T>, d: T | null, e?: T | null, f?: T, } } | \
                 { \"G\": [Array<T>, Array<T>, Record<string, T>] };"
    );
}

// NOTE: The generated type is actually not valid TS here, since the indirections rust enforces for recursive types
//       gets lost during the translation to TypeScript (e.g "Box<T>" => "T").
#[test]
#[cfg(feature = "serde-compat")]
fn enum_internally_tagged() {
    use serde::Serialize;
    #[derive(Serialize, TS)]
    #[ts(rename = "T")]
    #[serde(tag = "tag")]
    enum InternallyTagged {
        A(Box<InternallyTagged>),
        B(&'static InternallyTagged),
        C(Box<Self>),
        D(&'static Self),
        E(Vec<Self>),
        F {
            a: Box<Self>,
            b: &'static InternallyTagged,
            c: HashMap<InternallyTagged, InternallyTagged>,
            d: Option<&'static InternallyTagged>,
            #[ts(optional = nullable)]
            e: Option<&'static InternallyTagged>,
            #[ts(optional)]
            f: Option<&'static InternallyTagged>,
        },
    }

    assert_eq!(
        InternallyTagged::decl(),
        "type T = { \"tag\": \"A\" } & T | \
                  { \"tag\": \"B\" } & T | \
                  { \"tag\": \"C\" } & T | \
                  { \"tag\": \"D\" } & T | \
                  { \"tag\": \"E\" } & Array<T> | \
                  { \"tag\": \"F\", a: T, b: T, c: Record<T, T>, d: T | null, e?: T | null, f?: T, };"
    );
}

// NOTE: The generated type is actually not valid TS here, since the indirections rust enforces for recursive types
//       gets lost during the translation to TypeScript (e.g "Box<T>" => "T").
#[test]
#[cfg(feature = "serde-compat")]
fn enum_adjacently_tagged() {
    use serde::Serialize;
    #[derive(Serialize, TS)]
    #[ts(rename = "T")]
    #[serde(tag = "tag", content = "content")]
    enum AdjacentlyTagged {
        A(Box<AdjacentlyTagged>),
        B(&'static AdjacentlyTagged),
        C(Box<Self>),
        D(&'static Self),
        E(Vec<Self>),
        F {
            a: Box<Self>,
            b: &'static AdjacentlyTagged,
            c: HashMap<String, AdjacentlyTagged>,
            d: Option<&'static AdjacentlyTagged>,
            #[ts(optional = nullable)]
            e: Option<&'static AdjacentlyTagged>,
            #[ts(optional)]
            f: Option<&'static AdjacentlyTagged>,
        },
        G(
            Vec<Self>,
            [&'static AdjacentlyTagged; 4],
            HashMap<String, AdjacentlyTagged>,
        ),
    }

    assert_eq!(
        AdjacentlyTagged::decl(),
        "type T = { \"tag\": \"A\", \"content\": T } | \
                  { \"tag\": \"B\", \"content\": T } | \
                  { \"tag\": \"C\", \"content\": T } | \
                  { \"tag\": \"D\", \"content\": T } | \
                  { \"tag\": \"E\", \"content\": Array<T> } | \
                  { \
                     \"tag\": \"F\", \
                     \"content\": { \
                         a: T, \
                         b: T, \
                         c: Record<string, T>, \
                         d: T | null, \
                         e?: T | null, \
                         f?: T, \
                     } \
                  } | \
                  { \
                     \"tag\": \"G\", \
                     \"content\": [\
                         Array<T>, \
                         [T, T, T, T], \
                         Record<string, T>\
                     ] \
                  };"
    );
}
