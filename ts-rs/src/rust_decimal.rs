use std::sync::OnceLock;

use rust_decimal::Decimal;

use crate::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecimalSerdeShape {
    String,
    Number,
}

fn detect_decimal_serde_shape() -> DecimalSerdeShape {
    // `rust_decimal::Decimal` has an inherent method named `serialize`, so we must call the
    // *trait* method explicitly to invoke serde serialization.
    match serde::Serialize::serialize(&Decimal::default(), DecimalShapeProbeSerializer) {
        Ok(shape) => shape,
        Err(_) => DecimalSerdeShape::String,
    }
}

/// A minimal serializer which only cares whether `Serialize` emits a string or a number.
///
/// Any unexpected serialization shape is treated as an error and falls back to `string`.
struct DecimalShapeProbeSerializer;

impl serde::ser::Serializer for DecimalShapeProbeSerializer {
    type Ok = DecimalSerdeShape;
    type Error = ShapeProbeError;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::String)
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_i128(self, _value: i128) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_u128(self, _value: u128) -> Result<Self::Ok, Self::Error> {
        Ok(DecimalSerdeShape::Number)
    }

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ShapeProbeError)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ShapeProbeError)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ShapeProbeError)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ShapeProbeError)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ShapeProbeError)
    }
}

#[derive(Debug)]
struct ShapeProbeError;

impl std::fmt::Display for ShapeProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unsupported serialization shape")
    }
}

impl std::error::Error for ShapeProbeError {}

impl serde::ser::Error for ShapeProbeError {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        ShapeProbeError
    }
}

fn decimal_ts_binding() -> &'static str {
    static DECIMAL_BINDING: OnceLock<&'static str> = OnceLock::new();

    DECIMAL_BINDING.get_or_init(|| match detect_decimal_serde_shape() {
        DecimalSerdeShape::Number => "number",
        DecimalSerdeShape::String => "string",
    })
}

impl TS for Decimal {
    type WithoutGenerics = Decimal;
    type OptionInnerType = Decimal;

    fn decl() -> String {
        panic!("{} cannot be declared", <Self as TS>::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", <Self as TS>::name())
    }

    fn name() -> String {
        decimal_ts_binding().to_owned()
    }

    fn inline() -> String {
        <Self as TS>::name()
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", <Self as TS>::name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_ts_name_matches_actual_serialization() {
        let decimal = Decimal::new(123, 2); // 1.23
        let json = serde_json::to_value(&decimal).unwrap();

        match Decimal::name().as_str() {
            "number" => assert!(json.is_number()),
            "string" => assert!(json.is_string()),
            other => panic!("unexpected type: {}", other),
        }
    }
}
