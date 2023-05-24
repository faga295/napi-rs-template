#![deny(clippy::all)]
use napi_derive::napi;

#[napi]
pub fn template() -> String {
  String::from("Hello world!")
}
