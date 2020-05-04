//! Some types used to configure a `Client` instance.

use std::time::Duration;

/// Configures some parameters for a `Client` instance.
#[derive(Clone, Debug)]
pub struct ClientConfig {
    /// Enable auto gzip decompression checking the `ContentEncoding` response header.
    ///
    /// Default is enabled.
    pub gzip: bool,

    /// `RedirectPolicy` for this client.
    ///
    /// Default will follow up to 10 redirects.
    pub redirect: RedirectPolicy,

    /// Enable or disable automatic setting of the `Referer` header.
    ///
    /// Default is true.
    ///
    /// TODO: Not actually implemented yet.
    pub referer: bool,

    /// Timeout for both the read and write operations of a client.
    pub timeout: Option<Duration>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            gzip: true,
            redirect: RedirectPolicy::default(),
            referer: true,
            timeout: None,
        }
    }
}

impl ClientConfig {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO implement builder pattern
}

/// Specifies how to hande redirects.
#[derive(Clone, Debug)]
pub enum RedirectPolicy {
    Limit(usize),
    None,
}

impl Default for RedirectPolicy {
    fn default() -> Self {
        RedirectPolicy::Limit(10)
    }
}

impl From<RedirectPolicy> for ::reqwest::redirect::Policy {
    fn from(p: RedirectPolicy) -> Self {
        match p {
            RedirectPolicy::Limit(n) => ::reqwest::redirect::Policy::limited(n),
            RedirectPolicy::None => ::reqwest::redirect::Policy::none(),
        }
    }
}
