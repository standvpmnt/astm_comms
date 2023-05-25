use std::collections::HashMap;

use astm_comms::astm;
use tokio_serial::SerialPort;

#[tokio::main]
async fn main() {
    let machines: HashMap<String, Box<dyn SerialPort>> = HashMap::new();
    let machines = astm::communicator::all_machines(machines).await;
    for machine in machines {
        astm::communicator::process_incoming(machine.1).await;
    }
}
