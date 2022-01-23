use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
  path: &'buf str,
  query_string: Option<&'buf str>,
  method: Method,
}

// impl Request {
//   fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
  type Error = ParseError;

  fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
    // match str::from_utf8(buf) {
    //   Ok(request) => {}
    //   Err(_) => return Err(ParseError::InvalidEncoding),
    // }

    // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
    //   Ok(request) => {}
    //   Err(err) => return Err(err),
    // }

    let reqeust = str::from_utf8(buf)?;

    // match get_next_word(reqeust) {
    //   Some((method, request)) => {}
    //   None => return Err(ParseError::InvalidRequest),
    // }

    let (method, reqeust) = get_next_word(reqeust).ok_or(ParseError::InvalidRequest)?;
    let (mut path, reqeust) = get_next_word(reqeust).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = get_next_word(reqeust).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;

    let mut query_string = None;
    // match path.find('?') {
    //   Some(i) => {
    //     query_string = Some(&path[i + 1..]);
    //     path = &path[..i];
    //   }
    //   None => {}
    // }

    // let q = path.find('?');
    // if q.is_some() {
    //   let i = q.unwrap();
    //   query_string = Some(&path[i + 1..]);
    //   path = &path[..i];
    // }

    if let Some(i) = path.find('?') {
      query_string = Some(&path[i + 1..]);
      path = &path[..i];
    }
    Ok(Self {
      path,
      query_string,
      method,
    })
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  for (i, c) in request.chars().enumerate() {
    if c == ' ' || c == '\r' {
      return Some((&request[..i], &request[i + 1..]));
    }
  }
  None
}

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "InvalidRequest",
      Self::InvalidEncoding => "InvalidEncoding",
      Self::InvalidProtocol => "InvalidProtocol",
      Self::InvalidMethod => "InvalidMethod",
    }
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Error for ParseError {}
