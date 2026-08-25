use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{GridPos, ObjectiveDef, ObjectiveKind, StableId};

pub const BUILDING_MAX_HEALTH: i32 = 500;
pub const CURRENT_SIMULATION_SCHEMA: u32 = 2;
pub const MAX_ROLE_LEVEL: u16 = 99;
pub const RULER_VOTE_DURATION_SECONDS: f32 = 120.0;
pub const RULER_VOTE_INTERVAL_SECONDS: f32 = 3_600.0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RoleProgress {
    pub level: u16,
    pub experience: u32,
}

impl Default for RoleProgress {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Weather {
    Clear,
    Rain,
    Fog,
    Snow,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TownEvent {
    ResourceBoom(StableId),
    EnemyRaid,
    Festival,
    HarshWeather,
    FishGod,
}

/// The highest-priority Unity/Twitch privilege class retained for presentation and permissions.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum StreamUserType {
    GameMaster,
    Broadcaster,
    Moderator,
    Subscriber,
    #[default]
    Normal,
}

impl StreamUserType {
    #[must_use]
    pub const fn is_staff_or_subscriber(self) -> bool {
        !matches!(self, Self::Normal)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActorState {
    pub id: StableId,
    /// Twitch display name or imported legacy username. Stable IDs remain authoritative.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login_name: Option<String>,
    #[serde(default)]
    pub user_type: StreamUserType,
    pub role: StableId,
    /// Authored prefab archetype. Old native saves omit this field and use runtime fallbacks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<StableId>,
    pub position: GridPos,
    /// Last successful Twitch building placement, matching Unity's per-player cursor origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_building_position: Option<GridPos>,
    /// Cumulative 90-degree placer turns retained between builds.
    #[serde(default)]
    pub building_rotation_quarter_turns: i32,
    pub health: i32,
    pub max_health: i32,
    pub alive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub respawn_remaining_seconds: Option<f64>,
    pub inventory: BTreeMap<StableId, u32>,
    /// Stable runtime building ID, or `building:townhall` for the initial station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station: Option<StableId>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub role_progression: BTreeMap<StableId, RoleProgress>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub unlocked_pets: BTreeSet<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_pet: Option<StableId>,
    /// Explicit target selected through `!target`; automatic acquisition is the fallback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_target: Option<StableId>,
    /// Unity customization indices are one-based at the command boundary and zero-based here.
    #[serde(default, skip_serializing_if = "ActorCustomization::is_default")]
    pub customization: ActorCustomization,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActorCustomization {
    pub eyes: u8,
    pub hair: u8,
    pub facial_hair: u8,
    pub hair_color: u8,
    pub eye_color: u8,
    pub body_type: u8,
}

impl ActorCustomization {
    #[must_use]
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BuildingState {
    pub id: StableId,
    pub archetype: StableId,
    pub position: GridPos,
    /// Clockwise 90-degree turns. Values are normalized only for presentation and footprint use.
    #[serde(default)]
    pub rotation_quarter_turns: i32,
    pub level: u16,
    pub health: i32,
    pub complete: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnemyCampState {
    pub id: StableId,
    pub archetype: StableId,
    pub position: GridPos,
    pub health: i32,
    pub spawn_remaining_seconds: f64,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub spawned_enemies: BTreeSet<StableId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RaidState {
    pub current_wave: u16,
    pub total_waves: u16,
    pub enemies_per_wave: u16,
    pub enemy_archetype: StableId,
    pub boss_archetype: StableId,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub tracked_enemies: BTreeSet<StableId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FishGodState {
    pub praises_given: u16,
    pub praises_required: u16,
    pub remaining_seconds: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TechVote {
    pub technology: StableId,
    pub remaining_seconds: f32,
    pub votes: BTreeMap<StableId, bool>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RulerVoteKind {
    NewRuler,
    KeepRuler,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RulerVoteState {
    pub kind: RulerVoteKind,
    pub remaining_seconds: f32,
    pub votes: BTreeMap<StableId, StableId>,
    /// Preserves Unity's first-option tie behavior while remaining deterministic.
    pub option_order: Vec<StableId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ObjectiveProgress {
    pub objective: StableId,
    pub amount: u32,
    pub required_amount: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TownGoalState {
    pub technology: StableId,
    pub objectives: Vec<ObjectiveProgress>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveEvent {
    BuildingBuilt(StableId),
    ResourceGained { resource: StableId, amount: u32 },
    EnemyKilled(StableId),
    ResourceSold { resource: StableId, amount: u32 },
    ResourceBought { resource: StableId, amount: u32 },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WorldSimulation {
    pub schema_version: u32,
    pub world_seed: u64,
    pub elapsed_seconds: f64,
    pub day: u32,
    pub season: Season,
    pub weather: Weather,
    pub town_resources: BTreeMap<StableId, u32>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// Fractional passive-income numerators keyed by stable building and resource IDs.
    /// Values use the runtime's milli-resource/nanosecond fixed-point denominator.
    pub passive_resource_accumulators: BTreeMap<StableId, BTreeMap<StableId, u64>>,
    pub actors: BTreeMap<StableId, ActorState>,
    pub buildings: BTreeMap<StableId, BuildingState>,
    #[serde(default)]
    pub next_enemy_serial: u64,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub enemy_camps: BTreeMap<StableId, EnemyCampState>,
    pub unlocked_technology: BTreeSet<StableId>,
    pub active_vote: Option<TechVote>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_goals: Vec<TownGoalState>,
    pub active_event: Option<TownEvent>,
    #[serde(default, skip_serializing_if = "VecDeque::is_empty")]
    pub queued_events: VecDeque<TownEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_raid: Option<RaidState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fish_god: Option<FishGodState>,
    #[serde(default)]
    pub fish_god_attempts: u64,
    #[serde(default)]
    pub gathering_pet_attempts: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_ruler: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ruler_previous_role: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ruler_vote: Option<RulerVoteState>,
    #[serde(
        default = "default_ruler_vote_cooldown",
        skip_serializing_if = "is_default_ruler_vote_cooldown"
    )]
    pub ruler_vote_cooldown_seconds: f32,
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub ruler_vote_scheduled: bool,
    /// Unity-compatible game-master toggle. Disabled costs pass an empty cost map
    /// through the same construction and upgrade transactions.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub building_costs_enabled: bool,
    /// Unity-compatible game-master toggle for authored per-role user caps.
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub role_limits_enabled: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SimulationError {
    #[error("actor {0} does not exist")]
    MissingActor(StableId),
    #[error("actor {0} is not alive")]
    ActorDead(StableId),
    #[error("building {0} already exists")]
    DuplicateBuilding(StableId),
    #[error("building {0} does not exist")]
    MissingBuilding(StableId),
    #[error("building {0} is still under construction")]
    BuildingIncomplete(StableId),
    #[error("building {building} is already at maximum level {max_level}")]
    BuildingMaxLevel { building: StableId, max_level: u16 },
    #[error("town lacks {resource}: required {required}, available {available}")]
    InsufficientResource {
        resource: StableId,
        required: u32,
        available: u32,
    },
    #[error("a technology vote is already active")]
    VoteActive,
    #[error("there is no active technology vote")]
    NoVote,
    #[error("actor {0} has already voted")]
    AlreadyVoted(StableId),
    #[error("trade amount must be non-zero")]
    EmptyTrade,
    #[error("resource {0} cannot be traded")]
    InvalidTradeResource(StableId),
    #[error("actor {0} is not dead")]
    ActorAlive(StableId),
    #[error("there is no active Fish God event")]
    NoFishGodEvent,
    #[error("a ruler vote is already active")]
    RulerVoteActive,
    #[error("there is no active ruler vote")]
    NoRulerVote,
    #[error("{0} is not a valid ruler vote option")]
    InvalidRulerVoteOption(StableId),
    #[error("actor {0} is not the current ruler")]
    NotRuler(StableId),
    #[error("the current ruler cannot change role before resigning")]
    RulerRoleLocked,
}

impl WorldSimulation {
    #[must_use]
    pub fn new(world_seed: u64) -> Self {
        Self {
            schema_version: CURRENT_SIMULATION_SCHEMA,
            world_seed,
            elapsed_seconds: 0.0,
            day: 0,
            season: Season::Spring,
            weather: Weather::Clear,
            town_resources: BTreeMap::new(),
            passive_resource_accumulators: BTreeMap::new(),
            actors: BTreeMap::new(),
            buildings: BTreeMap::new(),
            next_enemy_serial: 0,
            enemy_camps: BTreeMap::new(),
            unlocked_technology: BTreeSet::new(),
            active_vote: None,
            active_goals: Vec::new(),
            active_event: None,
            queued_events: VecDeque::new(),
            active_raid: None,
            fish_god: None,
            fish_god_attempts: 0,
            gathering_pet_attempts: 0,
            current_ruler: None,
            ruler_previous_role: None,
            ruler_vote: None,
            ruler_vote_cooldown_seconds: 30.0,
            ruler_vote_scheduled: true,
            building_costs_enabled: true,
            role_limits_enabled: true,
        }
    }

    pub fn toggle_building_costs(&mut self) -> bool {
        self.building_costs_enabled = !self.building_costs_enabled;
        self.building_costs_enabled
    }

    pub fn toggle_role_limits(&mut self) -> bool {
        self.role_limits_enabled = !self.role_limits_enabled;
        self.role_limits_enabled
    }

    pub fn adjust_town_resource(&mut self, resource: StableId, amount: i32) -> u32 {
        let value = self.town_resources.entry(resource).or_default();
        *value = value.saturating_add_signed(amount);
        *value
    }

    pub fn join_player(&mut self, id: StableId, position: GridPos) -> bool {
        if self.actors.contains_key(&id) {
            return false;
        }
        self.actors.insert(
            id.clone(),
            ActorState {
                id,
                display_name: None,
                login_name: None,
                user_type: StreamUserType::Normal,
                role: StableId::new("role:villager").expect("static stable ID"),
                archetype: None,
                position,
                last_building_position: None,
                building_rotation_quarter_turns: 0,
                health: 100,
                max_health: 100,
                alive: true,
                respawn_remaining_seconds: None,
                inventory: BTreeMap::new(),
                station: None,
                role_progression: BTreeMap::from([(
                    StableId::new("role:villager").expect("static stable ID"),
                    RoleProgress::default(),
                )]),
                unlocked_pets: BTreeSet::new(),
                active_pet: None,
                preferred_target: None,
                customization: ActorCustomization::default(),
            },
        );
        true
    }

    pub fn spawn_enemy(
        &mut self,
        id: StableId,
        archetype: StableId,
        position: GridPos,
        max_health: i32,
    ) -> bool {
        if self.actors.contains_key(&id) || max_health <= 0 {
            return false;
        }
        self.actors.insert(
            id.clone(),
            ActorState {
                id,
                display_name: None,
                login_name: None,
                user_type: StreamUserType::Normal,
                role: StableId::new("role:enemy").expect("static stable ID"),
                archetype: Some(archetype),
                position,
                last_building_position: None,
                building_rotation_quarter_turns: 0,
                health: max_health,
                max_health,
                alive: true,
                respawn_remaining_seconds: None,
                inventory: BTreeMap::new(),
                station: None,
                role_progression: BTreeMap::new(),
                unlocked_pets: BTreeSet::new(),
                active_pet: None,
                preferred_target: None,
                customization: ActorCustomization::default(),
            },
        );
        true
    }

    pub fn start_raid(
        &mut self,
        total_waves: u16,
        enemies_per_wave: u16,
        enemy_archetype: StableId,
        boss_archetype: StableId,
    ) -> bool {
        if self.active_raid.is_some() || total_waves == 0 || enemies_per_wave == 0 {
            return false;
        }
        self.active_event = Some(TownEvent::EnemyRaid);
        self.active_raid = Some(RaidState {
            current_wave: 0,
            total_waves,
            enemies_per_wave,
            enemy_archetype,
            boss_archetype,
            tracked_enemies: BTreeSet::new(),
        });
        true
    }

    pub fn finish_raid(&mut self) {
        self.active_raid = None;
        if self.active_event == Some(TownEvent::EnemyRaid) {
            self.active_event = None;
        }
    }

    pub fn start_fish_god(&mut self, force: bool) -> bool {
        if self.active_event.is_some() || self.fish_god.is_some() {
            return false;
        }
        self.fish_god_attempts = self.fish_god_attempts.saturating_add(1);
        let answered = deterministic_fish_god_value(self.world_seed, self.fish_god_attempts)
            .is_multiple_of(10);
        if !force && !answered {
            return false;
        }
        self.active_event = Some(TownEvent::FishGod);
        self.fish_god = Some(FishGodState {
            praises_given: 0,
            praises_required: 20,
            remaining_seconds: 300.0,
        });
        true
    }

    pub fn praise_fish_god(&mut self, actor: &StableId) -> Result<bool, SimulationError> {
        let player = self
            .actors
            .get(actor)
            .ok_or_else(|| SimulationError::MissingActor(actor.clone()))?;
        if !player.alive {
            return Err(SimulationError::ActorDead(actor.clone()));
        }
        self.action_fish_god()
    }

    /// Applies the current event's action without requiring a player actor, matching
    /// Unity's game-master `!gaction` path.
    pub fn action_fish_god(&mut self) -> Result<bool, SimulationError> {
        let Some(event) = &mut self.fish_god else {
            return Err(SimulationError::NoFishGodEvent);
        };
        event.praises_given = event.praises_given.saturating_add(1);
        if event.praises_given < event.praises_required {
            return Ok(false);
        }
        let food = StableId::new("resource:food").expect("static stable ID");
        let current = self.town_resources.get(&food).copied().unwrap_or_default();
        self.town_resources
            .insert(food, current.saturating_add(1_000));
        let twitch_players: Vec<_> = self
            .actors
            .keys()
            .filter(|id| id.as_str().starts_with("twitch:"))
            .cloned()
            .collect();
        let roll = deterministic_fish_god_value(self.world_seed, self.fish_god_attempts + 1);
        if !twitch_players.is_empty() && roll % 100 < 70 {
            let index = usize::try_from(roll % twitch_players.len() as u64).unwrap_or(0);
            if let Some(recipient) = self.actors.get_mut(&twitch_players[index]) {
                recipient
                    .unlocked_pets
                    .insert(StableId::new("pet:fish_god").expect("static stable ID"));
            }
        }
        self.fish_god = None;
        self.active_event = None;
        Ok(true)
    }

    /// Queues a unique event type. Unity rejects duplicates that are active or queued.
    pub fn queue_event(&mut self, event: TownEvent) -> bool {
        if self.active_event.as_ref() == Some(&event) || self.queued_events.contains(&event) {
            return false;
        }
        self.queued_events.push_back(event);
        true
    }

    pub fn take_next_queued_event(&mut self) -> Option<TownEvent> {
        if self.active_event.is_some() {
            None
        } else {
            self.queued_events.pop_front()
        }
    }

    /// Stops the current event and returns raid actors that the runtime must despawn.
    pub fn stop_active_event(&mut self) -> Vec<StableId> {
        let raid_actors = self
            .active_raid
            .take()
            .map_or_else(Vec::new, |raid| raid.tracked_enemies.into_iter().collect());
        self.fish_god = None;
        self.active_event = None;
        raid_actors
    }

    pub fn start_ruler_vote(&mut self, kind: RulerVoteKind) -> Result<(), SimulationError> {
        if self.ruler_vote.is_some() {
            return Err(SimulationError::RulerVoteActive);
        }
        let kind = if kind == RulerVoteKind::KeepRuler && self.current_ruler.is_none() {
            RulerVoteKind::NewRuler
        } else {
            kind
        };
        let option_order = if kind == RulerVoteKind::KeepRuler {
            vec![stable("yes"), stable("no")]
        } else {
            Vec::new()
        };
        self.ruler_vote = Some(RulerVoteState {
            kind,
            remaining_seconds: RULER_VOTE_DURATION_SECONDS,
            votes: BTreeMap::new(),
            option_order,
        });
        self.ruler_vote_scheduled = false;
        Ok(())
    }

    pub fn cast_ruler_vote(
        &mut self,
        voter: &StableId,
        option: StableId,
    ) -> Result<(), SimulationError> {
        if !self.actors.contains_key(voter) {
            return Err(SimulationError::MissingActor(voter.clone()));
        }
        let kind = self
            .ruler_vote
            .as_ref()
            .ok_or(SimulationError::NoRulerVote)?
            .kind;
        if self
            .ruler_vote
            .as_ref()
            .is_some_and(|vote| vote.votes.contains_key(voter))
        {
            return Err(SimulationError::AlreadyVoted(voter.clone()));
        }
        match kind {
            RulerVoteKind::KeepRuler => {
                if !matches!(option.as_str(), "yes" | "no") {
                    return Err(SimulationError::InvalidRulerVoteOption(option));
                }
            }
            RulerVoteKind::NewRuler => {
                self.actors
                    .get(&option)
                    .filter(|actor| actor.role.as_str() != "role:enemy" && actor.alive)
                    .ok_or_else(|| SimulationError::InvalidRulerVoteOption(option.clone()))?;
            }
        }
        let vote = self
            .ruler_vote
            .as_mut()
            .expect("ruler vote was validated above");
        if kind == RulerVoteKind::NewRuler && !vote.option_order.contains(&option) {
            vote.option_order.push(option.clone());
        }
        vote.votes.insert(voter.clone(), option);
        Ok(())
    }

    pub fn set_ruler(&mut self, actor: StableId) -> Result<(), SimulationError> {
        if !self.actors.contains_key(&actor) {
            return Err(SimulationError::MissingActor(actor));
        }
        if self.current_ruler.as_ref() == Some(&actor) {
            return Ok(());
        }
        self.clear_ruler();
        let previous_role = self.actors[&actor].role.clone();
        let ruler_role = stable("role:ruler");
        let state = self
            .actors
            .get_mut(&actor)
            .expect("ruler candidate was validated");
        state.role = ruler_role.clone();
        state.role_progression.entry(ruler_role).or_default();
        self.current_ruler = Some(actor);
        self.ruler_previous_role = Some(previous_role);
        Ok(())
    }

    pub fn clear_ruler(&mut self) {
        if let Some(ruler) = self.current_ruler.take()
            && let Some(actor) = self.actors.get_mut(&ruler)
            && let Some(previous) = self.ruler_previous_role.take()
        {
            actor.role = previous.clone();
            actor.role_progression.entry(previous).or_default();
        }
        self.ruler_previous_role = None;
    }

    pub fn resign_ruler(&mut self, actor: &StableId) -> Result<(), SimulationError> {
        if self.current_ruler.as_ref() != Some(actor) {
            return Err(SimulationError::NotRuler(actor.clone()));
        }
        self.clear_ruler();
        self.ruler_vote = None;
        self.start_ruler_vote(RulerVoteKind::NewRuler)
    }

    #[must_use]
    pub fn is_ruler(&self, actor: &StableId) -> bool {
        self.current_ruler.as_ref() == Some(actor)
    }

    /// Resolves a completed ballot and returns the elected/retained ruler.
    pub fn resolve_ruler_vote(&mut self) -> Option<StableId> {
        let vote = self.ruler_vote.as_ref()?;
        if vote.remaining_seconds > f32::EPSILON || vote.votes.is_empty() {
            return None;
        }
        let mut tallies = BTreeMap::<StableId, usize>::new();
        for option in vote.votes.values() {
            *tallies.entry(option.clone()).or_default() += 1;
        }
        let winner = vote
            .option_order
            .iter()
            .cloned()
            .fold(None::<(StableId, usize)>, |best, option| {
                let tally = tallies.get(&option).copied().unwrap_or_default();
                match best {
                    Some((_, best_tally)) if best_tally >= tally => best,
                    _ => Some((option, tally)),
                }
            })?
            .0;
        let kind = vote.kind;
        self.ruler_vote = None;
        match kind {
            RulerVoteKind::NewRuler => {
                self.set_ruler(winner.clone()).ok()?;
                self.schedule_next_ruler_vote();
                Some(winner)
            }
            RulerVoteKind::KeepRuler if winner.as_str() == "yes" => {
                self.schedule_next_ruler_vote();
                self.current_ruler.clone()
            }
            RulerVoteKind::KeepRuler => {
                self.clear_ruler();
                let _ = self.start_ruler_vote(RulerVoteKind::NewRuler);
                None
            }
        }
    }

    pub fn schedule_next_ruler_vote(&mut self) {
        self.ruler_vote_scheduled = true;
        self.ruler_vote_cooldown_seconds = RULER_VOTE_INTERVAL_SECONDS;
    }

    pub fn assign_role(&mut self, actor: &StableId, role: StableId) -> Result<(), SimulationError> {
        if self.current_ruler.as_ref() == Some(actor) && role.as_str() != "role:ruler" {
            return Err(SimulationError::RulerRoleLocked);
        }
        let actor_state = self.actor_mut(actor)?;
        actor_state.role = role.clone();
        actor_state.station = None;
        actor_state.preferred_target = None;
        actor_state.role_progression.entry(role).or_default();
        Ok(())
    }

    pub fn grant_role_experience(
        &mut self,
        actor: &StableId,
        amount: u32,
        multiplier_per_thousand: u32,
    ) -> Result<u16, SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        let progress = actor_state
            .role_progression
            .entry(actor_state.role.clone())
            .or_default();
        if progress.level >= MAX_ROLE_LEVEL {
            progress.experience = 0;
            return Ok(0);
        }

        let adjusted = u64::from(amount).saturating_mul(u64::from(multiplier_per_thousand)) / 1_000;
        let adjusted = u32::try_from(adjusted).unwrap_or(u32::MAX).max(1);
        progress.experience = progress.experience.saturating_add(adjusted);
        let mut levels_gained = 0_u16;
        while progress.level < MAX_ROLE_LEVEL {
            let required = required_role_experience(progress.level);
            if progress.experience < required {
                break;
            }
            progress.experience -= required;
            progress.level += 1;
            levels_gained += 1;
        }
        Ok(levels_gained)
    }

    pub fn grant_role_levels(
        &mut self,
        actor: &StableId,
        amount: u16,
    ) -> Result<u16, SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        let progress = actor_state
            .role_progression
            .entry(actor_state.role.clone())
            .or_default();
        let before = progress.level;
        progress.level = progress.level.saturating_add(amount).min(MAX_ROLE_LEVEL);
        if progress.level >= MAX_ROLE_LEVEL {
            progress.experience = 0;
        }
        if actor_state.alive {
            actor_state.health = actor_state.max_health;
        }
        let levels_gained = progress.level - before;
        Ok(levels_gained)
    }

    pub fn unlock_pet(&mut self, actor: &StableId, pet: StableId) -> Result<bool, SimulationError> {
        Ok(self.actor_mut(actor)?.unlocked_pets.insert(pet))
    }

    pub fn gather(
        &mut self,
        actor: &StableId,
        resource: StableId,
        amount: u32,
    ) -> Result<(), SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        if !actor_state.alive {
            return Err(SimulationError::ActorDead(actor.clone()));
        }
        let current = actor_state.inventory.entry(resource).or_default();
        *current = current.saturating_add(amount);
        Ok(())
    }

    pub fn try_unlock_gathering_pet(
        &mut self,
        actor: &StableId,
        pet: StableId,
    ) -> Result<bool, SimulationError> {
        self.gathering_pet_attempts = self.gathering_pet_attempts.saturating_add(1);
        let unlocked = deterministic_fish_god_value(
            self.world_seed ^ 0x7065_745f_6472_6f70,
            self.gathering_pet_attempts,
        )
        .is_multiple_of(5_000);
        if !unlocked {
            return Ok(false);
        }
        Ok(self.actor_mut(actor)?.unlocked_pets.insert(pet))
    }

    pub fn deposit_all(&mut self, actor: &StableId) -> Result<u32, SimulationError> {
        self.deposit_all_with_capacities(actor, &BTreeMap::new())
    }

    /// Deposits inventory without allowing configured town storage to overflow.
    /// Resources without an entry remain unbounded for backwards-compatible domain callers.
    pub fn deposit_all_with_capacities(
        &mut self,
        actor: &StableId,
        capacities: &BTreeMap<StableId, u32>,
    ) -> Result<u32, SimulationError> {
        let inventory = std::mem::take(&mut self.actor_mut(actor)?.inventory);
        let mut total = 0_u32;
        let mut overflow = BTreeMap::new();
        for (resource, amount) in inventory {
            let current = self.town_resources.entry(resource.clone()).or_default();
            let capacity = capacities.get(&resource).copied().unwrap_or(u32::MAX);
            let deposited = amount.min(capacity.saturating_sub(*current));
            *current = current.saturating_add(deposited);
            total = total.saturating_add(deposited);
            if deposited < amount {
                overflow.insert(resource, amount - deposited);
            }
        }
        self.actor_mut(actor)?.inventory = overflow;
        Ok(total)
    }

    /// Deposits one resource while preserving every other inventory entry.
    ///
    /// Unity workers retain resources collected by previous roles and only
    /// transfer the resource assigned to their current role at a station.
    pub fn deposit_resource_with_capacity(
        &mut self,
        actor: &StableId,
        resource: &StableId,
        capacity: u32,
    ) -> Result<u32, SimulationError> {
        let amount = self
            .actor_mut(actor)?
            .inventory
            .remove(resource)
            .unwrap_or_default();
        if amount == 0 {
            return Ok(0);
        }

        let current = self.town_resources.entry(resource.clone()).or_default();
        let deposited = amount.min(capacity.saturating_sub(*current));
        *current = current.saturating_add(deposited);
        if deposited < amount {
            self.actor_mut(actor)?
                .inventory
                .insert(resource.clone(), amount - deposited);
        }
        Ok(deposited)
    }

    pub fn construct(
        &mut self,
        id: StableId,
        archetype: StableId,
        position: GridPos,
        max_health: u32,
        cost: &BTreeMap<StableId, u32>,
    ) -> Result<(), SimulationError> {
        self.construct_rotated(id, archetype, position, 0, max_health, cost)
    }

    pub fn construct_rotated(
        &mut self,
        id: StableId,
        archetype: StableId,
        position: GridPos,
        rotation_quarter_turns: i32,
        max_health: u32,
        cost: &BTreeMap<StableId, u32>,
    ) -> Result<(), SimulationError> {
        if self.buildings.contains_key(&id) {
            return Err(SimulationError::DuplicateBuilding(id));
        }
        self.spend_resources(cost)?;
        self.buildings.insert(
            id.clone(),
            BuildingState {
                id,
                archetype,
                position,
                rotation_quarter_turns,
                level: 1,
                health: i32::try_from(max_health.div_ceil(10)).unwrap_or(i32::MAX),
                complete: false,
            },
        );
        Ok(())
    }

    pub fn work_on_building(
        &mut self,
        building: &StableId,
        amount: u32,
        max_health: u32,
    ) -> Result<bool, SimulationError> {
        let state = self
            .buildings
            .get_mut(building)
            .ok_or_else(|| SimulationError::MissingBuilding(building.clone()))?;
        if state.complete {
            return Ok(true);
        }
        let amount = i32::try_from(amount).unwrap_or(i32::MAX);
        let max_health = i32::try_from(max_health).unwrap_or(i32::MAX);
        state.health = state.health.saturating_add(amount).clamp(0, max_health);
        state.complete = state.health >= max_health;
        Ok(state.complete)
    }

    pub fn repair_building(
        &mut self,
        building: &StableId,
        amount: u32,
        max_health: u32,
    ) -> Result<u32, SimulationError> {
        let state = self
            .buildings
            .get_mut(building)
            .ok_or_else(|| SimulationError::MissingBuilding(building.clone()))?;
        if !state.complete {
            return Err(SimulationError::BuildingIncomplete(building.clone()));
        }
        let before = state.health;
        let amount = i32::try_from(amount).unwrap_or(i32::MAX);
        let max_health = i32::try_from(max_health).unwrap_or(i32::MAX);
        state.health = state.health.saturating_add(amount).clamp(0, max_health);
        Ok(u32::try_from(state.health.saturating_sub(before)).unwrap_or_default())
    }

    pub fn damage_building(
        &mut self,
        building: &StableId,
        amount: u32,
    ) -> Result<i32, SimulationError> {
        let state = self
            .buildings
            .get_mut(building)
            .ok_or_else(|| SimulationError::MissingBuilding(building.clone()))?;
        let amount = i32::try_from(amount).unwrap_or(i32::MAX);
        state.health = state.health.saturating_sub(amount).max(0);
        Ok(state.health)
    }

    pub fn upgrade_building(
        &mut self,
        building: &StableId,
        max_level: u16,
        upgraded_max_health: u32,
        health_gain_per_level: u32,
        cost: &BTreeMap<StableId, u32>,
    ) -> Result<u16, SimulationError> {
        let state = self
            .buildings
            .get(building)
            .ok_or_else(|| SimulationError::MissingBuilding(building.clone()))?;
        if !state.complete {
            return Err(SimulationError::BuildingIncomplete(building.clone()));
        }
        if state.level >= max_level {
            return Err(SimulationError::BuildingMaxLevel {
                building: building.clone(),
                max_level,
            });
        }
        self.spend_resources(cost)?;
        let state = self
            .buildings
            .get_mut(building)
            .expect("building was validated before spending resources");
        state.level = state.level.saturating_add(1).min(max_level);
        state.health = state
            .health
            .saturating_add(i32::try_from(health_gain_per_level).unwrap_or(i32::MAX))
            .min(i32::try_from(upgraded_max_health).unwrap_or(i32::MAX));
        Ok(state.level)
    }

    pub fn damage_actor(&mut self, actor: &StableId, damage: u32) -> Result<bool, SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        let damage = i32::try_from(damage).unwrap_or(i32::MAX);
        actor_state.health = actor_state.health.saturating_sub(damage).max(0);
        actor_state.alive = actor_state.health > 0;
        if actor_state.alive {
            actor_state.respawn_remaining_seconds = None;
        }
        Ok(!actor_state.alive)
    }

    pub fn schedule_respawn(
        &mut self,
        actor: &StableId,
        duration_seconds: f64,
    ) -> Result<(), SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        if actor_state.alive {
            return Err(SimulationError::ActorAlive(actor.clone()));
        }
        actor_state.respawn_remaining_seconds = Some(duration_seconds.max(0.0));
        Ok(())
    }

    pub fn heal_actor(&mut self, actor: &StableId, amount: u32) -> Result<u32, SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        if !actor_state.alive {
            return Err(SimulationError::ActorDead(actor.clone()));
        }
        let previous = actor_state.health;
        actor_state.health = actor_state
            .health
            .saturating_add(i32::try_from(amount).unwrap_or(i32::MAX))
            .min(actor_state.max_health);
        Ok(u32::try_from(actor_state.health.saturating_sub(previous)).unwrap_or(u32::MAX))
    }

    /// Revives an actor after atomically paying the authored town-food cost.
    pub fn revive_actor_with_food_cost(
        &mut self,
        actor: &StableId,
        position: GridPos,
        food_cost: u32,
    ) -> Result<(), SimulationError> {
        if self
            .actors
            .get(actor)
            .ok_or_else(|| SimulationError::MissingActor(actor.clone()))?
            .alive
        {
            return Err(SimulationError::ActorAlive(actor.clone()));
        }
        let food = StableId::new("resource:food").expect("static resource ID");
        self.spend_resources(&BTreeMap::from([(food, food_cost)]))?;
        self.respawn_actor(actor, position)
    }

    pub fn respawn_actor(
        &mut self,
        actor: &StableId,
        position: GridPos,
    ) -> Result<(), SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        actor_state.position = position;
        actor_state.health = actor_state.max_health;
        actor_state.alive = true;
        actor_state.respawn_remaining_seconds = None;
        Ok(())
    }

    pub fn start_technology_vote(
        &mut self,
        technology: StableId,
        duration_seconds: f32,
    ) -> Result<(), SimulationError> {
        if self.active_vote.is_some() {
            return Err(SimulationError::VoteActive);
        }
        self.active_vote = Some(TechVote {
            technology,
            remaining_seconds: duration_seconds.max(0.0),
            votes: BTreeMap::new(),
        });
        Ok(())
    }

    pub fn cast_vote(&mut self, actor: &StableId, approve: bool) -> Result<(), SimulationError> {
        if !self.actors.contains_key(actor) {
            return Err(SimulationError::MissingActor(actor.clone()));
        }
        let vote = self.active_vote.as_mut().ok_or(SimulationError::NoVote)?;
        if vote.votes.insert(actor.clone(), approve).is_some() {
            return Err(SimulationError::AlreadyVoted(actor.clone()));
        }
        Ok(())
    }

    /// Resolves an expired vote. A successful vote starts its objectives; the
    /// technology unlock is intentionally deferred until the goal completes.
    pub fn resolve_technology_vote(
        &mut self,
        objective_ids: &[StableId],
        definitions: &BTreeMap<StableId, ObjectiveDef>,
        max_goals: usize,
    ) -> Option<StableId> {
        let vote = self.active_vote.as_ref()?;
        if vote.remaining_seconds > f32::EPSILON {
            return None;
        }
        let approvals = vote.votes.values().filter(|approve| **approve).count();
        let rejections = vote.votes.len().saturating_sub(approvals);
        let technology = vote.technology.clone();
        if approvals > rejections
            && !objective_ids.is_empty()
            && self.active_goals.len() >= max_goals
        {
            return None;
        }
        self.active_vote = None;
        if approvals <= rejections {
            return None;
        }
        if objective_ids.is_empty() {
            self.unlocked_technology.insert(technology.clone());
            return Some(technology);
        }
        self.active_goals.push(TownGoalState {
            technology: technology.clone(),
            objectives: objective_ids
                .iter()
                .filter_map(|objective| {
                    definitions
                        .get(objective)
                        .map(|definition| ObjectiveProgress {
                            objective: objective.clone(),
                            amount: 0,
                            required_amount: definition.required_amount,
                        })
                })
                .collect(),
        });
        Some(technology)
    }

    /// Starts a technology goal directly, as Unity's game-master random-tech command does.
    pub fn start_technology_goal(
        &mut self,
        technology: StableId,
        objective_ids: &[StableId],
        definitions: &BTreeMap<StableId, ObjectiveDef>,
        max_goals: usize,
    ) -> bool {
        if self.unlocked_technology.contains(&technology)
            || self
                .active_goals
                .iter()
                .any(|goal| goal.technology == technology)
        {
            return false;
        }
        if objective_ids.is_empty() {
            return self.unlocked_technology.insert(technology);
        }
        if self.active_goals.len() >= max_goals {
            return false;
        }
        let objectives = objective_ids
            .iter()
            .filter_map(|objective| {
                definitions
                    .get(objective)
                    .map(|definition| ObjectiveProgress {
                        objective: objective.clone(),
                        amount: 0,
                        required_amount: definition.required_amount,
                    })
            })
            .collect::<Vec<_>>();
        if objectives.is_empty() {
            return false;
        }
        self.active_goals.push(TownGoalState {
            technology,
            objectives,
        });
        true
    }

    pub fn force_complete_first_goal(&mut self) -> Option<StableId> {
        if self.active_goals.is_empty() {
            return None;
        }
        let technology = self.active_goals.remove(0).technology;
        self.unlocked_technology.insert(technology.clone());
        Some(technology)
    }

    pub fn record_objective_event(
        &mut self,
        definitions: &BTreeMap<StableId, ObjectiveDef>,
        event: &ObjectiveEvent,
    ) -> Vec<StableId> {
        for goal in &mut self.active_goals {
            for progress in &mut goal.objectives {
                let Some(definition) = definitions.get(&progress.objective) else {
                    continue;
                };
                let increment = objective_increment(definition, progress.required_amount, event);
                progress.amount = progress
                    .amount
                    .saturating_add(increment)
                    .min(progress.required_amount);
            }
        }
        let completed: Vec<_> = self
            .active_goals
            .iter()
            .filter(|goal| {
                goal.objectives.iter().all(|progress| {
                    definitions
                        .get(&progress.objective)
                        .is_some_and(|_| progress.amount >= progress.required_amount)
                })
            })
            .map(|goal| goal.technology.clone())
            .collect();
        if !completed.is_empty() {
            self.active_goals
                .retain(|goal| !completed.contains(&goal.technology));
            self.unlocked_technology.extend(completed.iter().cloned());
        }
        completed
    }

    pub fn trade(
        &mut self,
        offered_resource: &StableId,
        offered_amount: u32,
        received_resource: StableId,
        received_amount: u32,
    ) -> Result<(), SimulationError> {
        if offered_amount == 0 || received_amount == 0 {
            return Err(SimulationError::EmptyTrade);
        }
        let available = self
            .town_resources
            .get(offered_resource)
            .copied()
            .unwrap_or_default();
        if available < offered_amount {
            return Err(SimulationError::InsufficientResource {
                resource: offered_resource.clone(),
                required: offered_amount,
                available,
            });
        }
        *self
            .town_resources
            .get_mut(offered_resource)
            .expect("validated trade resource") -= offered_amount;
        let received = self.town_resources.entry(received_resource).or_default();
        *received = received.saturating_add(received_amount);
        Ok(())
    }

    /// Applies Unity's authored 0.25 sell rate and 0.5 sell tax.
    pub fn sell_resource(
        &mut self,
        resource: &StableId,
        requested_amount: u32,
    ) -> Result<(u32, u32), SimulationError> {
        validate_trade_resource(resource)?;
        if requested_amount == 0 {
            return Err(SimulationError::EmptyTrade);
        }
        let available = self
            .town_resources
            .get(resource)
            .copied()
            .unwrap_or_default();
        let sold = requested_amount.min(available);
        if sold == 0 {
            return Err(SimulationError::InsufficientResource {
                resource: resource.clone(),
                required: requested_amount,
                available,
            });
        }
        *self
            .town_resources
            .get_mut(resource)
            .expect("positive availability guarantees a resource entry") -= sold;
        // Unity truncates after the base rate and again after tax.
        let gross = sold / 4;
        let gold = gross.saturating_sub(gross / 2);
        *self
            .town_resources
            .entry(StableId::new("resource:gold").expect("static stable ID"))
            .or_default() += gold;
        Ok((sold, gold))
    }

    /// Applies Unity's authored 0.25 rate divided by its 0.6 buy tax, bounded
    /// by available gold and deterministic storage capacity.
    pub fn buy_resource(
        &mut self,
        resource: StableId,
        requested_amount: u32,
        capacity: u32,
    ) -> Result<(u32, u32), SimulationError> {
        validate_trade_resource(&resource)?;
        if requested_amount == 0 {
            return Err(SimulationError::EmptyTrade);
        }
        let current = self
            .town_resources
            .get(&resource)
            .copied()
            .unwrap_or_default();
        let available_gold = self
            .town_resources
            .get(&StableId::new("resource:gold").expect("static stable ID"))
            .copied()
            .unwrap_or_default();
        let storage_limited = requested_amount.min(capacity.saturating_sub(current));
        let mut bought = storage_limited;
        let mut cost = bought.saturating_mul(5) / 12;
        if cost > available_gold {
            bought =
                u32::try_from(u64::from(available_gold).saturating_mul(12) / 5).unwrap_or(u32::MAX);
            cost = bought.saturating_mul(5) / 12;
        }
        if bought == 0 {
            return Err(SimulationError::InsufficientResource {
                resource: StableId::new("resource:gold").expect("static stable ID"),
                required: 1,
                available: available_gold,
            });
        }
        *self
            .town_resources
            .entry(StableId::new("resource:gold").expect("static stable ID"))
            .or_default() -= cost;
        *self.town_resources.entry(resource).or_default() += bought;
        Ok((bought, cost))
    }

    pub fn trigger_event(&mut self, event: TownEvent) {
        self.active_event = Some(event);
    }

    pub fn tick(&mut self, delta_seconds: f32, seconds_per_day: u32) {
        let delta_seconds = delta_seconds.max(0.0);
        self.elapsed_seconds += f64::from(delta_seconds);
        self.recalculate_calendar(seconds_per_day);

        if let Some(vote) = &mut self.active_vote {
            vote.remaining_seconds = (vote.remaining_seconds - delta_seconds).max(0.0);
        }
        if self.ruler_vote.is_none() && self.ruler_vote_scheduled {
            self.ruler_vote_cooldown_seconds =
                (self.ruler_vote_cooldown_seconds - delta_seconds).max(0.0);
            if self.ruler_vote_cooldown_seconds <= f32::EPSILON
                && self.active_vote.is_none()
                && self.active_event.is_none()
            {
                let kind = if self.current_ruler.is_some() {
                    RulerVoteKind::KeepRuler
                } else {
                    RulerVoteKind::NewRuler
                };
                let _ = self.start_ruler_vote(kind);
            }
        }
        if let Some(vote) = &mut self.ruler_vote
            && !vote.votes.is_empty()
        {
            vote.remaining_seconds = (vote.remaining_seconds - delta_seconds).max(0.0);
        }
        let _ = self.resolve_ruler_vote();
        for actor in self.actors.values_mut().filter(|actor| !actor.alive) {
            if let Some(remaining) = actor.respawn_remaining_seconds.as_mut() {
                *remaining = (*remaining - f64::from(delta_seconds)).max(0.0);
            }
        }
        let fish_god_expired = self.fish_god.as_mut().is_some_and(|event| {
            event.remaining_seconds = (event.remaining_seconds - f64::from(delta_seconds)).max(0.0);
            event.remaining_seconds <= f64::EPSILON
        });
        if fish_god_expired {
            self.fish_god = None;
            if self.active_event == Some(TownEvent::FishGod) {
                self.active_event = None;
            }
        }
    }

    /// Reinterprets clocks written before the shipping Unity day length was
    /// restored. Timed gameplay state is preserved; only derived calendar data
    /// changes from the authoritative elapsed time.
    pub fn upgrade_time_schema(&mut self, seconds_per_day: u32) {
        if self.schema_version < CURRENT_SIMULATION_SCHEMA {
            self.schema_version = CURRENT_SIMULATION_SCHEMA;
            self.recalculate_calendar(seconds_per_day);
        }
    }

    fn recalculate_calendar(&mut self, seconds_per_day: u32) {
        let seconds_per_day = u64::from(seconds_per_day.max(1));
        self.day = u32::try_from(
            Duration::from_secs_f64(self.elapsed_seconds)
                .as_secs()
                .saturating_div(seconds_per_day),
        )
        .unwrap_or(u32::MAX);
        self.season = match (self.day / 7) % 4 {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            _ => Season::Winter,
        };
        self.weather = deterministic_weather(self.world_seed, self.day, self.season);
    }

    fn actor_mut(&mut self, actor: &StableId) -> Result<&mut ActorState, SimulationError> {
        self.actors
            .get_mut(actor)
            .ok_or_else(|| SimulationError::MissingActor(actor.clone()))
    }

    fn spend_resources(&mut self, cost: &BTreeMap<StableId, u32>) -> Result<(), SimulationError> {
        for (resource, required) in cost {
            let available = self
                .town_resources
                .get(resource)
                .copied()
                .unwrap_or_default();
            if available < *required {
                return Err(SimulationError::InsufficientResource {
                    resource: resource.clone(),
                    required: *required,
                    available,
                });
            }
        }
        for (resource, required) in cost {
            *self
                .town_resources
                .get_mut(resource)
                .expect("validated resource cost") -= *required;
        }
        Ok(())
    }
}

fn validate_trade_resource(resource: &StableId) -> Result<(), SimulationError> {
    if matches!(
        resource.as_str(),
        "resource:wood" | "resource:ore" | "resource:food"
    ) {
        Ok(())
    } else {
        Err(SimulationError::InvalidTradeResource(resource.clone()))
    }
}

fn deterministic_fish_god_value(seed: u64, attempt: u64) -> u64 {
    let mut mixed = seed.wrapping_add(attempt.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    mixed ^ (mixed >> 31)
}

fn stable(value: &str) -> StableId {
    StableId::new(value).expect("static stable ID")
}

fn default_ruler_vote_cooldown() -> f32 {
    30.0
}

fn default_true() -> bool {
    true
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_default_ruler_vote_cooldown(value: &f32) -> bool {
    (*value - default_ruler_vote_cooldown()).abs() <= f32::EPSILON
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_true(value: &bool) -> bool {
    *value
}

fn objective_increment(
    definition: &ObjectiveDef,
    required_amount: u32,
    event: &ObjectiveEvent,
) -> u32 {
    match (definition.kind, event) {
        (ObjectiveKind::Build, ObjectiveEvent::BuildingBuilt(building))
            if definition.building.as_ref() == Some(building) =>
        {
            1
        }
        (ObjectiveKind::BuildAny, ObjectiveEvent::BuildingBuilt(_))
        | (ObjectiveKind::KillAny, ObjectiveEvent::EnemyKilled(_)) => 1,
        (ObjectiveKind::Collect, ObjectiveEvent::ResourceGained { resource, amount })
            if definition.resource.as_ref() == Some(resource) =>
        {
            *amount
        }
        (ObjectiveKind::EarnPerHour, ObjectiveEvent::ResourceGained { resource, .. })
            if definition.resource.as_ref() == Some(resource) =>
        {
            required_amount
        }
        (ObjectiveKind::Kill, ObjectiveEvent::EnemyKilled(enemy))
            if definition.enemy.as_ref() == Some(enemy) =>
        {
            1
        }
        (ObjectiveKind::Sell, ObjectiveEvent::ResourceSold { resource, amount })
            if definition.resource.as_ref() == Some(resource) =>
        {
            *amount
        }
        (ObjectiveKind::SellAny, ObjectiveEvent::ResourceSold { amount, .. })
        | (ObjectiveKind::BuyAny, ObjectiveEvent::ResourceBought { amount, .. }) => *amount,
        (ObjectiveKind::Buy, ObjectiveEvent::ResourceBought { resource, amount })
            if definition.resource.as_ref() == Some(resource) =>
        {
            *amount
        }
        _ => 0,
    }
}

#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn required_role_experience(level: u16) -> u32 {
    if level == 0 || level >= MAX_ROLE_LEVEL {
        return 100_000;
    }
    let t = f32::from(level.saturating_add(1)) / 100.0;
    ((1.0 - (1.0 - t * t).sqrt()) * 100_000.0) as u32
}

fn deterministic_weather(seed: u64, day: u32, season: Season) -> Weather {
    let mut value = seed ^ u64::from(day).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let roll = value % 100;
    match season {
        Season::Winter if roll < 55 => Weather::Snow,
        Season::Spring | Season::Autumn if roll < 35 => Weather::Rain,
        _ if roll < 15 => Weather::Fog,
        _ => Weather::Clear,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SHIPPING_SECONDS_PER_DAY;

    #[test]
    fn role_progression_matches_unity_curve_and_carries_excess() {
        let actor = StableId::new("actor:test").unwrap();
        let role = StableId::new("role:builder").unwrap();
        let mut simulation = WorldSimulation::new(7);
        assert!(simulation.join_player(actor.clone(), GridPos { x: 1, z: 1 }));
        simulation.assign_role(&actor, role.clone()).unwrap();

        assert_eq!(required_role_experience(1), 20);
        assert_eq!(required_role_experience(99), 100_000);
        assert_eq!(simulation.grant_role_experience(&actor, 8, 3_000), Ok(1));
        assert_eq!(
            simulation.actors[&actor].role_progression[&role],
            RoleProgress {
                level: 2,
                experience: 4,
            }
        );
    }

    #[test]
    fn twitch_user_type_is_persisted_and_defaults_for_old_saves() {
        let actor = id("twitch:user_type");
        let mut simulation = WorldSimulation::new(5);
        assert!(simulation.join_player(actor.clone(), GridPos { x: 1, z: 1 }));
        simulation.actors.get_mut(&actor).unwrap().user_type = StreamUserType::Subscriber;
        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap().actors[&actor].user_type,
            StreamUserType::Subscriber
        );

        let old_actor = encoded.replace("user_type:Subscriber,", "");
        assert_eq!(
            ron::from_str::<WorldSimulation>(&old_actor).unwrap().actors[&actor].user_type,
            StreamUserType::Normal
        );
    }

    #[test]
    fn game_master_state_toggles_queue_and_direct_actions_round_trip() {
        let mut simulation = WorldSimulation::new(11);
        let actor = id("twitch:gm_target");
        assert!(simulation.join_player(actor.clone(), GridPos { x: 1, z: 1 }));
        assert!(!simulation.toggle_building_costs());
        assert!(!simulation.toggle_role_limits());
        assert_eq!(simulation.adjust_town_resource(id("resource:wood"), -20), 0);
        assert_eq!(
            simulation.adjust_town_resource(id("resource:wood"), 125),
            125
        );
        assert_eq!(simulation.grant_role_levels(&actor, 3), Ok(3));
        assert_eq!(
            simulation.actors[&actor].role_progression[&id("role:villager")].level,
            4
        );
        assert_eq!(simulation.unlock_pet(&actor, id("pet:duck")), Ok(true));

        assert!(simulation.queue_event(TownEvent::FishGod));
        assert!(!simulation.queue_event(TownEvent::FishGod));
        assert_eq!(
            simulation.take_next_queued_event(),
            Some(TownEvent::FishGod)
        );
        assert!(simulation.start_fish_god(true));
        assert_eq!(simulation.action_fish_god(), Ok(false));
        assert_eq!(simulation.fish_god.as_ref().unwrap().praises_given, 1);
        assert!(simulation.queue_event(TownEvent::EnemyRaid));
        assert_eq!(simulation.take_next_queued_event(), None);
        assert!(simulation.stop_active_event().is_empty());
        assert_eq!(
            simulation.take_next_queued_event(),
            Some(TownEvent::EnemyRaid)
        );

        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap(),
            simulation
        );
    }

    #[test]
    fn building_damage_and_repair_preserve_health_bounds() {
        let building = id("building:damage_test");
        let mut simulation = WorldSimulation::new(11);
        simulation.buildings.insert(
            building.clone(),
            BuildingState {
                id: building.clone(),
                archetype: id("archetype:building"),
                position: GridPos { x: 2, z: 3 },
                rotation_quarter_turns: 0,
                level: 1,
                health: BUILDING_MAX_HEALTH,
                complete: true,
            },
        );
        assert_eq!(
            simulation.damage_building(&building, 25),
            Ok(BUILDING_MAX_HEALTH - 25)
        );
        assert_eq!(
            simulation.repair_building(&building, 10, u32::try_from(BUILDING_MAX_HEALTH).unwrap()),
            Ok(10)
        );
        assert_eq!(
            simulation.buildings[&building].health,
            BUILDING_MAX_HEALTH - 15
        );
        assert_eq!(simulation.damage_building(&building, u32::MAX), Ok(0));
        assert_eq!(
            simulation.repair_building(
                &building,
                u32::MAX,
                u32::try_from(BUILDING_MAX_HEALTH).unwrap()
            ),
            Ok(u32::try_from(BUILDING_MAX_HEALTH).unwrap())
        );
        assert_eq!(simulation.buildings[&building].health, BUILDING_MAX_HEALTH);
    }

    #[test]
    fn game_master_can_start_and_force_complete_a_technology_goal() {
        let mut simulation = WorldSimulation::new(12);
        let technology = id("tech:test");
        let objective = id("objective:test");
        let definitions = BTreeMap::from([(
            objective.clone(),
            ObjectiveDef {
                kind: ObjectiveKind::BuildAny,
                required_amount: 3,
                float_value_milli: 0,
                resource: None,
                building: None,
                enemy: None,
            },
        )]);
        assert!(simulation.start_technology_goal(
            technology.clone(),
            std::slice::from_ref(&objective),
            &definitions,
            2,
        ));
        assert_eq!(simulation.active_goals.len(), 1);
        assert_eq!(
            simulation.force_complete_first_goal(),
            Some(technology.clone())
        );
        assert!(simulation.unlocked_technology.contains(&technology));
        assert!(!simulation.start_technology_goal(technology, &[objective], &definitions, 2));
    }

    fn id(value: &str) -> StableId {
        StableId::new(value).unwrap()
    }

    #[test]
    fn complete_gameplay_scenario_round_trips() {
        let mut simulation = WorldSimulation::new(42);
        let player = id("twitch:viewer");
        let spawn = GridPos { x: 10, z: 10 };
        assert!(simulation.join_player(player.clone(), spawn));
        simulation.assign_role(&player, id("role:builder")).unwrap();
        simulation
            .gather(&player, id("resource:wood"), 120)
            .unwrap();
        simulation.gather(&player, id("resource:ore"), 40).unwrap();
        assert_eq!(simulation.deposit_all(&player).unwrap(), 160);

        simulation
            .construct(
                id("building:house_1"),
                id("building:house"),
                GridPos { x: 12, z: 10 },
                u32::try_from(BUILDING_MAX_HEALTH).unwrap(),
                &BTreeMap::from([(id("resource:wood"), 100), (id("resource:ore"), 25)]),
            )
            .unwrap();
        simulation
            .start_technology_vote(id("tech:construction_1"), 10.0)
            .unwrap();
        simulation.cast_vote(&player, true).unwrap();
        simulation.tick(10.0, SHIPPING_SECONDS_PER_DAY);
        simulation.resolve_technology_vote(&[], &BTreeMap::new(), 2);
        assert!(
            simulation
                .unlocked_technology
                .contains(&id("tech:construction_1"))
        );

        simulation.trigger_event(TownEvent::EnemyRaid);
        assert!(simulation.damage_actor(&player, 500).unwrap());
        simulation.respawn_actor(&player, spawn).unwrap();
        simulation
            .trade(&id("resource:wood"), 10, id("resource:food"), 15)
            .unwrap();
        simulation.tick(3_600.0 * 22.0, SHIPPING_SECONDS_PER_DAY);
        assert_eq!(simulation.season, Season::Winter);

        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap(),
            simulation
        );
    }

    #[test]
    fn legacy_simulation_calendar_upgrades_without_advancing_timers() {
        let actor = id("actor:legacy_clock");
        let mut simulation = WorldSimulation::new(71);
        simulation.schema_version = 1;
        simulation.elapsed_seconds = 3_650.0;
        simulation.day = 30;
        simulation.season = Season::Winter;
        simulation.join_player(actor.clone(), GridPos { x: 1, z: 1 });
        let state = simulation.actors.get_mut(&actor).unwrap();
        state.alive = false;
        state.respawn_remaining_seconds = Some(45.0);

        simulation.upgrade_time_schema(SHIPPING_SECONDS_PER_DAY);

        assert_eq!(simulation.schema_version, 2);
        assert_eq!(simulation.day, 1);
        assert_eq!(simulation.season, Season::Spring);
        assert_eq!(
            simulation.actors[&actor].respawn_remaining_seconds,
            Some(45.0)
        );
    }

    #[test]
    fn scheduled_ruler_elections_pause_resolve_and_restore_roles() {
        let mut simulation = WorldSimulation::new(7);
        let first = id("twitch:first");
        let second = id("twitch:second");
        assert!(simulation.join_player(first.clone(), GridPos { x: 1, z: 1 }));
        assert!(simulation.join_player(second.clone(), GridPos { x: 2, z: 1 }));
        simulation.assign_role(&first, id("role:builder")).unwrap();
        simulation.assign_role(&second, id("role:miner")).unwrap();

        simulation.tick(30.0, SHIPPING_SECONDS_PER_DAY);
        assert_eq!(
            simulation.ruler_vote.as_ref().unwrap().kind,
            RulerVoteKind::NewRuler
        );
        simulation.tick(60.0, SHIPPING_SECONDS_PER_DAY);
        assert!(
            (simulation.ruler_vote.as_ref().unwrap().remaining_seconds - 120.0).abs()
                <= f32::EPSILON
        );
        simulation.cast_ruler_vote(&first, first.clone()).unwrap();
        simulation.cast_ruler_vote(&second, second.clone()).unwrap();
        simulation.tick(120.0, SHIPPING_SECONDS_PER_DAY);
        assert_eq!(simulation.current_ruler, Some(first.clone()));
        assert_eq!(simulation.actors[&first].role, id("role:ruler"));
        assert_eq!(simulation.ruler_previous_role, Some(id("role:builder")));
        assert!((simulation.ruler_vote_cooldown_seconds - 3_600.0).abs() <= f32::EPSILON);

        simulation.tick(3_600.0, SHIPPING_SECONDS_PER_DAY);
        assert_eq!(
            simulation.ruler_vote.as_ref().unwrap().kind,
            RulerVoteKind::KeepRuler
        );
        simulation.cast_ruler_vote(&first, id("no")).unwrap();
        simulation.tick(120.0, SHIPPING_SECONDS_PER_DAY);
        assert!(simulation.current_ruler.is_none());
        assert_eq!(simulation.actors[&first].role, id("role:builder"));
        assert_eq!(
            simulation.ruler_vote.as_ref().unwrap().kind,
            RulerVoteKind::NewRuler
        );

        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap(),
            simulation
        );
    }

    #[test]
    fn ruler_vote_rejects_duplicates_and_invalid_candidates() {
        let mut simulation = WorldSimulation::new(7);
        let voter = id("twitch:voter");
        assert!(simulation.join_player(voter.clone(), GridPos { x: 1, z: 1 }));
        simulation
            .start_ruler_vote(RulerVoteKind::NewRuler)
            .unwrap();
        assert!(matches!(
            simulation.cast_ruler_vote(&voter, id("twitch:missing")),
            Err(SimulationError::InvalidRulerVoteOption(_))
        ));
        simulation.cast_ruler_vote(&voter, voter.clone()).unwrap();
        assert_eq!(
            simulation.cast_ruler_vote(&voter, voter.clone()),
            Err(SimulationError::AlreadyVoted(voter.clone()))
        );
        simulation.tick(120.0, SHIPPING_SECONDS_PER_DAY);
        assert_eq!(simulation.current_ruler, Some(voter.clone()));
        simulation.resign_ruler(&voter).unwrap();
        assert!(simulation.current_ruler.is_none());
        assert_eq!(
            simulation.ruler_vote.as_ref().unwrap().kind,
            RulerVoteKind::NewRuler
        );
    }

    #[test]
    fn enemy_camps_and_raid_progress_round_trip_with_stable_archetypes() {
        let mut simulation = WorldSimulation::new(99);
        let enemy_archetype = id("archetype:prefab:minotaur");
        let boss_archetype = id("archetype:prefab:minotaur_boss");
        let enemy = id("actor:enemy_00000000");
        assert!(simulation.spawn_enemy(
            enemy.clone(),
            enemy_archetype.clone(),
            GridPos { x: 6, z: 7 },
            25,
        ));
        assert_eq!(
            simulation.actors[&enemy].archetype,
            Some(enemy_archetype.clone())
        );
        assert!(simulation.start_raid(5, 50, enemy_archetype, boss_archetype));
        simulation
            .active_raid
            .as_mut()
            .unwrap()
            .tracked_enemies
            .insert(enemy);
        simulation.enemy_camps.insert(
            id("enemy_camp:test"),
            EnemyCampState {
                id: id("enemy_camp:test"),
                archetype: id("archetype:prefab:camp"),
                position: GridPos { x: 2, z: 3 },
                health: 1_000,
                spawn_remaining_seconds: 2.5,
                spawned_enemies: BTreeSet::new(),
            },
        );

        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap(),
            simulation
        );
        simulation.finish_raid();
        assert!(simulation.active_event.is_none());
        assert!(simulation.active_raid.is_none());
    }

    #[test]
    fn fish_god_progress_rewards_food_unlocks_pet_and_expires() {
        let pet_seed = (0..1_000)
            .find(|seed| deterministic_fish_god_value(*seed, 2) % 100 < 70)
            .expect("a deterministic pet-winning seed");
        let mut simulation = WorldSimulation::new(pet_seed);
        let viewer = id("twitch:fish_friend");
        assert!(simulation.join_player(viewer.clone(), GridPos { x: 1, z: 1 }));
        assert!(simulation.start_fish_god(true));
        for praise in 1..20 {
            assert_eq!(simulation.praise_fish_god(&viewer), Ok(false));
            assert_eq!(simulation.fish_god.as_ref().unwrap().praises_given, praise);
        }
        assert_eq!(simulation.praise_fish_god(&viewer), Ok(true));
        assert_eq!(simulation.town_resources[&id("resource:food")], 1_000);
        assert!(
            simulation.actors[&viewer]
                .unlocked_pets
                .contains(&id("pet:fish_god"))
        );
        assert!(simulation.fish_god.is_none());
        assert!(simulation.active_event.is_none());

        assert!(simulation.start_fish_god(true));
        simulation.tick(300.0, SHIPPING_SECONDS_PER_DAY);
        assert!(simulation.fish_god.is_none());
        assert!(simulation.active_event.is_none());
        assert!(matches!(
            simulation.praise_fish_god(&viewer),
            Err(SimulationError::NoFishGodEvent)
        ));

        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap(),
            simulation
        );
    }

    #[test]
    fn fish_god_summon_chance_is_replay_deterministic() {
        fn attempts_until_answered(seed: u64) -> u64 {
            let mut simulation = WorldSimulation::new(seed);
            while !simulation.start_fish_god(false) {
                assert!(simulation.fish_god_attempts < 1_000);
            }
            simulation.fish_god_attempts
        }

        let attempts = attempts_until_answered(42);
        assert_eq!(attempts_until_answered(42), attempts);
        assert!(attempts > 0);
    }

    #[test]
    fn gathering_pet_unlock_roll_is_replay_deterministic_and_persistent() {
        let player = id("twitch:gatherer");
        let pet = id("pet:giraffe");
        let seed = (0..10_000)
            .find(|seed| {
                deterministic_fish_god_value(*seed ^ 0x7065_745f_6472_6f70, 1).is_multiple_of(5_000)
            })
            .expect("at least one seed unlocks the first gathering pet roll");
        let mut simulation = WorldSimulation::new(seed);
        assert!(simulation.join_player(player.clone(), GridPos { x: 1, z: 1 }));
        assert!(
            simulation
                .try_unlock_gathering_pet(&player, pet.clone())
                .unwrap()
        );
        assert!(simulation.actors[&player].unlocked_pets.contains(&pet));
        assert_eq!(simulation.gathering_pet_attempts, 1);

        let encoded = ron::ser::to_string(&simulation).unwrap();
        let restored: WorldSimulation = ron::from_str(&encoded).unwrap();
        assert_eq!(restored, simulation);
    }

    #[test]
    fn healing_and_food_revives_preserve_health_invariants() {
        let mut simulation = WorldSimulation::new(42);
        let actor = id("twitch:wounded");
        let food = id("resource:food");
        let spawn = GridPos { x: 4, z: 5 };
        assert!(simulation.join_player(actor.clone(), GridPos { x: 1, z: 2 }));

        assert!(!simulation.damage_actor(&actor, 30).unwrap());
        assert_eq!(simulation.heal_actor(&actor, 12), Ok(12));
        assert_eq!(simulation.actors[&actor].health, 82);
        assert_eq!(simulation.heal_actor(&actor, 1_000), Ok(18));
        assert_eq!(simulation.actors[&actor].health, 100);

        assert!(simulation.damage_actor(&actor, 1_000).unwrap());
        assert!(matches!(
            simulation.heal_actor(&actor, 1),
            Err(SimulationError::ActorDead(_))
        ));
        simulation.schedule_respawn(&actor, 60.0).unwrap();
        simulation.tick(0.25, SHIPPING_SECONDS_PER_DAY);
        assert_eq!(
            simulation.actors[&actor].respawn_remaining_seconds,
            Some(59.75)
        );
        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap(),
            simulation
        );
        simulation.town_resources.insert(food.clone(), 399);
        assert!(matches!(
            simulation.revive_actor_with_food_cost(&actor, spawn, 400),
            Err(SimulationError::InsufficientResource { .. })
        ));
        simulation.town_resources.insert(food.clone(), 400);
        simulation
            .revive_actor_with_food_cost(&actor, spawn, 400)
            .unwrap();
        assert!(simulation.actors[&actor].alive);
        assert_eq!(simulation.actors[&actor].position, spawn);
        assert_eq!(simulation.actors[&actor].health, 100);
        assert_eq!(simulation.town_resources[&food], 0);
        assert!(matches!(
            simulation.revive_actor_with_food_cost(&actor, spawn, 400),
            Err(SimulationError::ActorAlive(_))
        ));
    }

    #[test]
    fn technology_vote_starts_persistent_goal_and_unlocks_after_all_objectives() {
        let mut simulation = WorldSimulation::new(42);
        let player = id("actor:viewer");
        let technology = id("tech:forestry");
        let collect = id("objective:collect_wood");
        let build = id("objective:build_any");
        let wood = id("resource:wood");
        assert!(simulation.join_player(player.clone(), GridPos { x: 0, z: 0 }));
        simulation
            .start_technology_vote(technology.clone(), 1.0)
            .unwrap();
        simulation.cast_vote(&player, true).unwrap();
        simulation.tick(1.0, SHIPPING_SECONDS_PER_DAY);
        assert_eq!(
            simulation.resolve_technology_vote(
                &[collect.clone(), build.clone()],
                &BTreeMap::from([
                    (
                        collect.clone(),
                        ObjectiveDef {
                            kind: ObjectiveKind::Collect,
                            required_amount: 10,
                            float_value_milli: 0,
                            resource: Some(wood.clone()),
                            building: None,
                            enemy: None,
                        },
                    ),
                    (
                        build.clone(),
                        ObjectiveDef {
                            kind: ObjectiveKind::BuildAny,
                            required_amount: 1,
                            float_value_milli: 0,
                            resource: None,
                            building: None,
                            enemy: None,
                        },
                    ),
                ]),
                2,
            ),
            Some(technology.clone())
        );
        assert!(!simulation.unlocked_technology.contains(&technology));

        let definitions = BTreeMap::from([
            (
                collect,
                ObjectiveDef {
                    kind: ObjectiveKind::Collect,
                    required_amount: 10,
                    float_value_milli: 0,
                    resource: Some(wood.clone()),
                    building: None,
                    enemy: None,
                },
            ),
            (
                build,
                ObjectiveDef {
                    kind: ObjectiveKind::BuildAny,
                    required_amount: 1,
                    float_value_milli: 0,
                    resource: None,
                    building: None,
                    enemy: None,
                },
            ),
        ]);
        assert!(
            simulation
                .record_objective_event(
                    &definitions,
                    &ObjectiveEvent::ResourceGained {
                        resource: wood,
                        amount: 10,
                    },
                )
                .is_empty()
        );
        assert_eq!(
            simulation.record_objective_event(
                &definitions,
                &ObjectiveEvent::BuildingBuilt(id("building:house")),
            ),
            vec![technology.clone()]
        );
        assert!(simulation.active_goals.is_empty());
        assert!(simulation.unlocked_technology.contains(&technology));

        let encoded = ron::to_string(&simulation).unwrap();
        assert_eq!(
            ron::from_str::<WorldSimulation>(&encoded).unwrap(),
            simulation
        );
    }

    #[test]
    fn capped_deposit_preserves_inventory_overflow() {
        let mut simulation = WorldSimulation::new(42);
        let player = id("twitch:viewer");
        let wood = id("resource:wood");
        assert!(simulation.join_player(player.clone(), GridPos { x: 0, z: 0 }));
        simulation.gather(&player, wood.clone(), 25).unwrap();
        simulation.town_resources.insert(wood.clone(), 90);
        let deposited = simulation
            .deposit_all_with_capacities(&player, &BTreeMap::from([(wood.clone(), 100)]))
            .unwrap();
        assert_eq!(deposited, 10);
        assert_eq!(simulation.town_resources[&wood], 100);
        assert_eq!(simulation.actors[&player].inventory[&wood], 15);
    }

    #[test]
    fn role_resource_deposit_preserves_other_carried_resources() {
        let mut simulation = WorldSimulation::new(42);
        let player = id("twitch:viewer");
        let wood = id("resource:wood");
        let ore = id("resource:ore");
        assert!(simulation.join_player(player.clone(), GridPos { x: 0, z: 0 }));
        simulation.gather(&player, wood.clone(), 7).unwrap();
        simulation.gather(&player, ore.clone(), 12).unwrap();
        simulation.town_resources.insert(ore.clone(), 95);

        assert_eq!(
            simulation
                .deposit_resource_with_capacity(&player, &ore, 100)
                .unwrap(),
            5
        );
        assert_eq!(simulation.town_resources[&ore], 100);
        assert_eq!(simulation.actors[&player].inventory[&ore], 7);
        assert_eq!(simulation.actors[&player].inventory[&wood], 7);
    }

    #[test]
    fn authored_trade_rates_clamp_to_stock_gold_and_capacity() {
        let mut simulation = WorldSimulation::new(42);
        let wood = id("resource:wood");
        let ore = id("resource:ore");
        let gold = id("resource:gold");
        simulation.town_resources.insert(wood.clone(), 100);
        simulation.town_resources.insert(gold.clone(), 10);

        assert_eq!(simulation.sell_resource(&wood, 40), Ok((40, 5)));
        assert_eq!(simulation.town_resources[&wood], 60);
        assert_eq!(simulation.town_resources[&gold], 15);
        assert_eq!(simulation.buy_resource(ore.clone(), 100, 20), Ok((20, 8)));
        assert_eq!(simulation.town_resources[&ore], 20);
        assert_eq!(simulation.town_resources[&gold], 7);
        assert!(matches!(
            simulation.sell_resource(&gold, 1),
            Err(SimulationError::InvalidTradeResource(_))
        ));
    }
}
