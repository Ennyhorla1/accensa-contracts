# Deployments

Every Accensa contract deployment is recorded here with its contract ID and the
transaction that created it, so anyone can verify the deployment independently.

## Verify this yourself

To reproduce the byte-identical WASM files, use the exact environment provided by the Stellar SDK:

```bash
docker run --rm -v "$PWD":/workspace -w /workspace stellar/soroban-tools:latest \
  stellar contract build
sha256sum target/wasm32v1-none/release/*.wasm
```

## Testnet

| Contract | Contract ID | WASM Hash |
|---|---|---|
| `ReceiptAnchor` | `CBHRJU7CF4XIFRNDITFHNQHABKBMFM2FYFHLGWN3JGSFYYCDSMDAWPRV` | `f5dc42e6c2821607de6e35ed6e37d49623415e7221a77a290e853970f1a6c7b7` |
| `RefundVault` | `CCMBM44EJUGD52G4LSMGHSXMAH2KSAQZX7VOYY4TTBF5BK4D7M4IHRQA` | `f23f90605090e560d503e6c5d597ae5dd2642848a05b5aa67d1f8f87ec6847c9` |

- **Version:** `0.1.0`
- **Commit SHA:** `d6e0cbfa74f4eb6a55cd9333e4dfe828bd94089d`