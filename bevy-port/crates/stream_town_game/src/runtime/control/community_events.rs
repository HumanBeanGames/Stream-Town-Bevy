pub(crate) fn town_event_from_id(requested: &StableId) -> Option<TownEvent> {
    match requested.as_str().trim_start_matches("event:") {
        "festival" => Some(TownEvent::Festival),
        "raid" | "enemy_raid" => Some(TownEvent::EnemyRaid),
        "harsh_weather" | "weather" => Some(TownEvent::HarshWeather),
        "resource_boom" | "wood_boom" => Some(TownEvent::ResourceBoom(
            StableId::new("resource:wood").expect("static ID"),
        )),
        "fish_god" | "fishgod" => Some(TownEvent::FishGod),
        _ => None,
    }
}

pub(crate) fn community_event_from_id(requested: &StableId) -> Option<CommunityEvent> {
    match requested.as_str().trim_start_matches("event:") {
        "prospecting" | "prospecting_boom" | "prospectingboom" | "prospector" => {
            Some(CommunityEvent::ProspectingBoom)
        }
        "reforestation" | "reforestation_boom" | "reforestationboom" | "forester" => {
            Some(CommunityEvent::ReforestationBoom)
        }
        "agricultural" | "agricultural_boom" | "agriculturalboom" | "agriculture" | "tender" => {
            Some(CommunityEvent::AgriculturalBoom)
        }
        "rebalance" | "balanced" => Some(CommunityEvent::Rebalance),
        "awakening" | "xp" => Some(CommunityEvent::Awakening),
        "economic" | "economic_boom" | "economicboom" | "economy" => {
            Some(CommunityEvent::EconomicBoom)
        }
        "invasion" => Some(CommunityEvent::Invasion),
        "market" | "market_event" => Some(CommunityEvent::Market),
        _ => None,
    }
}

pub(crate) const fn community_event_name(event: CommunityEvent) -> &'static str {
    match event {
        CommunityEvent::ProspectingBoom => "Prospecting Boom",
        CommunityEvent::ReforestationBoom => "Reforestation Boom",
        CommunityEvent::AgriculturalBoom => "Agricultural Boom",
        CommunityEvent::Rebalance => "Rebalance",
        CommunityEvent::Awakening => "Awakening",
        CommunityEvent::EconomicBoom => "Economic Boom",
        CommunityEvent::Invasion => "Invasion",
        CommunityEvent::Market => "Market",
    }
}

pub(crate) const fn community_event_description(event: CommunityEvent) -> &'static str {
    match event {
        CommunityEvent::ProspectingBoom => {
            "Prospectors work at 3x speed; Foresters and Tenders at 0.5x."
        }
        CommunityEvent::ReforestationBoom => {
            "Foresters work at 3x speed; Prospectors and Tenders at 0.5x."
        }
        CommunityEvent::AgriculturalBoom => {
            "Tenders work at 3x speed; Prospectors and Foresters at 0.5x."
        }
        CommunityEvent::Rebalance => {
            "Prospectors, Foresters, and Tenders work at their normal rates."
        }
        CommunityEvent::Awakening => "All experience gains are increased by 20%.",
        CommunityEvent::EconomicBoom => "All gathering actions are 10% faster.",
        CommunityEvent::Invasion => {
            "Monster waves are 50% larger and monster gold rewards are doubled."
        }
        CommunityEvent::Market => {
            "Overflow gathered resources are sold automatically for 25% of normal trade value."
        }
    }
}

pub(crate) fn community_event_role_rate_multiplier(event: CommunityEvent, role: &StableId) -> f32 {
    let role = role.as_str();
    match event {
        CommunityEvent::ProspectingBoom if role == "role:prospector" => 3.0,
        CommunityEvent::ProspectingBoom if matches!(role, "role:forester" | "role:tender") => 0.5,
        CommunityEvent::ReforestationBoom if role == "role:forester" => 3.0,
        CommunityEvent::ReforestationBoom if matches!(role, "role:prospector" | "role:tender") => {
            0.5
        }
        CommunityEvent::AgriculturalBoom if role == "role:tender" => 3.0,
        CommunityEvent::AgriculturalBoom if matches!(role, "role:prospector" | "role:forester") => {
            0.5
        }
        _ => 1.0,
    }
}

pub(crate) fn event_adjusted_wave_size(base: u16, event: Option<CommunityEvent>) -> u16 {
    if event == Some(CommunityEvent::Invasion) {
        base.saturating_mul(3).saturating_add(1) / 2
    } else {
        base
    }
}
