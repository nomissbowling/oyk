//! err
//!

use crate::ode::*;

use std::fmt;
use std::error::Error;

/// for Result&lt;_, Box&lt;dyn Error&gt;&gt; handling
#[derive(Debug)]
pub struct ODEError {
  msg: String
}

/// for Result&lt;_, Box&lt;dyn Error&gt;&gt; handling
impl ODEError {
  /// construct
  pub fn no_key(k: String) -> ODEError {
    ODEError{msg: format!("no key [{}] in mbgs", k)}
  }

  /// construct
  pub fn no_id(id: dBodyID) -> ODEError {
    ODEError{msg: format!("no id {:018p} in obgs", id)}
  }
}

/// formatter
impl fmt::Display for ODEError {
  /// formatter
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ODEError: {}", self.msg)
  }
}

/// &lt;dyn Error&gt; handler
impl Error for ODEError {
  /// &lt;dyn Error&gt; handler
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(self)
  }
}
