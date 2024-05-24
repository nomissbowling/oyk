#![doc(html_root_url = "https://docs.rs/oyk/1.2.1")]
//! OYK is ODE (Open Dynamics Engine) bindings for Rust yaw kinetics
//!
//! oyk is replaced to submodule of crate ode-rs after version 1.0.1
//!
//! # Requirements
//!
//! - [ode](https://ode.org/)
//! - [ode-base for Rust crates.io](https://crates.io/crates/ode-base)
//! - [ode-base for Rust rep.](https://github.com/nomissbowling/ode-base)
//! - [https://crates.io/crates/asciiz](https://crates.io/crates/asciiz)
//! - [https://github.com/nomissbowling/asciiz](https://github.com/nomissbowling/asciiz)
//!
//! to build dll
//!
//! - premake4 --with-demos --only-double --with-libccd --cc=gcc --platform--x64 --os=windows codeblocks
//! - premake4 --with-demos --only-double --with-libccd --platform--x64 --os=windows vs2010
//!
//! in the running directory
//!
//! - ode.dll
//! - libstdc++-6.dll
//! - libgcc_s_seh-1.dll
//! - libwinpthread-1.dll
//!
//! # Examples
//!
//! see also
//!
//! - [https://crates.io/crates/ode-rs-0000](https://crates.io/crates/ode-rs-0000)
//! - [https://github.com/nomissbowling/ode-rs-0000](https://github.com/nomissbowling/ode-rs-0000)
//!

pub mod ode;
pub mod colors;

#[cfg(test)]
mod tests {
  use super::colors::*;
/*
  use super::ode::*;

  #[test]
  fn check_quaternion_matrix() {
    let q = dQuaternion::from_axis_and_angle([1.0, 0.0, 0.0], PIh);
    let p = dQuaternion::from_axis_and_angle([0.0, 1.0, 0.0], PIh);
    let o = dQuaternion::from_axis_and_angle([0.0, 0.0, 1.0], PIh);
    println!("q, p, o");
    println!("{}", q.as_vec());
    println!("{}", p.as_vec());
    println!("{}", o.as_vec());
    assert!(q.prec_eq(1e-15, [PIq.cos(), PIq.sin(), 0.0, 0.0]));
    assert!(p.prec_eq(1e-15, [PIq.cos(), 0.0, PIq.sin(), 0.0]));
    assert!(o.prec_eq(1e-15, [PIq.cos(), 0.0, 0.0, PIq.sin()]));

    let oq = dQuaternion::multiply0(o, q);
    let po = dQuaternion::multiply0(p, o);
    let pq = dQuaternion::multiply0(p, q);
    println!("oq, po, pq");
    println!("{}", oq.as_vec());
    println!("{}", po.as_vec());
    println!("{}", pq.as_vec());
    assert!(oq.prec_eq(1e-15, [0.5, 0.5, 0.5, 0.5])); // j
    assert!(po.prec_eq(1e-15, [0.5, 0.5, 0.5, 0.5])); // i
    assert!(pq.prec_eq(1e-15, [0.5, 0.5, 0.5, -0.5])); // k
    assert!(oq.prec_eq(1e-15, po));

    let nq = dMatrix4::from_P(q);
    let np = dMatrix4::from_P(p);
    let no = dMatrix4::from_P(o);
    println!("nq, np, no");
    println!("{}", nq.as_mat());
    println!("{}", np.as_mat());
    println!("{}", no.as_mat());
    assert!(dQuaternion::multiply0_441(nq, o).prec_eq(1e-15, oq));
    assert!(dQuaternion::multiply0_441(no, p).prec_eq(1e-15, po));
    assert!(dQuaternion::multiply0_441(nq, p).prec_eq(1e-15, pq));

    let nqno = dMatrix4::multiply0_444(nq, no);
    let nonp = dMatrix4::multiply0_444(no, np);
    let nqnp = dMatrix4::multiply0_444(nq, np);
    println!("nqno, nonp, nqnp");
    println!("{}", nqno.as_mat());
    println!("{}", nonp.as_mat());
    println!("{}", nqnp.as_mat());
    assert!(nqno.is_quaternion());
    assert!(nqno.to_Q().prec_eq(1e-15, oq));
    assert!(nonp.is_quaternion());
    assert!(nonp.to_Q().prec_eq(1e-15, po));
    assert!(nqnp.is_quaternion());
    assert!(nqnp.to_Q().prec_eq(1e-15, pq));
    assert!(nqno.prec_eq(1e-15, nonp));

    let mq = dMatrix4::from_Q(q);
    let mp = dMatrix4::from_Q(p);
    let mo = dMatrix4::from_Q(o);
    println!("mq, mp, mo");
    println!("{}", mq.as_mat());
    println!("{}", mp.as_mat());
    println!("{}", mo.as_mat());
    assert!(dQuaternion::multiply0_441(mo, q).prec_eq(1e-15, oq));
    assert!(dQuaternion::multiply0_441(mp, o).prec_eq(1e-15, po));
    assert!(dQuaternion::multiply0_441(mp, q).prec_eq(1e-15, pq));

    let momq = dMatrix4::multiply0_444(mo, mq);
    let mpmo = dMatrix4::multiply0_444(mp, mo);
    let mpmq = dMatrix4::multiply0_444(mp, mq);
    println!("momq, mpmo, mpmq");
    println!("{}", momq.as_mat());
    println!("{}", mpmo.as_mat());
    println!("{}", mpmq.as_mat());
    assert!(momq.is_quaternion());
    assert!(momq.to_Q().prec_eq(1e-15, oq));
    assert!(mpmo.is_quaternion());
    assert!(mpmo.to_Q().prec_eq(1e-15, po));
    assert!(mpmq.is_quaternion());
    assert!(mpmq.to_Q().prec_eq(1e-15, pq));
    assert!(momq.prec_eq(1e-15, mpmo));
  }
*/

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_ode() {
    assert_eq!(COLORS[0], 0xcccccccc);
    assert_eq!(vec4_from_u32(COLORS[COLORS.len() - 1]), [0.2, 0.2, 0.2, 0.8]);
  }
}
