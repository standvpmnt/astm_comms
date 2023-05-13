use astm_comms::astm;

#[tokio::main]
async fn main() {
    astm::finder::list_ports().await;
}
