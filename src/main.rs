use std::fs;
use std::path::Path;
use rfd::MessageDialog;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize)]
struct CosmeticItem {
    id: String,
    name: String,
    item_type: String,
}

struct App {
    client: Client,
}

impl App {
    async fn unlock_item(&self, item: &CosmeticItem) -> Result<(), String> {
        let url = format!("https://api.fortnite.com/unlock/{}", item.id);
        let response = self.client.post(&url).send().await.map_err(|e| e.to_string())?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err("Failed to unlock item".to_string())
        }
    }

    fn show_message(&self, title: &str, message: &str) {
        MessageDialog::new()
            .set_title(title)
            .set_description(message)
            .show();
    }

    async fn unlock_cosmetics(&self, items: Vec<CosmeticItem>) {
        for item in items {
            match self.unlock_item(&item).await {
                Ok(_) => self.show_message("Success", &format!("Unlocked: {}", item.name)),
                Err(e) => self.show_message("Error", &e),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app = App { client };

    let items = vec![
        CosmeticItem { id: "1".to_string(), name: "Cool Outfit".to_string(), item_type: "outfit".to_string() },
        CosmeticItem { id: "2".to_string(), name: "Epic Emote".to_string(), item_type: "emote".to_string() },
        CosmeticItem { id: "3".to_string(), name: "Legendary Pickaxe".to_string(), item_type: "pickaxe".to_string() },
    ];

    app.unlock_cosmetics(items).await;
}