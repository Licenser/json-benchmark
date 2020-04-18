#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature = "lib-rustc-serialize")]
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
use serde::de::{self, Deserialize, Deserializer, Unexpected};
#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
use serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct PrimStr<T>(T)
where
    T: Copy + Ord + Display + FromStr;

#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
impl<T> Serialize for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

#[cfg(feature = "lib-simd-json")]
impl<T> simd_json_derive::Serialize for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write!(writer, "{}", self.0)
    }
    // fn write_content<W>(&self, writer: &mut W) -> std::io::Result<()>
    // where W: std::io::Write
    // {
    //     write!(writer, "{}", self.0)
    // }

    // fn static_start() -> &'static [u8] { b"" }
    // fn static_end() -> &'static [u8] { b"" }
}

#[cfg(any(feature = "lib-serde", feature = "simd-json"))]
impl<'de, T> Deserialize<'de> for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::marker::PhantomData;
        struct Visitor<T>(PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for Visitor<T>
        where
            T: Copy + Ord + Display + FromStr,
        {
            type Value = PrimStr<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number represented as string")
            }

            fn visit_str<E>(self, value: &str) -> Result<PrimStr<T>, E>
            where
                E: de::Error,
            {
                match T::from_str(value) {
                    Ok(id) => Ok(PrimStr(id)),
                    Err(_) => Err(E::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl<T> Encodable for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
    where
        S: Encoder,
    {
        self.0.to_string().encode(s)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl<T> Decodable for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn decode<D>(d: &mut D) -> Result<PrimStr<T>, D::Error>
    where
        D: Decoder,
    {
        let string = d.read_str()?;
        match T::from_str(&string) {
            Ok(id) => Ok(PrimStr(id)),
            Err(_) => Err(d.error(&format!("failed to parse id: {}", string))),
        }
    }
}
