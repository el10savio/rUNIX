use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;

use crate::CustomError;

pub fn transmit_packets(
    address: String,
    count: u16,
) -> Result<Vec<std::time::Duration>, CustomError> {
    // Check For Empty Address
    if address.is_empty() {
        return Err(CustomError::EmptyAddress);
    }

    // Generate Pinger
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(_error) => return Err(CustomError::PingerCreationFailed),
    };

    // Set Up Pinger
    pinger.add_ipaddr(&address);
    pinger.run_pinger();

    // Run Pings
    // TODO: Loop if count is 0
    let mut rtt_list: Vec<std::time::Duration> = vec![];

    for _index in 0..count {
        let rtt = receive_packet(&results)?;
        rtt_list.push(rtt);
    }

    // Stop Pinger
    pinger.stop_pinger();

    Ok(rtt_list)
}

pub fn receive_packet(
    results: &std::sync::mpsc::Receiver<fastping_rs::PingResult>,
) -> Result<std::time::Duration, CustomError> {
    let bytes_count = "64 bytes";

    match results.recv() {
        Ok(result) => match result {
            Idle { addr } => {
                println!("Idle Address {}", addr);
                Ok(std::time::Duration::new(0, 0))
            }
            Receive { addr, rtt } => {
                println!("{} from {}: time={:?}", bytes_count, addr, rtt);
                Ok(rtt)
            }
        },
        Err(_) => panic!("Worker threads disconnected before the solution was found!"),
    }
}
