# randcli

CLI as a Pseudo Random Number Generator

## How?

```bash
$ rand 'gauss()'
# => x ~ Gaussian(mu=0, vr=1)

$ rand 'gauss(1, 100)'
# => x ~ Gaussian(mu=1, vr=100)

$ rand 'seed(42) | gauss()'
# Gaussian(0, 1) with fixed seed=42

$ rand 'gauss() | int'
# Truncated into Int
```

## Functions

| Name    | Args                                 | Effect                    | Example                 |
|---------|--------------------------------------|---------------------------|-------------------------|
| seed    | number (unsigned int, required)      | Set Random Seed           | `seed(42)`              |
| int     | number (required)                    | Truncate a float into int | `int`                   |
| floor   | same to int                          |                           | `floor`                 |
| round   | trancate into nearst int             |                           | `round`                 |
| gauss   | mean (optional), variance (optional) | Gaussian Distribution     | `gauss()` `gauss(0, 1)` |
| uniform | min (optional), max (optional)       | Uniform from [min, max)   | `uniform(1, 2)`         |
| exp     | lambda (optional)                    | Exponential Distribution  | `exp(0.5)`              |

