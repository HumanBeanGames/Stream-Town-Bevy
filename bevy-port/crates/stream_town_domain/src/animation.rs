use std::collections::BTreeMap;

use thiserror::Error;

use crate::{
    AnimationConditionDef, AnimationConditionMode, AnimationControllerDef, AnimationParameterKind,
    AnimationStateDef, StableId,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationParameterValue {
    Float(f32),
    Integer(i32),
    Boolean(bool),
    Trigger(bool),
}

impl AnimationParameterValue {
    fn kind(self) -> AnimationParameterKind {
        match self {
            Self::Float(_) => AnimationParameterKind::Float,
            Self::Integer(_) => AnimationParameterKind::Integer,
            Self::Boolean(_) => AnimationParameterKind::Boolean,
            Self::Trigger(_) => AnimationParameterKind::Trigger,
        }
    }

    fn scalar(self) -> f64 {
        match self {
            Self::Float(value) => f64::from(value),
            Self::Integer(value) => f64::from(value),
            Self::Boolean(value) | Self::Trigger(value) => {
                if value {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    fn blend_value(self) -> Option<f32> {
        match self {
            Self::Float(value) => Some(value),
            _ => None,
        }
    }

    fn boolean(self) -> bool {
        match self {
            Self::Boolean(value) | Self::Trigger(value) => value,
            Self::Float(value) => value != 0.0,
            Self::Integer(value) => value != 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeightedAnimationMotion {
    pub clip: StableId,
    pub weight: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationBlendSelection {
    pub first: WeightedAnimationMotion,
    pub second: Option<WeightedAnimationMotion>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnimationTransitionOutcome {
    None,
    Entered(StableId),
    Exited,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationTransitionPlayback {
    pub duration: f32,
    pub fixed_duration: bool,
    pub destination_offset: f32,
}

#[derive(Clone, Debug)]
pub struct AnimationControllerRuntime {
    current_state: StableId,
    parameters: BTreeMap<String, AnimationParameterValue>,
    last_transition: Option<AnimationTransitionPlayback>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum AnimationRuntimeError {
    #[error("animation controller has no usable default state")]
    MissingDefaultState,
    #[error("animation controller state {0} does not exist")]
    MissingState(StableId),
    #[error("animation parameter {0} does not exist")]
    MissingParameter(String),
    #[error("animation parameter {name} is {actual:?}, not {expected:?}")]
    ParameterType {
        name: String,
        expected: AnimationParameterKind,
        actual: AnimationParameterKind,
    },
}

impl AnimationControllerRuntime {
    pub fn new(controller: &AnimationControllerDef) -> Result<Self, AnimationRuntimeError> {
        let current_state = controller
            .default_states
            .first()
            .filter(|state| controller.states.contains_key(*state))
            .cloned()
            .ok_or(AnimationRuntimeError::MissingDefaultState)?;
        Self::in_state(controller, current_state)
    }

    pub fn in_state(
        controller: &AnimationControllerDef,
        current_state: StableId,
    ) -> Result<Self, AnimationRuntimeError> {
        if !controller.states.contains_key(&current_state) {
            return Err(AnimationRuntimeError::MissingState(current_state));
        }
        let parameters = controller
            .parameters
            .iter()
            .map(|parameter| {
                let value = match parameter.kind {
                    AnimationParameterKind::Float => {
                        AnimationParameterValue::Float(parameter.default_float)
                    }
                    AnimationParameterKind::Integer => {
                        AnimationParameterValue::Integer(parameter.default_integer)
                    }
                    AnimationParameterKind::Boolean => {
                        AnimationParameterValue::Boolean(parameter.default_boolean)
                    }
                    AnimationParameterKind::Trigger => AnimationParameterValue::Trigger(false),
                };
                (parameter.name.clone(), value)
            })
            .collect();
        Ok(Self {
            current_state,
            parameters,
            last_transition: None,
        })
    }

    #[must_use]
    pub fn current_state(&self) -> &StableId {
        &self.current_state
    }

    pub fn set_float(&mut self, name: &str, value: f32) -> Result<(), AnimationRuntimeError> {
        self.set_parameter(name, AnimationParameterValue::Float(value))
    }

    pub fn set_integer(&mut self, name: &str, value: i32) -> Result<(), AnimationRuntimeError> {
        self.set_parameter(name, AnimationParameterValue::Integer(value))
    }

    pub fn set_boolean(&mut self, name: &str, value: bool) -> Result<(), AnimationRuntimeError> {
        self.set_parameter(name, AnimationParameterValue::Boolean(value))
    }

    pub fn set_trigger(&mut self, name: &str) -> Result<(), AnimationRuntimeError> {
        self.set_parameter(name, AnimationParameterValue::Trigger(true))
    }

    pub fn reset_trigger(&mut self, name: &str) -> Result<(), AnimationRuntimeError> {
        self.set_parameter(name, AnimationParameterValue::Trigger(false))
    }

    pub fn enter_state(
        &mut self,
        controller: &AnimationControllerDef,
        state: StableId,
    ) -> Result<(), AnimationRuntimeError> {
        if !controller.states.contains_key(&state) {
            return Err(AnimationRuntimeError::MissingState(state));
        }
        self.current_state = state;
        self.last_transition = None;
        Ok(())
    }

    pub fn take_transition_playback(&mut self) -> Option<AnimationTransitionPlayback> {
        self.last_transition.take()
    }

    pub fn evaluate_transitions(
        &mut self,
        controller: &AnimationControllerDef,
        normalized_time: f32,
    ) -> Result<AnimationTransitionOutcome, AnimationRuntimeError> {
        self.last_transition = None;
        if !controller.states.contains_key(&self.current_state) {
            return Err(AnimationRuntimeError::MissingState(
                self.current_state.clone(),
            ));
        }
        let transition = controller.transitions.iter().find_map(|transition| {
            if transition.is_entry
                || !self.transition_source_is_active(controller, transition)
                || transition.has_exit_time && normalized_time < transition.exit_time
                || !transition
                    .conditions
                    .iter()
                    .all(|condition| self.condition_satisfied(condition))
            {
                return None;
            }
            let entry_transition =
                transition
                    .destination_state_machine
                    .as_ref()
                    .and_then(|machine| {
                        controller.transitions.iter().find(|candidate| {
                            candidate.is_entry
                                && candidate.source_state_machine.as_ref() == Some(machine)
                                && candidate
                                    .conditions
                                    .iter()
                                    .all(|condition| self.condition_satisfied(condition))
                        })
                    });
            let destination = transition
                .destination
                .as_ref()
                .or_else(|| entry_transition.and_then(|entry| entry.destination.as_ref()))
                .or_else(|| {
                    transition
                        .destination_state_machine
                        .as_ref()
                        .and_then(|machine| controller.state_machines.get(machine))
                        .and_then(|machine| machine.default_state.as_ref())
                })
                .cloned()
                .or_else(|| {
                    if !transition.is_exit {
                        return None;
                    }
                    transition
                        .source_state_machine
                        .as_ref()
                        .or_else(|| state_machine_for_state(controller, &self.current_state))
                        .and_then(|machine| parent_state_machine(controller, machine))
                        .and_then(|parent| controller.state_machines.get(parent))
                        .and_then(|parent| parent.default_state.clone())
                });
            // Unity Any State transitions do not transition to their current
            // state unless Can Transition To Self is explicitly enabled. The
            // serialized Stream Town controllers leave that option disabled.
            // A destination state-machine can resolve through its entry state,
            // so comparing only the transition's direct destination misses the
            // common Carry -> Carry case and restarts the clip every frame.
            (destination.as_ref() != Some(&self.current_state)).then_some((
                transition,
                entry_transition,
                destination,
            ))
        });
        let Some((transition, entry_transition, destination)) = transition else {
            return Ok(AnimationTransitionOutcome::None);
        };
        self.consume_transition_triggers(&transition.conditions);
        if let Some(entry) = entry_transition {
            self.consume_transition_triggers(&entry.conditions);
        }
        if let Some(destination) = destination {
            if !controller.states.contains_key(&destination) {
                return Err(AnimationRuntimeError::MissingState(destination));
            }
            self.current_state = destination.clone();
            self.last_transition = Some(AnimationTransitionPlayback {
                duration: transition.duration,
                fixed_duration: transition.fixed_duration,
                destination_offset: transition.offset,
            });
            Ok(AnimationTransitionOutcome::Entered(destination))
        } else if transition.is_exit {
            self.last_transition = Some(AnimationTransitionPlayback {
                duration: transition.duration,
                fixed_duration: transition.fixed_duration,
                destination_offset: transition.offset,
            });
            Ok(AnimationTransitionOutcome::Exited)
        } else {
            Ok(AnimationTransitionOutcome::None)
        }
    }

    pub fn motion_selection(
        &self,
        controller: &AnimationControllerDef,
    ) -> Result<Option<AnimationBlendSelection>, AnimationRuntimeError> {
        let state = controller
            .states
            .get(&self.current_state)
            .ok_or_else(|| AnimationRuntimeError::MissingState(self.current_state.clone()))?;
        Ok(self.motion_selection_for_state(state))
    }

    pub fn state_speed(
        &self,
        controller: &AnimationControllerDef,
    ) -> Result<f32, AnimationRuntimeError> {
        let state = controller
            .states
            .get(&self.current_state)
            .ok_or_else(|| AnimationRuntimeError::MissingState(self.current_state.clone()))?;
        let parameter_speed = state.speed_parameter.as_ref().map_or(1.0, |parameter| {
            self.parameters
                .get(parameter)
                .copied()
                .and_then(AnimationParameterValue::blend_value)
                .unwrap_or(1.0)
        });
        Ok(state.speed * parameter_speed)
    }

    fn motion_selection_for_state(
        &self,
        state: &AnimationStateDef,
    ) -> Option<AnimationBlendSelection> {
        let first_motion = state.motions.first()?;
        let Some(parameter) = &state.blend_parameter else {
            return Some(single_motion(first_motion.clip.clone()));
        };
        let value = self.parameters.get(parameter).copied()?.blend_value()?;
        let mut motions: Vec<_> = state
            .motions
            .iter()
            .filter_map(|motion| Some((motion.threshold?, motion.clip.clone())))
            .collect();
        motions.sort_by(|left, right| {
            left.0
                .total_cmp(&right.0)
                .then_with(|| left.1.cmp(&right.1))
        });
        let (first_threshold, first_clip) = motions.first()?.clone();
        if value <= first_threshold {
            return Some(single_motion(first_clip));
        }
        let (last_threshold, last_clip) = motions.last()?.clone();
        if value >= last_threshold {
            return Some(single_motion(last_clip));
        }
        for pair in motions.windows(2) {
            let [(lower_threshold, lower_clip), (upper_threshold, upper_clip)] = pair else {
                continue;
            };
            if value > *upper_threshold {
                continue;
            }
            let span = upper_threshold - lower_threshold;
            let upper_weight = if span.abs() <= f32::EPSILON {
                1.0
            } else {
                ((value - lower_threshold) / span).clamp(0.0, 1.0)
            };
            return Some(AnimationBlendSelection {
                first: WeightedAnimationMotion {
                    clip: lower_clip.clone(),
                    weight: 1.0 - upper_weight,
                },
                second: Some(WeightedAnimationMotion {
                    clip: upper_clip.clone(),
                    weight: upper_weight,
                }),
            });
        }
        Some(single_motion(first_clip))
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: AnimationParameterValue,
    ) -> Result<(), AnimationRuntimeError> {
        let Some(current) = self.parameters.get_mut(name) else {
            return Err(AnimationRuntimeError::MissingParameter(name.to_owned()));
        };
        if current.kind() != value.kind() {
            return Err(AnimationRuntimeError::ParameterType {
                name: name.to_owned(),
                expected: current.kind(),
                actual: value.kind(),
            });
        }
        *current = value;
        Ok(())
    }

    fn transition_source_is_active(
        &self,
        controller: &AnimationControllerDef,
        transition: &crate::AnimationTransitionDef,
    ) -> bool {
        if transition.source.as_ref() == Some(&self.current_state) {
            return true;
        }
        if transition.source.is_some() {
            return false;
        }
        let Some(source_machine) = &transition.source_state_machine else {
            return !transition.conditions.is_empty();
        };
        let Some(active_machine) = state_machine_for_state(controller, &self.current_state) else {
            return false;
        };
        transition.is_any_state
            && state_machine_contains_machine(controller, source_machine, active_machine)
            || source_machine == active_machine
    }

    fn consume_transition_triggers(&mut self, conditions: &[AnimationConditionDef]) {
        for condition in conditions {
            if matches!(
                self.parameters.get(&condition.parameter),
                Some(AnimationParameterValue::Trigger(true))
            ) {
                self.parameters.insert(
                    condition.parameter.clone(),
                    AnimationParameterValue::Trigger(false),
                );
            }
        }
    }

    fn condition_satisfied(&self, condition: &AnimationConditionDef) -> bool {
        let Some(value) = self.parameters.get(&condition.parameter).copied() else {
            return false;
        };
        match condition.mode {
            AnimationConditionMode::If => value.boolean(),
            AnimationConditionMode::IfNot => !value.boolean(),
            AnimationConditionMode::Greater => value.scalar() > f64::from(condition.threshold),
            AnimationConditionMode::Less => value.scalar() < f64::from(condition.threshold),
            AnimationConditionMode::Equals => {
                (value.scalar() - f64::from(condition.threshold)).abs() <= f64::EPSILON
            }
            AnimationConditionMode::NotEqual => {
                (value.scalar() - f64::from(condition.threshold)).abs() > f64::EPSILON
            }
        }
    }
}

fn state_machine_for_state<'a>(
    controller: &'a AnimationControllerDef,
    state: &StableId,
) -> Option<&'a StableId> {
    controller
        .state_machines
        .iter()
        .find_map(|(id, machine)| machine.states.contains(state).then_some(id))
}

fn parent_state_machine<'a>(
    controller: &'a AnimationControllerDef,
    child: &StableId,
) -> Option<&'a StableId> {
    controller
        .state_machines
        .iter()
        .find_map(|(id, machine)| machine.child_state_machines.contains(child).then_some(id))
}

fn state_machine_contains_machine(
    controller: &AnimationControllerDef,
    ancestor: &StableId,
    descendant: &StableId,
) -> bool {
    if ancestor == descendant {
        return true;
    }
    let mut pending = vec![ancestor];
    let mut visited = std::collections::BTreeSet::new();
    while let Some(machine) = pending.pop() {
        if !visited.insert(machine) {
            continue;
        }
        let Some(definition) = controller.state_machines.get(machine) else {
            continue;
        };
        if definition.child_state_machines.contains(descendant) {
            return true;
        }
        pending.extend(&definition.child_state_machines);
    }
    false
}

fn single_motion(clip: StableId) -> AnimationBlendSelection {
    AnimationBlendSelection {
        first: WeightedAnimationMotion { clip, weight: 1.0 },
        second: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnimationMotionDef, AnimationParameterDef, AnimationStateMachineDef, AnimationTransitionDef,
    };

    fn id(value: &str) -> StableId {
        StableId::new(value).unwrap()
    }

    fn controller() -> AnimationControllerDef {
        let idle = id("state:idle");
        let action = id("state:action");
        AnimationControllerDef {
            display_name: "Test".into(),
            source_guid: "test".into(),
            source_path: "test.controller".into(),
            parameters: vec![
                AnimationParameterDef {
                    name: "Move Speed".into(),
                    kind: AnimationParameterKind::Float,
                    default_float: 0.0,
                    default_integer: 0,
                    default_boolean: false,
                    inferred: false,
                },
                AnimationParameterDef {
                    name: "Action".into(),
                    kind: AnimationParameterKind::Trigger,
                    default_float: 0.0,
                    default_integer: 0,
                    default_boolean: false,
                    inferred: false,
                },
            ],
            states: BTreeMap::from([
                (
                    idle.clone(),
                    AnimationStateDef {
                        display_name: "Locomotion".into(),
                        speed: 1.0,
                        speed_parameter: None,
                        blend_parameter: Some("Move Speed".into()),
                        motions: vec![
                            AnimationMotionDef {
                                clip: id("clip:idle"),
                                threshold: Some(0.0),
                            },
                            AnimationMotionDef {
                                clip: id("clip:walk"),
                                threshold: Some(0.5),
                            },
                        ],
                    },
                ),
                (
                    action.clone(),
                    AnimationStateDef {
                        display_name: "Action".into(),
                        speed: 1.0,
                        speed_parameter: None,
                        blend_parameter: None,
                        motions: vec![AnimationMotionDef {
                            clip: id("clip:action"),
                            threshold: None,
                        }],
                    },
                ),
            ]),
            transitions: vec![AnimationTransitionDef {
                source: None,
                destination: Some(action),
                source_state_machine: None,
                destination_state_machine: None,
                is_entry: false,
                is_any_state: true,
                is_exit: false,
                has_exit_time: false,
                exit_time: 0.0,
                duration: 0.1,
                fixed_duration: true,
                offset: 0.0,
                conditions: vec![AnimationConditionDef {
                    parameter: "Action".into(),
                    mode: AnimationConditionMode::If,
                    threshold: 0.0,
                }],
            }],
            state_machines: BTreeMap::new(),
            layers: Vec::new(),
            default_states: vec![idle],
        }
    }

    #[test]
    fn blends_between_authored_thresholds() {
        let controller = controller();
        let mut runtime = AnimationControllerRuntime::new(&controller).unwrap();
        runtime.set_float("Move Speed", 0.25).unwrap();
        let blend = runtime.motion_selection(&controller).unwrap().unwrap();
        assert_eq!(blend.first.clip, id("clip:idle"));
        assert!((blend.first.weight - 0.5).abs() < f32::EPSILON);
        assert_eq!(blend.second.as_ref().unwrap().clip, id("clip:walk"));
        assert!((blend.second.unwrap().weight - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn consumes_trigger_when_any_state_transition_fires() {
        let mut controller = controller();
        controller.transitions.insert(
            0,
            AnimationTransitionDef {
                source: Some(id("state:idle")),
                destination: Some(id("state:idle")),
                source_state_machine: None,
                destination_state_machine: None,
                is_entry: false,
                is_any_state: false,
                is_exit: false,
                has_exit_time: false,
                exit_time: 0.0,
                duration: 0.0,
                fixed_duration: true,
                offset: 0.0,
                conditions: vec![AnimationConditionDef {
                    parameter: "Action".into(),
                    mode: AnimationConditionMode::IfNot,
                    threshold: 0.0,
                }],
            },
        );
        let mut runtime = AnimationControllerRuntime::new(&controller).unwrap();
        runtime.set_trigger("Action").unwrap();
        assert_eq!(
            runtime.evaluate_transitions(&controller, 0.0).unwrap(),
            AnimationTransitionOutcome::Entered(id("state:action"))
        );
        assert_eq!(
            runtime.take_transition_playback(),
            Some(AnimationTransitionPlayback {
                duration: 0.1,
                fixed_duration: true,
                destination_offset: 0.0,
            })
        );
        assert_eq!(
            runtime.evaluate_transitions(&controller, 0.0).unwrap(),
            AnimationTransitionOutcome::None
        );
    }

    #[test]
    fn nested_any_state_transition_does_not_restart_its_resolved_current_state() {
        let mut controller = controller();
        let parent = id("machine:parent");
        let child = id("machine:child");
        let action = id("state:action");
        controller.state_machines = BTreeMap::from([
            (
                parent.clone(),
                AnimationStateMachineDef {
                    display_name: "Parent".into(),
                    states: vec![id("state:idle")],
                    child_state_machines: vec![child.clone()],
                    default_state: Some(id("state:idle")),
                },
            ),
            (
                child.clone(),
                AnimationStateMachineDef {
                    display_name: "Child".into(),
                    states: vec![action.clone()],
                    child_state_machines: Vec::new(),
                    default_state: Some(action.clone()),
                },
            ),
        ]);
        controller.transitions = vec![AnimationTransitionDef {
            source: None,
            destination: None,
            source_state_machine: Some(parent),
            destination_state_machine: Some(child),
            is_entry: false,
            is_any_state: true,
            is_exit: false,
            has_exit_time: false,
            exit_time: 0.0,
            duration: 0.1,
            fixed_duration: true,
            offset: 0.0,
            conditions: vec![AnimationConditionDef {
                parameter: "Action".into(),
                mode: AnimationConditionMode::If,
                threshold: 0.0,
            }],
        }];
        let mut runtime =
            AnimationControllerRuntime::in_state(&controller, action.clone()).unwrap();
        runtime.set_trigger("Action").unwrap();

        assert_eq!(
            runtime.evaluate_transitions(&controller, 0.0).unwrap(),
            AnimationTransitionOutcome::None
        );
        assert_eq!(runtime.current_state(), &action);
        assert_eq!(runtime.take_transition_playback(), None);
    }

    #[test]
    fn authored_state_speed_multiplies_float_parameter() {
        let mut controller = controller();
        let action = id("state:action");
        controller.states.get_mut(&action).unwrap().speed = 1.5;
        controller.states.get_mut(&action).unwrap().speed_parameter = Some("Move Speed".into());
        let mut runtime = AnimationControllerRuntime::in_state(&controller, action).unwrap();
        runtime.set_float("Move Speed", 2.0).unwrap();
        assert!((runtime.state_speed(&controller).unwrap() - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn ignores_unconditional_source_less_state_machine_records() {
        let mut controller = controller();
        controller.transitions.insert(
            0,
            AnimationTransitionDef {
                source: None,
                destination: Some(id("state:action")),
                source_state_machine: None,
                destination_state_machine: None,
                is_entry: false,
                is_any_state: false,
                is_exit: false,
                has_exit_time: false,
                exit_time: 0.0,
                duration: 0.0,
                fixed_duration: true,
                offset: 0.0,
                conditions: Vec::new(),
            },
        );
        let mut runtime = AnimationControllerRuntime::new(&controller).unwrap();
        assert_eq!(
            runtime.evaluate_transitions(&controller, 0.0).unwrap(),
            AnimationTransitionOutcome::None
        );
        assert_eq!(runtime.current_state(), &id("state:idle"));
    }

    #[test]
    fn rejects_parameter_type_mismatches() {
        let controller = controller();
        let mut runtime = AnimationControllerRuntime::new(&controller).unwrap();
        assert!(matches!(
            runtime.set_boolean("Move Speed", true),
            Err(AnimationRuntimeError::ParameterType { .. })
        ));
    }
}
