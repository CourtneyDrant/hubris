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
use fw_device_api::{FwDevice, FDError};

task_slot!(MCTP, mctp_server);
task_slot!(FW_DEVICE, fw_device);

#[export_name = "main"]
fn main() -> ! {
    let stack = mctp_api::Stack::from(MCTP.get_task_id());

    stack.set_eid(Eid(8)).unwrap_lite();
    let mut listener = stack.listener(MsgType(1), None).unwrap_lite();
    let mut recv_buf = [0; 255];

    // Create FwDevice client from task ID
    let fw_device = FwDevice::from(FW_DEVICE.get_task_id());

    loop {
        let (_, _, msg, mut resp) = listener.recv(&mut recv_buf).unwrap_lite();

        // Process PLDM message through FwDevice
        let mut response_buf = [0u8; 255]; // Buffer for response
        
        match fw_device.process_request(msg, &mut response_buf) {
            Ok(response_len) => {
                // Send the processed response back via MCTP
                let response = &response_buf[..response_len];
                match resp.send(response) {
                    Ok(_) => {}
                    Err(_e) => {
                        // Error sending response to peer
                    }
                }
            }
            Err(FDError::ServerRestarted) => {
                // FW Device server restarted, may need to handle this
                // For now, send original message as echo
                match resp.send(msg) {
                    Ok(_) => {}
                    Err(_e) => {}
                }
            }
            Err(_other_error) => {
                // Other FW Device errors, send error response or echo
                match resp.send(msg) {
                    Ok(_) => {}
                    Err(_e) => {}
                }
            }
        }
    }
}