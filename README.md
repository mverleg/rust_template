
[<img src="https://travis-ci.org/mverleg/rust_template.svg?branch=master">](https://travis-ci.org/mverleg/rust_template)
[<img src="https://deps.rs/repo/github/mverleg/rust_template/status.svg">](https://deps.rs/repo/github/mverleg/rust_template)
[<img src="https://img.shields.io/badge/License-BSD%203--Clause-blue.svg">](https://opensource.org/licenses/BSD-3-Clause)

Rust template project
===============================

This is an example layout for a Rust project. It is focused towards

1) Having a Rust webserver, available through Kubernetes with accompanying services.
2) A reusable core of business logic, accessible from multiple platforms (WebAssembly, Android, iOS).

It's not ready yet, but may contain useful examples.

Checklist
-------------------------------

* `rustup component add clippy-preview`
* `rustup component add rustfmt`
* Update the name and other details in Cargo.toml.
* Change to another license than BSD if desired (LICENSE.txt, Cargo.toml).
* Possibly update dependencies.
* Replace this readme.
* Enable Travis CI for the repo.
* Update the badges at the top of this file and in Cargo.toml.
* Add date in ``RELEASES.md``

Troubleshooting
-------------------------------

* If you get an error like this while compiling::

      undefined reference to `cblas_sgemm'

  then you may need to [integrate](https://github.com/blas-lapack-rs/blas-lapack-rs.github.io/wiki) ``openblas``, or disable ``ndarray``-related dependencies (if you do not need them).
