oyk
===

OYK is ODE (Open Dynamics Engine) bindings for Rust yaw kinetics


[oyk_512x100]: https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x100.png?raw=true
![ODE][oyk_512x100]
 * https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x100.png?raw=true

Now this crate is tested on ode-0.16.2 dll version.

ode.dll drawstuff.dll for x64 Windows binary compiled with -DdDOUBLE by mingw

(It may work with VC, or other platforms.)


Requirements
------------

- [ https://github.com/nomissbowling/asciiz ]( https://github.com/nomissbowling/asciiz )
- [ ode and drawstuff ]( https://ode.org/ )

to build dll

- premake4 --with-demos --only-double --with-libccd --cc=gcc --platform--x64 --os=windows codeblocks
- premake4 --with-demos --only-double --with-libccd --platform--x64 --os=windows vs2010

in the running directory

- drawstuff.dll
- ode.dll
- libstdc++-6.dll
- libgcc_s_seh-1.dll
- libwinpthread-1.dll


Samples
-------

```rust
use oyk::ode::*;

use impl_sim::{impl_sim_fn, impl_sim_derive};

pub struct SimApp {
  cnt: usize
}

impl SimApp {

pub fn objs_info(&mut self, f: bool, s: &str) {
  let rode = self.super_get();
  let obgs = &rode.obgs;
  let l = obgs.len();
  if f || (l != self.cnt) {
    println!("obgs: {} in {}", self.cnt, s);
    for (k, v) in &rode.mbgs {
      println!("{}: {:?}", k, obgs[*v].col);
      // println!("{}: {:?}", k, rode.find(k.to_string()).col); // same
    }
    self.cnt = l;
  }
}

}

#[impl_sim_derive(near_callback, stop_callback)]
impl Sim for SimApp {

fn draw_objects(&mut self) {
  self.objs_info(false, "draw"); // twice (after step)
  self.super_mut().draw_objects();
}

fn start_callback(&mut self) {
  let rode = self.super_mut();
  let t_delta = &mut rode.t_delta;
  *t_delta = 0.002;
  let m: dReal = 1.0;
  let r: dReal = 0.2;
  for i in 0..16 {
    let c: dVector4 = vec4_from_u32(COLORS[i]);
    let p: dVector3 = [(i%4) as dReal - 1.5, (i/4) as dReal - 1.5, 2.0, 1.0];
    rode.mk_sphere(format!("ball_{:08X}", i), m, r, &c, &p);
  }
  let c: dVector4 = [1.0, 1.0, 0.0, 0.8];
  let p: dVector3 = [0.0, 0.0, 10.0, 1.0];
  rode.mk_sphere("ball_big".to_string(), 0.1, 1.0, &c, &p);
  rode.start_callback();
}

fn step_callback(&mut self, pause: i32) {
  self.objs_info(false, "step"); // twice (before draw)
  self.super_mut().step_callback(pause);
}

fn command_callback(&mut self, cmd: i32) {
  match cmd as u8 as char {
    'o' => {
      println!("{:?}", self.super_mut().find_mut("ball".to_string()).col);
    },
    'a' => {
      self.objs_info(true, "cmd");
    },
    _ => {}
  }
  self.super_mut().command_callback(cmd);
}

} // impl Sim for SimApp

fn main() {
  ODE::open();
  ODE::sim_loop(
    640, 480, // 800, 600,
    Some(Box::new(SimApp{cnt: 0})),
    b"./resources");
  ODE::close();
}
```


see also

- [ https://github.com/nomissbowling/ode-rs-0000 ]( https://github.com/nomissbowling/ode-rs-0000 )


License
-------

MIT License

