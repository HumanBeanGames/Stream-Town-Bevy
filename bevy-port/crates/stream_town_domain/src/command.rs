use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChatCommand {
    Join,
    SelectRole(StableId),
    Build(StableId),
    Vote(StableId),
    TriggerEvent(StableId),
    Save,
    Help,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CommandParseError {
    #[error("commands must start with !")]
    MissingPrefix,
    #[error("unknown command {0}")]
    Unknown(String),
    #[error("command {0} requires one argument")]
    MissingArgument(String),
    #[error("command has too many arguments")]
    TooManyArguments,
    #[error("invalid content identifier: {0}")]
    InvalidId(String),
}

impl FromStr for ChatCommand {
    type Err = CommandParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();
        let command = parts.next().ok_or(CommandParseError::MissingPrefix)?;
        let command = command
            .strip_prefix('!')
            .ok_or(CommandParseError::MissingPrefix)?
            .to_ascii_lowercase();
        let argument = parts.next();
        if parts.next().is_some() {
            return Err(CommandParseError::TooManyArguments);
        }
        match command.as_str() {
            "join" => no_argument(argument, Self::Join),
            "save" => no_argument(argument, Self::Save),
            "help" => no_argument(argument, Self::Help),
            "role" => with_id(&command, argument, Self::SelectRole),
            "build" => with_id(&command, argument, Self::Build),
            "vote" => with_id(&command, argument, Self::Vote),
            "event" => with_id(&command, argument, Self::TriggerEvent),
            _ => Err(CommandParseError::Unknown(command)),
        }
    }
}

fn no_argument(
    argument: Option<&str>,
    command: ChatCommand,
) -> Result<ChatCommand, CommandParseError> {
    if argument.is_some() {
        Err(CommandParseError::TooManyArguments)
    } else {
        Ok(command)
    }
}

fn with_id(
    command: &str,
    argument: Option<&str>,
    constructor: impl FnOnce(StableId) -> ChatCommand,
) -> Result<ChatCommand, CommandParseError> {
    let argument =
        argument.ok_or_else(|| CommandParseError::MissingArgument(command.to_owned()))?;
    let normalized = argument.to_ascii_lowercase().replace(' ', "_");
    StableId::new(normalized)
        .map(constructor)
        .map_err(|error| CommandParseError::InvalidId(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_shipping_command_grammar() {
        assert_eq!("!join".parse(), Ok(ChatCommand::Join));
        assert_eq!(
            "!build building:house".parse(),
            Ok(ChatCommand::Build(StableId::new("building:house").unwrap()))
        );
        assert!(matches!(
            "build house".parse::<ChatCommand>(),
            Err(CommandParseError::MissingPrefix)
        ));
    }
}
