use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct FakeDatabase {
    urls: Arc<RwLock<HashMap<String, String>>>,
}

impl FakeDatabase {
    pub async fn insert(&self, code: String, long_url: String) {
        let mut urls = self.urls.write().await;
        urls.insert(code, long_url);
    }

    pub async fn find(&self, code: &str) -> Option<String> {
        let urls = self.urls.read().await;
        urls.get(code).cloned()
    }
}
