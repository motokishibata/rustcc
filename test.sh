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
assert 50 "10 + 25 - 5 + 20"

assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'

assert 10 "^-10+20"
assert 1 "+5-4"

echo OK