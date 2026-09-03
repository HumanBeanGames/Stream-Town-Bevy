use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

/// Shipping Unity accepted these misspellings as aliases for character creation.
pub const UNITY_CREATE_COMMAND_ALIASES: [&str; 10] = [
    "create", "crate", "crete", "join", "start", "creta", "ceate", "cate", "crtea", "ligma",
];

/// Returns the source-authored usage string for a recognized Unity command.
///
/// This intentionally takes the raw chat input so malformed commands can still
/// receive the same useful response Unity sent before dispatch.
#[must_use]
pub fn unity_command_usage(input: &str) -> Option<&'static str> {
    let command = input
        .split_whitespace()
        .next()?
        .strip_prefix('!')?
        .to_ascii_lowercase();
    if UNITY_CREATE_COMMAND_ALIASES.contains(&command.as_str()) {
        return Some("!join");
    }
    Some(match command.as_str() {
        "role" => "!role <role> (or !role to view your current role)",
        "build" => "!build <building>",
        "cost" | "buildcost" => "!cost <building>",
        "move" => "!move <up|down|left|right|rotate> [amount]",
        "up" => "!up [amount]",
        "down" => "!down [amount]",
        "left" => "!left [amount]",
        "right" => "!right [amount]",
        "rotate" => "!rotate [amount]",
        "level" => "!level <role> OR !level <building> <id> [amount]",
        "remove" => "!remove <building> <id>",
        "bid" => "!bid <building>",
        "upgrade" => "!upgrade <building> <BID>",
        "beginplace" => "!beginplace",
        "endplace" => "!endplace",
        "rotatebuilding" => "!rotatebuilding <building> <BID> [quarter turns]",
        "station" => "!station <id> (or !station to list IDs)",
        "target" => "!target <id> (or !target to list IDs)",
        "hair" => "!hair <index>",
        "facialhair" => "!facialhair <index>",
        "eyes" => "!eyes <index>",
        "body" => "!body <index>",
        "haircolor" => "!haircolor <index>",
        "eyecolor" => "!eyecolor <index>",
        "light" | "lightcolor" | "lightcolour" => "!light <name|#RRGGBB>",
        "namecolor" | "namecolour" => "!namecolor <name|#RRGGBB>",
        "buildinglight" => "!buildinglight <building> <BID> <name|#RRGGBB> (Ruler only)",
        "addresource" => "!addresource <resource> <amount>",
        "vote" => "!vote <option number>",
        "modrole" => "!modrole <player> <role>",
        "kill" => "!kill <player>",
        "grevive" => "!grevive <player>",
        "revive" => "!revive [player]",
        "givexp" => "!givexp <player> <amount>",
        "givexpall" => "!givexpall <amount>",
        "levelup" => "!levelup <player> [amount]",
        "qevent" => "!qevent <event>",
        "buy" => "!buy <amount> <resource>",
        "sell" => "!sell <amount> <resource>",
        "levelall" => "!levelall <building> <level>",
        "recruit" => "!recruit <role> [amount]",
        "givepet" => "!givepet <player> <pet>",
        "pet" => "!pet <pet> (or !pet to list pets)",
        "cam" => "!cam <up|down|left|right|in|out> [amount] OR !cam home",
        "follow" => "!follow <username|me>",
        "focus" => "!focus <building> <BID>",
        "info" => "!info <resource|role|building|enemy> [id]",
        "rrole" => "!rrole <id> <role>",
        "rinfo" => "!rinfo <id>",
        "rdismiss" => "!rdismiss <id>",
        // The Unity handler consumes both values even though its validator and
        // usage table accidentally claimed there was only one argument.
        "resetid" => "!resetid <kind> <value>",
        "roles" => "!roles",
        "help" => "!help",
        "stdiscord" => "!stdiscord",
        "townstats" => "!townstats",
        "population" => "!population",
        // These source commands use the validator's `!{command}` fallback.
        "health" => "!health",
        "confirm" => "!confirm",
        "accept" => "!accept",
        "cancel" => "!cancel",
        "tbuildcosts" => "!tbuildcosts",
        "trolelimits" => "!trolelimits",
        "ping" => "!ping",
        "rulervote" => "!rulervote",
        "stopevent" => "!stopevent",
        "cobj" => "!cobj",
        "randtech" => "!randtech",
        "techvote" => "!techvote",
        "pets" => "!pets",
        "gaction" => "!gaction",
        "unlockall" => "!unlockall",
        "unlockage2" => "!unlockage2",
        "resetcam" => "!resetcam",
        "stuck" => "!stuck",
        "praise" => "!praise",
        "buildings" => "!buildings",
        "rid" => "!rid",
        "recruits" => "!recruits",
        "resign" => "!resign",
        // Bevy-only debug/convenience commands deliberately have no Unity usage.
        _ => return None,
    })
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChatCommand {
    Join,
    SelectRole(StableId),
    Role,
    Health,
    Build(StableId),
    BuildingCost(StableId),
    MoveBuilding(Vec<BuildingAction>),
    BeginBuildingLine,
    EndBuildingLine,
    ConfirmBuilding,
    CancelBuilding,
    Buildings,
    BuildingIds(StableId),
    Upgrade {
        building: StableId,
        index: u16,
    },
    RotateBuilding {
        building: StableId,
        index: u16,
        quarter_turns: i32,
    },
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
    SetNightLight([u8; 3]),
    SetNameColor([u8; 3]),
    SetBuildingNightLight {
        building: StableId,
        index: u16,
        color: [u8; 3],
    },
    Roles,
    TownStats,
    Population,
    Info {
        item: StableId,
        instance: Option<u16>,
    },
    Camera(Vec<CameraAction>),
    ResetCamera,
    Follow(Option<StableId>),
    FocusBuilding {
        building: StableId,
        index: u16,
    },
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
    Discord,
    ToggleBuildCosts,
    ToggleRoleLimits,
    AddResource {
        resource: StableId,
        amount: i32,
    },
    KillPlayer(StableId),
    GameMasterRevive(StableId),
    GiveExperience {
        player: StableId,
        amount: u32,
    },
    GiveExperienceAll(u32),
    LevelUpPlayer {
        player: StableId,
        amount: u16,
    },
    QueueEvent(StableId),
    GivePet {
        player: StableId,
        pet: StableId,
    },
    StopEvent,
    CompleteObjective,
    RandomTechnology,
    TechnologyVote,
    GameEventAction,
    UnlockAllTechnology,
    UnlockAgeTwo,
    ResetId {
        kind: StableId,
        value: StableId,
    },
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
    #[error("invalid colour {0}; use a colour name or #RRGGBB")]
    InvalidColor(String),
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
            "addresource" => {
                let resource = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let amount = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?
                    .parse::<i32>()
                    .map_err(|_| CommandParseError::InvalidAmount)?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::AddResource { resource, amount })
            }
            "givexp" => parse_player_amount(parts, false)
                .map(|(player, amount)| Self::GiveExperience { player, amount }),
            "levelup" => parse_player_amount(parts, true).and_then(|(player, amount)| {
                u16::try_from(amount)
                    .map(|amount| Self::LevelUpPlayer { player, amount })
                    .map_err(|_| CommandParseError::InvalidAmount)
            }),
            "givepet" => {
                let player = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let pet = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::GivePet { player, pet })
            }
            "resetid" => {
                let kind = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let value = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::ResetId { kind, value })
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
            "cam" => {
                let arguments = parts.collect::<Vec<_>>();
                if matches!(arguments.as_slice(), [home] if home.eq_ignore_ascii_case("home")) {
                    Ok(Self::ResetCamera)
                } else {
                    parse_camera_actions(arguments.into_iter()).map(Self::Camera)
                }
            }
            "follow" => {
                let requested = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                if requested.eq_ignore_ascii_case("me") {
                    Ok(Self::Follow(None))
                } else {
                    content_id(requested.trim_start_matches('@'))
                        .map(|requested| Self::Follow(Some(requested)))
                }
            }
            "focus" => {
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
                Ok(Self::FocusBuilding { building, index })
            }
            "info" => {
                let item = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let instance = parts.next().map(parse_index).transpose()?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::Info { item, instance })
            }
            "move" => parse_building_actions(None, parts).map(Self::MoveBuilding),
            "up" | "down" | "left" | "right" | "rotate" => {
                parse_building_actions(Some(command.as_str()), parts).map(Self::MoveBuilding)
            }
            "level" => parse_level_command(parts),
            "rotatebuilding" => {
                let building = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let index = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))
                    .and_then(parse_index)?;
                let quarter_turns = parts.next().map_or(Ok(1), |value| {
                    value
                        .parse::<i32>()
                        .ok()
                        .filter(|turns| *turns != 0)
                        .ok_or(CommandParseError::InvalidAmount)
                })?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::RotateBuilding {
                    building,
                    index,
                    quarter_turns,
                })
            }
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
            "upgrade" => {
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
                Ok(Self::Upgrade { building, index })
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
            "light" | "lightcolor" | "lightcolour" | "namecolor" | "namecolour" => {
                let value = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                let color = parse_chat_color(value)?;
                if matches!(command.as_str(), "namecolor" | "namecolour") {
                    Ok(Self::SetNameColor(color))
                } else {
                    Ok(Self::SetNightLight(color))
                }
            }
            "buildinglight" => {
                let building = content_id(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                let index = parts
                    .next()
                    .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))
                    .and_then(parse_index)?;
                let color = parse_chat_color(
                    parts
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?,
                )?;
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                Ok(Self::SetBuildingNightLight {
                    building,
                    index,
                    color,
                })
            }
            _ => {
                let argument = parts.next();
                if parts.next().is_some() {
                    return Err(CommandParseError::TooManyArguments);
                }
                match command.as_str() {
                    command if UNITY_CREATE_COMMAND_ALIASES.contains(&command) => {
                        no_argument(argument, Self::Join)
                    }
                    "experience" | "exp" => no_argument(argument, Self::Experience),
                    "health" => no_argument(argument, Self::Health),
                    "roles" => no_argument(argument, Self::Roles),
                    "townstats" => no_argument(argument, Self::TownStats),
                    "population" => no_argument(argument, Self::Population),
                    "stdiscord" => no_argument(argument, Self::Discord),
                    "buildings" => no_argument(argument, Self::Buildings),
                    "rid" => no_argument(argument, Self::RecruitIds),
                    "stuck" => no_argument(argument, Self::Unstuck),
                    "ping" => no_argument(argument, Self::Ping),
                    "resetcam" => no_argument(argument, Self::ResetCamera),
                    "save" => no_argument(argument, Self::Save),
                    "help" => no_argument(argument, Self::Help),
                    "confirm" | "accept" => no_argument(argument, Self::ConfirmBuilding),
                    "beginplace" => no_argument(argument, Self::BeginBuildingLine),
                    "endplace" => no_argument(argument, Self::EndBuildingLine),
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
                    "bid" => with_id(&command, argument, Self::BuildingIds),
                    "rinfo" => with_index(&command, argument, Self::RecruitInfo),
                    "rdismiss" => with_index(&command, argument, Self::DismissRecruit),
                    "build" => with_id(&command, argument, Self::Build),
                    "cost" | "buildcost" => with_id(&command, argument, Self::BuildingCost),
                    "vote" => with_id(&command, argument, Self::Vote),
                    "event" => with_id(&command, argument, Self::TriggerEvent),
                    "revive" => optional_id(argument).map(Self::Revive),
                    "praise" => no_argument(argument, Self::Praise),
                    "tbuildcosts" => no_argument(argument, Self::ToggleBuildCosts),
                    "trolelimits" => no_argument(argument, Self::ToggleRoleLimits),
                    "stopevent" => no_argument(argument, Self::StopEvent),
                    "cobj" => no_argument(argument, Self::CompleteObjective),
                    "randtech" => no_argument(argument, Self::RandomTechnology),
                    "techvote" => no_argument(argument, Self::TechnologyVote),
                    "gaction" => no_argument(argument, Self::GameEventAction),
                    "unlockall" => no_argument(argument, Self::UnlockAllTechnology),
                    "unlockage2" => no_argument(argument, Self::UnlockAgeTwo),
                    "kill" => with_id(&command, argument, Self::KillPlayer),
                    "grevive" => with_id(&command, argument, Self::GameMasterRevive),
                    "qevent" => with_id(&command, argument, Self::QueueEvent),
                    "givexpall" => argument
                        .ok_or_else(|| CommandParseError::MissingArgument(command.clone()))?
                        .parse::<u32>()
                        .ok()
                        .filter(|amount| *amount > 0)
                        .map(Self::GiveExperienceAll)
                        .ok_or(CommandParseError::InvalidAmount),
                    _ => Err(CommandParseError::Unknown(command)),
                }
            }
        }
    }
}

fn parse_player_amount<'a>(
    mut parts: impl Iterator<Item = &'a str>,
    default_amount: bool,
) -> Result<(StableId, u32), CommandParseError> {
    let player = content_id(
        parts
            .next()
            .ok_or_else(|| CommandParseError::MissingArgument("player".to_owned()))?,
    )?;
    let amount = parts
        .next()
        .map(str::parse::<u32>)
        .transpose()
        .map_err(|_| CommandParseError::InvalidAmount)?
        .unwrap_or(u32::from(default_amount));
    if amount == 0 {
        return Err(CommandParseError::InvalidAmount);
    }
    if parts.next().is_some() {
        return Err(CommandParseError::TooManyArguments);
    }
    Ok((player, amount))
}

fn content_id(value: &str) -> Result<StableId, CommandParseError> {
    StableId::new(value.to_ascii_lowercase().replace(' ', "_"))
        .map_err(|error| CommandParseError::InvalidId(error.to_string()))
}

fn parse_chat_color(value: &str) -> Result<[u8; 3], CommandParseError> {
    let normalized = value.trim().to_ascii_lowercase();
    let named = match normalized.as_str() {
        "red" => Some([255, 64, 64]),
        "orange" => Some([255, 144, 48]),
        "yellow" => Some([255, 224, 72]),
        "green" => Some([72, 224, 112]),
        "cyan" | "aqua" => Some([64, 224, 255]),
        "blue" => Some([72, 128, 255]),
        "purple" | "violet" => Some([176, 96, 255]),
        "pink" => Some([255, 112, 192]),
        "white" => Some([255, 255, 255]),
        "warmwhite" | "warm-white" => Some([255, 224, 176]),
        _ => None,
    };
    if let Some(color) = named {
        return Ok(color);
    }
    let hex = normalized.strip_prefix('#').unwrap_or(&normalized);
    if hex.len() != 6 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(CommandParseError::InvalidColor(value.to_owned()));
    }
    let channel = |range| {
        u8::from_str_radix(&hex[range], 16)
            .map_err(|_| CommandParseError::InvalidColor(value.to_owned()))
    };
    Ok([channel(0..2)?, channel(2..4)?, channel(4..6)?])
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
    use std::collections::BTreeSet;

    const UNITY_COMMAND_DICTIONARY_SOURCE: &str =
        include_str!("../../../../Assets/Scripts/Twitch/Commands/CommandDictionary.cs");

    fn source_registered_commands() -> BTreeSet<String> {
        UNITY_COMMAND_DICTIONARY_SOURCE
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .filter_map(|line| line.split(".Add(\"").nth(1))
            .filter_map(|tail| tail.split('"').next())
            .map(str::to_owned)
            .collect()
    }

    fn valid_source_command(command: &str) -> String {
        let arguments = match command {
            "build" | "bid" => " house",
            "upgrade" | "remove" => " house 1",
            "move" | "cam" => " up 2",
            "hair" | "facialhair" | "eyes" | "body" | "haircolor" | "eyecolor" | "vote"
            | "rinfo" | "rdismiss" | "givexpall" => " 1",
            "addresource" => " wood 1",
            "modrole" => " viewer logger",
            "kill" | "grevive" => " viewer",
            "givexp" | "levelup" => " viewer 1",
            "qevent" => " fishgod",
            "buy" | "sell" => " 8 wood",
            "levelall" => " house 2",
            "recruit" => " logger 1",
            "givepet" => " viewer duck",
            "info" => " wood",
            "rrole" => " 1 logger",
            "resetid" => " building house",
            _ => "",
        };
        format!("!{command}{arguments}")
    }

    #[test]
    fn every_source_registered_command_has_usage_and_a_valid_bevy_parser_path() {
        let commands = source_registered_commands();
        assert_eq!(commands.len(), 68, "Unity command surface changed");
        for command in commands {
            let input = valid_source_command(&command);
            assert!(
                unity_command_usage(&input).is_some(),
                "missing Unity usage for {input}"
            );
            assert!(
                input.parse::<ChatCommand>().is_ok(),
                "missing Bevy parser path for {input}"
            );
        }
        for alias in UNITY_CREATE_COMMAND_ALIASES {
            let input = format!("!{alias}");
            assert_eq!(input.parse(), Ok(ChatCommand::Join));
            assert_eq!(unity_command_usage(&input), Some("!join"));
        }
    }

    #[test]
    fn parses_shipping_command_grammar() {
        assert_eq!("!join".parse(), Ok(ChatCommand::Join));
        for alias in UNITY_CREATE_COMMAND_ALIASES {
            assert_eq!(format!("!{alias}").parse(), Ok(ChatCommand::Join));
        }
        assert_eq!("!experience".parse(), Ok(ChatCommand::Experience));
        assert_eq!("!exp".parse(), Ok(ChatCommand::Experience));
        assert_eq!("!population".parse(), Ok(ChatCommand::Population));
        assert_eq!(
            "!build building:house".parse(),
            Ok(ChatCommand::Build(StableId::new("building:house").unwrap()))
        );
        assert_eq!(
            "!cost ore_storage".parse(),
            Ok(ChatCommand::BuildingCost(
                StableId::new("ore_storage").unwrap()
            ))
        );
        assert_eq!(
            "!buildcost house".parse(),
            Ok(ChatCommand::BuildingCost(StableId::new("house").unwrap()))
        );
        assert_eq!(
            "!upgrade Tower 3".parse(),
            Ok(ChatCommand::Upgrade {
                building: StableId::new("tower").unwrap(),
                index: 3,
            })
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
        assert_eq!("!beginplace".parse(), Ok(ChatCommand::BeginBuildingLine));
        assert_eq!("!endplace".parse(), Ok(ChatCommand::EndBuildingLine));
        assert_eq!(
            "!upgrade OreStorage 4".parse(),
            Ok(ChatCommand::Upgrade {
                building: StableId::new("orestorage").unwrap(),
                index: 4,
            })
        );
        assert_eq!(
            "!rotatebuilding Tower 4 -1".parse(),
            Ok(ChatCommand::RotateBuilding {
                building: StableId::new("tower").unwrap(),
                index: 4,
                quarter_turns: -1,
            })
        );
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
        assert_eq!("!cam home".parse(), Ok(ChatCommand::ResetCamera));
        assert_eq!("!follow me".parse(), Ok(ChatCommand::Follow(None)));
        assert_eq!(
            "!follow @Some_Viewer".parse(),
            Ok(ChatCommand::Follow(Some(
                StableId::new("some_viewer").unwrap()
            )))
        );
        assert_eq!(
            "!info house 2".parse(),
            Ok(ChatCommand::Info {
                item: StableId::new("house").unwrap(),
                instance: Some(2),
            })
        );
        assert_eq!(
            "!body 3".parse(),
            Ok(ChatCommand::Customize {
                kind: CustomizationKind::Body,
                index: 3,
            })
        );
        assert_eq!(
            "!light warmwhite".parse(),
            Ok(ChatCommand::SetNightLight([255, 224, 176]))
        );
        assert_eq!(
            "!namecolour #72C8FF".parse(),
            Ok(ChatCommand::SetNameColor([0x72, 0xC8, 0xFF]))
        );
        assert_eq!(
            "!buildinglight House 2 blue".parse(),
            Ok(ChatCommand::SetBuildingNightLight {
                building: StableId::new("house").unwrap(),
                index: 2,
                color: [72, 128, 255],
            })
        );
        assert_eq!("!rulervote".parse(), Ok(ChatCommand::StartRulerVote));
        assert_eq!("!stdiscord".parse(), Ok(ChatCommand::Discord));
        assert_eq!("!tbuildcosts".parse(), Ok(ChatCommand::ToggleBuildCosts));
        assert_eq!("!trolelimits".parse(), Ok(ChatCommand::ToggleRoleLimits));
        assert_eq!("!stopevent".parse(), Ok(ChatCommand::StopEvent));
        assert_eq!("!cobj".parse(), Ok(ChatCommand::CompleteObjective));
        assert_eq!("!randtech".parse(), Ok(ChatCommand::RandomTechnology));
        assert_eq!("!techvote".parse(), Ok(ChatCommand::TechnologyVote));
        assert_eq!("!gaction".parse(), Ok(ChatCommand::GameEventAction));
        assert_eq!("!unlockall".parse(), Ok(ChatCommand::UnlockAllTechnology));
        assert_eq!("!unlockage2".parse(), Ok(ChatCommand::UnlockAgeTwo));
        assert_eq!(
            "!addresource wood -20".parse(),
            Ok(ChatCommand::AddResource {
                resource: StableId::new("wood").unwrap(),
                amount: -20,
            })
        );
        assert_eq!(
            "!kill viewer".parse(),
            Ok(ChatCommand::KillPlayer(StableId::new("viewer").unwrap()))
        );
        assert_eq!(
            "!grevive viewer".parse(),
            Ok(ChatCommand::GameMasterRevive(
                StableId::new("viewer").unwrap()
            ))
        );
        assert_eq!(
            "!focus OreStorage 3".parse(),
            Ok(ChatCommand::FocusBuilding {
                building: StableId::new("orestorage").unwrap(),
                index: 3,
            })
        );
        assert_eq!(
            "!givexp viewer 200000".parse(),
            Ok(ChatCommand::GiveExperience {
                player: StableId::new("viewer").unwrap(),
                amount: 200_000,
            })
        );
        assert_eq!(
            "!givexpall 500".parse(),
            Ok(ChatCommand::GiveExperienceAll(500))
        );
        assert_eq!(
            "!levelup viewer".parse(),
            Ok(ChatCommand::LevelUpPlayer {
                player: StableId::new("viewer").unwrap(),
                amount: 1,
            })
        );
        assert_eq!(
            "!qevent monsterraid".parse(),
            Ok(ChatCommand::QueueEvent(
                StableId::new("monsterraid").unwrap()
            ))
        );
        assert_eq!(
            "!givepet viewer redpanda".parse(),
            Ok(ChatCommand::GivePet {
                player: StableId::new("viewer").unwrap(),
                pet: StableId::new("redpanda").unwrap(),
            })
        );
        assert_eq!(
            "!resetid building house".parse(),
            Ok(ChatCommand::ResetId {
                kind: StableId::new("building").unwrap(),
                value: StableId::new("house").unwrap(),
            })
        );
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

    #[test]
    fn reports_source_authored_usage_for_invalid_shipping_commands() {
        assert_eq!(
            unity_command_usage("!move sideways"),
            Some("!move <up|down|left|right|rotate> [amount]")
        );
        assert_eq!(unity_command_usage("!CrTeA extra"), Some("!join"));
        assert_eq!(unity_command_usage("!health extra"), Some("!health"));
        assert_eq!(unity_command_usage("!not-a-command"), None);
    }
}
