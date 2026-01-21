/// A string representing HTML. Suitable to render on a webpage.
#[derive(Debug)]
pub struct Html(pub String);

/// The status of the server.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerStatus {
    /// Genesis hash of the chain.
    pub chain_genesis_hash: String,
    /// Name of the chain.
    pub chain_name: String,
    /// The block height of the most work chain.
    pub best_block_height: u32,
    /// The tip hash of the most work chain.
    pub best_block_hash: String,
    /// The best known filter header.
    pub best_filter_header: String,
    /// The best known filter height, possibly less than the best block height.
    pub best_filter_height: u32,
    /// The best known silent payments partial secret (tweak) data.
    pub best_sptweak_height: u32,
    /// All files are synced to the tip height.
    pub all_files_synced: bool,
    /// Entries per header file.
    pub entries_per_header_file: u64,
    /// Entries per filter file.
    pub entries_per_filter_file: u32,
    /// Entries per silent payments tweak file.
    pub entries_per_sptweak_file: u32,
}
