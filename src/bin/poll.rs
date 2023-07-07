// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use std::net::UdpSocket;

//project
use libferroxid_artnet::packets::art_poll;
use libferroxid_artnet::protocol;


fn main() {

    let mut local_address : String = "0.0.0.0".to_owned();
    local_address.push_str(& ":".to_owned());
    local_address.push_str(& protocol::constants::DEFAULT_IP_PORT.to_string());

    let socket = UdpSocket::bind(local_address)
                                     .expect("couldn't bind to address");
    socket.set_broadcast(true).expect("failed to set socket broadcast");

    // let sleep_time = std::time::Duration::from_secs(1);

    let packet = art_poll::compose();

    let mut remote_address : String = "255.255.255.255".to_owned();
    remote_address.push_str(& ":".to_owned());
    remote_address.push_str(& protocol::constants::DEFAULT_IP_PORT.to_string());

    // loop {
        // std::thread::sleep(sleep_time);
        println!("sending packet...");
        socket.send_to(&packet, remote_address)
              .expect("Packet could not be sent");
    // }
}
