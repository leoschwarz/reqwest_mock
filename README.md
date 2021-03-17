# reqwest_mock
[![crates.io](http://meritbadge.herokuapp.com/reqwest_mock)](https://crates.io/crates/reqwest_mock)
[![Docs](https://docs.rs/reqwest_mock/badge.svg)](https://docs.rs/reqwest_mock/)
[![Build Status](https://travis-ci.org/leoschwarz/reqwest_mock.svg?branch=master)](https://travis-ci.org/leoschwarz/reqwest_mock)

Provides a mockable [reqwest][]-like HTTP client.

Write your code generic over the [Client](https://docs.rs/reqwest_mock/latest/reqwest_mock/client/trait.Client.html) trait,
and in production use [DirectClient](https://docs.rs/reqwest_mock/latest/reqwest_mock/client/struct.DirectClient.html) while in testing
you can use [ReplayClient](https://docs.rs/reqwest_mock/latest/reqwest_mock/client/struct.ReplayClient.html), which will record a request
the first time and replay it every time the exact same request is made in the
future.

