// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Client API for the Dump Agent task.

#![no_std]

use derive_idol_err::IdolError;
use idol_runtime::ClientError;
use userlib::*;


#[derive(
    Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, counters::Count, IdolError,
)]
#[repr(u32)]
pub enum FDError {
    InvalidMessage = 1,
    MessageTooLarge = 2,
    PldmError = 3,
    NotReady = 4,
    #[idol(server_death)]
    ServerRestarted = 5,
}

impl From<ClientError> for FDError {
    fn from(_: ClientError) -> Self {
        FDError::NotReady
    }
}

include!(concat!(env!("OUT_DIR"), "/client_stub.rs"));
