oyk
===

CAUTION
-------

oyk is replaced to submodule of crate ode-rs after version 1.0.1
----------------------------------------------------------------

OYK is ODE (Open Dynamics Engine) bindings for Rust yaw kinetics


[oyk_512x424]: https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x424.png?raw=true
![ODE][oyk_512x424]
 * https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x424.png?raw=true

[oyk_512x100]: https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x100.png?raw=true
![ODE][oyk_512x100]
 * https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x100.png?raw=true

Now this crate is tested on ode-0.16.2 dll version.

ode.dll for x64 Windows binary compiled with -DdDOUBLE by mingw

(It may work with VC, or other platforms.)


Links
-----

- [https://crates.io/crates/ode-rs](https://crates.io/crates/ode-rs)
- [https://github.com/nomissbowling/ode-rs](https://github.com/nomissbowling/ode-rs)


Requirements
------------

- [ode](https://ode.org/)
- [https://crates.io/crates/ode-base](https://crates.io/crates/ode-base)
- [https://github.com/nomissbowling/ode-base](https://github.com/nomissbowling/ode-base)
- [https://crates.io/crates/asciiz](https://crates.io/crates/asciiz)
- [https://github.com/nomissbowling/asciiz](https://github.com/nomissbowling/asciiz)

to build dll

- premake4 --with-demos --only-double --with-libccd --cc=gcc --platform--x64 --os=windows codeblocks
- premake4 --with-demos --only-double --with-libccd --platform--x64 --os=windows vs2010

in the running directory

- ode.dll
- libstdc++-6.dll
- libgcc_s_seh-1.dll
- libwinpthread-1.dll


Exsamples
---------

- [https://crates.io/crates/ode-rs-0000](https://crates.io/crates/ode-rs-0000)
- [https://github.com/nomissbowling/ode-rs-0000](https://github.com/nomissbowling/ode-rs-0000)


License
-------

MIT License
