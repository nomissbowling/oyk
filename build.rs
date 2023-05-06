/*
  build.rs for oyk

  (with C/C++ Bridge)

  cc-rs bindgen
  and generate link option
*/

extern crate cc;
extern crate bindgen;

use std::path::PathBuf;

fn main() {
  let mk_cc = |dname: &str, sname: &str, iname: &str, oname: &str| {
    let sd = PathBuf::from(dname);
    let fname = format!("{}", sd.join(sname).to_str().expect("invalid path"));
    println!("cargo:rerun-if-changed={}", fname);
    cc::Build::new()
      .file(fname)
      // .flag("-std=c++11") // gcc
      .flag("-std:c11") // cl
      // .flag("-std:c++14") // cl
      .include(iname)
      .compile(oname)
  };
  let od = if cfg!(docsrs) { std::env::var("OUT_DIR").unwrap() }else{ ".".to_string() };
  if od == "." {
    mk_cc("./src", "bridge.cpp", "./include", "bridge");
  }
  let ipath = if od != "." { od.as_str() }else{ "./include" };
  let opath = if od != "." { od.as_str() }else{ "./ode" };

  let mk_bindings = |hdd: &str, header: &str, rsd: &str, rsfile: &str| {
    let hd = PathBuf::from(hdd);
    let hf = format!("{}", hd.join(header).to_str().expect("invalid path"));
    println!("cargo:rerun-if-changed={}", hf);
    let bindings = bindgen::Builder::default()
      .header(hf)
      .parse_callbacks(Box::new(bindgen::CargoCallbacks))
      .generate()
      .expect("Unable to generate bindings");
    let rs = PathBuf::from(rsd);
    bindings
      .write_to_file(rs.join(rsfile))
      .expect("Could not write bindings!");
  };
  mk_bindings("./include", "bridge.hpp", ipath, "bridge_bindings.rs");
  mk_bindings("./ode", "drawstuff.h", opath, "drawstuff_bindings.rs");
  mk_bindings("./ode", "ode.hpp", opath, "ode_bindings.rs");

  println!("cargo:rustc-link-search=./ode/lib");
  println!("cargo:rustc-link-lib=drawstuff");
  println!("cargo:rustc-link-lib=ode");
}
