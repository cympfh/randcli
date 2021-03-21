# randcli

CLI as a Pseudo Random Number Generator

## How?

```bash
$ rand 'gauss()'
0.7164763224859116  # ~ Gaussian(mu=0, vr=1)

$ rand 'gauss(1, 100)'
3.6845312036504216  # ~ Gaussian(mu=1, vr=100)

$ rand 'seed(42) | gauss(0, 100)'
8.327121583181412  # ~ Gaussian(0, 1) with fixed seed=42

$ rand 'seed(42) | gauss(0, 100) | int'
8  # Truncated into Int
```

## Functions

| Name    | Args                                 | Effect                    | Example                 |
|---------|--------------------------------------|---------------------------|-------------------------|
| seed    | number (unsigned int, required)      | Set Random Seed           | `seed(42)`              |
| int     | number (required)                    | Truncate a float into int | `int`                   |
| floor   |                                      | same to int               | `floor`                 |
| round   |                                      | trancate into nearst int  | `round`                 |
| gauss   | mean (optional), variance (optional) | Gaussian Distribution     | `gauss()` `gauss(0, 1)` |
| uniform | min (optional), max (optional)       | Uniform from [min, max)   | `uniform(1, 2)`         |
| exp     | lambda (optional)                    | Exponential Distribution  | `exp(0.5)`              |

