use crate::command::{Command, CommandInput};
use crate::error::InteractionError;
use crate::helpers::asset::format_assets;
use crate::interaction::{
    ApplicationCommandOption, ApplicationCommandOptionType,
    InteractionApplicationCommandCallbackData,
};
use async_trait::async_trait;
use reqwest::Client;

pub(crate) struct Search {}

#[async_trait(?Send)]
impl Command for Search {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        if let Some(query) = _input.get_option("query") {
            // if query is less than 3 characters
            if query.len() < 3 {
                return Ok(InteractionApplicationCommandCallbackData {
                    content: Some("Query must be at least 3 characters long".to_string()),
                    choices: None,
                    embeds: None,
                });
            }
            let client = Client::new();
            let response = client
                .get("https://v2-api-testing.wanderer.moe/search")
                .query(&[("query", query)])
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
        } else {
            Ok(InteractionApplicationCommandCallbackData {
                content: Some("Query must be provided".to_string()),
                choices: None,
                embeds: None,
            })
        }
    }

    fn name(&self) -> String {
        "search".into()
    }

    fn description(&self) -> String {
        "Search for assets".into()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![ApplicationCommandOption {
            name: "query".into(),
            autocomplete: Some(true),
            description: "Name of file".into(),
            required: Some(false),
            ty: ApplicationCommandOptionType::String,
            choices: None,
        }])
    }

    async fn autocomplete(
        &self,
        _input: &CommandInput,
    ) -> Result<Option<InteractionApplicationCommandCallbackData>, InteractionError> {
        Ok(Some(InteractionApplicationCommandCallbackData {
            content: None,
            embeds: None,
            choices: None,
        }))
    }
}
