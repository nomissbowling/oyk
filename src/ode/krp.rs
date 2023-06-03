//! krp
//!

use crate::ode::*;

/// krp
#[derive(Clone, Copy)]
pub struct Krp {
  /// kinetics (false: keep position)
  pub k: bool,
  /// gravity sensitive (false: call dBodyDisable())
  pub g: bool,
  /// collision
  pub c: bool,
  /// bounce
  pub bounce: dReal
}

impl Krp {
  /// construct
  pub fn new(k: bool, g: bool, c: bool, bounce: dReal) -> Krp {
    Krp{k: k, g: g, c: c, bounce: bounce}
  }
}

/// static krp n k
pub static KRPnk: Krp = Krp{k: false, g: false, c: false, bounce: 0.0001};
/// static krp 1.0
pub static KRP100: Krp = Krp{k: true, g: true, c: true, bounce: 1.0};
/// static krp 0.95
pub static KRP095: Krp = Krp{k: true, g: true, c: true, bounce: 0.95};
/// static krp 0.8
pub static KRP080: Krp = Krp{k: true, g: true, c: true, bounce: 0.8};
/// static krp 0.01
pub static KRP001: Krp = Krp{k: true, g: true, c: true, bounce: 0.01};
