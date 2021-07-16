#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    cargo run "$input"
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
      echo "$input => $actual"
    else
      echo "$input => $expected expected, but got $actual"
      exit 1
    fi
}

assert 0 0
assert 42 42

assert 2 1+1
assert 10 5+2+3
assert 50 10 + 25 - 5 + 20

echo OK