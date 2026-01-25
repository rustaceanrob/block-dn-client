use block_dn_client::{Builder, Client, Endpoint, Timeout};

fn default_client() -> Client<'static> {
    Builder::default().build()
}

#[test]
fn test_html() {
    let client = default_client();
    assert!(client.index_html().is_ok());
}

#[test]
fn test_status() {
    let client = default_client();
    assert!(client.status().is_ok());
}

#[test]
fn test_headers() {
    let client = default_client();
    assert!(client.block_headers(0).is_ok());
}

#[test]
fn test_filters() {
    let client = default_client();
    let filters = client.filters(0).unwrap();
    assert!(filters.len() == 2000);
}

#[test]
fn test_tap_tweaks() {
    let client = Builder::new().timeout(Timeout::from_seconds(10)).build();
    let tweaks = client.tweaks(900_000).unwrap();
    assert!(!tweaks.blocks.is_empty());
    let _ = tweaks.fallible_into_iterator();
}

#[test]
fn test_block() {
    let client = default_client();
    assert!(
        client
            .block(
                "0000000000000000000320283a032748cef8227873ff4872689bf23f1cda83a5"
                    .parse()
                    .unwrap()
            )
            .is_ok()
    )
}

#[test]
fn test_estimate_fee() {
    let client = Builder::new().endpoint(Endpoint::DEV_2140).build();
    assert!(client.estimate_smart_fee(1).is_ok());
}
