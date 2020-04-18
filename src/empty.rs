#[cfg(feature = "lib-rustc-serialize")]
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
use std::fmt;

#[derive(Clone, Copy)]
pub struct Array;

#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
impl Serialize for Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [(); 0].serialize(serializer)
    }
}
#[cfg(feature = "lib-simd-json")]
impl simd_json_derive::Serialize for Array {
    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        use std::io::Write;
        writer.write_all(b"[]")
    }
    // fn write_content<W>(&self, writer: &mut W) -> std::io::Result<()>
    // where W: std::io::Write
    // {
    //     Ok(())
    // }
    // fn static_start() -> &'static [u8] { b"[" }
    // fn static_end() -> &'static [u8] { b"]" }
}

#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
impl<'de> Deserialize<'de> for Array {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Array;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty array")
            }

            fn visit_seq<V>(self, _: V) -> Result<Array, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                Ok(Array)
            }
        }

        deserializer.deserialize_tuple(0, Visitor)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Encodable for Array {
    fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
    where
        S: Encoder,
    {
        [(); 0].encode(s)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Decodable for Array {
    fn decode<D>(d: &mut D) -> Result<Array, D::Error>
    where
        D: Decoder,
    {
        d.read_tuple(0, |_| Ok(Array))
    }
}
