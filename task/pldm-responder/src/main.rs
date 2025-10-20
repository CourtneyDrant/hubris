// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

use userlib::*;
use firmware_device_api::pldm_initiator_task;
use firmware_device_api::FDError;
use pldm_lib::cmd_interface::CmdInterface;
use pldm_lib::transport::MctpTransport;
//TODO: We need an inteface to loop on with IPC into firmware-device-api
//use firmware_device_api::

pub const MAX_MCTP_PLDM_MSG_SIZE: usize = 1024;

#[export_name = "main"]
fn main() -> ! {
    //let pldm_responder_task = pldm_initiator_task {
        //cmd_interface: &CmdInterface::new(MctpTransport::new()),
    //};
    loop {
        /*
        // TODO: Add PLDM message processing loop
        // IPC to handle message if PLDM
        Receive request
        If PLDM message
            Process PLDM message
            Send response
        Else
            Ignore or handle other message types
        */
    }
}

include!(concat!(env!("OUT_DIR"), "/client_stub.rs"));