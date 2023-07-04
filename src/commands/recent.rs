use crate::command::{Command, CommandInput};
use crate::error::InteractionError;
use crate::helpers::asset::format_assets;
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
                let assets = json["results"]
                    .as_array()
                    .map_or_else(Vec::new, |v| v.to_vec());
                format_assets(&assets)
            }
            Err(_) => "An error occurred while fetching recent assets".to_string(),
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
        "Get top 5 recently uploaded assets".into()
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
