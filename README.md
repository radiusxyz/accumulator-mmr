# operator-mmr

Merkle Mountain Range Implementation for Operator Commitment

### Run program

```sh
cargo run MODE INPUT
```

1-1. generate proof

```sh
cargo run proof d773d9a4ac1c79d55063b454c2f05bd5a23826860451e6af4c10feee50883b0
```

1-2. expected output

```
{"order" : 2, "proof" : Ok(
    MerkleProof {
        mmr_size: 3,
        proof: [
            StringHash(
                b"\xf5\t\xb5[j\x01\xc6Q4n\xf2\xd8\x1d\x01\xb6\t\xec\x85\xed;\x07\xe4\x18du^\x1b\x0fx\xdd\xc3?",
            ),
        ],
        merge: PhantomData,
    },
)}
```

### References

https://github.com/nervosnetwork/merkle-mountain-range
