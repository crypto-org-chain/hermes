- Verify ML-DSA-65 validator keys and signatures in light client headers, using a
  tendermint-rs v0.40.4 backport of cometbft/tendermint-rs#1533. Client updates for
  ML-DSA-65 validator sets are about 7 KB per validator, so raise `max_tx_size` and
  `max_gas` for those chains.
