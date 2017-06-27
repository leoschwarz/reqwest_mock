use client::{Client, Response};
use client::{DirectClient, RecordingTarget, ReplayClient};
use config::ClientConfig;
use error::Error;
use request::Request;
use std::path::PathBuf;

enum InnerClient {
    Direct(DirectClient),
    Replay(ReplayClient),
}

/// Provides an interface over the different client types which you can use in your code
/// if you want to avoid it having to be generic over the `Client` trait.
pub struct GenericClient {
    inner: InnerClient,
}

impl GenericClient {
    /// Create a `GenericClient` using `DirectClient` internally.
    pub fn direct() -> Self {
        DirectClient::new().into()
    }

    /// Create a `GenericClient` using `ReplayClient` internally, recording one single
    /// request to one single replay file. If a differing request is made, the file will be
    /// overwritten again.
    pub fn replay_file<P: Into<PathBuf>>(replay_file: P) -> Self {
        ReplayClient::new(RecordingTarget::File(replay_file.into())).into()
    }

    /// Create a `GenericClient` using `ReplayClient` internally, recording multiple requests
    /// to a single directory. Each unique request will get its own replay file independent of
    /// other requests in the specified directory.
    pub fn replay_dir<P: Into<PathBuf>>(replay_dir: P) -> Self {
        ReplayClient::new(RecordingTarget::Dir(replay_dir.into())).into()
    }

    /*
    /// Convert the current instance to a `ReplayClient` replaying the file at the provided path.
    ///
    /// This can also be used to just switch the replay file as each file is only used for one
    /// request/response pair.
    pub fn replay_file<P: Into<PathBuf>>(&mut self, path: P) {
        self.inner = InnerClient::Replay(ReplayClient::new(RecordingTarget::File(path.into())));
    }
    */
}

impl From<DirectClient> for GenericClient {
    fn from(c: DirectClient) -> Self {
        GenericClient { inner: InnerClient::Direct(c) }
    }
}

impl From<ReplayClient> for GenericClient {
    fn from(c: ReplayClient) -> Self {
        GenericClient { inner: InnerClient::Replay(c) }
    }
}

impl Client for GenericClient {
    fn execute(&self, config: Option<&ClientConfig>, request: Request) -> Result<Response, Error> {
        match self.inner {
            InnerClient::Direct(ref client) => client.execute(config, request),
            InnerClient::Replay(ref client) => client.execute(config, request),
        }
    }

    fn config(&self) -> &ClientConfig {
        match self.inner {
            InnerClient::Direct(ref client) => client.config(),
            InnerClient::Replay(ref client) => client.config(),
        }
    }

    fn config_mut(&mut self) -> &mut ClientConfig {
        match self.inner {
            InnerClient::Direct(ref mut client) => client.config_mut(),
            InnerClient::Replay(ref mut client) => client.config_mut(),
        }
    }
}
