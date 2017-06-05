use config::ClientConfig;
use error::Error;
use request::Request;
use response::Response;

pub trait Client {
    fn execute(&self, config: &ClientConfig, request: Request) -> Result<Response, Error>;
}

mod direct;
pub use self::direct::DirectClient;

mod replay;
pub use self::replay::ReplayClient;
