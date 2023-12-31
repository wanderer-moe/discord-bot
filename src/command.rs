use crate::commands;
use crate::error::InteractionError;
use crate::interaction::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub(crate) struct CommandInput<'a> {
    pub(crate) options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub(crate) guild_id: Option<String>,
    pub(crate) channel_id: Option<String>,
    pub(crate) user: Option<User>,
    pub(crate) member: Option<Member>,
    pub(crate) ctx: &'a mut worker::RouteContext<()>,
}

#[allow(dead_code)]
impl CommandInput<'_> {
    pub fn get_option(&self, name: &str) -> Option<&str> {
        match &self.options {
            Some(options) => {
                for option in options {
                    if option.name == name {
                        match option.value {
                            Some(ref value) => return Some(value),
                            None => return None,
                        }
                    }
                }
                None
            }
            None => None,
        }
    }

    pub async fn kv_get(
        &self,
        namespace: &str,
        key: &str,
    ) -> Result<Option<String>, InteractionError> {
        let kv = self
            .ctx
            .kv(namespace)
            .map_err(|_| InteractionError::WorkerError("Bind to kv".into()))?;
        let value = kv
            .get(key)
            .text()
            .await
            .map_err(|_| InteractionError::WorkerError("Fetching from KV".into()))?;
        Ok(value)
    }

    pub async fn kv_put(
        &self,
        namespace: &str,
        key: &str,
        value: &str,
    ) -> Result<(), InteractionError> {
        let kv = self
            .ctx
            .kv(namespace)
            .map_err(|_| InteractionError::WorkerError("bind to kv".into()))?;
        kv.put(key, value)
            .map_err(|_| InteractionError::WorkerError("bind to KV".into()))?
            .execute()
            .await
            .map_err(|_| InteractionError::WorkerError("KV put".into()))?;
        Ok(())
    }

    pub fn admin_or_bail(&self) -> Option<InteractionApplicationCommandCallbackData> {
        match &self.member {
            Some(member) => {
                if member.is_admin() {
                    None
                } else {
                    Some(InteractionApplicationCommandCallbackData {
                        content: Some("You must be an admin to use this command!".to_string()),
                        choices: None,
                        embeds: None,
                    })
                }
            }
            None => Some(InteractionApplicationCommandCallbackData {
                content: Some("You must use this command inside a discord server.".to_string()),
                choices: None,
                embeds: None,
            }),
        }
    }
}

#[async_trait(?Send)]
pub(crate) trait Command {
    async fn respond(
        &self,
        _input: &CommandInput,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        unimplemented!()
    }

    fn name(&self) -> String {
        unimplemented!()
    }

    fn description(&self) -> String {
        unimplemented!()
    }

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        unimplemented!()
    }

    async fn autocomplete(
        &self,
        _input: &CommandInput,
    ) -> Result<Option<InteractionApplicationCommandCallbackData>, InteractionError> {
        // If your command supports autocomplete implement the logic here
        unimplemented!()
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RegisteredCommand {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) options: Option<Vec<ApplicationCommandOption>>,
}

pub(crate) fn init_commands() -> Vec<Box<dyn Command + Sync>> {
    vec![
        Box::new(commands::ping::Ping {}),
        Box::new(commands::recent::Recent {}),
        Box::new(commands::search::Search {}),
        Box::new(commands::lookup::Lookup {}),
    ]
}
