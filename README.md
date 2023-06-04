# Rust example of retrieving info from http api

## Background

* Robotmower - opensource project uses YardForce robot mower as base platform.
* The YardForce website is very bad/broken thus try to extract the info directly from the API to find counties that sell the unit.

## Run

1. Install rust
2. Checkout git repo
3. cargo run

## Notes

* 2023-06 use rust serde to extract info, runtime 42seconds
  * async step1 countries -> runtime 42sec -> 9.1sec  x5 improvement repeat down to 5.3s (API cache?)
  * All async(tokyo) now get all api info in 4.5s  :) 10x improvement by using async.

## Docker container

* Tried to build bare rust container, was blocked with issues building rust with musl clib and openssl
* Build libc image and and use google distroless/cc-debian11 to generate 30MB docker with rust exe.
