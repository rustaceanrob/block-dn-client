use core::time::Duration;

const ENDPOINT: block_dn_client::Endpoint<'static> = block_dn_client::Endpoint::BLOCKDNORG;
const TIMEOUT: Duration = Duration::from_secs(2);

fn main() {
    let mut client_builder = block_dn_client::Builder::new();
    client_builder = client_builder.timeout(TIMEOUT);
    client_builder = client_builder.endpoint(ENDPOINT);
    let client = client_builder.build();
    let status = client.status().unwrap();
    println!("Server sync height {}", status.best_block_height);
}
