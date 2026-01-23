use block_dn_client::Timeout;

const ENDPOINT: block_dn_client::Endpoint<'static> = block_dn_client::Endpoint::BLOCK_DN_ORG;
const TIMEOUT: Timeout = Timeout::from_seconds(2);

fn main() {
    let mut client_builder = block_dn_client::Builder::new();
    client_builder = client_builder.timeout(TIMEOUT);
    client_builder = client_builder.endpoint(ENDPOINT);
    let client = client_builder.build();
    let status = client.status().unwrap();
    println!("Server sync height {}", status.best_block_height);
}
