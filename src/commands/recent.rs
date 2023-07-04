use crate::command::{Command, CommandInput};
use crate::error::InteractionError;
use crate::interaction::{ApplicationCommandOption, InteractionApplicationCommandCallbackData};
use async_trait::async_trait;
use reqwest::Client;

pub(crate) struct Recent {}

#[async_trait(?Send)]
impl Command for Recent {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        let client = Client::new();
        let response = client
            .get("https://v2-api-testing.wanderer.moe/recent")
            .send()
            .await;
        let content = match response {
            Ok(response) => {
                let json = response.json::<serde_json::Value>().await.unwrap();
                let mut content = String::new();
                if let Some(assets) = json["results"].as_array() {
                    let mut i = 1;
                    for asset in assets.iter().take(10) {
                        content.push_str(&format!(
                            "{}. {}\n",
                            i,
                            asset["name"].as_str().unwrap_or("Unknown")
                        ));
                        i += 1;
                    }
                } else {
                    content = "Error: No results".to_string();
                }
                content
            }
            Err(e) => format!("Error fetching recent assets: {}", e),
        };
        Ok(InteractionApplicationCommandCallbackData {
            content: Some(content),
            choices: None,
            embeds: None,
        })
    }

    fn name(&self) -> String {
        "recent".into()
    }

    fn description(&self) -> String {
        "Get top 10 recently uploaded assets".into()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        None
    }

    async fn autocomplete(
        &self,
        _input: &CommandInput,
    ) -> Result<Option<InteractionApplicationCommandCallbackData>, InteractionError> {
        Ok(None)
    }
}
