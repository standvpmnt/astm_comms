use std::collections::HashMap;

use astm_comms::astm;
use tokio_serial::SerialPort;

#[tokio::main]
async fn main() {
    let machines: HashMap<String, Box<dyn SerialPort>> = HashMap::new();
    let machines = astm::communicator::all_machines(machines).await;
    if machines.len() < 1 {
        eprintln!("No machines were found");
    }
    let h = tokio::task::spawn(async move {
        for machine in machines {
            println!("starting communication with {}", machine.0);
            astm::communicator::process_incoming(machine.1).await;
        }
    });
    h.await.unwrap();
}
