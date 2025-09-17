#!/bin/bash

PKG_NAME=tcp-rust
./target/release/$PKG_NAME& 
pid=$!
make ip
trap "kill $pid" INT TERM
wait $pid
