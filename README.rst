
.. image:: https://github.com/mverleg/rust_template/workflows/Works/badge.svg
    :target: https://github.com/mverleg/rust_template/blob/master/.github/workflows/works.yml

.. image:: https://github.com/mverleg/rust_template/workflows/Polished/badge.svg
    :target: https://github.com/mverleg/rust_template/blob/master/.github/workflows/polished.yml

.. image:: https://github.com/mverleg/rust_template/workflows/Platforms/badge.svg
    :target: https://github.com/mverleg/rust_template/blob/master/.github/workflows/platforms.yml

.. image:: https://github.com/mverleg/rust_template/workflows/Dependencies/badge.svg
    :target: https://github.com/mverleg/rust_template/blob/master/.github/workflows/dependencies.yml

.. image:: https://img.shields.io/badge/License-BSD%203--Clause-blue.svg
    :target: https://opensource.org/licenses/BSD-3-Clause

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
* Add date in ``RELEASES.rst``

Troubleshooting
-------------------------------

* Blas error::

      undefined reference to `cblas_sgemm'

  You may need to integrate_ ``openblas``, or disable ``ndarray``-related dependencies (if you do not need them).

* gfortran error::

      = note: /usr/bin/ld: cannot find -lgfortran
          collect2: error: ld returned 1 exit status

  You may need to install ``gfortran``, or disable ``ndarray``-related dependencies (if you do not need them).


.. _integrate: https://github.com/blas-lapack-rs/blas-lapack-rs.github.io/wiki
