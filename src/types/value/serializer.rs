use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Write};

use serde::ser::{self};
use serde_json::Number;

use crate::GqlValue;
#[derive(Debug)]
pub struct SerializerError(String);

impl Display for SerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
impl Error for SerializerError {
    fn description(&self) -> &str {
        "GqlValue serializer error"
    }
}

impl ser::Error for SerializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializerError(msg.to_string())
    }
}

struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = GqlValue;

    type Error = SerializerError;

    type SerializeSeq = SerializeSeq;

    type SerializeTuple = SerializeTuple;

    type SerializeTupleStruct = SerializeTupleStruct;

    type SerializeTupleVariant = SerializeTupleVariant;

    type SerializeMap = SerializeMap;

    type SerializeStruct = SerializeStruct;

    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Number(Number::from(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match Number::from_f64(v as f64) {
            Some(v) => Ok(GqlValue::Number(v)),
            None => Err(SerializerError(format!("{}: f32 failed to serialize", v))),
        }
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match Number::from_f64(v) {
            Some(v) => Ok(GqlValue::Number(v)),
            None => Err(SerializerError(format!("{}: f32 failed to serialize", v))),
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

struct SerializeSeq(Vec<GqlValue>);

impl ser::SerializeSeq for SerializeSeq {
    type Ok = GqlValue;

    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

struct SerializeTuple(Vec<GqlValue>);

impl ser::SerializeTuple for SerializeTuple {
    type Ok = GqlValue;

    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

struct SerializeTupleStruct(Vec<GqlValue>);

impl ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.push(value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::List(self.0))
    }
}

struct SerializeTupleVariant(String, Vec<GqlValue>);

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.1.push(value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = BTreeMap::new();
        map.insert(self.0, GqlValue::List(self.1));
        Ok(GqlValue::Object(map))
    }
}

struct SerializeMap {
    map: BTreeMap<String, GqlValue>,
    key: Option<String>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        // let key = key.serialize(MapKeySerializer)?;
        // self.key = Some(key);
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.map.insert(self.key.take().unwrap(), value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Object(self.map))
    }
}

struct SerializeStruct(BTreeMap<String, GqlValue>);

impl ser::SerializeStruct for SerializeStruct {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.0.insert(key.to_string(), value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GqlValue::Object(self.0))
    }
}

struct SerializeStructVariant(String, BTreeMap<String, GqlValue>);

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = GqlValue;
    type Error = SerializerError;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        let value = value.serialize(Serializer)?;
        self.1.insert(key.to_string(), value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = BTreeMap::new();
        map.insert(self.0, GqlValue::Object(self.1));
        Ok(GqlValue::Object(map))
    }
}
