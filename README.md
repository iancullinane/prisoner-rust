[![Build Status](https://cloud.drone.io/api/badges/iancullinane/prisoner-rust/status.svg)](https://cloud.drone.io/iancullinane/prisoner-rust)

prisoner
========

An implementation. Planned MVC should provide an application as well as library. This is the intent of the `Cargo.toml` definitions. 

Current features/limitations
* Play single rounds or round robin tournaments
* Provides a trait for Players, as well as a concrete Entity base type
* Currently implements three base peronalities (AlwaysCheat, AlwaysCooperate, and Copycat)
* Provides table rendered output for single rounds and end of game results
* Does not implement classic `T > R > P > S` return
  * Currently returns a "reward" based version [link](https://github.com/iancullinane/prisoner-rust/blob/master/src/lib.rs#L13-L19)
    * This is purely to provide positive numbers because in a game that is better
    * Current dev is around providing a more generic `Outcome` logic

## Usage

```
prisoner 0.1.0

USAGE:
    prisoner [OPTIONS] --players <PLAYERS>

OPTIONS:
    -h, --help                 Print help information
    -p, --players <PLAYERS>    
    -r, --rounds <ROUNDS>      
    -V, --version              Print version information
```
