# raccoon [![Build Status](https://travis-ci.com/jakobbeckmann/raccoon.svg?token=6RwG9cGf5RW9JwThwdpc&branch=master)](https://travis-ci.com/jakobbeckmann/raccoon) [![Version Badge](http://meritbadge.herokuapp.com/raccoon)](https://crates.io/crates/raccoon)
A library providing similar capabilities to `pandas`'s `DataFrame` in python. The intent of this library is to facilitate the handling of large amounts of data.

## Documentation
Can be found at [https://docs.rs/raccoon/0.0.1/raccoon/](https://docs.rs/raccoon/0.0.1/raccoon/).

## Roadmap
I wish to implement the following features:

- [x] a cell-like entry that is data independent.
- [x] a `Series` object that contains a list of cells of the same type.
- [ ] a `DataFrame` object containing a list of `Series`.
- [ ] `DataFrame` loading from csv and excel files.
- [ ] operations on `DataFrame`s similar to the supported operations in `pandas`.
- [ ] full and clear documentation of the entire crate.
- [x] push the crate to [crates.io](https://crates.io/).

## Contributors
- [jakobbeckmann](https://github.com/jakobbeckmann) - Creator and maintainer.
