[![Build Status](https://dev.azure.com/mimblewimble/mugle/_apis/build/status/mimblewimble.mugle?branchName=master)](https://dev.azure.com/mimblewimble/mugle/_build/latest?definitionId=1&branchName=master)
[![Coverage Status](https://img.shields.io/codecov/c/github/mimblewimble/mugle/master.svg)](https://codecov.io/gh/mimblewimble/mugle)
[![Chat](https://img.shields.io/gitter/room/mugle_community/Lobby.svg)](https://gitter.im/mugle_community/Lobby)
[![Support](https://img.shields.io/badge/support-on%20gitter-brightgreen.svg)](https://gitter.im/mugle_community/support)
[![Documentation Wiki](https://img.shields.io/badge/doc-wiki-blue.svg)](https://github.com/mimblewimble/docs/wiki)
[![Release Version](https://img.shields.io/github/release/mimblewimble/mugle.svg)](https://github.com/mugleproject/mugle/releases)
[![License](https://img.shields.io/github/license/mimblewimble/mugle.svg)](https://github.com/mugleproject/mugle/blob/master/LICENSE)

# Mugle

Mugle is an in-progress implementation of the Mimblewimble protocol. Many characteristics are still undefined but the following constitutes a first set of choices:

  * Clean and minimal implementation, and aiming to stay as such.
  * Follows the Mimblewimble protocol, which provides hidden amounts and scaling advantages.
  * Cuckoo Cycle proof of work in two variants named Cuckaroo (ASIC-resistant) and Cuckatoo (ASIC-targeted).
  * Relatively fast block time: one minute.
  * Fixed block reward over time with a decreasing dilution.
  * Transaction fees are based on the number of Outputs created/destroyed and total transaction size.
  * Smooth curve for difficulty adjustments.

To learn more, read our [introduction to Mimblewimble and Mugle](doc/intro.md).

## Status

Mugle is live with mainnet. Still, much is left to be done and [contributions](CONTRIBUTING.md) are welcome (see below). Check our [mailing list archives](https://lists.launchpad.net/mimblewimble/) for the latest status.

## Contributing

To get involved, read our [contributing docs](CONTRIBUTING.md).

Find us:

* Chat: [Keybase](https://keybase.io/team/muglecoin), more instructions on how to join [here](https://mugle.mw/community).
* Mailing list: join the [~Mimblewimble team](https://launchpad.net/~mimblewimble) and subscribe on Launchpad.
* Twitter for the Mugle council: [@muglecouncil](https://twitter.com/muglecouncil)

## Getting Started

To learn more about the technology, read our [introduction](doc/intro.md).

To build and try out Mugle, see the [build docs](doc/build.md).

## Philosophy

Mugle likes itself small and easy on the eyes. It wants to be inclusive and welcoming for all walks of life, without judgement. Mugle is terribly ambitious, but not at the detriment of others, rather to further us all. It may have strong opinions to stay in line with its objectives, which doesn't mean disrespect of others' ideas.

We believe in pull requests, data and scientific research. We do not believe in unfounded beliefs.

## Credits

Tom Elvis Jedusor for the first formulation of Mimblewimble.

Andrew Poelstra for his related work and improvements.

John Tromp for the Cuckoo Cycle proof of work.

## License

Apache License v2.0.
