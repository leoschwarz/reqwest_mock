# reqwest_mock: Discontinued
Thank you for your interest in this project.
This project was a proof of concept to create a generic interface for the Rust reqwest crate that would allow users to mock and record and replay HTTP requests for testing purposes.

This project is no longer updated or maintained and this repository will be archived.

The following non-exhaustive list of alternative solutions are recommended instead to mock HTTP requests in Rust (as of 18.5.2023):

- https://github.com/chorusone/rvcr
- https://github.com/lukemathwalker/wiremock-rs
- https://github.com/beltram/stubr
- https://github.com/alexliesenfeld/httpmock
- https://github.com/lipanski/mockito

## Old readme
[Crates.io Link](https://crates.io/crates/reqwest_mock)

Provides a mockable reqwest-like HTTP client.

Write your code generic over the [Client](https://docs.rs/reqwest_mock/latest/reqwest_mock/client/trait.Client.html) trait,
and in production use [DirectClient](https://docs.rs/reqwest_mock/latest/reqwest_mock/client/struct.DirectClient.html) while in testing
you can use [ReplayClient](https://docs.rs/reqwest_mock/latest/reqwest_mock/client/struct.ReplayClient.html), which will record a request
the first time and replay it every time the exact same request is made in the
future.

