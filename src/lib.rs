pub mod api {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct CosmeticItem {
        pub id: String,
        pub name: String,
        pub item_type: String,
    }

    pub struct ApiClient {
        client: Client,
    }

    impl ApiClient {
        pub fn new() -> Self {
            let client = Client::new();
            ApiClient { client }
        }

        pub async fn unlock_item(&self, item: &CosmeticItem) -> Result<(), String> {
            let url = format!("https://api.fortnite.com/unlock/{}", item.id);
            let response = self.client.post(&url).send().await.map_err(|e| e.to_string())?;
            if response.status().is_success() {
                Ok(())
            } else {
                Err("Failed to unlock item".to_string())
            }
        }
    }
}