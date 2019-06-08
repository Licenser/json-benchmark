use json_benchmark::*;

use std::fs::File;
use std::io::{self, Read, Write};

macro_rules! bench {
    {
        name: $name:expr,
        $($args:tt)*
    } => {
        let name = format!(" {} ", $name);
        println!("\n{:=^26} parse|stringify ===== parse|stringify ====", name);

        #[cfg(feature = "file-canada")]
        bench_file! {
            path: "data/canada.json",
            structure: canada::Canada,
            $($args)*
        }

        #[cfg(feature = "file-citm-catalog")]
        bench_file! {
            path: "data/citm_catalog.json",
            structure: citm_catalog::CitmCatalog,
            $($args)*
        }

        #[cfg(feature = "file-twitter")]
        bench_file! {
            path: "data/twitter.json",
            structure: twitter::Twitter,
            $($args)*
        }
    }
}

macro_rules! bench_file {
    {
        path: $path:expr,
        structure: $structure:ty,
        dom: $dom:ty,
        parse_dom: $parse_dom:expr,
        stringify_dom: $stringify_dom:expr,
        $(
            parse_struct: $parse_struct:expr,
            stringify_struct: $stringify_struct:expr,
        )*
    } => {
        let num_trials = num_trials().unwrap_or(256);

        print!("{:22}", $path);
        io::stdout().flush().unwrap();

        let contents = {
            let mut vec = Vec::new();
            File::open($path).unwrap().read_to_end(&mut vec).unwrap();
            vec
        };

        #[cfg(feature = "parse-dom")]
        {
            use std::iter;
            let mut data: Vec<Vec<u8>> = iter::repeat(contents.clone()).take(num_trials).collect();
            let mut i = 0;
            let dur = timer::bench(num_trials, || {
                let parsed: $dom = $parse_dom(&mut data[i]).unwrap();
                i += 1;
                parsed
            });
            print!("{:6} MB/s", throughput(dur, contents.len()));
            io::stdout().flush().unwrap();
        }
        #[cfg(not(feature = "parse-dom"))]
        print!("          ");

        #[cfg(feature = "stringify-dom")]
        {
            let len = contents.len();
            let mut data = contents.clone();
            let dom: $dom = $parse_dom(&mut data).unwrap();
            let dur = timer::bench_with_buf(num_trials, len, |out| {
                $stringify_dom(out, &dom).unwrap()
            });
            let mut serialized = Vec::new();
            $stringify_dom(&mut serialized, &dom).unwrap();
            print!("{:6} MB/s", throughput(dur, serialized.len()));
            io::stdout().flush().unwrap();
        }
        #[cfg(not(feature = "stringify-dom"))]
        print!("          ");

        $(
            #[cfg(feature = "parse-struct")]
            {
                use std::iter;
                let mut data: Vec<Vec<u8>> = iter::repeat(contents.clone()).take(num_trials).collect();
                let mut i = 0;
                let dur = timer::bench(num_trials, || {
                    let _parsed: $structure = $parse_struct(&mut data[i]).unwrap();
                    i += 1;
                    //parsed
                });
                print!("{:6} MB/s", throughput(dur, contents.len()));
                io::stdout().flush().unwrap();
            }
            #[cfg(not(feature = "parse-struct"))]
            print!("          ");

            #[cfg(feature = "stringify-struct")]
            {
                let len = contents.len();
                let mut data = contents.clone();
                let parsed: $structure = $parse_struct(&mut data).unwrap();
                let dur = timer::bench_with_buf(num_trials, len, |out| {
                    $stringify_struct(out, &parsed).unwrap()
                });
                let mut serialized = Vec::new();
                $stringify_struct(&mut serialized, &parsed).unwrap();
                print!("{:6} MB/s", throughput(dur, serialized.len()));
                io::stdout().flush().unwrap();
            }
        )*

        println!();
    }
}

fn main() {
    print!("{:>35}{:>24}", "DOM", "STRUCT");

    #[cfg(feature = "lib-serde")]
    bench! {
        name: "serde_json",
        dom: serde_json::Value,
        parse_dom: serde_json_parse_dom,
        stringify_dom: serde_json::to_writer,
        parse_struct: serde_json_parse_struct,
        stringify_struct: serde_json::to_writer,
    }

    #[cfg(feature = "lib-json-rust")]
    bench! {
        name: "json-rust",
        dom: json::JsonValue,
        parse_dom: json_rust_parse_dom,
        stringify_dom: json_rust_stringify_dom,
    }

    #[cfg(feature = "lib-rustc-serialize")]
    bench! {
        name: "rustc_serialize",
        dom: rustc_serialize::json::Json,
        parse_dom: rustc_serialize_parse_dom,
        stringify_dom: rustc_serialize_stringify,
        parse_struct: rustc_serialize_parse_struct,
        stringify_struct: rustc_serialize_stringify,
    }

    #[cfg(feature = "lib-simd-json")]
    bench! {
        name: "serde_json",
        dom: simd_json::OwnedValue,
        parse_dom: simd_json_parse_dom,
        stringify_dom: simd_json_stringify_dom,
        parse_struct: simd_json_parse_struct,
        stringify_struct: serde_json::to_writer,
        }
}

#[cfg(all(
    feature = "lib-serde",
    any(feature = "parse-dom", feature = "stringify-dom")
))]
fn serde_json_parse_dom(bytes: &[u8]) -> serde_json::Result<serde_json::Value> {
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    serde_json::from_str(s)
}

#[cfg(all(
    feature = "lib-serde",
    any(feature = "parse-struct", feature = "stringify-struct")
))]
fn serde_json_parse_struct<'de, T>(bytes: &'de [u8]) -> serde_json::Result<T>
where
    T: serde::Deserialize<'de>,
{
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    serde_json::from_str(s)
}

#[cfg(all(
    feature = "lib-json-rust",
    any(feature = "parse-dom", feature = "stringify-dom")
))]
fn json_rust_parse_dom(bytes: &[u8]) -> json::Result<json::JsonValue> {
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    json::parse(&s)
}

#[cfg(all(feature = "lib-json-rust", feature = "stringify-dom"))]
fn json_rust_stringify_dom<W: io::Write>(write: &mut W, dom: &json::JsonValue) -> io::Result<()> {
    dom.write(write)
}

#[cfg(all(
    feature = "lib-rustc-serialize",
    any(feature = "parse-dom", feature = "stringify-dom")
))]
fn rustc_serialize_parse_dom(
    mut bytes: &[u8],
) -> Result<rustc_serialize::json::Json, rustc_serialize::json::BuilderError> {
    rustc_serialize::json::Json::from_reader(&mut bytes)
}

#[cfg(all(
    feature = "lib-rustc-serialize",
    any(feature = "parse-struct", feature = "stringify-struct")
))]
fn rustc_serialize_parse_struct<T>(bytes: &[u8]) -> rustc_serialize::json::DecodeResult<T>
where
    T: rustc_serialize::Decodable,
{
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    rustc_serialize::json::decode(s)
}

#[cfg(all(
    feature = "lib-rustc-serialize",
    any(feature = "stringify-dom", feature = "stringify-struct")
))]
fn rustc_serialize_stringify<W, T: ?Sized>(
    writer: W,
    value: &T,
) -> rustc_serialize::json::EncodeResult<()>
where
    W: Write,
    T: rustc_serialize::Encodable,
{
    let mut writer = adapter::IoWriteAsFmtWrite::new(writer);
    let mut encoder = rustc_serialize::json::Encoder::new(&mut writer);
    value.encode(&mut encoder)
}

#[cfg(all(
    feature = "lib-somd-json",
    any(feature = "parse-dom", feature = "stringify-dom")
))]
fn serde_json_parse_dom(bytes: &[u8]) -> serde_json::Result<serde_json::Value> {
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    serde_json::from_str(s)
}

#[cfg(all(
    feature = "lib-simd-json",
    any(feature = "parse-dom", feature = "stringify-dom")
))]
fn simd_json_parse_dom(bytes: &mut [u8]) -> simd_json::Result<simd_json::OwnedValue> {
    simd_json::to_owned_value(bytes)
}
#[cfg(all(
    feature = "lib-simd-json",
    any(feature = "parse-struct", feature = "stringify-struct")
))]
fn simd_json_parse_struct<'de, T>(bytes: &'de mut [u8]) -> simd_json::Result<T>
where
    T: serde::Deserialize<'de>,
{
    simd_json::serde::from_slice(bytes)
}

#[cfg(all(feature = "lib-simd-json", feature = "stringify-dom"))]
fn simd_json_stringify_dom<W: io::Write>(write: &mut W, dom: &simd_json::OwnedValue) -> io::Result<()> {
    dom.write(write)
}
