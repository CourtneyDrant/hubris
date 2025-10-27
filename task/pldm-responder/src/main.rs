// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A demo task that echoes MCTP messages.
//!
//! The task configures the MCTP server for EID 8
//! and starts listening for MCTP Message Type `1` (PLDM) packets.
//! Received messages are echoed as is through the response channel.

#![no_std]
#![no_main]

use mctp::{Eid, Listener, MsgType, RespChannel};
use userlib::*;
use fw_device_api::FwDevice;

task_slot!(MCTP, mctp_server);

#[export_name = "main"]
fn main() -> ! {
    let stack = mctp_api::Stack::from(MCTP.get_task_id());

    stack.set_eid(Eid(8)).unwrap_lite();
    let mut listener = stack.listener(MsgType(1), None).unwrap_lite();
    let mut recv_buf = [0; 255];

    let mut fw_device = FwDevice::new();

    loop {
        let (_, _, msg, mut resp) = listener.recv(&mut recv_buf).unwrap_lite();

        match resp.send(msg) {
            Ok(_) => {
                fw_device.process_request(&msg, &mut []).ok();
            }
            Err(_e) => {
                // Error sending response to peer
            }
        }
    }
}