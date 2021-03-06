
<p align="center">
    <a href="https://crates.io/crates/randcli"><img src="https://img.shields.io/crates/v/randcli.svg" alt="crates.io" /></a>
    <a href="https://github.com/cympfh/randcli/actions"><img src="https://github.com/cympfh/randcli/workflows/Rust/badge.svg" /></a>
    <a href="https://github.com/cympfh/randcli/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/randcli.svg" /></a>
</p>

# randcli

CLI as a Pseudo Random Number Generator

## How?

```bash
$ rand 'gauss'
0.7164763224859116  # ~ Gaussian(mu=0, vr=1)
# 'gauss' is same to 'gauss()' or 'gauss(0, 1)'

$ rand 'gauss(1, 100)'
3.6845312036504216  # ~ Gaussian(mu=1, vr=100)

$ rand 'seed(42) | gauss(0, 100)'
8.327121583181412  # ~ Gaussian(0, 1) with fixed seed=42

$ rand 'seed(42) | gauss(0, 100) | int'
8  # Truncated into Int
```

## Functions

| Category     | Name      | Effect                                 | Type                                   | Default Args | Example                 |
|--------------|-----------|----------------------------------------|----------------------------------------|--------------|-------------------------|
| Seed         | seed      | Set Random Seed                        | unsigned int -> () -> ()               |              | `seed(42)`              |
| Truncate     | int       | Truncate a float into int              | () -> number -> int                    |              | `int`                   |
| ..           | floor     | same to `int`                          | () -> number -> int                    |              | `floor`                 |
| ..           | round     | trancate into nearst int               | () -> number -> int                    |              | `round`                 |
| Distribution | gauss     | Gaussian Distribution (mean, variance) | (number, number) -> () -> number       | (0, 1)       | `gauss()` `gauss(0, 1)` |
| ..           | uniform   | Uniform Distribution of [min, max)     | (number, number) -> () -> number       | (0, 1)       | `uniform(1, 2)`         |
| ..           | exp       | Exponential Distribution (lambda)      | number -> () -> number                 | 1            | `exp(0.5)`              |
| ..           | binom     | Binomial Distribution (n, p)           | (unsigned int, number) -> () -> number |              | `binom(10, 0.5)`        |
| ..           | bernoulli | Alias to `binom(1, p)`                 | number -> () -> number                 | 0.5          | `bernoulli(0.5)`        |

Type Notation forms `X -> Y -> Z`, which means

1. It requires `X` data as arguments
    - E.g. `42` for `seed(42)`
2. And then catches `Y` data from the previous pipe `|`
    - E.g. `int` function reads a number data from the previous functions
3. Lastly outputs `Z` data
    - E.g. Distribution functions will output a number data

Type `()` is None data.

## Install

```bash
cargo install randcli
```
