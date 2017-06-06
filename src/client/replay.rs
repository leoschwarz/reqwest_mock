use client::{Client, DirectClient};
use config::ClientConfig;
use error::Error;
use request::Request;
use response::Response;
use std::path::PathBuf;
use std::fs::File;

/// Records responses to requests and replays them if the request is unchanged.
pub struct ReplayClient {
    config: ClientConfig,
    replay_file: PathBuf,
}

impl ReplayClient {
    /// Create a new `ReplayClient` instance reading and writing to the specified replay file.
    pub fn new<P: Into<PathBuf>>(replay_file: P) -> Self {
        ReplayClient {
            replay_file: replay_file.into(),
            config: ClientConfig::default(),
        }
    }

    /// Err(…)      → something went wrong.
    /// Ok(None)    → no data was stored yet, i.e. the file doesn't exist yet.
    /// Ok(Some(…)) → the actual data
    fn get_data(&self) -> Result<Option<ReplayData>, Error> {
        if !self.replay_file.exists() {
            Ok(None)
        } else {
            let file = File::open(&self.replay_file)?;
            Ok(::serde_json::from_reader(file)?)
        }
    }

    fn store_data(&self, data: &ReplayData) -> Result<(), Error> {
        let file = File::create(&self.replay_file)?;
        ::serde_json::to_writer(file, data)?;
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

        let data = self.get_data()?;
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
