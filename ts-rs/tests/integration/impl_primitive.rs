use ts_rs::Config;

#[cfg(feature = "bigdecimal-impl")]
#[test]
fn impl_primitive_bigdecimal() {
    let cfg = Config::from_env();
    assert_eq!(
        <bigdecimal::BigDecimal as ts_rs::TS>::name(&cfg),
        <String as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <bigdecimal::BigDecimal as ts_rs::TS>::inline(&cfg),
        <String as ts_rs::TS>::inline(&cfg)
    )
}

#[cfg(feature = "smol_str-impl")]
#[test]
fn impl_primitive_smolstr() {
    let cfg = Config::from_env();
    assert_eq!(
        <smol_str::SmolStr as ts_rs::TS>::name(&cfg),
        <String as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <smol_str::SmolStr as ts_rs::TS>::inline(&cfg),
        <String as ts_rs::TS>::inline(&cfg)
    )
}

#[cfg(feature = "uuid-impl")]
#[test]
fn impl_primitive_uuid() {
    let cfg = Config::from_env();
    assert_eq!(
        <uuid::Uuid as ts_rs::TS>::name(&cfg),
        <String as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <uuid::Uuid as ts_rs::TS>::inline(&cfg),
        <String as ts_rs::TS>::inline(&cfg)
    )
}

#[cfg(feature = "url-impl")]
#[test]
fn impl_primitive_url() {
    let cfg = Config::from_env();
    assert_eq!(
        <url::Url as ts_rs::TS>::name(&cfg),
        <String as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <url::Url as ts_rs::TS>::inline(&cfg),
        <String as ts_rs::TS>::inline(&cfg)
    )
}

#[cfg(feature = "ordered-float-impl")]
#[test]
fn impl_primitive_order_float() {
    let cfg = Config::from_env();
    assert_eq!(
        <ordered_float::OrderedFloat<f64> as ts_rs::TS>::name(&cfg),
        <f64 as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <ordered_float::OrderedFloat<f64> as ts_rs::TS>::inline(&cfg),
        <f64 as ts_rs::TS>::inline(&cfg)
    );
    assert_eq!(
        <ordered_float::OrderedFloat<f32> as ts_rs::TS>::name(&cfg),
        <f32 as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <ordered_float::OrderedFloat<f32> as ts_rs::TS>::inline(&cfg),
        <f32 as ts_rs::TS>::inline(&cfg)
    )
}

#[cfg(feature = "bson-uuid-impl")]
#[test]
fn impl_primitive_bson_uuid() {
    let cfg = Config::from_env();
    assert_eq!(
        <bson::oid::ObjectId as ts_rs::TS>::name(&cfg),
        <String as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <bson::oid::ObjectId as ts_rs::TS>::inline(&cfg),
        <String as ts_rs::TS>::inline(&cfg)
    );
    assert_eq!(
        <bson::Uuid as ts_rs::TS>::name(&cfg),
        <String as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <bson::Uuid as ts_rs::TS>::inline(&cfg),
        <String as ts_rs::TS>::inline(&cfg)
    )
}

#[cfg(feature = "semver-impl")]
#[test]
fn impl_primitive_semver() {
    let cfg = Config::from_env();
    assert_eq!(
        <semver::Version as ts_rs::TS>::name(&cfg),
        <String as ts_rs::TS>::name(&cfg)
    );
    assert_eq!(
        <semver::Version as ts_rs::TS>::inline(&cfg),
        <String as ts_rs::TS>::inline(&cfg)
    )
}
