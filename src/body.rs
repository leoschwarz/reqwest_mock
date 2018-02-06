use std::fs::File;
use std::io::{self, Read};

/// A HTTP request body.
///
/// This is either a file pointer or a memory sequence of bytes.
/// This distinction only matters when using `DirectClient`, in which case a file
/// might be read chunked.
///
/// Note that this is **not** the same type as the one found in the `reqwest` crate.
#[derive(Debug)]
pub struct Body {
    value: BodyValue,
}

#[derive(Debug)]
enum BodyValue {
    /// Bytes kept in memory.
    Bytes(Vec<u8>),

    /// A pointer to a file yet to be read.
    File(File),
}

impl Body {
    // TODO: Consider whether this should be public for everyone.
    pub(crate) fn try_to_vec(self) -> Result<Vec<u8>, io::Error> {
        match self.value {
            BodyValue::Bytes(bs) => Ok(bs),
            BodyValue::File(mut f) => {
                let mut bytes = Vec::new();
                f.read_to_end(&mut bytes)?;
                Ok(bytes)
            }
        }
    }
}

impl From<Body> for ::reqwest::Body {
    fn from(b: Body) -> ::reqwest::Body {
        match b.value {
            BodyValue::Bytes(b) => b.into(),
            BodyValue::File(f) => f.into(),
        }
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(v: Vec<u8>) -> Self {
        Body {
            value: BodyValue::Bytes(v.into()),
        }
    }
}

impl From<String> for Body {
    #[inline]
    fn from(s: String) -> Self {
        Body {
            value: BodyValue::Bytes(s.into()),
        }
    }
}

impl<'a> From<&'a str> for Body {
    #[inline]
    fn from(s: &'a str) -> Self {
        Body {
            value: BodyValue::Bytes(s.into()),
        }
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from(s: &'static [u8]) -> Self {
        Body {
            value: BodyValue::Bytes(s.into()),
        }
    }
}

impl From<File> for Body {
    #[inline]
    fn from(f: File) -> Self {
        Body {
            value: BodyValue::File(f),
        }
    }
}
