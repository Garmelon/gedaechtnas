use gdn::store::Store;

pub struct AppState {
    pub store: Store,
    pub store_last_id: Option<u64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
            store_last_id: None,
        }
    }
}
