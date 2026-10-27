# Tausen

The Solana program behind [Tausen](https://tausen.fun). It holds traders' USDC and writes every explorer entry on chain.

Program ID: `8YSZ78x9iPVRCMhz1vYTSkF4Ka1Jkv6K9349ypFijLc9`

## Instructions

| Instruction | Signer | Effect |
|---|---|---|
| `initialize` | upgrade authority | Creates the config and the USDC vault, once |
| `deposit` | trader | Moves USDC into the vault |
| `withdraw` | operator | Pays a withdrawal from the vault; a receipt stops it being paid twice |
| `record` | operator | Writes one explorer entry and extends the journal hash |
| `set_operator`, `set_paused`, `propose_admin` | admin | Settings |
| `accept_admin` | proposed admin | Takes the admin role |

USDC leaves the vault only through `withdraw`.

## Journal

Every explorer entry extends a hash chain kept in the config, starting from 32 zero bytes:

```
chain = SHA-256(previous ‖ seq ‖ kind ‖ wallet ‖ token ‖ amount ‖ data_hash ‖ at)
```

Replaying the explorer's entries in order must arrive at `Config.journal_hash`.
