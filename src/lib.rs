#![doc(html_root_url = "https://docs.rs/oyk/0.6.6")]
//! OYK is ODE (Open Dynamics Engine) bindings for Rust yaw kinetics
//!
//! # Requirements
//!
//! - [ https://github.com/nomissbowling/asciiz ]( https://github.com/nomissbowling/asciiz )
//! - [ ode and drawstuff ]( https://ode.org/ )
//!
//! to build dll
//!
//! - premake4 --with-demos --only-double --with-libccd --cc=gcc --platform--x64 --os=windows codeblocks
//! - premake4 --with-demos --only-double --with-libccd --platform--x64 --os=windows vs2010
//!
//! in the running directory
//!
//! - drawstuff.dll
//! - ode.dll
//! - libstdc++-6.dll
//! - libgcc_s_seh-1.dll
//! - libwinpthread-1.dll
//!
//! # Examples
//!
//! see also
//!
//! - [ https://github.com/nomissbowling/ode-rs-0000 ]( https://github.com/nomissbowling/ode-rs-0000 )
//!

pub mod ode;
pub mod colors;
