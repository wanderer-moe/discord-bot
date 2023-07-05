use crate::command::{Command, CommandInput};
use crate::error::InteractionError;
use crate::helpers::asset::format_asset;
use crate::interaction::{
    ApplicationCommandOption, ApplicationCommandOptionType,
    InteractionApplicationCommandCallbackData,
};
use async_trait::async_trait;
use reqwest::Client;

pub(crate) struct Lookup {}

#[async_trait(?Send)]
impl Command for Lookup {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        if let Some(id) = _input.get_option("id") {
            let client = Client::new();
            let response = client
                .get(format!("https://v2-api-testing.wanderer.moe/asset/{}", id).as_str())
                .send()
                .await;
            let content = match response {
                Ok(response) => {
                    let json = response.json::<serde_json::Value>().await.unwrap();
                    let asset = json["asset"].clone();
                    format_asset(&asset)
                }
                Err(_) => "An error occurred while getting the asset".to_string(),
            };
            Ok(InteractionApplicationCommandCallbackData {
                content: Some(content),
                choices: None,
                embeds: None,
            })
        } else {
            Ok(InteractionApplicationCommandCallbackData {
                content: Some("ID must be provided".to_string()),
                choices: None,
                embeds: None,
            })
        }
    }

    fn name(&self) -> String {
        "lookup".into()
    }

    fn description(&self) -> String {
        "Lookup asset information by ID".into()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![ApplicationCommandOption {
            name: "id".into(),
            autocomplete: Some(true),
            description: "ID of the asset to lookup".into(),
            required: Some(true),
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
