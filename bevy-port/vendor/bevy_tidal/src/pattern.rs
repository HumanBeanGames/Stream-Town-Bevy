use crate::audio::TidalEvent;

#[derive(Clone, Debug)]
pub(crate) enum ProgramCommand {
    Set {
        track: u64,
        definition: TrackDefinition,
    },
    Silence(u64),
    Hush,
    SetCps(f64),
    Once(TrackDefinition),
    Panic,
}

#[derive(Clone, Debug)]
pub(crate) struct TrackDefinition {
    layers: Vec<Layer>,
}

impl TrackDefinition {
    pub(crate) fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub(crate) fn layer_period(&self, layer: usize, base_cycle_seconds: f64) -> f64 {
        base_cycle_seconds * self.layers[layer].cycle_scale
    }

    pub(crate) fn events_for_layer(
        &self,
        layer: usize,
        track: u64,
        cycle: u64,
        period_seconds: f64,
    ) -> Vec<(f64, TidalEvent)> {
        self.layers[layer].events(track, layer, cycle, period_seconds)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RhythmSource {
    Sound,
    Note,
}

#[derive(Clone, Copy, Debug)]
enum TransformCondition {
    Always,
    Every { period: u64, offset: u64 },
    Probability { chance: f32, per_event: bool },
}

impl TransformCondition {
    fn allows(self, cycle: u64, event_index: usize) -> bool {
        match self {
            Self::Always => true,
            Self::Every { period, offset } => cycle % period.max(1) == offset % period.max(1),
            Self::Probability { chance, per_event } => {
                let salt = if per_event { event_index as u64 } else { 0 };
                deterministic_unit(mix_seed(cycle, salt ^ 0xD1B5_4A32_D192_ED03)) < chance
            }
        }
    }

    fn is_always(self) -> bool {
        matches!(self, Self::Always)
    }
}

#[derive(Clone, Debug)]
enum Transform {
    Reverse(TransformCondition),
    Palindrome,
    Degrade {
        keep: f32,
        condition: TransformCondition,
    },
    Ply {
        count: u32,
        condition: TransformCondition,
    },
    Press {
        amount: f64,
        condition: TransformCondition,
    },
    Rotate {
        amount: f64,
        condition: TransformCondition,
    },
    Iter {
        count: u32,
        backwards: bool,
    },
    Swing {
        amount: f64,
        subdivisions: u32,
        condition: TransformCondition,
    },
    Compress {
        start: f64,
        end: f64,
        condition: TransformCondition,
    },
    Zoom {
        start: f64,
        end: f64,
        condition: TransformCondition,
    },
    Truncate {
        length: f64,
        condition: TransformCondition,
    },
    Shuffle {
        condition: TransformCondition,
    },
    FastGap {
        factor: f64,
        condition: TransformCondition,
    },
    Fast {
        factor: u32,
        condition: TransformCondition,
    },
    Hurry {
        factor: f32,
        condition: TransformCondition,
    },
    Jux {
        amount: f32,
        reverse: bool,
        condition: TransformCondition,
    },
    Spin {
        count: u32,
        condition: TransformCondition,
    },
    Echo {
        count: u32,
        offset: f64,
        decay: f32,
        condition: TransformCondition,
    },
    Off {
        offset: f64,
        note_offset: f32,
        condition: TransformCondition,
    },
    Ghost(TransformCondition),
    Slice {
        count: u32,
        random: bool,
        condition: TransformCondition,
    },
    LoopAt {
        cycles: f32,
        condition: TransformCondition,
    },
    Mask {
        pattern: Sequence,
        structure: bool,
        condition: TransformCondition,
    },
    LoopFirst,
}

#[derive(Clone, Debug)]
struct Layer {
    source: RhythmSource,
    rhythm: Sequence,
    sound: Option<Sequence>,
    note: Option<Sequence>,
    scale: Option<String>,
    roll: Option<f64>,
    controls: Controls,
    transforms: Vec<Transform>,
    cycle_scale: f64,
}

impl Layer {
    fn events(
        &self,
        track: u64,
        layer: usize,
        cycle: u64,
        period_seconds: f64,
    ) -> Vec<(f64, TidalEvent)> {
        let source_cycle = if self
            .transforms
            .iter()
            .any(|transform| matches!(transform, Transform::LoopFirst))
        {
            0
        } else {
            cycle
        };
        let hits = self.rhythm.hits(source_cycle, (track << 32) | layer as u64);
        let hits = apply_hit_transforms(hits, &self.transforms, cycle);
        let mut events = Vec::with_capacity(hits.len() * 3);
        for (event_index, hit) in hits.into_iter().enumerate() {
            let mut base_event = TidalEvent {
                track,
                ..TidalEvent::default()
            };
            let mut chord = vec![base_event.note];
            match self.source {
                RhythmSource::Sound => apply_sound(&mut base_event, &hit.value),
                RhythmSource::Note => {
                    chord = parse_note_or_chord(&hit.value).unwrap_or_else(|| vec![0.0]);
                    if let Some(scale) = &self.scale {
                        for note in &mut chord {
                            *note = scale_degree(scale, *note).unwrap_or(*note);
                        }
                    }
                }
            }

            if let Some(sound) = &self.sound
                && let Some(value) = sound.value_at(event_index, cycle)
            {
                apply_sound(&mut base_event, &value);
            }
            if let Some(note) = &self.note
                && let Some(value) = note.value_at(event_index, cycle)
            {
                chord = parse_note_or_chord(&value).unwrap_or(chord);
            }

            let position = cycle as f64 + hit.phase;
            for (chord_index, note) in chord.iter().copied().enumerate() {
                let mut event = base_event.clone();
                event.note = note;
                self.controls
                    .apply(&mut event, position, event_index, cycle);
                if let Some(legato) = self.controls.legato {
                    event.sustain = (period_seconds * hit.slot_length * f64::from(legato)) as f32;
                }
                let roll = self.roll.unwrap_or(0.0);
                let phase_offset = if chord.len() > 1 {
                    hit.slot_length * roll * chord_index as f64 / chord.len() as f64
                } else {
                    0.0
                };
                events.push((hit.phase + phase_offset, event));
            }
        }
        apply_event_transforms(events, &self.transforms, cycle)
    }
}

fn apply_hit_transforms(
    mut hits: Vec<PatternHit>,
    transforms: &[Transform],
    cycle: u64,
) -> Vec<PatternHit> {
    for transform in transforms {
        match *transform {
            Transform::Reverse(condition) => {
                for (index, hit) in hits.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        hit.phase = 1.0 - hit.phase - hit.slot_length;
                    }
                }
            }
            Transform::Palindrome if cycle % 2 == 1 => {
                for hit in &mut hits {
                    hit.phase = 1.0 - hit.phase - hit.slot_length;
                }
            }
            Transform::Degrade { keep, condition } => {
                hits = hits
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, hit)| {
                        let applies = condition.allows(cycle, index);
                        (!applies
                            || deterministic_unit(mix_seed(
                                cycle ^ 0x94D0_49BB_1331_11EB,
                                index as u64,
                            )) < keep)
                            .then_some(hit)
                    })
                    .collect();
            }
            Transform::Ply { count, condition } => {
                let mut repeated = Vec::with_capacity(hits.len() * count as usize);
                for (index, hit) in hits.into_iter().enumerate() {
                    if condition.allows(cycle, index) {
                        let count = count.max(1);
                        let length = hit.slot_length / f64::from(count);
                        for repetition in 0..count {
                            repeated.push(PatternHit {
                                phase: hit.phase + f64::from(repetition) * length,
                                slot_length: length,
                                value: hit.value.clone(),
                            });
                        }
                    } else {
                        repeated.push(hit);
                    }
                }
                hits = repeated;
            }
            Transform::Press { amount, condition } => {
                for (index, hit) in hits.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        let amount = amount.clamp(0.0, 0.999);
                        hit.phase += hit.slot_length * amount;
                        hit.slot_length *= 1.0 - amount;
                    }
                }
            }
            Transform::Rotate { amount, condition } => {
                for (index, hit) in hits.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        hit.phase = (hit.phase + amount).rem_euclid(1.0);
                    }
                }
            }
            Transform::Iter { count, backwards } => {
                let direction = if backwards { -1.0 } else { 1.0 };
                let rotation =
                    direction * (cycle % u64::from(count.max(1))) as f64 / f64::from(count.max(1));
                for hit in &mut hits {
                    hit.phase = (hit.phase + rotation).rem_euclid(1.0);
                }
            }
            Transform::Swing {
                amount,
                subdivisions,
                condition,
            } => {
                let half_slice = 1.0 / (f64::from(subdivisions.max(1)) * 2.0);
                for (index, hit) in hits.iter_mut().enumerate() {
                    let step = (hit.phase / half_slice).floor() as u64;
                    if step % 2 == 1 && condition.allows(cycle, index) {
                        hit.phase = (hit.phase + half_slice * amount).rem_euclid(1.0);
                    }
                }
            }
            Transform::Compress {
                start,
                end,
                condition,
            } => {
                let span = (end - start).max(0.001);
                for (index, hit) in hits.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        hit.phase = start + hit.phase * span;
                        hit.slot_length *= span;
                    }
                }
            }
            Transform::Zoom {
                start,
                end,
                condition,
            } => {
                let span = (end - start).max(0.001);
                hits = hits
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, mut hit)| {
                        if !condition.allows(cycle, index) {
                            return Some(hit);
                        }
                        (hit.phase >= start && hit.phase < end).then(|| {
                            hit.phase = (hit.phase - start) / span;
                            hit.slot_length = (hit.slot_length / span).min(1.0 - hit.phase);
                            hit
                        })
                    })
                    .collect();
            }
            Transform::Truncate { length, condition } => {
                hits = hits
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, mut hit)| {
                        if !condition.allows(cycle, index) || hit.phase < length {
                            hit.slot_length = hit.slot_length.min((length - hit.phase).max(0.0));
                            (hit.slot_length > 0.0).then_some(hit)
                        } else {
                            None
                        }
                    })
                    .collect();
            }
            Transform::Shuffle { condition } => {
                let mut values = hits.iter().map(|hit| hit.value.clone()).collect::<Vec<_>>();
                for index in (1..values.len()).rev() {
                    let choice = (deterministic_unit(mix_seed(cycle, index as u64))
                        * (index + 1) as f32)
                        .floor() as usize;
                    values.swap(index, choice.min(index));
                }
                for (index, hit) in hits.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        hit.value.clone_from(&values[index]);
                    }
                }
            }
            Transform::FastGap { factor, condition } => {
                for (index, hit) in hits.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        hit.phase /= factor.max(0.001);
                        hit.slot_length /= factor.max(0.001);
                    }
                }
            }
            Transform::Fast { factor, condition } => {
                let mut fast = Vec::with_capacity(hits.len() * factor as usize);
                for (index, hit) in hits.into_iter().enumerate() {
                    if condition.allows(cycle, index) {
                        let factor = factor.max(1);
                        for repetition in 0..factor {
                            fast.push(PatternHit {
                                phase: (hit.phase + f64::from(repetition)) / f64::from(factor),
                                slot_length: hit.slot_length / f64::from(factor),
                                value: hit.value.clone(),
                            });
                        }
                    } else {
                        fast.push(hit);
                    }
                }
                hits = fast;
            }
            Transform::Mask {
                ref pattern,
                structure,
                condition,
            } => {
                let mask = pattern.hits(cycle, 0xE703_7ED1_A0B4_28DB);
                if structure {
                    if !hits.is_empty() {
                        hits = mask
                            .into_iter()
                            .enumerate()
                            .filter_map(|(index, mask_hit)| {
                                condition.allows(cycle, index).then(|| PatternHit {
                                    phase: mask_hit.phase,
                                    slot_length: mask_hit.slot_length,
                                    value: hits[index % hits.len()].value.clone(),
                                })
                            })
                            .collect();
                    }
                } else {
                    hits = hits
                        .into_iter()
                        .enumerate()
                        .filter(|(index, hit)| {
                            !condition.allows(cycle, *index)
                                || mask.iter().any(|mask_hit| {
                                    hit.phase >= mask_hit.phase
                                        && hit.phase < mask_hit.phase + mask_hit.slot_length
                                })
                        })
                        .map(|(_, hit)| hit)
                        .collect();
                }
            }
            Transform::Hurry { .. }
            | Transform::Jux { .. }
            | Transform::Spin { .. }
            | Transform::Echo { .. }
            | Transform::Off { .. }
            | Transform::Ghost(_)
            | Transform::Slice { .. }
            | Transform::LoopAt { .. }
            | Transform::LoopFirst
            | Transform::Palindrome => {}
        }
        hits.sort_by(|left, right| left.phase.total_cmp(&right.phase));
    }
    hits
}

fn apply_event_transforms(
    mut events: Vec<(f64, TidalEvent)>,
    transforms: &[Transform],
    cycle: u64,
) -> Vec<(f64, TidalEvent)> {
    for transform in transforms {
        match *transform {
            Transform::Hurry { factor, condition } => {
                for (index, (_, event)) in events.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        event.speed *= factor;
                    }
                }
            }
            Transform::Jux {
                amount,
                reverse,
                condition,
            } => {
                let mut effected = Vec::new();
                for (index, (phase, event)) in events.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        let spread = amount.clamp(0.0, 1.0) * 0.5;
                        event.pan = 0.5 - spread;
                        let mut copy = event.clone();
                        copy.pan = 0.5 + spread;
                        let copy_phase = if reverse { 1.0 - *phase } else { *phase };
                        effected.push((copy_phase.rem_euclid(1.0), copy));
                    }
                }
                events.extend(effected);
            }
            Transform::Spin { count, condition } => {
                let source = events.clone();
                let mut spun = Vec::new();
                for copy in 1..count.max(1) {
                    for (index, (phase, event)) in source.iter().enumerate() {
                        if condition.allows(cycle, index) {
                            let mut event = event.clone();
                            event.pan = copy as f32 / (count.max(1) - 1).max(1) as f32;
                            spun.push((
                                (phase + f64::from(copy) / f64::from(count)).rem_euclid(1.0),
                                event,
                            ));
                        }
                    }
                }
                events.extend(spun);
            }
            Transform::Echo {
                count,
                offset,
                decay,
                condition,
            } => {
                let source = events.clone();
                for repetition in 1..count.max(1) {
                    for (index, (phase, event)) in source.iter().enumerate() {
                        if condition.allows(cycle, index) {
                            let mut event = event.clone();
                            event.gain *= decay.powi(repetition as i32);
                            events.push((phase + offset * f64::from(repetition), event));
                        }
                    }
                }
            }
            Transform::Off {
                offset,
                note_offset,
                condition,
            } => {
                let source = events.clone();
                for (index, (phase, event)) in source.into_iter().enumerate() {
                    if condition.allows(cycle, index) {
                        let mut event = event;
                        event.note += note_offset;
                        events.push((phase + offset, event));
                    }
                }
            }
            Transform::Ghost(condition) => {
                let source = events.clone();
                for (index, (phase, event)) in source.into_iter().enumerate() {
                    if condition.allows(cycle, index) {
                        let mut event = event;
                        event.gain *= 0.7;
                        event.pan = 1.0 - event.pan;
                        events.push((phase + 0.125, event));
                    }
                }
            }
            Transform::Slice {
                count,
                random,
                condition,
            } => {
                let source = events.clone();
                let mut sliced = Vec::new();
                for (index, (phase, event)) in source.iter().enumerate() {
                    if !condition.allows(cycle, index) {
                        sliced.push((*phase, event.clone()));
                        continue;
                    }
                    let next_phase = source
                        .iter()
                        .skip(index + 1)
                        .map(|(phase, _)| *phase)
                        .find(|next| *next > *phase)
                        .unwrap_or(1.0);
                    let slot = (next_phase - phase).max(1.0 / f64::from(count.max(1)));
                    for repetition in 0..count.max(1) {
                        let slice = if random {
                            (deterministic_unit(mix_seed(
                                cycle ^ index as u64,
                                u64::from(repetition),
                            )) * count as f32)
                                .floor() as u32
                        } else {
                            repetition
                        };
                        let mut event = event.clone();
                        event.begin = slice as f32 / count as f32;
                        event.end = (slice + 1) as f32 / count as f32;
                        sliced.push((
                            phase + slot * f64::from(repetition) / f64::from(count),
                            event,
                        ));
                    }
                }
                events = sliced;
            }
            Transform::LoopAt { cycles, condition } => {
                for (index, (_, event)) in events.iter_mut().enumerate() {
                    if condition.allows(cycle, index) {
                        event.speed /= cycles.max(0.001);
                    }
                }
            }
            _ => {}
        }
    }
    events.sort_by(|left, right| left.0.total_cmp(&right.0));
    events
}

fn apply_sound(event: &mut TidalEvent, value: &str) {
    if let Some((sound, index)) = value.rsplit_once(':')
        && let Ok(index) = index.parse::<f32>()
    {
        event.sound = sound.to_owned();
        event.note = index;
    } else {
        event.sound = value.to_owned();
    }
}

#[derive(Clone, Debug, Default)]
struct Controls {
    gain: Option<ControlValue>,
    amp: Option<ControlValue>,
    pan: Option<ControlValue>,
    speed: Option<ControlValue>,
    accelerate: Option<ControlValue>,
    frequency: Option<ControlValue>,
    sustain: Option<ControlValue>,
    legato: Option<f32>,
    attack: Option<ControlValue>,
    hold: Option<ControlValue>,
    release: Option<ControlValue>,
    begin: Option<ControlValue>,
    end: Option<ControlValue>,
    cut: Option<ControlValue>,
    crush: Option<ControlValue>,
    coarse: Option<ControlValue>,
    shape: Option<ControlValue>,
    distort: Option<ControlValue>,
    triode: Option<ControlValue>,
    cutoff: Option<ControlValue>,
    resonance: Option<ControlValue>,
    hpf: Option<ControlValue>,
    hpq: Option<ControlValue>,
    bpf: Option<ControlValue>,
    bpq: Option<ControlValue>,
    room: Option<ControlValue>,
    dry: Option<ControlValue>,
    delay: Option<ControlValue>,
    delay_time: Option<ControlValue>,
    delay_feedback: Option<ControlValue>,
    tremolo_depth: Option<ControlValue>,
    tremolo_rate: Option<ControlValue>,
    ring: Option<ControlValue>,
    ring_frequency: Option<ControlValue>,
}

impl Controls {
    fn apply(&self, event: &mut TidalEvent, position: f64, event_index: usize, cycle: u64) {
        if let Some(value) = &self.gain {
            event.gain = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.amp {
            event.amp = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.pan {
            event.pan = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.speed {
            event.speed = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.accelerate {
            event.accelerate = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.frequency {
            event.frequency = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.sustain {
            event.sustain = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.attack {
            event.attack = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.hold {
            event.hold = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.release {
            event.release = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.begin {
            event.begin = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.end {
            event.end = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.cut {
            event.cut = value.at(position, event_index, cycle) as i32;
        }
        if let Some(value) = &self.crush {
            event.crush = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.coarse {
            event.coarse = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.shape {
            event.shape = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.distort {
            event.distort = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.triode {
            event.triode = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.cutoff {
            event.cutoff = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.resonance {
            event.resonance = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.hpf {
            event.hpf = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.hpq {
            event.hpq = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.bpf {
            event.bpf = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.bpq {
            event.bpq = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.room {
            event.room = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.dry {
            event.dry = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.delay {
            event.delay = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.delay_time {
            event.delay_time = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.delay_feedback {
            event.delay_feedback = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.tremolo_depth {
            event.tremolo_depth = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.tremolo_rate {
            event.tremolo_rate = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.ring {
            event.ring = value.at(position, event_index, cycle);
        }
        if let Some(value) = &self.ring_frequency {
            event.ring_frequency = value.at(position, event_index, cycle);
        }
    }
}

#[derive(Clone, Debug)]
enum ControlValue {
    Constant(f32),
    Pattern(Sequence),
    Signal {
        waveform: Waveform,
        minimum: f32,
        maximum: f32,
        cycles: f64,
        integer: Option<u32>,
        exponential: bool,
    },
    Choice {
        values: Vec<f32>,
        per_cycle: bool,
    },
    Quantized {
        value: Box<ControlValue>,
        amount: f32,
    },
    Segmented {
        value: Box<ControlValue>,
        steps: u32,
    },
}

impl ControlValue {
    fn at(&self, position: f64, event_index: usize, cycle: u64) -> f32 {
        match self {
            Self::Constant(value) => *value,
            Self::Pattern(sequence) => sequence
                .value_at(event_index, cycle)
                .and_then(|value| value.parse::<f32>().ok())
                .unwrap_or(0.0),
            Self::Signal {
                waveform,
                minimum,
                maximum,
                cycles,
                integer,
                exponential,
            } => {
                let phase = position / cycles.max(0.001);
                let unit = waveform.at(phase, event_index as u64);
                let value = if *exponential && *minimum > 0.0 && *maximum > 0.0 {
                    minimum * (maximum / minimum).powf(unit)
                } else {
                    minimum + (maximum - minimum) * unit
                };
                integer.map_or(value, |maximum| (value * maximum as f32).floor())
            }
            Self::Choice { values, per_cycle } => {
                let salt = if *per_cycle { 0 } else { event_index as u64 };
                let index = (deterministic_unit(mix_seed(cycle, salt)) * values.len() as f32)
                    .floor() as usize;
                values[index.min(values.len() - 1)]
            }
            Self::Quantized { value, amount } => {
                let amount = amount.abs().max(0.001);
                (value.at(position, event_index, cycle) * amount).round() / amount
            }
            Self::Segmented { value, steps } => {
                let steps = (*steps).max(1);
                let position = (position * f64::from(steps)).floor() / f64::from(steps);
                value.at(position, event_index, cycle)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Waveform {
    Sine,
    Cosine,
    Square,
    Triangle,
    Saw,
    InverseSaw,
    Random,
    Perlin,
}

impl Waveform {
    fn at(self, phase: f64, salt: u64) -> f32 {
        let wrapped = phase.rem_euclid(1.0);
        match self {
            Self::Sine => (((phase * std::f64::consts::TAU).sin() + 1.0) * 0.5) as f32,
            Self::Cosine => (((phase * std::f64::consts::TAU).cos() + 1.0) * 0.5) as f32,
            Self::Square => {
                if wrapped >= 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Triangle => (1.0 - (wrapped * 2.0 - 1.0).abs()) as f32,
            Self::Saw => wrapped as f32,
            Self::InverseSaw => (1.0 - wrapped) as f32,
            Self::Random => deterministic_unit(mix_seed(phase.to_bits(), salt)),
            Self::Perlin => {
                let left = phase.floor();
                let fraction = phase - left;
                let smooth = fraction * fraction * (3.0 - 2.0 * fraction);
                let a = deterministic_unit(mix_seed(left.to_bits(), salt));
                let b = deterministic_unit(mix_seed((left + 1.0).to_bits(), salt));
                a + (b - a) * smooth as f32
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Sequence {
    root: MiniNode,
}

impl Sequence {
    fn parse(source: &str) -> Result<Self, String> {
        Ok(Self {
            root: parse_mini_node(source)?,
        })
    }

    fn hits(&self, cycle: u64, seed: u64) -> Vec<PatternHit> {
        let mut hits = Vec::new();
        self.root.render(cycle, 0.0, 1.0, seed, &mut hits);
        hits.sort_by(|left, right| left.phase.total_cmp(&right.phase));
        hits
    }

    fn value_at(&self, index: usize, cycle: u64) -> Option<String> {
        let hits = self.hits(cycle, 0xA076_1D64_78BD_642F);
        hits.get(index % hits.len().max(1))
            .map(|hit| hit.value.clone())
    }
}

#[derive(Clone, Debug)]
struct PatternHit {
    phase: f64,
    slot_length: f64,
    value: String,
}

#[derive(Clone, Debug)]
struct WeightedNode {
    node: MiniNode,
    weight: f64,
}

#[derive(Clone, Debug)]
enum MiniNode {
    Atom(String),
    Rest,
    Sequence(Vec<WeightedNode>),
    Stack(Vec<MiniNode>),
    Alternate(Vec<MiniNode>),
    Choice(Vec<MiniNode>),
    RandomRun(u32),
    Repeat(Box<MiniNode>, u32),
    Divide(Box<MiniNode>, u32),
    Euclid {
        node: Box<MiniNode>,
        pulses: u32,
        steps: u32,
        offset: u32,
    },
    Degrade(Box<MiniNode>, f32),
    Polymeter {
        parts: Vec<MiniNode>,
        steps: Option<usize>,
    },
}

impl MiniNode {
    fn render(&self, cycle: u64, start: f64, length: f64, seed: u64, output: &mut Vec<PatternHit>) {
        match self {
            Self::Atom(value) => output.push(PatternHit {
                phase: start,
                slot_length: length,
                value: value.clone(),
            }),
            Self::Rest => {}
            Self::Sequence(nodes) => {
                let total = nodes.iter().map(|node| node.weight).sum::<f64>().max(0.001);
                let mut cursor = start;
                for (index, node) in nodes.iter().enumerate() {
                    let node_length = length * node.weight / total;
                    node.node.render(
                        cycle,
                        cursor,
                        node_length,
                        mix_seed(seed, index as u64),
                        output,
                    );
                    cursor += node_length;
                }
            }
            Self::Stack(nodes) => {
                for (index, node) in nodes.iter().enumerate() {
                    node.render(cycle, start, length, mix_seed(seed, index as u64), output);
                }
            }
            Self::Alternate(nodes) => {
                if let Some(node) = nodes.get(cycle as usize % nodes.len()) {
                    node.render(cycle, start, length, seed, output);
                }
            }
            Self::Choice(nodes) => {
                let choice = (deterministic_unit(mix_seed(seed, cycle)) * nodes.len() as f32)
                    .floor() as usize;
                if let Some(node) = nodes.get(choice.min(nodes.len().saturating_sub(1))) {
                    node.render(cycle, start, length, seed, output);
                }
            }
            Self::RandomRun(count) => {
                let count = (*count).max(1);
                let mut values = (0..count).collect::<Vec<_>>();
                for index in (1..values.len()).rev() {
                    let choice = (deterministic_unit(mix_seed(seed ^ cycle, index as u64))
                        * (index + 1) as f32)
                        .floor() as usize;
                    values.swap(index, choice.min(index));
                }
                let part = length / f64::from(count);
                for (index, value) in values.into_iter().enumerate() {
                    output.push(PatternHit {
                        phase: start + index as f64 * part,
                        slot_length: part,
                        value: value.to_string(),
                    });
                }
            }
            Self::Repeat(node, count) => {
                let count = (*count).max(1);
                let part = length / f64::from(count);
                for index in 0..count {
                    node.render(
                        cycle,
                        start + f64::from(index) * part,
                        part,
                        mix_seed(seed, u64::from(index)),
                        output,
                    );
                }
            }
            Self::Divide(node, divisor) => {
                if cycle.is_multiple_of(u64::from((*divisor).max(1))) {
                    node.render(
                        cycle / u64::from((*divisor).max(1)),
                        start,
                        length,
                        seed,
                        output,
                    );
                }
            }
            Self::Euclid {
                node,
                pulses,
                steps,
                offset,
            } => {
                let steps = (*steps).max(1);
                let part = length / f64::from(steps);
                for step in 0..steps {
                    if is_euclidean_hit(step, *pulses, steps, *offset) {
                        node.render(
                            cycle,
                            start + f64::from(step) * part,
                            part,
                            mix_seed(seed, u64::from(step)),
                            output,
                        );
                    }
                }
            }
            Self::Degrade(node, probability) => {
                let mut candidate = Vec::new();
                node.render(cycle, start, length, seed, &mut candidate);
                output.extend(
                    candidate
                        .into_iter()
                        .enumerate()
                        .filter_map(|(index, hit)| {
                            (deterministic_unit(mix_seed(seed ^ cycle, index as u64))
                                <= *probability)
                                .then_some(hit)
                        }),
                );
            }
            Self::Polymeter { parts, steps } => {
                let target_steps = steps
                    .unwrap_or_else(|| top_level_nodes(&parts[0]).len())
                    .max(1);
                for (part_index, node) in parts.iter().enumerate() {
                    let nodes = top_level_nodes(node);
                    for step in 0..target_steps {
                        let source_index =
                            (cycle as usize * target_steps + step) % nodes.len().max(1);
                        if let Some(source) = nodes.get(source_index) {
                            let step_length = length / target_steps as f64;
                            source.render(
                                cycle,
                                start + step as f64 * step_length,
                                step_length,
                                mix_seed(seed ^ part_index as u64, step as u64),
                                output,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn top_level_nodes(node: &MiniNode) -> Vec<&MiniNode> {
    match node {
        MiniNode::Sequence(nodes) => nodes.iter().map(|node| &node.node).collect(),
        _ => vec![node],
    }
}

pub(crate) fn parse_program(source: &str) -> Result<Vec<ProgramCommand>, String> {
    let mut commands = Vec::new();
    for statement in split_statements(source) {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        if statement == "hush" {
            commands.push(ProgramCommand::Hush);
            continue;
        }
        if statement == "panic" {
            commands.push(ProgramCommand::Panic);
            continue;
        }
        if let Some(value) = statement
            .strip_prefix("setcps")
            .or_else(|| statement.strip_prefix("setCps"))
        {
            let cycles_per_second = parse_division_expression(value.trim())?;
            if !cycles_per_second.is_finite() || cycles_per_second <= 0.0 {
                return Err("`setcps` needs a positive finite value".to_owned());
            }
            commands.push(ProgramCommand::SetCps(cycles_per_second));
            continue;
        }
        if let Some(expression) = statement.strip_prefix("once") {
            let expression = expression.trim().trim_start_matches('$').trim();
            let layers = parse_layers(expression)?;
            commands.push(ProgramCommand::Once(TrackDefinition { layers }));
            continue;
        }
        let (track, expression) = parse_track_header(statement)?;
        let expression = expression.trim().trim_start_matches('$').trim();
        if expression == "silence" {
            commands.push(ProgramCommand::Silence(track));
            continue;
        }
        let layers = parse_layers(expression)?;
        commands.push(ProgramCommand::Set {
            track,
            definition: TrackDefinition { layers },
        });
    }
    if commands.is_empty() {
        return Err("No pattern, `setcps`, or `hush` command was found".to_owned());
    }
    Ok(commands)
}

fn parse_layers(expression: &str) -> Result<Vec<Layer>, String> {
    if expression.starts_with("stack") {
        parse_stack(expression)
    } else {
        Ok(vec![parse_layer(expression)?])
    }
}

fn parse_division_expression(source: &str) -> Result<f64, String> {
    let source = source.trim().trim_matches(['(', ')']);
    let mut parts = source.split('/').map(str::trim);
    let first = parts
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "`setcps` is missing its value".to_owned())?
        .parse::<f64>()
        .map_err(|_| format!("Invalid tempo expression `{source}`"))?;
    parts.try_fold(first, |value, divisor| {
        let divisor = divisor
            .parse::<f64>()
            .map_err(|_| format!("Invalid tempo expression `{source}`"))?;
        if divisor == 0.0 {
            Err("A tempo expression cannot divide by zero".to_owned())
        } else {
            Ok(value / divisor)
        }
    })
}

fn parse_track_header(statement: &str) -> Result<(u64, &str), String> {
    if let Some(rest) = statement.strip_prefix('p') {
        let rest = rest.trim_start();
        if let Some(rest) = rest.strip_prefix('"') {
            let close = rest.find('"').ok_or_else(|| {
                format!("Named pattern is missing its closing quote in `{statement}`")
            })?;
            let name = &rest[..close];
            return Ok((hash_pattern_name(name), &rest[close + 1..]));
        }
        let digit_count = rest.chars().take_while(char::is_ascii_digit).count();
        if digit_count == 0 {
            return Err(format!(
                "Expected a number or quoted name after `p` in `{statement}`"
            ));
        }
        let number = rest[..digit_count]
            .parse::<u64>()
            .map_err(|_| format!("Invalid pattern number in `{statement}`"))?;
        return Ok((number, &rest[digit_count..]));
    }
    let Some(rest) = statement.strip_prefix('d') else {
        return Err(format!(
            "Expected a d1/d2/... or p \"name\" assignment, found `{statement}`"
        ));
    };
    let digit_count = rest.chars().take_while(char::is_ascii_digit).count();
    if digit_count == 0 {
        return Err(format!("Expected a numbered track, found `{statement}`"));
    }
    let track = rest[..digit_count]
        .parse::<u64>()
        .map_err(|_| format!("Track number is out of range in `{statement}`"))?;
    Ok((track, &rest[digit_count..]))
}

fn hash_pattern_name(name: &str) -> u64 {
    name.bytes().fold(0xCBF2_9CE4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01B3)
    })
}

fn parse_stack(expression: &str) -> Result<Vec<Layer>, String> {
    let open = expression
        .find('[')
        .ok_or_else(|| "`stack` needs a bracketed list".to_owned())?;
    let close = expression
        .rfind(']')
        .ok_or_else(|| "`stack` is missing its closing `]`".to_owned())?;
    let layers = split_top_level(&expression[open + 1..close], ',')
        .into_iter()
        .map(|layer| parse_layer(layer.trim()))
        .collect::<Result<Vec<_>, _>>()?;
    if layers.is_empty() {
        return Err("`stack` cannot be empty".to_owned());
    }
    Ok(layers)
}

fn parse_layer(expression: &str) -> Result<Layer, String> {
    let mut chain = split_top_level(expression, '#');
    let mut primary = chain.remove(0).trim();
    let mut outer_scale = 1.0;
    let mut transforms = Vec::new();
    if let Some(index) = last_top_level_character(primary, '$') {
        let prefix = &primary[..index];
        outer_scale = parse_top_level_rate_scale(prefix)?;
        transforms = parse_transforms(prefix)?;
        primary = primary[index + 1..].trim();
    }
    let (source, rhythm, inner_scale) = parse_primary(primary)?;
    let mut layer = Layer {
        source,
        rhythm,
        sound: None,
        note: None,
        scale: parse_scale_name(primary),
        roll: parse_roll(expression),
        controls: Controls::default(),
        transforms,
        cycle_scale: outer_scale * inner_scale,
    };

    for control in chain {
        let control = control.trim();
        let key_end = control.find(char::is_whitespace).unwrap_or(control.len());
        let key = &control[..key_end];
        let value = control[key_end..].trim();
        match key {
            "sound" | "s" => layer.sound = Some(parse_sequence_value(value)?),
            "n" | "note" => layer.note = Some(parse_sequence_value(value)?),
            "gain" => layer.controls.gain = Some(parse_control(value)?),
            "amp" => layer.controls.amp = Some(parse_control(value)?),
            "pan" => layer.controls.pan = Some(parse_control(value)?),
            "speed" => layer.controls.speed = Some(parse_control(value)?),
            "accelerate" => layer.controls.accelerate = Some(parse_control(value)?),
            "freq" | "frequency" => layer.controls.frequency = Some(parse_control(value)?),
            "sustain" => layer.controls.sustain = Some(parse_control(value)?),
            "legato" => layer.controls.legato = Some(parse_number(value)?),
            "attack" | "att" => layer.controls.attack = Some(parse_control(value)?),
            "hold" => layer.controls.hold = Some(parse_control(value)?),
            "release" | "rel" => layer.controls.release = Some(parse_control(value)?),
            "begin" => layer.controls.begin = Some(parse_control(value)?),
            "end" => layer.controls.end = Some(parse_control(value)?),
            "cut" => layer.controls.cut = Some(parse_control(value)?),
            "crush" => layer.controls.crush = Some(parse_control(value)?),
            "coarse" => layer.controls.coarse = Some(parse_control(value)?),
            "shape" => layer.controls.shape = Some(parse_control(value)?),
            "distort" => layer.controls.distort = Some(parse_control(value)?),
            "triode" => layer.controls.triode = Some(parse_control(value)?),
            "cutoff" | "lpf" => layer.controls.cutoff = Some(parse_control(value)?),
            "resonance" | "lpq" => layer.controls.resonance = Some(parse_control(value)?),
            "hcutoff" | "hpf" => layer.controls.hpf = Some(parse_control(value)?),
            "hresonance" | "hpq" => layer.controls.hpq = Some(parse_control(value)?),
            "bandf" | "bpf" => layer.controls.bpf = Some(parse_control(value)?),
            "bandq" | "bpq" => layer.controls.bpq = Some(parse_control(value)?),
            "room" | "sz" => layer.controls.room = Some(parse_control(value)?),
            "dry" => layer.controls.dry = Some(parse_control(value)?),
            "delay" => layer.controls.delay = Some(parse_control(value)?),
            "delaytime" | "delayt" => layer.controls.delay_time = Some(parse_control(value)?),
            "delayfeedback" | "delayfb" => {
                layer.controls.delay_feedback = Some(parse_control(value)?);
            }
            "tremolodepth" | "tremdp" => {
                layer.controls.tremolo_depth = Some(parse_control(value)?);
            }
            "tremolorate" | "tremr" => {
                layer.controls.tremolo_rate = Some(parse_control(value)?);
            }
            "ring" => layer.controls.ring = Some(parse_control(value)?),
            "ringf" => layer.controls.ring_frequency = Some(parse_control(value)?),
            // Unknown SuperDirt controls are intentionally ignored so a file can
            // migrate incrementally while its audio implementation is added.
            _ => {}
        }
    }
    Ok(layer)
}

fn parse_primary(primary: &str) -> Result<(RhythmSource, Sequence, f64), String> {
    let primary = primary
        .trim()
        .trim_matches(|character| character == '(' || character == ')');
    if let Some(collection) = parse_collection_primary(primary)? {
        return Ok(collection);
    }
    let source = if primary.starts_with("sound ") || primary.starts_with("s ") {
        RhythmSource::Sound
    } else if primary.starts_with("n ") || primary.starts_with("note ") {
        RhythmSource::Note
    } else {
        return Err(format!(
            "Unsupported pattern expression `{primary}`; expected `sound \"...\"` or `n \"...\"`"
        ));
    };
    let sequence = match parse_generated_sequence(primary)? {
        Some(sequence) => sequence,
        None => parse_quoted_sequence(primary)?,
    };
    let scale = parse_rate_scale(primary)?;
    Ok((source, sequence, scale))
}

fn parse_collection_primary(
    primary: &str,
) -> Result<Option<(RhythmSource, Sequence, f64)>, String> {
    let Some(name) = ["slowcat", "cat", "fastcat", "randcat", "timeCat"]
        .into_iter()
        .find(|name| primary.starts_with(name))
    else {
        return Ok(None);
    };
    let open = primary
        .find('[')
        .ok_or_else(|| format!("`{name}` needs a bracketed pattern list"))?;
    let close = primary
        .rfind(']')
        .ok_or_else(|| format!("`{name}` is missing its closing `]`"))?;
    let entries = split_top_level(&primary[open + 1..close], ',');
    if entries.is_empty() {
        return Err(format!("`{name}` cannot be empty"));
    }

    let mut source = None;
    let mut nodes = Vec::new();
    for entry in entries {
        let (weight, pattern) = if name == "timeCat" {
            let entry = entry.trim().trim_matches(['(', ')']);
            let pair = split_top_level(entry, ',');
            if pair.len() != 2 {
                return Err("`timeCat` entries must be `(weight, pattern)` pairs".to_owned());
            }
            (parse_ratio(pair[0].trim(), entry)?, pair[1].trim())
        } else {
            (1.0, entry.trim())
        };
        let (entry_source, sequence, _) = parse_primary(pattern)?;
        if source.is_some_and(|source| source != entry_source) {
            return Err(format!(
                "`{name}` cannot mix sound patterns with note patterns"
            ));
        }
        source = Some(entry_source);
        nodes.push(WeightedNode {
            node: sequence.root,
            weight: weight.max(0.001),
        });
    }

    let root = match name {
        "cat" | "slowcat" => {
            MiniNode::Alternate(nodes.into_iter().map(|entry| entry.node).collect())
        }
        "randcat" => MiniNode::Choice(nodes.into_iter().map(|entry| entry.node).collect()),
        "fastcat" | "timeCat" => MiniNode::Sequence(nodes),
        _ => unreachable!(),
    };
    Ok(Some((
        source.expect("a non-empty collection has a source"),
        Sequence { root },
        parse_rate_scale(&primary[..open])?,
    )))
}

fn parse_generated_sequence(primary: &str) -> Result<Option<Sequence>, String> {
    for name in ["randrun", "scan", "run"] {
        if contains_function(primary, name) {
            let count = number_after(primary, name)?.unwrap_or(0.0).round().max(0.0) as u32;
            if count == 0 {
                return Err(format!("`{name}` needs a positive length"));
            }
            let root = match name {
                "randrun" => MiniNode::RandomRun(count),
                "scan" => MiniNode::Alternate(
                    (1..=count)
                        .map(|length| {
                            MiniNode::Sequence(
                                (0..length)
                                    .map(|value| WeightedNode {
                                        node: MiniNode::Atom(value.to_string()),
                                        weight: 1.0,
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                ),
                "run" => MiniNode::Sequence(
                    (0..count)
                        .map(|value| WeightedNode {
                            node: MiniNode::Atom(value.to_string()),
                            weight: 1.0,
                        })
                        .collect(),
                ),
                _ => unreachable!(),
            };
            return Ok(Some(Sequence { root }));
        }
    }

    for name in [
        "fromMaybes",
        "fromList",
        "listToPat",
        "cycleChoose",
        "choose",
    ] {
        if !contains_function(primary, name) {
            continue;
        }
        let open = primary
            .find('[')
            .ok_or_else(|| format!("`{name}` needs a bracketed value list"))?;
        let close = primary
            .rfind(']')
            .ok_or_else(|| format!("`{name}` is missing its closing `]`"))?;
        let mut nodes = split_top_level(&primary[open + 1..close], ',')
            .into_iter()
            .map(|value| {
                let value = value.trim();
                if matches!(value, "Nothing" | "~") {
                    MiniNode::Rest
                } else {
                    MiniNode::Atom(value.trim_start_matches("Just ").to_owned())
                }
            })
            .collect::<Vec<_>>();
        if nodes.is_empty() {
            return Err(format!("`{name}` cannot be empty"));
        }
        let root = if matches!(name, "choose" | "cycleChoose") {
            MiniNode::Choice(nodes)
        } else {
            MiniNode::Sequence(
                nodes
                    .drain(..)
                    .map(|node| WeightedNode { node, weight: 1.0 })
                    .collect(),
            )
        };
        return Ok(Some(Sequence { root }));
    }
    Ok(None)
}

fn parse_rate_scale(source: &str) -> Result<f64, String> {
    let normalized = source.replace(['(', ')', '$'], " ");
    let words: Vec<_> = normalized.split_whitespace().collect();
    let mut scale = 1.0;
    for pair in words.windows(2) {
        if pair[0] == "slow" {
            scale *= pair[1]
                .parse::<f64>()
                .map_err(|_| format!("Invalid slow factor `{}`", pair[1]))?;
        } else if pair[0] == "fast" {
            let factor = pair[1]
                .parse::<f64>()
                .map_err(|_| format!("Invalid fast factor `{}`", pair[1]))?;
            if factor <= 0.0 {
                return Err("`fast` needs a positive factor".to_owned());
            }
            scale /= factor;
        }
    }
    if !scale.is_finite() || scale <= 0.0 {
        return Err("Pattern rate must be positive and finite".to_owned());
    }
    Ok(scale)
}

fn parse_top_level_rate_scale(source: &str) -> Result<f64, String> {
    let mut top_level = String::with_capacity(source.len());
    let mut depth = 0_u32;
    for character in source.chars() {
        match character {
            '(' => {
                depth += 1;
                top_level.push(' ');
            }
            ')' => {
                depth = depth.saturating_sub(1);
                top_level.push(' ');
            }
            _ if depth == 0 => top_level.push(character),
            _ => top_level.push(' '),
        }
    }
    let mut scale = parse_rate_scale(&top_level)?;
    let condition = parse_transform_condition(source)?;
    if condition.is_always() && contains_function(&top_level, "hurry") {
        let factor = number_after(&top_level, "hurry")?.unwrap_or(1.0);
        if factor <= 0.0 {
            return Err("`hurry` needs a positive factor".to_owned());
        }
        scale /= factor;
    }
    Ok(scale)
}

fn parse_quoted_sequence(source: &str) -> Result<Sequence, String> {
    let quoted = quoted_strings(source)?;
    Sequence::parse(
        quoted
            .last()
            .ok_or_else(|| format!("Expected a quoted mini-notation pattern in `{source}`"))?,
    )
}

fn parse_sequence_value(source: &str) -> Result<Sequence, String> {
    if source.contains('"') {
        parse_quoted_sequence(source)
    } else {
        Sequence::parse(source.trim().trim_matches(['(', ')']))
    }
}

fn quoted_strings(source: &str) -> Result<Vec<&str>, String> {
    let mut output = Vec::new();
    let mut start = None;
    for (index, character) in source.char_indices() {
        if character != '"' {
            continue;
        }
        if let Some(open) = start.take() {
            output.push(&source[open..index]);
        } else {
            start = Some(index + 1);
        }
    }
    if start.is_some() {
        return Err(format!("Missing closing quote in `{source}`"));
    }
    Ok(output)
}

fn parse_scale_name(source: &str) -> Option<String> {
    source
        .contains("scale")
        .then(|| {
            quoted_strings(source)
                .ok()?
                .first()
                .map(|value| (*value).to_owned())
        })
        .flatten()
}

fn parse_roll(source: &str) -> Option<f64> {
    if let Some(index) = source.find("rolledBy") {
        return source[index + "rolledBy".len()..]
            .split_whitespace()
            .next()
            .and_then(|value| parse_ratio(value.trim_matches(['(', ')']), source).ok());
    }
    (source.contains("arpeggiate") || source.contains(" arp "))
        .then_some(1.0)
        .or_else(|| source.contains("rolled").then_some(0.25))
}

fn parse_transforms(source: &str) -> Result<Vec<Transform>, String> {
    let condition = parse_transform_condition(source)?;
    let mut output = Vec::new();

    if contains_function(source, "palindrome") {
        output.push(Transform::Palindrome);
    }
    let is_jux = contains_function(source, "jux") || contains_function(source, "juxBy");
    if contains_function(source, "rev") && !is_jux {
        output.push(Transform::Reverse(condition));
    }

    if contains_function(source, "unDegradeBy") {
        let keep = number_after(source, "unDegradeBy")?.unwrap_or(0.5) as f32;
        output.push(Transform::Degrade {
            keep: keep.clamp(0.0, 1.0),
            condition,
        });
    } else if contains_function(source, "degradeBy") {
        let drop = number_after(source, "degradeBy")?.unwrap_or(0.5) as f32;
        output.push(Transform::Degrade {
            keep: 1.0 - drop.clamp(0.0, 1.0),
            condition,
        });
    } else if contains_function(source, "degrade") {
        output.push(Transform::Degrade {
            keep: 0.5,
            condition,
        });
    }

    if contains_function(source, "ply") {
        output.push(Transform::Ply {
            count: number_after(source, "ply")?.unwrap_or(1.0).round().max(1.0) as u32,
            condition,
        });
    }
    if contains_function(source, "pressBy") {
        output.push(Transform::Press {
            amount: number_after(source, "pressBy")?.unwrap_or(0.5),
            condition,
        });
    } else if contains_function(source, "press") {
        output.push(Transform::Press {
            amount: 0.5,
            condition,
        });
    }
    if contains_function(source, "rotL") {
        output.push(Transform::Rotate {
            amount: -number_after(source, "rotL")?.unwrap_or(0.0),
            condition,
        });
    }
    if contains_function(source, "rotR") {
        output.push(Transform::Rotate {
            amount: number_after(source, "rotR")?.unwrap_or(0.0),
            condition,
        });
    }
    if contains_function(source, "iterBack") {
        output.push(Transform::Iter {
            count: number_after(source, "iterBack")?
                .unwrap_or(1.0)
                .round()
                .max(1.0) as u32,
            backwards: true,
        });
    } else if contains_function(source, "iter") {
        output.push(Transform::Iter {
            count: number_after(source, "iter")?
                .unwrap_or(1.0)
                .round()
                .max(1.0) as u32,
            backwards: false,
        });
    }

    if contains_function(source, "swingBy") {
        let values = numbers_after(source, "swingBy", 2)?;
        output.push(Transform::Swing {
            amount: values.first().copied().unwrap_or(1.0 / 3.0),
            subdivisions: values.get(1).copied().unwrap_or(4.0).round().max(1.0) as u32,
            condition,
        });
    } else if contains_function(source, "swing") {
        output.push(Transform::Swing {
            amount: 1.0 / 3.0,
            subdivisions: number_after(source, "swing")?
                .unwrap_or(4.0)
                .round()
                .max(1.0) as u32,
            condition,
        });
    }

    if let Some((start, end)) = arc_after(source, "compress")? {
        output.push(Transform::Compress {
            start,
            end,
            condition,
        });
    }
    if let Some((start, end)) = arc_after(source, "zoom")? {
        output.push(Transform::Zoom {
            start,
            end,
            condition,
        });
    }
    if contains_function(source, "trunc") {
        output.push(Transform::Truncate {
            length: number_after(source, "trunc")?
                .unwrap_or(1.0)
                .clamp(0.001, 1.0),
            condition,
        });
    }
    if contains_function(source, "linger") {
        let factor = number_after(source, "linger")?.unwrap_or(1.0).max(1.0);
        output.push(Transform::Zoom {
            start: 0.0,
            end: 1.0 / factor,
            condition,
        });
    }
    if contains_function(source, "shuffle") || contains_function(source, "scramble") {
        output.push(Transform::Shuffle { condition });
    }
    if contains_function(source, "fastGap") || contains_function(source, "densityGap") {
        let name = if contains_function(source, "fastGap") {
            "fastGap"
        } else {
            "densityGap"
        };
        output.push(Transform::FastGap {
            factor: number_after(source, name)?.unwrap_or(1.0).max(0.001),
            condition,
        });
    }

    let conditional_rate = !condition.is_always();
    if conditional_rate && contains_function(source, "fast") {
        output.push(Transform::Fast {
            factor: number_after(source, "fast")?
                .unwrap_or(1.0)
                .round()
                .max(1.0) as u32,
            condition,
        });
    }
    if contains_function(source, "hurry") {
        let factor = number_after(source, "hurry")?.unwrap_or(1.0) as f32;
        if conditional_rate {
            output.push(Transform::Fast {
                factor: factor.round().max(1.0) as u32,
                condition,
            });
        }
        output.push(Transform::Hurry { factor, condition });
    }

    if is_jux {
        let amount = if contains_function(source, "juxBy") {
            number_after(source, "juxBy")?.unwrap_or(1.0) as f32
        } else {
            1.0
        };
        output.push(Transform::Jux {
            amount,
            reverse: contains_function(source, "rev"),
            condition,
        });
    }
    if contains_function(source, "spin") {
        output.push(Transform::Spin {
            count: number_after(source, "spin")?
                .unwrap_or(1.0)
                .round()
                .max(1.0) as u32,
            condition,
        });
    }
    if contains_function(source, "echo") || contains_function(source, "stut") {
        let name = if contains_function(source, "echo") {
            "echo"
        } else {
            "stut"
        };
        let values = numbers_after(source, name, 3)?;
        output.push(Transform::Echo {
            count: values.first().copied().unwrap_or(2.0).round().max(1.0) as u32,
            offset: values.get(1).copied().unwrap_or(0.125),
            decay: values.get(2).copied().unwrap_or(0.5) as f32,
            condition,
        });
    }
    if contains_function(source, "off") {
        let offset = number_after(source, "off")?.unwrap_or(0.125);
        let note_offset = source
            .split('+')
            .nth(1)
            .and_then(|value| {
                value
                    .trim_start()
                    .split(|character: char| !character.is_ascii_digit() && character != '.')
                    .next()
            })
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.0);
        output.push(Transform::Off {
            offset,
            note_offset,
            condition,
        });
    }
    if contains_function(source, "ghost") {
        output.push(Transform::Ghost(condition));
    }
    for (name, random) in [
        ("randslice", true),
        ("striateBy", false),
        ("striate", false),
        ("splice", false),
        ("slice", false),
        ("chop", false),
    ] {
        if contains_function(source, name) {
            output.push(Transform::Slice {
                count: number_after(source, name)?.unwrap_or(1.0).round().max(1.0) as u32,
                random,
                condition,
            });
            break;
        }
    }
    if contains_function(source, "loopAt") {
        output.push(Transform::LoopAt {
            cycles: number_after(source, "loopAt")?.unwrap_or(1.0) as f32,
            condition,
        });
    }
    for (name, structure) in [("substruct", true), ("struct", true), ("mask", false)] {
        if contains_function(source, name) {
            let quoted = quoted_strings(source)?;
            let mask = quoted
                .first()
                .ok_or_else(|| format!("`{name}` needs a quoted boolean pattern"))?;
            output.push(Transform::Mask {
                pattern: Sequence::parse(mask)?,
                structure,
                condition,
            });
            break;
        }
    }
    if contains_function(source, "loopFirst") {
        output.push(Transform::LoopFirst);
    }
    Ok(output)
}

fn parse_transform_condition(source: &str) -> Result<TransformCondition, String> {
    if contains_function(source, "every'") {
        let values = numbers_after(source, "every'", 2)?;
        return Ok(TransformCondition::Every {
            period: values.first().copied().unwrap_or(1.0).round().max(1.0) as u64,
            offset: values.get(1).copied().unwrap_or(0.0).round().max(0.0) as u64,
        });
    }
    if contains_function(source, "every") {
        return Ok(TransformCondition::Every {
            period: number_after(source, "every")?
                .unwrap_or(1.0)
                .round()
                .max(1.0) as u64,
            offset: 0,
        });
    }
    for (name, chance, per_event) in [
        ("someCyclesBy", None, false),
        ("sometimesBy", None, true),
        ("someCycles", Some(0.5), false),
        ("almostAlways", Some(0.9), true),
        ("often", Some(0.75), true),
        ("sometimes", Some(0.5), true),
        ("rarely", Some(0.25), true),
        ("almostNever", Some(0.1), true),
        ("always", Some(1.0), true),
        ("never", Some(0.0), true),
    ] {
        if contains_function(source, name) {
            let chance = match chance {
                Some(chance) => chance,
                None => number_after(source, name)?.unwrap_or(0.5),
            };
            return Ok(TransformCondition::Probability {
                chance: chance.clamp(0.0, 1.0) as f32,
                per_event,
            });
        }
    }
    Ok(TransformCondition::Always)
}

fn contains_function(source: &str, name: &str) -> bool {
    function_tokens(source).contains(&name)
}

fn function_tokens(source: &str) -> Vec<&str> {
    source
        .split(|character: char| {
            character.is_whitespace()
                || matches!(character, '(' | ')' | '[' | ']' | '{' | '}' | ',' | '$')
        })
        .filter(|token| !token.is_empty())
        .collect()
}

fn number_after(source: &str, name: &str) -> Result<Option<f64>, String> {
    Ok(numbers_after(source, name, 1)?.first().copied())
}

fn numbers_after(source: &str, name: &str, count: usize) -> Result<Vec<f64>, String> {
    let tokens = function_tokens(source);
    let Some(index) = tokens.iter().position(|token| *token == name) else {
        return Ok(Vec::new());
    };
    let mut values = Vec::new();
    for token in &tokens[index + 1..] {
        if values.len() == count || token.starts_with('"') {
            break;
        }
        let token = token.trim_matches(|character| character == '"' || character == ';');
        if let Ok(value) = parse_ratio(token, source) {
            values.push(value);
        } else if !values.is_empty() {
            break;
        }
    }
    Ok(values)
}

fn arc_after(source: &str, name: &str) -> Result<Option<(f64, f64)>, String> {
    let Some(function) = source.find(name) else {
        return Ok(None);
    };
    let rest = &source[function + name.len()..];
    let Some(open) = rest.find('(') else {
        return Ok(None);
    };
    let close = matching_close(&rest[open..], '(', ')')? + open;
    let values = rest[open + 1..close]
        .split(',')
        .map(|value| parse_ratio(value.trim(), source))
        .collect::<Result<Vec<_>, _>>()?;
    if values.len() != 2 || values[1] <= values[0] {
        return Err(format!("`{name}` needs an increasing `(start, end)` arc"));
    }
    Ok(Some((values[0], values[1])))
}

fn parse_control(source: &str) -> Result<ControlValue, String> {
    for (name, segmented) in [("quantise", false), ("discretise", true), ("segment", true)] {
        if contains_function(source, name) {
            let amount = number_after(source, name)?.unwrap_or(1.0);
            let inner = source
                .split_once('$')
                .map(|(_, inner)| inner.trim())
                .ok_or_else(|| format!("`{name}` needs a control signal after `$`"))?;
            let value = Box::new(parse_control(inner)?);
            return Ok(if segmented {
                ControlValue::Segmented {
                    value,
                    steps: amount.round().max(1.0) as u32,
                }
            } else {
                ControlValue::Quantized {
                    value,
                    amount: amount as f32,
                }
            });
        }
    }
    if source.contains('"') {
        return Ok(ControlValue::Pattern(parse_quoted_sequence(source)?));
    }
    let normalized = source.replace(['(', ')', '$'], " ");
    let words: Vec<_> = normalized.split_whitespace().collect();
    if matches!(words.first(), Some(&"choose") | Some(&"cycleChoose")) {
        let open = source
            .find('[')
            .ok_or_else(|| format!("Random choice needs a list in `{source}`"))?;
        let close = source
            .rfind(']')
            .ok_or_else(|| format!("Random choice is missing `]` in `{source}`"))?;
        let values = source[open + 1..close]
            .split(',')
            .map(parse_number)
            .collect::<Result<Vec<_>, _>>()?;
        if values.is_empty() {
            return Err("Random choice cannot be empty".to_owned());
        }
        return Ok(ControlValue::Choice {
            values,
            per_cycle: words.first() == Some(&"cycleChoose"),
        });
    }

    let waveform = words
        .iter()
        .find_map(|word| waveform_named(word))
        .unwrap_or(Waveform::Sine);
    let mut cycles = 1.0;
    for pair in words.windows(2) {
        if pair[0] == "slow" {
            cycles *= pair[1]
                .parse::<f64>()
                .map_err(|_| format!("Invalid modulation rate `{}`", pair[1]))?;
        } else if pair[0] == "fast" {
            cycles /= pair[1]
                .parse::<f64>()
                .map_err(|_| format!("Invalid modulation rate `{}`", pair[1]))?;
        }
    }

    if matches!(words.first(), Some(&"range") | Some(&"rangex")) && words.len() >= 3 {
        let minimum = parse_number(words[1])?;
        let maximum = parse_number(words[2])?;
        return Ok(ControlValue::Signal {
            waveform,
            minimum,
            maximum,
            cycles,
            integer: None,
            exponential: words.first() == Some(&"rangex"),
        });
    }

    if words.first() == Some(&"irand") && words.len() >= 2 {
        return Ok(ControlValue::Signal {
            waveform: Waveform::Random,
            minimum: 0.0,
            maximum: 1.0,
            cycles,
            integer: Some(parse_number(words[1])?.max(1.0) as u32),
            exponential: false,
        });
    }
    if words.iter().any(|word| waveform_named(word).is_some()) {
        return Ok(ControlValue::Signal {
            waveform,
            minimum: 0.0,
            maximum: 1.0,
            cycles,
            integer: None,
            exponential: false,
        });
    }
    Ok(ControlValue::Constant(parse_number(source)?))
}

fn waveform_named(source: &str) -> Option<Waveform> {
    match source.trim_matches(|value: char| !value.is_ascii_alphabetic()) {
        "sine" | "smooth" => Some(Waveform::Sine),
        "cosine" => Some(Waveform::Cosine),
        "square" => Some(Waveform::Square),
        "tri" | "triangle" => Some(Waveform::Triangle),
        "saw" => Some(Waveform::Saw),
        "isaw" => Some(Waveform::InverseSaw),
        "rand" => Some(Waveform::Random),
        "perlin" | "perlinWith" | "perlin2" | "perlin2With" => Some(Waveform::Perlin),
        _ => None,
    }
}

fn parse_number(source: &str) -> Result<f32, String> {
    source
        .trim()
        .trim_matches(|character| character == '(' || character == ')')
        .parse::<f32>()
        .map_err(|_| format!("Expected a number, found `{source}`"))
}

fn parse_note(source: &str) -> Option<f32> {
    if let Ok(number) = source.parse::<f32>() {
        return Some(number);
    }
    let lower = source.to_ascii_lowercase();
    let mut chars = lower.chars();
    let base = match chars.next()? {
        'c' => 0.0,
        'd' => 2.0,
        'e' => 4.0,
        'f' => 5.0,
        'g' => 7.0,
        'a' => 9.0,
        'b' => 11.0,
        _ => return None,
    };
    let remainder: String = chars.collect();
    let (accidental, octave) = if let Some(rest) = remainder.strip_prefix(['s', '#']) {
        (1.0, rest)
    } else if let Some(rest) = remainder.strip_prefix(['f', 'b']) {
        (-1.0, rest)
    } else {
        (0.0, remainder.as_str())
    };
    let octave_offset = if octave.is_empty() {
        0.0
    } else {
        (octave.parse::<i32>().ok()? - 5) as f32 * 12.0
    };
    Some(base + accidental + octave_offset)
}

fn parse_note_or_chord(source: &str) -> Option<Vec<f32>> {
    let mut parts = source.split('\'');
    let root = parse_note(parts.next()?)?;
    let Some(chord_name) = parts.next() else {
        return Some(vec![root]);
    };
    let mut intervals = chord_intervals(chord_name)?.to_vec();
    for modifier in parts {
        if modifier == "o" {
            if let Some(root) = intervals.get_mut(0) {
                *root -= 12.0;
            }
            if let Some(fifth) = intervals.get_mut(2) {
                *fifth -= 12.0;
            }
        } else if let Some(drop) = modifier
            .strip_prefix('d')
            .and_then(|value| value.parse().ok())
        {
            let drop: usize = drop;
            if (1..=intervals.len()).contains(&drop) {
                let index = intervals.len() - drop;
                intervals[index] -= 12.0;
            }
        } else if let Some(inversion) = modifier
            .strip_prefix('i')
            .and_then(|value| value.parse::<usize>().ok())
            .or_else(|| {
                (!modifier.is_empty() && modifier.chars().all(|value| value == 'i'))
                    .then_some(modifier.len())
            })
        {
            intervals.sort_by(f32::total_cmp);
            for note in intervals.iter_mut().take(inversion) {
                *note += 12.0;
            }
        } else if let Ok(count) = modifier.parse::<usize>() {
            if count < intervals.len() {
                intervals.truncate(count);
            } else {
                let base = intervals.clone();
                while intervals.len() < count {
                    let index = intervals.len();
                    intervals.push(base[index % base.len()] + 12.0 * (index / base.len()) as f32);
                }
            }
        }
    }
    intervals.sort_by(f32::total_cmp);
    Some(
        intervals
            .into_iter()
            .map(|interval| root + interval)
            .collect(),
    )
}

fn chord_intervals(name: &str) -> Option<&'static [f32]> {
    if name == "M" {
        return Some(&[0.0, 4.0, 7.0]);
    }
    let name = name.to_ascii_lowercase();
    Some(match name.as_str() {
        "major" | "maj" => &[0.0, 4.0, 7.0],
        "m" | "minor" | "min" => &[0.0, 3.0, 7.0],
        "aug" | "plus" | "sharp5" => &[0.0, 4.0, 8.0],
        "six" | "6" => &[0.0, 4.0, 7.0, 9.0],
        "sixnine" | "six9" | "sixby9" | "6by9" => &[0.0, 4.0, 7.0, 9.0, 14.0],
        "major7" | "maj7" | "m7major" => &[0.0, 4.0, 7.0, 11.0],
        "major9" | "maj9" => &[0.0, 4.0, 7.0, 11.0, 14.0],
        "add9" => &[0.0, 4.0, 7.0, 14.0],
        "major11" | "maj11" => &[0.0, 4.0, 7.0, 11.0, 14.0, 17.0],
        "add11" => &[0.0, 4.0, 7.0, 17.0],
        "major13" | "maj13" => &[0.0, 4.0, 7.0, 11.0, 14.0, 21.0],
        "add13" => &[0.0, 4.0, 7.0, 21.0],
        "dom7" => &[0.0, 4.0, 7.0, 10.0],
        "dom9" => &[0.0, 4.0, 7.0, 14.0],
        "dom11" => &[0.0, 4.0, 7.0, 17.0],
        "dom13" => &[0.0, 4.0, 7.0, 21.0],
        "sevenflat5" | "7f5" => &[0.0, 4.0, 6.0, 10.0],
        "sevensharp5" | "7s5" => &[0.0, 4.0, 8.0, 10.0],
        "sevenflat9" | "7f9" => &[0.0, 4.0, 7.0, 10.0, 13.0],
        "nine" => &[0.0, 4.0, 7.0, 10.0, 14.0],
        "eleven" | "11" => &[0.0, 4.0, 7.0, 10.0, 14.0, 17.0],
        "thirteen" | "13" => &[0.0, 4.0, 7.0, 10.0, 14.0, 17.0, 21.0],
        "diminished" | "dim" => &[0.0, 3.0, 6.0],
        "minorsharp5" | "msharp5" | "ms5" => &[0.0, 3.0, 8.0],
        "minor6" | "min6" | "m6" => &[0.0, 3.0, 7.0, 9.0],
        "minorsixnine" | "minor69" | "min69" | "minsixnine" | "m69" | "msixnine" | "m6by9" => {
            &[0.0, 3.0, 9.0, 7.0, 14.0]
        }
        "minor7flat5" | "minor7f5" | "min7flat5" | "min7f5" | "m7flat5" | "m7f5" => {
            &[0.0, 3.0, 6.0, 10.0]
        }
        "minor7" | "min7" | "m7" => &[0.0, 3.0, 7.0, 10.0],
        "minor7sharp5" | "minor7s5" | "min7sharp5" | "min7s5" | "m7sharp5" | "m7s5" => {
            &[0.0, 3.0, 8.0, 10.0]
        }
        "minor7flat9" | "minor7f9" | "min7flat9" | "min7f9" | "m7flat9" | "m7f9" => {
            &[0.0, 3.0, 7.0, 10.0, 13.0]
        }
        "minor7sharp9" | "minor7s9" | "min7sharp9" | "min7s9" | "m7sharp9" | "m7s9" => {
            &[0.0, 3.0, 7.0, 10.0, 15.0]
        }
        "diminished7" | "dim7" => &[0.0, 3.0, 6.0, 9.0],
        "minor9" | "min9" | "m9" => &[0.0, 3.0, 7.0, 10.0, 14.0],
        "minor11" | "min11" | "m11" => &[0.0, 3.0, 7.0, 10.0, 14.0, 17.0],
        "minor13" | "min13" | "m13" => &[0.0, 3.0, 7.0, 10.0, 14.0, 17.0, 21.0],
        "minormajor7" | "minmaj7" | "mmaj7" => &[0.0, 3.0, 7.0, 11.0],
        "one" | "1" => &[0.0],
        "five" | "5" => &[0.0, 7.0],
        "sus2" => &[0.0, 2.0, 7.0],
        "sus4" => &[0.0, 5.0, 7.0],
        "sevensus2" | "7sus2" => &[0.0, 2.0, 7.0, 10.0],
        "sevensus4" | "7sus4" => &[0.0, 5.0, 7.0, 10.0],
        "ninesus4" | "9sus4" => &[0.0, 5.0, 7.0, 10.0, 14.0],
        "sevenflat10" | "7f10" => &[0.0, 4.0, 7.0, 10.0, 15.0],
        "ninesharp5" | "9sharp5" | "9s5" => &[0.0, 1.0, 13.0],
        "minor9sharp5" | "minor9s5" | "min9sharp5" | "min9s5" | "m9sharp5" | "m9s5" => {
            &[0.0, 1.0, 14.0]
        }
        "sevensharp5flat9" | "7s5f9" => &[0.0, 4.0, 8.0, 10.0, 13.0],
        "minor7sharp5flat9" | "m7sharp5flat9" => &[0.0, 3.0, 8.0, 10.0, 13.0],
        "elevensharp" | "11s" => &[0.0, 4.0, 7.0, 10.0, 14.0, 18.0],
        "minor11sharp" | "m11sharp" | "m11s" => &[0.0, 3.0, 7.0, 10.0, 14.0, 18.0],
        _ => return None,
    })
}

fn scale_degree(name: &str, degree: f32) -> Option<f32> {
    let scale = scale_intervals(name)?;
    let whole = degree.floor() as i32;
    let fraction = degree - degree.floor();
    let length = scale.len() as i32;
    let octave = whole.div_euclid(length);
    let index = whole.rem_euclid(length) as usize;
    Some(scale[index] + octave as f32 * 12.0 + fraction)
}

fn scale_intervals(name: &str) -> Option<&'static [f32]> {
    let name = name.to_ascii_lowercase();
    Some(match name.as_str() {
        "minpent" => &[0.0, 3.0, 5.0, 7.0, 10.0],
        "majpent" | "gong" => &[0.0, 2.0, 4.0, 7.0, 9.0],
        "ritusen" | "zhi" => &[0.0, 2.0, 5.0, 7.0, 9.0],
        "egyptian" | "shang" => &[0.0, 2.0, 5.0, 7.0, 10.0],
        "kumai" => &[0.0, 2.0, 3.0, 7.0, 9.0],
        "hirajoshi" => &[0.0, 2.0, 3.0, 7.0, 8.0],
        "iwato" => &[0.0, 1.0, 5.0, 6.0, 10.0],
        "chinese" => &[0.0, 4.0, 6.0, 7.0, 11.0],
        "indian" => &[0.0, 4.0, 5.0, 7.0, 10.0],
        "pelog" => &[0.0, 1.0, 3.0, 7.0, 8.0],
        "prometheus" => &[0.0, 2.0, 4.0, 6.0, 11.0],
        "scriabin" => &[0.0, 1.0, 4.0, 7.0, 9.0],
        "jiao" => &[0.0, 3.0, 5.0, 8.0, 10.0],
        "yu" => &[0.0, 3.0, 5.0, 7.0, 10.0],
        "whole" | "wholetone" | "messiaen1" => &[0.0, 2.0, 4.0, 6.0, 8.0, 10.0],
        "augmented" => &[0.0, 3.0, 4.0, 7.0, 8.0, 11.0],
        "augmented2" => &[0.0, 1.0, 4.0, 5.0, 8.0, 9.0],
        "hexmajor7" => &[0.0, 2.0, 4.0, 7.0, 9.0, 11.0],
        "hexdorian" => &[0.0, 2.0, 3.0, 5.0, 7.0, 10.0],
        "hexphrygian" => &[0.0, 1.0, 3.0, 5.0, 8.0, 10.0],
        "hexsus" => &[0.0, 2.0, 5.0, 7.0, 9.0, 10.0],
        "hexmajor6" => &[0.0, 2.0, 4.0, 5.0, 7.0, 9.0],
        "hexaeolian" => &[0.0, 3.0, 5.0, 7.0, 8.0, 10.0],
        "major" | "ionian" => &[0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0],
        "dorian" => &[0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0],
        "phrygian" => &[0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0],
        "lydian" => &[0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0],
        "mixolydian" => &[0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0],
        "aeolian" | "minor" | "melodicminordesc" => &[0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0],
        "locrian" => &[0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 10.0],
        "harmonicminor" => &[0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0],
        "harmonicmajor" => &[0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 11.0],
        "melodicminor" => &[0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0],
        "melodicmajor" | "bartok" | "hindu" => &[0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0],
        "todi" => &[0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 11.0],
        "purvi" => &[0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0],
        "marva" => &[0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0],
        "bhairav" => &[0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0],
        "ahirbhairav" => &[0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0],
        "superlocrian" => &[0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 10.0],
        "romanianminor" => &[0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0],
        "hungarianminor" => &[0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0],
        "neapolitanminor" => &[0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 11.0],
        "enigmatic" => &[0.0, 1.0, 4.0, 6.0, 8.0, 10.0, 11.0],
        "spanish" => &[0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0],
        "leadingwhole" => &[0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 11.0],
        "lydianminor" => &[0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0],
        "neapolitanmajor" => &[0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0],
        "locrianmajor" => &[0.0, 2.0, 4.0, 5.0, 6.0, 8.0, 10.0],
        "diminished" | "octatonic" | "messiaen2" => &[0.0, 1.0, 3.0, 4.0, 6.0, 7.0, 9.0, 10.0],
        "diminished2" | "octatonic2" => &[0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 9.0, 11.0],
        "messiaen3" => &[0.0, 2.0, 3.0, 4.0, 6.0, 7.0, 8.0, 10.0, 11.0],
        "messiaen4" => &[0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 8.0, 11.0],
        "messiaen5" => &[0.0, 1.0, 5.0, 6.0, 7.0, 11.0],
        "messiaen6" => &[0.0, 2.0, 4.0, 5.0, 6.0, 8.0, 10.0, 11.0],
        "messiaen7" => &[0.0, 1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 8.0, 9.0, 11.0],
        "chromatic" => &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
        "bayati" => &[0.0, 1.5, 3.0, 5.0, 7.0, 8.0, 10.0],
        "hijaz" => &[0.0, 1.0, 4.0, 5.0, 7.0, 8.5, 10.0],
        "sikah" => &[0.0, 1.5, 3.5, 5.5, 7.0, 8.5, 10.5],
        "rast" => &[0.0, 2.0, 3.5, 5.0, 7.0, 9.0, 10.5],
        "saba" => &[0.0, 1.5, 3.0, 4.0, 6.0, 8.0, 10.0],
        "iraq" => &[0.0, 1.5, 3.5, 5.0, 6.5, 8.5, 10.5],
        _ => return None,
    })
}

fn split_statements(source: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut square_depth = 0_i32;
    let mut paren_depth = 0_i32;
    let mut quoted = false;

    for raw_line in source.lines() {
        let line = raw_line.split("--").next().unwrap_or_default().trim();
        if line.is_empty() || line == "do {" || line == "do" || line == "{" || line == "}" {
            continue;
        }
        if is_statement_start(line) && square_depth == 0 && paren_depth == 0 && !current.is_empty()
        {
            statements.push(current.trim().to_owned());
            current.clear();
        }
        for character in line.chars() {
            if character == '"' {
                quoted = !quoted;
            }
            if !quoted {
                match character {
                    '[' => square_depth += 1,
                    ']' => square_depth -= 1,
                    '(' => paren_depth += 1,
                    ')' => paren_depth -= 1,
                    ';' if square_depth == 0 && paren_depth == 0 => {
                        if !current.trim().is_empty() {
                            statements.push(current.trim().to_owned());
                            current.clear();
                        }
                        continue;
                    }
                    _ => {}
                }
            }
            current.push(character);
        }
        current.push(' ');
    }
    let current = current.trim().trim_end_matches('}').trim();
    if !current.is_empty() {
        statements.push(current.to_owned());
    }
    statements
}

fn is_statement_start(line: &str) -> bool {
    line == "hush"
        || line == "panic"
        || line.starts_with("once ")
        || line.starts_with("setcps ")
        || line.starts_with("setCps ")
        || line.starts_with("p ")
        || line.strip_prefix('d').is_some_and(|rest| {
            rest.chars()
                .next()
                .is_some_and(|value| value.is_ascii_digit())
        })
}

fn split_top_level(source: &str, delimiter: char) -> Vec<&str> {
    let mut output = Vec::new();
    let mut start = 0;
    let mut square_depth = 0_i32;
    let mut paren_depth = 0_i32;
    let mut angle_depth = 0_i32;
    let mut quoted = false;
    for (index, character) in source.char_indices() {
        if character == '"' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match character {
            '[' => square_depth += 1,
            ']' => square_depth -= 1,
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '<' => angle_depth += 1,
            '>' => angle_depth -= 1,
            value
                if value == delimiter
                    && square_depth == 0
                    && paren_depth == 0
                    && angle_depth == 0 =>
            {
                output.push(&source[start..index]);
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }
    output.push(&source[start..]);
    output
}

fn last_top_level_character(source: &str, target: char) -> Option<usize> {
    let mut result = None;
    let mut square_depth = 0_i32;
    let mut paren_depth = 0_i32;
    let mut quoted = false;
    for (index, character) in source.char_indices() {
        if character == '"' {
            quoted = !quoted;
        } else if !quoted {
            match character {
                '[' => square_depth += 1,
                ']' => square_depth -= 1,
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                value if value == target && square_depth == 0 && paren_depth == 0 => {
                    result = Some(index);
                }
                _ => {}
            }
        }
    }
    result
}

struct ParsedMini {
    node: MiniNode,
    weight: f64,
    replicate: usize,
}

fn parse_mini_node(source: &str) -> Result<MiniNode, String> {
    let source = source.trim();
    if source.is_empty() {
        return Err("A mini-notation pattern cannot be empty".to_owned());
    }

    let dotted = split_mini_dots(source);
    if dotted.len() > 1 {
        let mut groups = Vec::new();
        for group in dotted {
            groups.push(WeightedNode {
                node: parse_mini_node(group)?,
                weight: 1.0,
            });
        }
        return Ok(MiniNode::Sequence(groups));
    }

    let words = split_mini_whitespace(source);
    if words.len() > 1 || words.first().is_some_and(|word| *word == "_") {
        let mut nodes = Vec::<WeightedNode>::new();
        for word in words {
            if word == "_" {
                let Some(previous) = nodes.last_mut() else {
                    return Err("`_` needs a preceding mini-notation event".to_owned());
                };
                previous.weight += 1.0;
                continue;
            }
            let parsed = parse_mini_item(word)?;
            for _ in 0..parsed.replicate {
                nodes.push(WeightedNode {
                    node: parsed.node.clone(),
                    weight: parsed.weight,
                });
            }
        }
        if nodes.is_empty() {
            return Err("A mini-notation sequence cannot be empty".to_owned());
        }
        return Ok(if nodes.len() == 1 {
            nodes.remove(0).node
        } else {
            MiniNode::Sequence(nodes)
        });
    }

    let parsed = parse_mini_item(words.first().copied().unwrap_or(source))?;
    if parsed.replicate == 1 {
        Ok(parsed.node)
    } else {
        Ok(MiniNode::Sequence(
            (0..parsed.replicate)
                .map(|_| WeightedNode {
                    node: parsed.node.clone(),
                    weight: parsed.weight,
                })
                .collect(),
        ))
    }
}

fn parse_mini_item(source: &str) -> Result<ParsedMini, String> {
    let source = source.trim();
    let (mut node, suffix) = match source.chars().next() {
        Some('[') => {
            let close = matching_close(source, '[', ']')?;
            let body = &source[1..close];
            let stacks = split_mini(body, ',');
            let choices = split_mini(body, '|');
            let node = if stacks.len() > 1 {
                MiniNode::Stack(
                    stacks
                        .into_iter()
                        .map(parse_mini_node)
                        .collect::<Result<_, _>>()?,
                )
            } else if choices.len() > 1 {
                MiniNode::Choice(
                    choices
                        .into_iter()
                        .map(parse_mini_node)
                        .collect::<Result<_, _>>()?,
                )
            } else {
                parse_mini_node(body)?
            };
            (node, &source[close + 1..])
        }
        Some('<') => {
            let close = matching_close(source, '<', '>')?;
            let options = split_mini_whitespace(&source[1..close])
                .into_iter()
                .map(parse_mini_node)
                .collect::<Result<Vec<_>, _>>()?;
            if options.is_empty() {
                return Err("An alternation cannot be empty".to_owned());
            }
            (MiniNode::Alternate(options), &source[close + 1..])
        }
        Some('{') => {
            let close = matching_close(source, '{', '}')?;
            let parts = split_mini(&source[1..close], ',')
                .into_iter()
                .map(parse_mini_node)
                .collect::<Result<Vec<_>, _>>()?;
            if parts.is_empty() {
                return Err("A polymeter cannot be empty".to_owned());
            }
            (
                MiniNode::Polymeter { parts, steps: None },
                &source[close + 1..],
            )
        }
        Some(_) => {
            let suffix_start = source
                .char_indices()
                .find(|(_, value)| matches!(value, '*' | '!' | '/' | '@' | '?' | '('))
                .map_or(source.len(), |(index, _)| index);
            let atom = &source[..suffix_start];
            let node = if matches!(atom, "~" | "-") {
                MiniNode::Rest
            } else if atom.is_empty() {
                return Err(format!("Missing mini-notation value in `{source}`"));
            } else {
                MiniNode::Atom(atom.to_owned())
            };
            (node, &source[suffix_start..])
        }
        None => return Err("A mini-notation event cannot be empty".to_owned()),
    };

    let mut weight = 1.0;
    let mut replicate = 1_usize;
    let mut cursor = 0;
    while cursor < suffix.len() {
        let marker = suffix[cursor..]
            .chars()
            .next()
            .expect("cursor is in bounds");
        cursor += marker.len_utf8();
        if marker == '(' {
            let relative_close = suffix[cursor..]
                .find(')')
                .ok_or_else(|| format!("Missing `)` in mini-notation suffix `{suffix}`"))?;
            let values = suffix[cursor..cursor + relative_close]
                .split(',')
                .map(str::trim)
                .collect::<Vec<_>>();
            if !(2..=3).contains(&values.len()) {
                return Err(format!(
                    "Euclidean suffix needs 2 or 3 values in `{source}`"
                ));
            }
            let pulses = parse_u32(values[0], source)?;
            let steps = parse_u32(values[1], source)?.max(1);
            let offset = values
                .get(2)
                .map_or(Ok(0), |value| parse_u32(value, source))?;
            node = MiniNode::Euclid {
                node: Box::new(node),
                pulses: pulses.min(steps),
                steps,
                offset,
            };
            cursor += relative_close + 1;
            continue;
        }

        let value_end = suffix[cursor..]
            .char_indices()
            .find(|(_, value)| matches!(value, '*' | '!' | '/' | '@' | '?' | '('))
            .map_or(suffix.len(), |(index, _)| cursor + index);
        let raw_value = suffix[cursor..value_end].trim();
        cursor = value_end;
        match marker {
            '*' => {
                let count = parse_ratio(raw_value, source)?.round().max(1.0) as u32;
                node = MiniNode::Repeat(Box::new(node), count);
            }
            '!' => replicate = parse_ratio(raw_value, source)?.round().max(1.0) as usize,
            '/' => {
                let divisor = parse_ratio(raw_value, source)?.round().max(1.0) as u32;
                node = MiniNode::Divide(Box::new(node), divisor);
            }
            '@' => weight = parse_ratio(raw_value, source)?.max(0.001),
            '?' => {
                let probability = if raw_value.is_empty() {
                    0.5
                } else {
                    parse_ratio(raw_value, source)? as f32
                };
                node = MiniNode::Degrade(Box::new(node), probability.clamp(0.0, 1.0));
            }
            '%' => {
                if let MiniNode::Polymeter { steps, .. } = &mut node {
                    *steps = Some(parse_ratio(raw_value, source)?.round().max(1.0) as usize);
                } else {
                    return Err(format!(
                        "`%` subdivision only follows a polymeter in `{source}`"
                    ));
                }
            }
            value => return Err(format!("Unsupported mini-notation suffix `{value}`")),
        }
    }
    Ok(ParsedMini {
        node,
        weight,
        replicate,
    })
}

fn parse_ratio(source: &str, context: &str) -> Result<f64, String> {
    if source.is_empty() {
        return Err(format!("Missing suffix value in `{context}`"));
    }
    if let Some((numerator, denominator)) = source.split_once(['%', '/']) {
        let numerator = numerator
            .parse::<f64>()
            .map_err(|_| format!("Invalid ratio `{source}` in `{context}`"))?;
        let denominator = denominator
            .parse::<f64>()
            .map_err(|_| format!("Invalid ratio `{source}` in `{context}`"))?;
        if denominator == 0.0 {
            return Err("A mini-notation ratio cannot divide by zero".to_owned());
        }
        Ok(numerator / denominator)
    } else {
        source
            .parse::<f64>()
            .map_err(|_| format!("Invalid number `{source}` in `{context}`"))
    }
}

fn parse_u32(source: &str, context: &str) -> Result<u32, String> {
    source
        .parse::<u32>()
        .map_err(|_| format!("Expected a positive integer in `{context}`"))
}

fn matching_close(source: &str, open: char, close: char) -> Result<usize, String> {
    let mut depth = 0_i32;
    for (index, character) in source.char_indices() {
        if character == open {
            depth += 1;
        } else if character == close {
            depth -= 1;
            if depth == 0 {
                return Ok(index);
            }
        }
    }
    Err(format!("Missing `{close}` in mini-notation `{source}`"))
}

fn split_mini_whitespace(source: &str) -> Vec<&str> {
    split_mini_where(source, |character, _, _| character.is_whitespace())
}

fn split_mini(source: &str, delimiter: char) -> Vec<&str> {
    split_mini_where(source, |character, _, _| character == delimiter)
}

fn split_mini_dots(source: &str) -> Vec<&str> {
    split_mini_where(source, |character, previous, next| {
        character == '.'
            && previous.is_some_and(char::is_whitespace)
            && next.is_some_and(char::is_whitespace)
    })
}

fn split_mini_where(
    source: &str,
    delimiter: impl Fn(char, Option<char>, Option<char>) -> bool,
) -> Vec<&str> {
    let mut output = Vec::new();
    let mut start = 0;
    let mut depths = [0_i32; 4];
    let characters = source.char_indices().collect::<Vec<_>>();
    for (position, &(index, character)) in characters.iter().enumerate() {
        match character {
            '[' => depths[0] += 1,
            ']' => depths[0] -= 1,
            '(' => depths[1] += 1,
            ')' => depths[1] -= 1,
            '<' => depths[2] += 1,
            '>' => depths[2] -= 1,
            '{' => depths[3] += 1,
            '}' => depths[3] -= 1,
            _ => {}
        }
        if depths == [0; 4]
            && delimiter(
                character,
                position.checked_sub(1).map(|value| characters[value].1),
                characters.get(position + 1).map(|value| value.1),
            )
        {
            if !source[start..index].trim().is_empty() {
                output.push(source[start..index].trim());
            }
            start = index + character.len_utf8();
        }
    }
    if !source[start..].trim().is_empty() {
        output.push(source[start..].trim());
    }
    output
}

fn is_euclidean_hit(step: u32, pulses: u32, steps: u32, offset: u32) -> bool {
    pulses != 0 && (((step + offset) % steps) * pulses) % steps < pulses
}

fn mix_seed(seed: u64, value: u64) -> u64 {
    seed ^ value.wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

fn deterministic_unit(mut value: u64) -> f32 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    (value as f64 / u64::MAX as f64) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_tracks_without_a_haskell_do_block() {
        let commands =
            parse_program("d1 $ sound \"bd sd\"\n\nd2 $ n (slow 2 \"c e g\") # sound \"cpu\"")
                .expect("program should parse");
        assert_eq!(commands.len(), 2);
        let ProgramCommand::Set { definition, .. } = &commands[0] else {
            panic!("expected a set command");
        };
        assert_eq!(definition.layer_count(), 1);
        assert_eq!(definition.events_for_layer(0, 1, 0, 2.0).len(), 2);
    }

    #[test]
    fn stack_euclid_ratchets_and_probability_are_native() {
        let commands =
            parse_program("d1 $ stack [sound \"bd(3,8)\", sound \"hh*4\", sound \"cp?0\"]")
                .expect("stack should parse");
        let ProgramCommand::Set { definition, .. } = &commands[0] else {
            panic!("expected a set command");
        };
        assert_eq!(definition.layer_count(), 3);
        assert_eq!(definition.events_for_layer(0, 1, 0, 2.0).len(), 3);
        assert_eq!(definition.events_for_layer(1, 1, 0, 2.0).len(), 4);
        assert_eq!(definition.events_for_layer(2, 1, 0, 2.0).len(), 0);
    }

    #[test]
    fn alternation_and_note_names_are_native() {
        let sequence = Sequence::parse("<c e g>").expect("sequence should parse");
        assert_eq!(
            parse_note(&sequence.value_at(0, 0).expect("cycle zero")),
            Some(0.0)
        );
        assert_eq!(
            parse_note(&sequence.value_at(0, 1).expect("cycle one")),
            Some(4.0)
        );
        assert_eq!(parse_note("cs5"), Some(1.0));
    }

    #[test]
    fn supports_rate_functions_inside_or_outside_the_source() {
        for source in ["d1 $ fast 2 $ sound \"bd\"", "d1 $ sound (fast 2 \"bd\")"] {
            let ProgramCommand::Set { definition, .. } = parse_program(source)
                .expect("rate expression should parse")
                .remove(0)
            else {
                panic!("expected a set command");
            };
            assert_eq!(definition.layer_period(0, 2.0), 1.0);
        }
    }

    #[test]
    fn parses_original_multiline_console_shape() {
        let source = r#"do {
          d1 $ stack
            [ sound "bd(3,8)" # gain 1
            , sound "hh(5,8)" # gain 0.8 # cut 1
            ]
        ;
          d2 $ n (slow 2 "0 3 5 7") # sound "cpu"
        ;
        }"#;
        let commands = parse_program(source).expect("console source should parse");
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn parses_nested_grouping_superposition_and_dot_shorthand() {
        let sequence = Sequence::parse("[bd [hh [cp sn:2] hh]] . [bd*2,hh*3]")
            .expect("nested mini-notation should parse");
        let hits = sequence.hits(0, 1);
        assert_eq!(hits.iter().filter(|hit| hit.value == "bd").count(), 3);
        assert!(hits.iter().any(|hit| hit.value == "sn:2"));
        assert_eq!(hits.iter().filter(|hit| hit.value == "hh").count(), 5);
    }

    #[test]
    fn implements_replicate_elongate_division_choice_and_euclid_offset() {
        assert_eq!(Sequence::parse("bd!3 sd").unwrap().hits(0, 1).len(), 4);
        let elongated = Sequence::parse("bd _ _ sd").unwrap().hits(0, 1);
        assert_eq!(elongated[0].slot_length, 0.75);
        assert_eq!(Sequence::parse("bd/2").unwrap().hits(0, 1).len(), 1);
        assert_eq!(Sequence::parse("bd/2").unwrap().hits(1, 1).len(), 0);
        assert_eq!(Sequence::parse("[bd|sd|cp]").unwrap().hits(0, 1).len(), 1);
        let base = Sequence::parse("bd(3,8)").unwrap().hits(0, 1);
        let offset = Sequence::parse("bd(3,8,1)").unwrap().hits(0, 1);
        assert_eq!(base.len(), 3);
        assert_eq!(offset.len(), 3);
        assert_ne!(base[0].phase, offset[0].phase);
    }

    #[test]
    fn polymeter_wraps_shorter_parts_across_cycles() {
        let sequence = Sequence::parse("{bd sd cp hh, arpy cr bass}").unwrap();
        let first = sequence.hits(0, 9);
        let second = sequence.hits(1, 9);
        assert_eq!(first.len(), 8);
        assert_eq!(second.len(), 8);
        let first_layer_cycle_two = second
            .iter()
            .filter(|hit| matches!(hit.value.as_str(), "arpy" | "cr" | "bass"))
            .map(|hit| hit.value.as_str())
            .collect::<Vec<_>>();
        assert_eq!(first_layer_cycle_two, vec!["cr", "bass", "arpy", "cr"]);
    }

    #[test]
    fn all_documented_scales_map_degrees_across_octaves() {
        assert_eq!(scale_degree("major", 0.0), Some(0.0));
        assert_eq!(scale_degree("major", 7.0), Some(12.0));
        assert_eq!(scale_degree("minor", -1.0), Some(-2.0));
        assert_eq!(scale_degree("rast", 2.0), Some(3.5));
        assert_eq!(scale_degree("messiaen7", 9.0), Some(11.0));
        let commands =
            parse_program("d1 $ n (scale \"ritusen\" \"0 1 2 3 4 5\") # sound \"superpiano\"")
                .unwrap();
        let ProgramCommand::Set { definition, .. } = &commands[0] else {
            panic!("expected set");
        };
        let notes = definition
            .events_for_layer(0, 1, 0, 2.0)
            .into_iter()
            .map(|(_, event)| event.note)
            .collect::<Vec<_>>();
        assert_eq!(notes, vec![0.0, 2.0, 5.0, 7.0, 9.0, 12.0]);
    }

    #[test]
    fn documented_chords_voicings_and_rolls_expand_to_native_events() {
        assert_eq!(parse_note("c4"), Some(-12.0));
        assert_eq!(parse_note("bf3"), Some(-14.0));
        assert_eq!(
            parse_note_or_chord("c'maj7").unwrap(),
            vec![0.0, 4.0, 7.0, 11.0]
        );
        assert_eq!(
            parse_note_or_chord("c'min9'i2").unwrap(),
            vec![7.0, 10.0, 12.0, 14.0, 15.0]
        );
        let commands = parse_program("d1 $ rolled $ n \"c'maj\" # sound \"superpiano\"").unwrap();
        let ProgramCommand::Set { definition, .. } = &commands[0] else {
            panic!("expected set");
        };
        let events = definition.events_for_layer(0, 1, 0, 2.0);
        assert_eq!(events.len(), 3);
        assert!(events[0].0 < events[1].0 && events[1].0 < events[2].0);
    }

    #[test]
    fn documented_cycle_time_and_stereo_transforms_are_native() {
        let ProgramCommand::Set { definition, .. } =
            parse_program("d1 $ every 2 rev $ sound \"bd sd cp\" # pan (range 0.1 0.9 $ saw)")
                .unwrap()
                .remove(0)
        else {
            panic!("expected set");
        };
        let reversed = definition.events_for_layer(0, 1, 0, 2.0);
        let forward = definition.events_for_layer(0, 1, 1, 2.0);
        assert_eq!(
            reversed
                .iter()
                .map(|(_, event)| event.sound.as_str())
                .collect::<Vec<_>>(),
            vec!["cp", "sd", "bd"]
        );
        assert_eq!(
            forward
                .iter()
                .map(|(_, event)| event.sound.as_str())
                .collect::<Vec<_>>(),
            vec!["bd", "sd", "cp"]
        );
        assert!(
            reversed
                .iter()
                .all(|(_, event)| (0.1..=0.9).contains(&event.pan))
        );

        let ProgramCommand::Set { definition, .. } =
            parse_program("d1 $ jux rev $ ply 2 $ sound \"bd sd\"")
                .unwrap()
                .remove(0)
        else {
            panic!("expected set");
        };
        let events = definition.events_for_layer(0, 1, 0, 2.0);
        assert_eq!(events.len(), 8);
        assert!(events.iter().any(|(_, event)| event.pan == 0.0));
        assert!(events.iter().any(|(_, event)| event.pan == 1.0));
    }

    #[test]
    fn documented_compression_echo_and_hurry_are_native() {
        let ProgramCommand::Set { definition, .. } =
            parse_program("d1 $ compress (1/4, 3/4) $ echo 3 0.125 0.5 $ hurry 2 $ sound \"bd\"")
                .unwrap()
                .remove(0)
        else {
            panic!("expected set");
        };
        assert_eq!(definition.layer_period(0, 2.0), 1.0);
        let events = definition.events_for_layer(0, 1, 0, 1.0);
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].0, 0.25);
        assert_eq!(events[0].1.speed, 2.0);
        assert!(events[0].1.gain > events[1].1.gain);
    }

    #[test]
    fn concatenation_generators_and_runtime_tempo_parse_natively() {
        let ProgramCommand::Set { definition, .. } =
            parse_program("d1 $ cat [sound \"bd\", sound \"sn\"]")
                .unwrap()
                .remove(0)
        else {
            panic!("expected set");
        };
        assert_eq!(definition.events_for_layer(0, 1, 0, 2.0)[0].1.sound, "bd");
        assert_eq!(definition.events_for_layer(0, 1, 1, 2.0)[0].1.sound, "sn");

        let ProgramCommand::Set { definition, .. } =
            parse_program("d1 $ n (run 4) # sound \"superpiano\"")
                .unwrap()
                .remove(0)
        else {
            panic!("expected set");
        };
        assert_eq!(definition.events_for_layer(0, 1, 0, 2.0).len(), 4);

        let commands = parse_program("setcps (120/60/4)").unwrap();
        assert!(matches!(commands.as_slice(), [ProgramCommand::SetCps(value)] if *value == 0.5));
    }

    #[test]
    fn named_numbered_one_shot_and_panic_patterns_parse_natively() {
        let named = parse_program("p \"romeo\" $ sound \"bd\"").unwrap();
        let numbered = parse_program("p 1234 $ sound \"sn\"").unwrap();
        assert!(matches!(named.as_slice(), [ProgramCommand::Set { .. }]));
        assert!(matches!(numbered.as_slice(), [ProgramCommand::Set { .. }]));
        assert!(matches!(
            parse_program("once $ sound \"cp\"").unwrap().as_slice(),
            [ProgramCommand::Once(_)]
        ));
        assert!(matches!(
            parse_program("panic").unwrap().as_slice(),
            [ProgramCommand::Panic]
        ));
    }

    #[test]
    fn documented_structure_slicing_and_signal_sampling_are_native() {
        let ProgramCommand::Set { definition, .. } =
            parse_program("d1 $ struct \"t ~ t ~\" $ chop 4 $ sound \"breaks\"")
                .unwrap()
                .remove(0)
        else {
            panic!("expected set");
        };
        let events = definition.events_for_layer(0, 1, 0, 2.0);
        assert_eq!(events.len(), 8);
        assert_eq!(events[0].1.begin, 0.0);
        assert_eq!(events[1].1.begin, 0.25);
        assert_eq!(events[1].1.end, 0.5);

        let value = parse_control("quantise 2 $ segment 4 $ saw").unwrap();
        for position in [0.1, 0.3, 0.6, 0.9] {
            let sampled = value.at(position, 0, 0);
            assert_eq!(sampled * 2.0, (sampled * 2.0).round());
        }
    }
}
