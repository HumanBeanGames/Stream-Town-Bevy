use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{GridPos, StableId};

pub const BUILDING_MAX_HEALTH: i32 = 500;
pub const MAX_ROLE_LEVEL: u16 = 99;

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
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActorState {
    pub id: StableId,
    pub role: StableId,
    pub position: GridPos,
    pub health: i32,
    pub max_health: i32,
    pub alive: bool,
    pub inventory: BTreeMap<StableId, u32>,
    /// Stable runtime building ID, or `building:townhall` for the initial station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station: Option<StableId>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub role_progression: BTreeMap<StableId, RoleProgress>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BuildingState {
    pub id: StableId,
    pub archetype: StableId,
    pub position: GridPos,
    pub level: u16,
    pub health: i32,
    pub complete: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TechVote {
    pub technology: StableId,
    pub remaining_seconds: f32,
    pub votes: BTreeMap<StableId, bool>,
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
    pub actors: BTreeMap<StableId, ActorState>,
    pub buildings: BTreeMap<StableId, BuildingState>,
    pub unlocked_technology: BTreeSet<StableId>,
    pub active_vote: Option<TechVote>,
    pub active_event: Option<TownEvent>,
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
}

impl WorldSimulation {
    #[must_use]
    pub fn new(world_seed: u64) -> Self {
        Self {
            schema_version: 1,
            world_seed,
            elapsed_seconds: 0.0,
            day: 0,
            season: Season::Spring,
            weather: Weather::Clear,
            town_resources: BTreeMap::new(),
            actors: BTreeMap::new(),
            buildings: BTreeMap::new(),
            unlocked_technology: BTreeSet::new(),
            active_vote: None,
            active_event: None,
        }
    }

    pub fn join_player(&mut self, id: StableId, position: GridPos) -> bool {
        if self.actors.contains_key(&id) {
            return false;
        }
        self.actors.insert(
            id.clone(),
            ActorState {
                id,
                role: StableId::new("role:villager").expect("static stable ID"),
                position,
                health: 100,
                max_health: 100,
                alive: true,
                inventory: BTreeMap::new(),
                station: None,
                role_progression: BTreeMap::from([(
                    StableId::new("role:villager").expect("static stable ID"),
                    RoleProgress::default(),
                )]),
            },
        );
        true
    }

    pub fn assign_role(&mut self, actor: &StableId, role: StableId) -> Result<(), SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        actor_state.role = role.clone();
        actor_state.station = None;
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

    pub fn construct(
        &mut self,
        id: StableId,
        archetype: StableId,
        position: GridPos,
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
                level: 1,
                health: BUILDING_MAX_HEALTH / 10,
                complete: false,
            },
        );
        Ok(())
    }

    pub fn work_on_building(
        &mut self,
        building: &StableId,
        amount: u32,
    ) -> Result<bool, SimulationError> {
        let state = self
            .buildings
            .get_mut(building)
            .ok_or_else(|| SimulationError::MissingBuilding(building.clone()))?;
        if state.complete {
            return Ok(true);
        }
        let amount = i32::try_from(amount).unwrap_or(i32::MAX);
        state.health = state
            .health
            .saturating_add(amount)
            .clamp(0, BUILDING_MAX_HEALTH);
        state.complete = state.health >= BUILDING_MAX_HEALTH;
        Ok(state.complete)
    }

    pub fn upgrade_building(
        &mut self,
        building: &StableId,
        max_level: u16,
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
        state.health = BUILDING_MAX_HEALTH;
        Ok(state.level)
    }

    pub fn damage_actor(&mut self, actor: &StableId, damage: u32) -> Result<bool, SimulationError> {
        let actor_state = self.actor_mut(actor)?;
        let damage = i32::try_from(damage).unwrap_or(i32::MAX);
        actor_state.health = actor_state.health.saturating_sub(damage).max(0);
        actor_state.alive = actor_state.health > 0;
        Ok(!actor_state.alive)
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

    pub fn trigger_event(&mut self, event: TownEvent) {
        self.active_event = Some(event);
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        let delta_seconds = delta_seconds.max(0.0);
        self.elapsed_seconds += f64::from(delta_seconds);
        self.day = u32::try_from(
            Duration::from_secs_f64(self.elapsed_seconds)
                .as_secs()
                .saturating_div(120),
        )
        .unwrap_or(u32::MAX);
        self.season = match (self.day / 7) % 4 {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            _ => Season::Winter,
        };
        self.weather = deterministic_weather(self.world_seed, self.day, self.season);

        if let Some(vote) = &mut self.active_vote {
            vote.remaining_seconds = (vote.remaining_seconds - delta_seconds).max(0.0);
            if vote.remaining_seconds <= f32::EPSILON {
                let approvals = vote.votes.values().filter(|vote| **vote).count();
                let rejections = vote.votes.len().saturating_sub(approvals);
                if approvals > rejections {
                    self.unlocked_technology.insert(vote.technology.clone());
                }
                self.active_vote = None;
            }
        }
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
                &BTreeMap::from([(id("resource:wood"), 100), (id("resource:ore"), 25)]),
            )
            .unwrap();
        simulation
            .start_technology_vote(id("tech:construction_1"), 10.0)
            .unwrap();
        simulation.cast_vote(&player, true).unwrap();
        simulation.tick(10.0);
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
        simulation.tick(120.0 * 22.0);
        assert_eq!(simulation.season, Season::Winter);

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
}
