use crate::command::{Command, CommandInput};
use crate::error::InteractionError;
use crate::interaction::{ApplicationCommandOption, InteractionApplicationCommandCallbackData};
use async_trait::async_trait;

pub(crate) struct Ping {}

#[async_trait(?Send)]
impl Command for Ping {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        Ok(InteractionApplicationCommandCallbackData {
            content: Some("test".to_string()),
            choices: None,
            embeds: None,
        })
    }

    fn name(&self) -> String {
        "ping".into()
    }

    fn description(&self) -> String {
        "Send a ping".into()
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
