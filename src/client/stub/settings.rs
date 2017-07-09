/// Control how strict the `StubClient` is about matching stubs to the requests you make through
/// the client.
///
/// When you make a request it consists of a url, a HTTP method, headers and a body. To determine which
/// response to return the `StubClient` checks the different fields against the ones stored in its
/// list of stubs. You can control which fields are checked for equality.
///
/// TODO: Determine the need of matching the full header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StubStrictness {
    /// Full equality in all fields between stub and actual request required.
    Full,

    /// `body`, `method` and `url` have to be equal.
    BodyMethodUrl,

    /// `headers`, `method` and `url` have to be equal.
    HeadersMethodUrl,

    /// `method` and `url` have to be equal.
    MethodUrl,

    /// `url` has to be equal.
    Url,
}

/// Define the default action to be taken by the `StubClient` when no matching stub is found.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StubDefault {
    /// Just directly perform any requests. Warning: only use this if you are sure it is ok.
    PerformRequest,

    /// Panic if such a request is made.
    Panic,

    /// Return an `Err` if such a request is made.
    Error,
}

/// Some settings for the `StubClient`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StubSettings {
    /// Specifies the default action to be taken when no stub is found for a request.
    pub default: StubDefault,

    /// Specifies how strict matching requests to stubs is.
    pub strictness: StubStrictness,
}

impl Default for StubSettings {
    fn default() -> Self {
        StubSettings {
            default: StubDefault::Error,
            strictness: StubStrictness::Full,
        }
    }
}
