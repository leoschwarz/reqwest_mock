error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    links {
    }

    foreign_links {
        Io(::std::io::Error);
        Reqwest(::reqwest::Error);
    }

    errors {
    }
}
