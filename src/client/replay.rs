use client::{Client, DirectClient};
use config::ClientConfig;
use error::Error;
use request::{Request, RequestMem};
use response::Response;

use std::fs::{create_dir_all, File};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use twox_hash::XxHash;

/// The version of the storage format. The code is only compatible with files of the same version,
/// everything else will be discarded and recorded again.
const FORMAT_VERSION: u8 = 3;

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
    force_record_next: AtomicBool,
}

impl ReplayClient {
    /// Create a new `ReplayClient` instance reading and writing to the specified target.
    pub fn new(target: RecordingTarget) -> Self {
        ReplayClient {
            config: ClientConfig::default(),
            target: target,
            force_record_next: AtomicBool::new(false),
        }
    }

    /// Calling this method ensures that whatever next request is performed it will be recorded
    /// again, even the exact same request was already made before.
    pub fn force_record_next(&self) {
        self.force_record_next.store(true, Ordering::SeqCst);
    }

    fn replay_file_path(&self, request: &RequestMem) -> PathBuf {
        match self.target {
            RecordingTarget::File(ref file) => file.clone(),
            RecordingTarget::Dir(ref dir) => {
                // TODO: I took this hash function as unlike DefaultHasher it is specified.
                //       However more evaluation should be done before settling on this
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
    fn get_data(&self, request: &RequestMem) -> Result<Option<ReplayData>, Error> {
        let file = self.replay_file_path(request);
        let force_record = self.force_record_next.swap(false, Ordering::SeqCst);
        debug!("Checking presence of replay file: {:?}", file);

        if !file.exists() {
            debug!("Existing replay file was found.");
            Ok(None)
        } else if force_record {
            debug!("Replay file exists but force record was requested.");
            Ok(None)
        } else {
            use serde_json::Value;

            debug!("Reading existing replay file.");
            let f = File::open(&file)?;
            let value: Value = ::serde_json::from_reader(f)?;

            // Check the format version.
            let format_version = match value {
                Value::Object(ref obj) => obj.get("format_version")
                    .and_then(|val| val.as_u64())
                    .map(|n| n as u8),
                _ => None,
            };

            if format_version == Some(FORMAT_VERSION) {
                Ok(::serde_json::from_value(value)?)
            } else {
                debug!(
                    "Replay file exists but has wrong format version: {:?}",
                    format_version
                );
                Ok(None)
            }
        }
    }

    fn store_data(&self, data: &ReplayData) -> Result<(), Error> {
        let file = self.replay_file_path(&data.request);
        debug!("Writing replay file at: {:?}", file);

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
        let req: RequestMem = request.to_mem()?;

        // Some information potentially useful for debugging.
        debug!(
            "ReplayClient performing {} request of URL: {}",
            req.header.method, req.header.url
        );
        trace!("request headers: {:?}", req.header.headers);
        trace!("request body: {:?}", req.body);

        // Use internal config if none was provided together with the request.
        let config = config.unwrap_or_else(|| &self.config);

        // Check if the request was already performed with this exact arguments,
        // if it was just return the existing result otherwise perform the request and store
        // the output.

        let data = self.get_data(&req)?;
        if let Some(d) = data {
            if d.request == req {
                return Ok(d.response);
            } else {
                // TODO better message
                info!("reqwest_mock: Request has changed, recording again now.");
            }
        }

        // We actually have to perform the request and store the response.
        let client = DirectClient::new();
        let response = client.execute(Some(config), req.clone().into())?;

        self.store_data(&ReplayData {
            request: req,
            response: response.clone(),
            format_version: FORMAT_VERSION,
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
#[derive(Debug, Serialize, Deserialize)]
struct ReplayData {
    request: RequestMem,
    response: Response,
    format_version: u8,
}
