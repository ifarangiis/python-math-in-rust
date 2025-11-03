# pymath

A Rust implementation of Python’s `math` library — **bit-for-bit compatible** with CPython.

---

Challenge:
Create a number crunching (Math tools, statistics, algorithms) project with the professional builder (500 lines max) limit while following the one-loop warrior constraint.

![challenge](/challenge.png)

---

## 🧩 Overview

`pymath` is a direct, line-by-line port of CPython’s `math` module from C to Rust.
Every function is designed to produce **identical binary results** to Python’s standard math library — ensuring complete numerical compatibility across both languages.

---

## ⚙️ Implementation

Each function has been carefully rewritten from CPython’s original C source:

* Uses the **same algorithms** and **mathematical constants**
* Reproduces all **corner cases** and **rounding behavior**
* Preserves the exact **numerical properties** of Python’s `math`

The result: Python-accurate floating-point math, implemented safely in Rust.

---

## 🚀 Usage

```rust
use pymath::{gamma, lgamma};

fn main() {
    // Produces the exact same results as Python's math.gamma and math.lgamma
    let gamma_result = gamma(4.5).unwrap();
    let lgamma_result = lgamma(4.5).unwrap();

    println!("gamma(4.5) = {}", gamma_result);
    println!("lgamma(4.5) = {}", lgamma_result);
}
```
