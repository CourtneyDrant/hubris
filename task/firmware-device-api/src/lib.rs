// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]

use derive_idol_err::IdolError;
use userlib::*;
use zerocopy::{FromZeros, IntoBytes};
use pldm_lib::{cmd_interface, firmware_device};

// Error types
#[derive(
    Copy, Clone, Debug, FromPrimitive, Eq, PartialEq, counters::Count, IdolError,
)]
pub enum FDError {
    InvalidState = 1,
    InvalidParameter = 2,
    NotSupported = 3,
    #[idol(server_death)]
    ServerRestarted = 4,
}

/* 
impl core::convert::TryFrom<u32> for FDError {
    type Error = ();
    
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use userlib::FromPrimitive;
        Self::from_u32(value).ok_or(())
    }
}
*/

pub struct pldm_responder_task<'a> {
    pub cmd_interface: &'a cmd_interface::CmdInterface<'a, pldm_lib::transport::MctpTransport>,
}

pub struct pldm_initiator_task<'a> {
    pub cmd_interface: &'a cmd_interface::CmdInterface<'a, pldm_lib::transport::MctpTransport>,
}

include!(concat!(env!("OUT_DIR"), "/server_stub.rs"));

//include!(concat!(env!("OUT_DIR"), "/client_stub-device.rs"));