use std::io;

// TODO: Hide what is not needed.
// TODO: impl Error

// TODO: Should be crate or module visible.
pub struct FieldError {
    /// The name of the missing field.
    pub field_name: &'static str,
    /// The strictness level, which implied this field being required.
    pub strictness: &'static str,
}

// TODO: Variants should be private.
pub enum RegisterStubError {
    // "Tried registering stub without `{}` even though `{}` requires its presence."
    MissingField(FieldError),

    // "Tried registering stub with `{}` in the request, even though `{}` means you don't want to check it in requests. Please remove the field or set a higher `StubStrictness`."
    UnescessaryField(FieldError),
    ReadFile(io::Error),
}
