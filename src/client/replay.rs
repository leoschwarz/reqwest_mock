use client::{Client, DirectClient};
use config::ClientConfig;
use error::Error;
use request::Request;
use response::Response;
use std::fs::{File, create_dir_all};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use twox_hash::XxHash;

/// The recording target.
pub enum RecordingTarget {
    /// A single file is used for recording one request, if the request changes the file is
    /// replaced by a new one.
    File(PathBuf),

    /// A directory is used in which multiple replay files are managed for each request data
    /// an individual file is created.
    Dir(PathBuf),
}

impl RecordingTarget {
    /// Shorthand to specify `RecordingTarget::File`.
    pub fn file<P: Into<PathBuf>>(file: P) -> Self {
        RecordingTarget::File(file.into())
    }

    /// Shorthand to specify `RecordingTarget::Dir`.
    pub fn dir<P: Into<PathBuf>>(dir: P) -> Self {
        RecordingTarget::Dir(dir.into())
    }
}

/// Records responses to requests and replays them if the request is unchanged.
pub struct ReplayClient {
    config: ClientConfig,
    target: RecordingTarget,
}

impl ReplayClient {
    /// Create a new `ReplayClient` instance reading and writing to the specified target.
    pub fn new(target: RecordingTarget) -> Self {
        ReplayClient {
            config: ClientConfig::default(),
            target: target,
        }
    }

    fn replay_file_path(&self, request: &Request) -> PathBuf {
        match self.target {
            RecordingTarget::File(ref file) => file.clone(),
            RecordingTarget::Dir(ref dir) => {
                // TODO: took this hash function as unlike DefaultHasher it is specified.
                //       however more evaluation should be done before settling on this
                //       one as the hasher for the stable release.
                let mut hasher = XxHash::with_seed(42);
                request.hash(&mut hasher);
                let filename = format!("{:x}.json", hasher.finish());

                dir.join(filename)
            }
        }
    }

    /// The possible results:
    ///
    /// Err(_)      → something went wrong.
    /// Ok(None)    → no data was stored yet, i. e. the file doesn't exist yet.
    /// Ok(Some(_)) → the actual data
    fn get_data(&self, request: &Request) -> Result<Option<ReplayData>, Error> {
        let file = self.replay_file_path(request);
        if !file.exists() {
            Ok(None)
        } else {
            let f = File::open(&file)?;
            Ok(::serde_json::from_reader(f)?)
        }
    }

    fn store_data(&self, data: &ReplayData) -> Result<(), Error> {
        let file = self.replay_file_path(&data.request);

        // Attempt to create the directory of the file if it doesn't exist yet.
        if let Some(parent) = file.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        // Write the file.
        let f = File::create(&file)?;
        ::serde_json::to_writer(f, data)?;
        Ok(())
    }
}

impl Client for ReplayClient {
    fn execute(&self, config: Option<&ClientConfig>, request: Request) -> Result<Response, Error> {
        // Use internal config if none was provided together with the request.
        let config = config.unwrap_or_else(|| &self.config);

        // Check if the request was already performed with this exact arguments,
        // if it was just return the existing result otherwise perform the request and store
        // the output.

        let data = self.get_data(&request)?;
        if let Some(d) = data {
            if d.request == request {
                return Ok(d.response);
            } else {
                // TODO better message
                println!("reqwest_mock: Request has changed, recording again now.");
            }
        }

        // We actually have to perform the request and store the response.
        let client = DirectClient::new();
        let response = client.execute(Some(config), request.clone())?;

        self.store_data(&ReplayData {
            request: request,
            response: response.clone(),
        })?;

        // Return the response.
        Ok(response)
    }

    fn config(&self) -> &ClientConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut ClientConfig {
        &mut self.config
    }
}

/// The data stored inside of a replay file.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReplayData {
    request: Request,
    response: Response,
}
