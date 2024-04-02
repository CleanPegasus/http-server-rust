use std::str::FromStr;
pub enum Method {
  GET,
  POST,
  PUT,
  DELETE,
  HEAD,
  CONNECT,
  OPTION,
  TRACE,
  PATCH
}

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "DELETE" => Ok(Self::DELETE),
      "HEAD" => Ok(Self::HEAD),
      "CONNECT" => Ok(Self::CONNECT),
      "OPTION" => Ok(Self::OPTION),
      "TRACE" => Ok(Self::TRACE),
      "PATCH" => Ok(Self::PATCH),
      _ => Err(MethodError),
    }
  }
}

pub struct MethodError;