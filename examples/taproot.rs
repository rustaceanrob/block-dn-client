use std::time::Instant;

use block_dn_client::Timeout;

const ENDPOINT: block_dn_client::Endpoint<'static> = block_dn_client::Endpoint::BLOCK_DN_ORG;
const TIMEOUT: Timeout = Timeout::from_seconds(5);
const TAPROOT_ACTIVATION_HEIGHT: u32 = 700_000;

fn main() {
    let mut client_builder = block_dn_client::Builder::new();
    client_builder = client_builder.timeout(TIMEOUT);
    client_builder = client_builder.endpoint(ENDPOINT);
    let client = client_builder.build();
    let status = client.status().unwrap();
    let now = Instant::now();
    let mut start_height = TAPROOT_ACTIVATION_HEIGHT;
    let stop_height = status.best_filter_height;
    let mut total_bytes = 0;
    println!("Syncing to height {stop_height}");
    while start_height < stop_height {
        let filters = client.filters(start_height).unwrap();
        start_height += filters.len() as u32;
        let bytes = filters
            .into_iter()
            .map(|filter| filter.content.len())
            .sum::<usize>();
        total_bytes += bytes;
        println!("{start_height}/{stop_height}");
    }
    println!("Total bytes downloaded: {total_bytes}");
    println!("Time elapsed: {} seconds", now.elapsed().as_secs());
    println!("Done");
}
