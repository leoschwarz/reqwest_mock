//! Defines the `Error` type we use in this library (error-chain).

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    links {
    }

    foreign_links {
        Io(::std::io::Error);
        Reqwest(::reqwest::Error);
        SerdeJson(::serde_json::Error);
        FromUtf8(::std::string::FromUtf8Error);
    }

    errors {
    }
}
