[![Build Status](https://cloud.drone.io/api/badges/iancullinane/prisoner-rust/status.svg)](https://cloud.drone.io/iancullinane/prisoner-rust)
[![Crate](https://img.shields.io/crates/v/prisoner)](https://crates.io/crates/prisoner)
[![docs.rs](https://img.shields.io/docsrs/prisoner?color=blue)](https://docs.rs/prisoner)

prisoner
========

A library for simulating "[The Prisoner's Dilemna](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma)". Also comes with a `main.rs` to play with. 

Provides enums for `Choice` and `Outcome`, these are all you really need to play the game. The public function `determine` will give you a result. The `Outcome` enum also provides methods for calculating a score. It supports the classic reward mechanism (0, -1, -2, -3) reward based scoring system, and an algebraic return (ie `T > R > P > S`). Currently does not implement `Ord`.

## Usage

`main.rs` shows how to use `clap` generate a CLI. This is also the entry point for the docker container.

```
prisoner 0.5.0
Ian Cullinane <ian@iancullinane.com>
A library for simulating "The Prisoner's Dilemna"

USAGE:
    prisoner [OPTIONS] --players <PLAYERS>

OPTIONS:
    -h, --help                 Print help information
    -p, --players <PLAYERS>    
    -r, --rounds <ROUNDS>      
    -V, --version              Print version information
```
