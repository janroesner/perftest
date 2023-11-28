# Simple Interpreter Performance Test

Implements the Sieve of Eratosthenes for prime number calculation.

# Test Execution

## Node.js

```
  time node prime.js 5000000 20
```

## Python

```
  time python prime.py 5000000 20
```

## Ruby

```
  time ruby prime.rb 5000000 20
```

## Rust

First build a release candidate via:

```
  cd prime.rs && cargo build --release
```

Then run the executable:

```
  time ./prime 5000000 20
```
