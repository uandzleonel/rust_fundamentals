use super::Method;

#[derive(Debug)]
pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method
}
