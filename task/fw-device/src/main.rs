// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Firmware Device PLDM Task
//!
//! This task implements a PLDM firmware device that can handle
//! firmware update operations and commands.

#![no_std]
#![no_main]

use derive_idol_err::IdolError;
use idol_runtime::{NotificationHandler, RequestError, ClientError};
use userlib::{RecvMessage, FromPrimitive};
use pldm_lib::cmd_interface::{self, CmdInterface};
use pldm_lib::config::PLDM_PROTOCOL_CAPABILITIES;
use fw_device_api::FDError;
use core::char::MAX;
use core::ops::DerefMut;

use crate::idl::InOrderFwDeviceImpl;

pub const MAX_MCTP_PLDM_MSG_SIZE: usize = 1024;


struct ServerImpl<'a> {
    cmd_interface: CmdInterface<'a>,
}

impl<'a> ServerImpl<'a> {
    fn new() -> Self {
        Self {
            cmd_interface: CmdInterface::new(&PLDM_PROTOCOL_CAPABILITIES),
        }
    }

}

impl NotificationHandler for ServerImpl<'_> {
    fn current_notification_mask(&self) -> u32 {
        // We don't use notifications, don't listen for any.
        0
    }

    fn handle_notification(&mut self, _bits: u32) {
    }
}

impl InOrderFwDeviceImpl for ServerImpl<'_> {
    fn process_request(
        &mut self,
        msg: &userlib::RecvMessage,
        request: idol_runtime::Leased<idol_runtime::R, [u8]>,
        response: idol_runtime::Leased<idol_runtime::W, [u8]>,
    ) -> Result<usize, idol_runtime::RequestError<FDError>> {
        let mut msg_buf = [0u8; MAX_MCTP_PLDM_MSG_SIZE];

        request.read_range(0..MAX_MCTP_PLDM_MSG_SIZE, &mut msg_buf);

        //CAD if msg_buf.len() < buf.len() {
            //CAD sys_reply(msg.sender, ServerError::NoSpace.into(), &[]);
        //CAD }

        //let msg_buf = &msg_buf[..buf.len()];

        self.cmd_interface.handle_initiator_msg(&mut msg_buf);

        response.write_range(0..MAX_MCTP_PLDM_MSG_SIZE, &mut msg_buf);
        /* 
        if msg_buf.len() < buf.len() {
            sys_reply(msg.sender, ServerError::NoSpace.into(), &[]);
        }
        if buf.read_range(0..buf.len(), &mut msg_buf).is_err() {
            todo!("client died?");
        }
        */
        userlib::sys_reply(msg.sender, 0, &msg_buf);

        Ok((0))
    }

}

#[export_name = "main"]
fn main() -> ! {
    let mut server = ServerImpl::new();
    let mut buffer = [0; idl::INCOMING_SIZE];

    loop {
        idol_runtime::dispatch(&mut buffer, &mut server);
    }
}

mod idl {
    use super::FDError;
    use userlib::FromPrimitive;
    use zerocopy::{FromBytes, KnownLayout, Immutable, Unaligned};

    include!(concat!(env!("OUT_DIR"), "/server_stub.rs"));
}