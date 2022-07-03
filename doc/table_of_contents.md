# Documentation structure

*Read this in other languages: [Korean](translations/table_of_contents_KR.md), [简体中文](translations/table_of_contents_ZH-CN.md).*

## Explaining mugle

- [intro](intro.md) - Technical introduction to mugle
- [mugle4bitcoiners](mugle4bitcoiners.md) - Explaining mugle from a bitcoiner's perspective

## Understand the mugle implementation

- [chain_sync](chain/chain_sync.md) - About how Mugle's blockchain is synchronized
- [blocks_and_headers](chain/blocks_and_headers.md) - How Mugle tracks blocks and headers on the chain
- [contract_ideas](contract_ideas.md) - Ideas on how to implement contracts
- [dandelion/dandelion](dandelion/dandelion.md) - About transaction propagation and cut-through. Stemming and fluffing!
- [dandelion/simulation](dandelion/simulation.md) - Dandelion simulation - aggregating transaction without lock_height Stemming and fluffing!
- [internal/pool](internal/pool.md) - Technical explanation of the transaction pool
- [merkle](merkle.md) - Technical explanation of mugle's favorite kind of merkle trees
- [merkle_proof graph](merkle_proof/merkle_proof.png) - Example merkle proof with pruning applied
- [pruning](pruning.md) - Technical explanation of pruning
- [stratum](stratum.md) - Technical explanation of Mugle Stratum RPC protocol
- [transaction UML](https://github.com/mugleproject/mugle-wallet/blob/master/doc/transaction/basic-transaction-wf.png) - UML of an interactive transaction (aggregating transaction without `lock_height`)
- [rangeproof output format](rangeproof_byte_format.md) - Explanation of the byte output of a range proof in a Mugle transaction

## Build and use

- [api](api/api.md) - Explaining the different APIs in Mugle and how to use them
- [build](build.md) - Explaining how to build and run the Mugle binaries
- [release](release_instruction.md) - Instructions of making a release
- [usage](usage.md) - Explaining how to use mugle in Testnet3
- [wallet](wallet/usage.md) - Explains the wallet design and `mugle wallet` sub-commands

## External (wiki)

- [FAQ](https://github.com/mugleproject/docs/wiki/FAQ) - Frequently Asked Questions
- [Building mugle](https://github.com/mugleproject/docs/wiki/Building)
- [How to use mugle](https://github.com/mugleproject/docs/wiki/How-to-use-mugle)
- [Hacking and contributing](https://github.com/mugleproject/docs/wiki/Hacking-and-contributing)
