#!/bin/bash
cargo run "$1"
cc -o tmp tmp.s
./tmp
echo $?