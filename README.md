# accumulator-mmr

Merkle Mountain Range implementation for operator's commitment.
Generates and verifies MMR proof.

### Run program

```sh
cargo run MODE INPUT
```

1-1. MODE: proof -> generate proof

```sh
cargo run proof d773d9a4ac1c79d55063b454c2f05bd5a23826860451e6af4c10feee50883b0
```

or

```sh
sh ./scripts/proof.sh
```

1-2. expected output

```
{"order" : 6,
"mmr_size": 10,
 "proof" : [
    "66de524367429352c91f4a7ccb7fcf29f61d9d75021da7bfec2907f2428eddd1",
    "3b3601cc774112b3cb96308697e9b1190fd02b6b1a0114350b6384494c1d9d78",
],
 "hash" : "f896aae2e91995104bd4402a17a91a125feb835337a039c26989b53ab4811607"}
```

2-1. MODE : verify -> verify proof

```sh
cargo run verify ORDER PROOF_LENGTH PROOF.. ELEM_HASH
```

```sh
cargo run verify 6 2 66de524367429352c91f4a7ccb7fcf29f61d9d75021da7bfec2907f2428eddd1 3b3601cc774112b3cb96308697e9b1190fd02b6b1a0114350b6384494c1d9d78 f896aae2e91995104bd4402a17a91a125feb835337a039c26989b53ab4811607
```

or

```sh
sh ./scripts/verify.sh
```

2-2. expected output

```
Merkle proof verification : true

```

### References

https://github.com/nervosnetwork/merkle-mountain-range
