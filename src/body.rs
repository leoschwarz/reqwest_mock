pub trait IntoBody: Into<Vec<u8>> {
    fn into_body(self) -> Vec<u8> {
        self.into()
    }
}
