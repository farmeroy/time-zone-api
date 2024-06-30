# Time Zone Lambda Function

This is a serverless function built with the AWS Lambda sdk for Rust.

The function is called with query parameters for the lon and lat of a location and the time zone is returned.

It uses the `tzf-rs` crate to calculate the timezone. 

To run locally, you have to install the `cargo lambda` crate, clone this repo and run `cargo lambda watch`
