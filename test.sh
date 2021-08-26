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

assert 0 "0;"
assert 42 "42;"

assert 2 "1+1;"
assert 10 "5+2+3;"
assert 50 "10 + 25 - 5 + 20;"

assert 47 '5+6*7;'
assert 15 '5*(9-6);'
assert 4 '(3+5)/2;'

assert 10 "^-10+20;"
assert 1 "+5-4;"

assert 0 '0==1;'
assert 1 '42==42;'
assert 1 '0!=1;'
assert 0 '42!=42;'

assert 1 '0<1;'
assert 0 '1<1;'
assert 0 '2<1;'
assert 1 '0<=1;'
assert 1 '1<=1;'
assert 0 '2<=1;'

assert 1 '1>0;'
assert 0 '1>1;'
assert 0 '1>2;'
assert 1 '1>=0;'
assert 1 '1>=1;'
assert 0 '1>=2;'

assert 5 "a=5;"
assert 10 "a=7;b=3;a+b;"

assert 1 "return 1;"
assert 1 "return_x = 1;"
assert 3 "return_x = 1; return_y = 2; return return_x+return_y;"

assert 10 "if (1<5) 10;"
assert 20 "if (1>2) 10; else 20;"
assert 10 "if (1<2) if (1>3) 20; else 10; else 30;"

echo OK