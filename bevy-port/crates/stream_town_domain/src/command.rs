use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChatCommand {
    Join,
    SelectRole(StableId),
    Role,
    Health,
    Build(StableId),
    MoveBuilding(Vec<BuildingAction>),
    ConfirmBuilding,
    CancelBuilding,
    Buildings,
    BuildingIds(StableId),
    Upgrade(StableId),
    Level(StableId),
    LevelBuilding {
        building: StableId,
        index: u16,
        iterations: u16,
    },
    LevelAll {
        building: StableId,
        target_level: u16,
    },
    RemoveBuilding {
        building: StableId,
        index: u16,
    },
    Buy {
        amount: u32,
        resource: StableId,
    },
    Sell {
        amount: u32,
        resource: StableId,
    },
    Recruit {
        role: StableId,
        amount: u16,
    },
    RecruitCount,
    RecruitIds,
    RecruitInfo(u16),
    RecruitRole {
        recruit: u16,
        role: StableId,
    },
    DismissRecruit(u16),
    Station(Option<u16>),
    Target(Option<u16>),
    Unstuck,
    Pets,
    Pet(Option<StableId>),
    Ping,
    Customize {
        kind: CustomizationKind,
        index: u8,
    },
    Roles,
    TownStats,
    Info(StableId),
    Camera(Vec<CameraAction>),
    ResetCamera,
    ModRole {
        player: StableId,
        role: StableId,
    },
    Revive(Option<StableId>),
    Praise,
    Vote(StableId),
    StartRulerVote,
    Resign,
    TriggerEvent(StableId),
    Experience,
    Save,
    Help,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CameraDirection {
    Up,
    Down,
    Left,
    Right,
    In,
    Out,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CameraAction {
    pub direction: CameraDirection,
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BuildingDirection {
    Up,
    Down,
    Left,
    Right,
    Rotate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuildingAction {
    pub direction: BuildingDirection,
    /// Movement is measured in grid cells; rotation is measured in quarter-turns.
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CustomizationKind {
    Hair,
    Eyes,
    FacialHair,
    Body,
    HairColor,
    EyeColor,
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
    #[error("invalid numeric identifier")]
    InvalidIndex,
    #[error("invalid camera direction {0}")]
    InvalidDirection(String),
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
            "recruit" => {
                let role = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let amount = parts
                    .next()
                    .map(str::parse::<u16>)
                    .transpose()
                    .map_err(|_| CommandParseError::InvalidAmount)?
                    .unwrap_or(1);
                if amount == 0 {
                    return Err(CommandParseError::InvalidAmount);
                }
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::Recruit { role, amount })
            }
            "rrole" | "modrole" => {
                let first = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?;
                let role = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                if command == "rrole" {
                    Ok(Self::RecruitRole {
                        recruit: parse_index(first)?,
                        role,
                    })
                } else {
                    Ok(Self::ModRole {
                        player: content_id(first)?,
                        role,
                    })
                }
            }
            "cam" => parse_camera_actions(parts).map(Self::Camera),
            "move" => parse_building_actions(None, parts).map(Self::MoveBuilding),
            "up" | "down" | "left" | "right" | "rotate" => {
                parse_building_actions(Some(command.as_str()), parts).map(Self::MoveBuilding)
            }
            "level" => parse_level_command(parts),
            "levelall" => {
                let building = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let target_level = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))
                    .and_then(parse_index)?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::LevelAll {
                    building,
                    target_level,
                })
            }
            "remove" => {
                let building = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let index = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))
                    .and_then(parse_index)?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::RemoveBuilding { building, index })
            }
            "hair" | "eyes" | "facialhair" | "body" | "haircolor" | "eyecolor" => {
                let index = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?
                    .parse::<u8>()
                    .ok()
                    .filter(|index| *index > 0)
                    .ok_or(CommandParseError::InvalidIndex)?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                let kind = match command.as_str() {
                    "hair" => CustomizationKind::Hair,
                    "eyes" => CustomizationKind::Eyes,
                    "facialhair" => CustomizationKind::FacialHair,
                    "body" => CustomizationKind::Body,
                    "haircolor" => CustomizationKind::HairColor,
                    "eyecolor" => CustomizationKind::EyeColor,
                    _ => unreachable!(),
                };
                Ok(Self::Customize { kind, index })
            }
            _ => {
                let argument = parts.next();
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                match command.as_str() {
                    "join" => no_argument(argument, Self::Join),
                    "experience" | "exp" => no_argument(argument, Self::Experience),
                    "health" => no_argument(argument, Self::Health),
                    "roles" => no_argument(argument, Self::Roles),
                    "townstats" => no_argument(argument, Self::TownStats),
                    "buildings" => no_argument(argument, Self::Buildings),
                    "rid" => no_argument(argument, Self::RecruitIds),
                    "stuck" => no_argument(argument, Self::Unstuck),
                    "ping" => no_argument(argument, Self::Ping),
                    "resetcam" => no_argument(argument, Self::ResetCamera),
                    "save" => no_argument(argument, Self::Save),
                    "help" => no_argument(argument, Self::Help),
                    "confirm" | "accept" => no_argument(argument, Self::ConfirmBuilding),
                    "cancel" => no_argument(argument, Self::CancelBuilding),
                    "recruits" => no_argument(argument, Self::RecruitCount),
                    "rulervote" => no_argument(argument, Self::StartRulerVote),
                    "resign" => no_argument(argument, Self::Resign),
                    "role" => {
                        optional_id(argument).map(|role| role.map_or(Self::Role, Self::SelectRole))
                    }
                    "station" => optional_index(argument).map(Self::Station),
                    "target" => optional_index(argument).map(Self::Target),
                    "pet" => optional_id(argument).map(Self::Pet),
                    "pets" => no_argument(argument, Self::Pets),
                    "info" => with_id(&command, argument, Self::Info),
                    "bid" => with_id(&command, argument, Self::BuildingIds),
                    "rinfo" => with_index(&command, argument, Self::RecruitInfo),
                    "rdismiss" => with_index(&command, argument, Self::DismissRecruit),
                    "build" => with_id(&command, argument, Self::Build),
                    "upgrade" => with_id(&command, argument, Self::Upgrade),
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

fn parse_index(value: &str) -> Result<u16, CommandParseError> {
    value
        .parse::<u16>()
        .ok()
        .filter(|value| *value > 0)
        .ok_or(CommandParseError::InvalidIndex)
}

fn optional_index(argument: Option<&str>) -> Result<Option<u16>, CommandParseError> {
    argument.map(parse_index).transpose()
}

fn with_index(
    command: &str,
    argument: Option<&str>,
    constructor: impl FnOnce(u16) -> ChatCommand,
) -> Result<ChatCommand, CommandParseError> {
    let argument =
        argument.ok_or_else(|| CommandParseError::MissingArgument(command.to_owned()))?;
    parse_index(argument).map(constructor)
}

fn parse_camera_actions<'a>(
    parts: impl Iterator<Item = &'a str>,
) -> Result<Vec<CameraAction>, CommandParseError> {
    let mut parts = parts.peekable();
    let mut actions = Vec::new();
    while let Some(direction) = parts.next() {
        let direction = match direction.to_ascii_lowercase().as_str() {
            "up" => CameraDirection::Up,
            "down" => CameraDirection::Down,
            "left" => CameraDirection::Left,
            "right" => CameraDirection::Right,
            "in" => CameraDirection::In,
            "out" => CameraDirection::Out,
            _ => return Err(CommandParseError::InvalidDirection(direction.to_owned())),
        };
        let amount = parts
            .peek()
            .and_then(|value| value.parse::<i32>().ok())
            .map_or(1, |amount| {
                let _ = parts.next();
                amount
            });
        actions.push(CameraAction { direction, amount });
    }
    if actions.is_empty() {
        Err(CommandParseError::MissingArgument("cam".to_owned()))
    } else {
        Ok(actions)
    }
}

fn parse_building_actions<'a>(
    alias: Option<&'a str>,
    parts: impl Iterator<Item = &'a str>,
) -> Result<Vec<BuildingAction>, CommandParseError> {
    let mut tokens = alias.into_iter().chain(parts).peekable();
    let mut actions = Vec::new();
    while let Some(direction) = tokens.next() {
        let direction = match direction.to_ascii_lowercase().as_str() {
            "up" => BuildingDirection::Up,
            "down" => BuildingDirection::Down,
            "left" => BuildingDirection::Left,
            "right" => BuildingDirection::Right,
            "rotate" => BuildingDirection::Rotate,
            _ => return Err(CommandParseError::InvalidDirection(direction.to_owned())),
        };
        let amount = tokens
            .peek()
            .and_then(|value| value.parse::<i32>().ok())
            .map_or(1, |amount| {
                let _ = tokens.next();
                amount
            });
        actions.push(BuildingAction { direction, amount });
    }
    if actions.is_empty() {
        Err(CommandParseError::MissingArgument("move".to_owned()))
    } else {
        Ok(actions)
    }
}

fn parse_level_command<'a>(
    mut parts: impl Iterator<Item = &'a str>,
) -> Result<ChatCommand, CommandParseError> {
    let Some(first) = parts.next() else {
        return Ok(ChatCommand::Experience);
    };
    let Some(second) = parts.next() else {
        return content_id(first).map(ChatCommand::Level);
    };
    let building = content_id(first)?;
    let index = parse_index(second)?;
    let iterations = parts.next().map(parse_index).transpose()?.unwrap_or(1);
    if parts.next().is_some() {
        return Err(CommandParseError::TooManyArguments);
    }
    Ok(ChatCommand::LevelBuilding {
        building,
        index,
        iterations,
    })
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
            "!level logger".parse(),
            Ok(ChatCommand::Level(StableId::new("logger").unwrap()))
        );
        assert_eq!("!level".parse(), Ok(ChatCommand::Experience));
        assert_eq!(
            "!move up 2 left rotate -1".parse(),
            Ok(ChatCommand::MoveBuilding(vec![
                BuildingAction {
                    direction: BuildingDirection::Up,
                    amount: 2,
                },
                BuildingAction {
                    direction: BuildingDirection::Left,
                    amount: 1,
                },
                BuildingAction {
                    direction: BuildingDirection::Rotate,
                    amount: -1,
                },
            ]))
        );
        assert_eq!(
            "!right 3".parse(),
            Ok(ChatCommand::MoveBuilding(vec![BuildingAction {
                direction: BuildingDirection::Right,
                amount: 3,
            }]))
        );
        assert_eq!("!accept".parse(), Ok(ChatCommand::ConfirmBuilding));
        assert_eq!("!cancel".parse(), Ok(ChatCommand::CancelBuilding));
        assert_eq!(
            "!level lumbermill 2 3".parse(),
            Ok(ChatCommand::LevelBuilding {
                building: StableId::new("lumbermill").unwrap(),
                index: 2,
                iterations: 3,
            })
        );
        assert_eq!(
            "!levelall house 5".parse(),
            Ok(ChatCommand::LevelAll {
                building: StableId::new("house").unwrap(),
                target_level: 5,
            })
        );
        assert_eq!(
            "!remove house 2".parse(),
            Ok(ChatCommand::RemoveBuilding {
                building: StableId::new("house").unwrap(),
                index: 2,
            })
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
            "!recruit miner 3".parse(),
            Ok(ChatCommand::Recruit {
                role: StableId::new("miner").unwrap(),
                amount: 3,
            })
        );
        assert_eq!("!recruits".parse(), Ok(ChatCommand::RecruitCount));
        assert_eq!("!role".parse(), Ok(ChatCommand::Role));
        assert_eq!("!station 2".parse(), Ok(ChatCommand::Station(Some(2))));
        assert_eq!("!target".parse(), Ok(ChatCommand::Target(None)));
        assert_eq!("!pet".parse(), Ok(ChatCommand::Pet(None)));
        assert_eq!("!pets".parse(), Ok(ChatCommand::Pets));
        assert_eq!(
            "!rrole 3 ranger".parse(),
            Ok(ChatCommand::RecruitRole {
                recruit: 3,
                role: StableId::new("ranger").unwrap(),
            })
        );
        assert_eq!(
            "!cam up 2 in 3".parse(),
            Ok(ChatCommand::Camera(vec![
                CameraAction {
                    direction: CameraDirection::Up,
                    amount: 2
                },
                CameraAction {
                    direction: CameraDirection::In,
                    amount: 3
                },
            ]))
        );
        assert_eq!(
            "!body 3".parse(),
            Ok(ChatCommand::Customize {
                kind: CustomizationKind::Body,
                index: 3,
            })
        );
        assert_eq!("!rulervote".parse(), Ok(ChatCommand::StartRulerVote));
        assert_eq!("!resign".parse(), Ok(ChatCommand::Resign));
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
