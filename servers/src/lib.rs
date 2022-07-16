// Copyright 2021 The Mugle Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Main crate putting together all the other crates that compose Mugle into a
//! binary.

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use mugle_api as api;
use mugle_chain as chain;
use mugle_core as core;
use mugle_keychain as keychain;
use mugle_p2p as p2p;
use mugle_pool as pool;
use mugle_store as store;
use mugle_util as util;

pub mod common;
mod mining;
mod mugle;

pub use crate::common::stats::{DiffBlock, PeerStats, ServerStats, StratumStats, WorkerStats};
pub use crate::common::types::{ServerConfig, StratumServerConfig};
pub use crate::mugle::server::{Server, ServerTxPool};
