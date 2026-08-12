use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChatCommand {
    Join,
    SelectRole(StableId),
    Build(StableId),
    Upgrade(StableId),
    Buy { amount: u32, resource: StableId },
    Sell { amount: u32, resource: StableId },
    Revive(Option<StableId>),
    Praise,
    Vote(StableId),
    TriggerEvent(StableId),
    Experience,
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
    #[error("trade amount must be a positive integer")]
    InvalidAmount,
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
        match command.as_str() {
            "buy" | "sell" => {
                let amount = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?
                    .parse::<u32>()
                    .map_err(|_| CommandParseError::InvalidAmount)?;
                if amount == 0 {
                    return Err(CommandParseError::InvalidAmount);
                }
                let resource = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                if command == "buy" {
                    Ok(Self::Buy { amount, resource })
                } else {
                    Ok(Self::Sell { amount, resource })
                }
            }
            _ => {
                let argument = parts.next();
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                match command.as_str() {
                    "join" => no_argument(argument, Self::Join),
                    "experience" | "exp" => no_argument(argument, Self::Experience),
                    "save" => no_argument(argument, Self::Save),
                    "help" => no_argument(argument, Self::Help),
                    "role" => with_id(&command, argument, Self::SelectRole),
                    "build" => with_id(&command, argument, Self::Build),
                    "upgrade" | "level" => with_id(&command, argument, Self::Upgrade),
                    "vote" => with_id(&command, argument, Self::Vote),
                    "event" => with_id(&command, argument, Self::TriggerEvent),
                    "revive" => optional_id(argument).map(Self::Revive),
                    "praise" => no_argument(argument, Self::Praise),
                    _ => Err(CommandParseError::Unknown(command)),
                }
            }
        }
    }
}

fn content_id(value: &str) -> Result<StableId, CommandParseError> {
    StableId::new(value.to_ascii_lowercase().replace(' ', "_"))
        .map_err(|error| CommandParseError::InvalidId(error.to_string()))
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

fn optional_id(argument: Option<&str>) -> Result<Option<StableId>, CommandParseError> {
    argument.map(content_id).transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_shipping_command_grammar() {
        assert_eq!("!join".parse(), Ok(ChatCommand::Join));
        assert_eq!("!experience".parse(), Ok(ChatCommand::Experience));
        assert_eq!("!exp".parse(), Ok(ChatCommand::Experience));
        assert_eq!(
            "!build building:house".parse(),
            Ok(ChatCommand::Build(StableId::new("building:house").unwrap()))
        );
        assert_eq!(
            "!upgrade house".parse(),
            Ok(ChatCommand::Upgrade(StableId::new("house").unwrap()))
        );
        assert_eq!(
            "!buy 25 wood".parse(),
            Ok(ChatCommand::Buy {
                amount: 25,
                resource: StableId::new("wood").unwrap(),
            })
        );
        assert_eq!(
            "!sell 10 resource:ore".parse(),
            Ok(ChatCommand::Sell {
                amount: 10,
                resource: StableId::new("resource:ore").unwrap(),
            })
        );
        assert_eq!("!revive".parse(), Ok(ChatCommand::Revive(None)));
        assert_eq!("!praise".parse(), Ok(ChatCommand::Praise));
        assert_eq!(
            "!revive twitch:friend".parse(),
            Ok(ChatCommand::Revive(Some(
                StableId::new("twitch:friend").unwrap()
            )))
        );
        assert!(matches!(
            "build house".parse::<ChatCommand>(),
            Err(CommandParseError::MissingPrefix)
        ));
    }
}
