# domain-crunch

Tool to generate subdomains for efficient domain hashes, used in [Seaport order attribution](https://docs.reservoir.tools/docs/calldata-attribution).

Uses 8 threads by default.

## Usage

`cargo run <domain> <1-4 byte hex pattern> [--suffix]`

## Example

```
> cargo run opensea.io 000000 --suffix
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/domain-crunch opensea.io 000000 --suffix`
Input: efficient_c907852efb567ad7.opensea.io
Keccak Hash: [70, 0, 0, 0]
```

```
> cargo run opensea.io 0x5ea0
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/domain-crunch opensea.io 0x5ea0`
Input: efficient_3d3f71d22cfd009d.opensea.io
Keccak Hash: [94, 160, 199, 67]
```
