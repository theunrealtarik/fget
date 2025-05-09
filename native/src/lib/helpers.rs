use super::args::Args;
use rand::Rng;
use reqwest::{
    Url,
    header::{CONTENT_DISPOSITION, HeaderMap, HeaderValue},
};

#[derive(strum_macros::Display, strum_macros::EnumString, Clone, Copy, PartialEq, Eq)]
pub enum FileExistAction {
    Overwrite,
    Continue,
    Cancel,
}

#[derive(Debug, Default)]
pub struct FileSize {
    pub bytes: u64,
}

impl FileSize {
    fn from_bytes(bytes: u64) -> (f64, Unit) {
        Unit::from_bytes(bytes).unwrap_or((bytes as f64, Unit::B))
    }

    /// Returns the size scaled to the appropriate unit (e.g., 2048 [`Self::bytes`] → `2.0` [`Unit::KB`]).
    pub fn size(&self) -> f64 {
        let (size, _) = Self::from_bytes(self.bytes);
        size
    }
    /// Returns the most suitable unit for the file size (e.g., 2048 bytes → `Unit::KB`)
    pub fn unit(&self) -> Unit {
        let (_, unit) = Self::from_bytes(self.bytes);
        unit
    }

    /// Returns the formatted string, e.g., `"2.0 KB"`.
    pub fn pretty(&self) -> String {
        format!("{:.1} {}", self.size(), self.unit())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
#[allow(dead_code)]
pub enum Unit {
    #[default]
    B = 0,
    KB = 1,
    MB = 2,
    GB = 3,
    TB = 4,
}

impl TryFrom<u8> for Unit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Unit::B),
            1 => Ok(Unit::KB),
            2 => Ok(Unit::MB),
            3 => Ok(Unit::GB),
            4 => Ok(Unit::TB),
            _ => Err(()),
        }
    }
}

impl Unit {
    /// Calculates a suitable (value, unit) pair for a given byte size.
    /// Returns `(scaled_value, Unit)` or `Err(())` if out of bounds.
    pub fn from_bytes(bytes: u64) -> Result<(f64, Self), ()> {
        if let Ok(unit) = Unit::try_from((bytes as f64).log(1024.0).floor() as u8) {
            return Ok((bytes as f64 / unit.multiplier(), unit));
        }
        Err(())
    }

    /// Computes the multiplier (e.g., 1024 for KB, 1024² for MB, etc.).
    pub fn multiplier(self) -> f64 {
        1024f32.powi(self as i32) as f64
    }
}

#[test]
fn test_unit_from_bytes() {
    let a: u64 = 6_000;
    let b: u64 = 6_000_000;
    let c: u64 = 6_000_000_000;

    assert_eq!(
        Unit::from_bytes(a),
        Ok((a as f64 / Unit::KB.multiplier(), Unit::KB))
    );

    assert_eq!(
        Unit::from_bytes(b),
        Ok((b as f64 / Unit::MB.multiplier(), Unit::MB))
    );

    assert_eq!(
        Unit::from_bytes(c),
        Ok((c as f64 / Unit::GB.multiplier(), Unit::GB))
    );
}

pub fn random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(rand::distr::Alphabetic)
        .take(length)
        .map(char::from)
        .collect::<String>()
}

// Files

pub mod resolvers {
    use super::*;

    pub fn url(args: &Args) -> Url {
        args.url
            .as_ref()
            .and_then(|raw| Url::parse(raw).ok())
            .unwrap_or_else(|| {
                let raw_input: String = cliclack::input("Place the file's direct download link")
                    .placeholder("https://example.com/meme.png")
                    .required(true)
                    .validate(|input: &String| match Url::parse(input).is_err() {
                        true => Err("Invalid URL"),
                        false => Ok(()),
                    })
                    .interact()
                    .unwrap();

                Url::parse(&raw_input).unwrap()
            })
    }

    /// A resolver of static method on way to retreive the file's name
    pub mod filename {
        use super::*;

        pub fn from_args(args: &Args) -> Option<String> {
            args.filename.clone()
        }

        pub fn from_header(headers: &HeaderMap<HeaderValue>) -> Option<String> {
            let value = headers.get(CONTENT_DISPOSITION)?.to_str().ok()?;

            if !value.contains("filename") {
                return None;
            };

            let parsed = value
                .rsplit(";")
                .next()
                .unwrap_or("")
                .rsplit("=")
                .next()
                .unwrap_or("")
                .trim_start_matches("\"")
                .trim_end_matches("\"");

            if !parsed.is_empty() {
                Some(parsed.to_string())
            } else {
                None
            }
        }

        pub fn from_url(url: &Url) -> Option<String> {
            url.path_segments()
                .and_then(|segments| {
                    let this = segments.filter(|s| !s.is_empty());
                    #[inline]
                    fn some<T>(_: Option<T>, x: T) -> Option<T> {
                        Some(x)
                    }

                    this.fold(None, some)
                })
                .map(str::to_string)
        }

        pub fn random_name() -> String {
            format!("{}.unk", { random_string(8) })
        }

        pub fn attempt_all(args: &Args, url: &Url, headers: &HeaderMap<HeaderValue>) -> String {
            let filename = self::from_args(args)
                .or_else(|| self::from_header(headers))
                .or_else(|| self::from_url(url))
                .unwrap_or(self::random_name());
            sanitize_filename::sanitize(filename)
        }
    }
}
