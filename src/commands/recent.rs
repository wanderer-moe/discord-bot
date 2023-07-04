use crate::command::{Command, CommandInput};
use crate::error::InteractionError;
use crate::interaction::{ApplicationCommandOption, InteractionApplicationCommandCallbackData};
use async_trait::async_trait;

pub(crate) struct Recent {}

#[async_trait(?Send)]
impl Command for Recent {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        Ok(InteractionApplicationCommandCallbackData {
            content: Some("Test Response".to_string()),
            choices: None,
            embeds: None,
        })
    }

    fn name(&self) -> String {
        "recent".into()
    }

    fn description(&self) -> String {
        "Get top 30 recently uploaded assets".into()
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
