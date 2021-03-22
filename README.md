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

| Category     | Name    | Effect                                 | Type                             | Default Args | Example                 |
|--------------|---------|----------------------------------------|----------------------------------|--------------|-------------------------|
| Seed         | seed    | Set Random Seed                        | unsigned int -> 0                |              | `seed(42)`              |
| Truncate     | int     | Truncate a float into int              | -> int                           |              | `int`                   |
| ..           | floor   | same to `int`                          | -> int                           |              | `floor`                 |
| ..           | round   | trancate into nearst int               | -> int                           |              | `round`                 |
| Distribution | gauss   | Gaussian Distribution (mean, variance) | (number, number) -> number       | (0, 1)       | `gauss()` `gauss(0, 1)` |
| ..           | uniform | Uniform Distribution of [min, max)     | (number, number) -> number       | (0, 1)       | `uniform(1, 2)`         |
| ..           | exp     | Exponential Distribution (lambda)      | number -> number                 | 1            | `exp(0.5)`              |
| ..           | binom   | Binomial Distribution (n, p)           | (unsigned int, number) -> number |              | `binom(10, 0.5)`        |

## Install

```bash
cargo install randcli
```
