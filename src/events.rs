//! Random events that have nothing to do with the attackers: the broken wagon wheels.

use crate::game::Area;

/// What an event or a choice does. Valuation is in thousandths of the base valuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Effect {
    pub trust: i32,
    pub brand: i32,
    pub budget: i64,
    pub valuation: i64,
    /// Burnout for every analyst.
    pub burnout: i32,
    /// Extra burnout for the analyst the event names.
    pub patient: i32,
    pub posture: &'static [(Area, i32)],
    pub coffee: i32,
    /// Added to the daily odds, in thousandths, of a new attacker showing up.
    pub threat: u32,
    /// Days of thin coverage, which lowers Detection.
    pub holiday: u32,
    /// Plants a hidden attacker who is already inside.
    pub intruder: bool,
    /// Pushes the IPO back two weeks.
    pub delay: bool,
}

pub const NOTHING: Effect = Effect {
    trust: 0,
    brand: 0,
    budget: 0,
    valuation: 0,
    burnout: 0,
    patient: 0,
    posture: &[],
    coffee: 0,
    threat: 0,
    holiday: 0,
    intruder: false,
    delay: false,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Choice {
    pub label: &'static str,
    pub text: &'static str,
    pub effect: Effect,
}

/// `{name}` in the text is replaced with a random analyst's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub text: &'static str,
    /// Applied right away when there are no choices.
    pub effect: Effect,
    pub choices: &'static [Choice],
}

/// The CIO's data center migration, which a leadership briefing can also set off.
pub const PET_PROJECT: usize = 5;

pub const EVENTS: [Event; 11] = [
    Event {
        text: "{name} has the flu and is coughing on the keyboard.",
        effect: NOTHING,
        choices: &[
            Choice {
                label: "Send them home",
                text: "Sent home with soup. Everyone else picks up the slack.",
                effect: Effect {
                    burnout: 4,
                    patient: -15,
                    ..NOTHING
                },
            },
            Choice {
                label: "Let them work through it",
                text: "They work through it. Heroic, disgusting, and exhausting.",
                effect: Effect {
                    patient: 15,
                    ..NOTHING
                },
            },
        ],
    },
    Event {
        text: "A major cloud provider is down for a day. Customers blame you anyway.",
        effect: Effect {
            brand: -3,
            valuation: -5,
            ..NOTHING
        },
        choices: &[],
    },
    Event {
        text: "A zero-day drops for the VPN you run. The exploit is already on social media.",
        effect: NOTHING,
        choices: &[
            Choice {
                label: "Patch it tonight",
                text: "Patched by 3 AM. The team is tired but the VPN is safe.",
                effect: Effect {
                    burnout: 8,
                    posture: &[(Area::Perimeter, 5), (Area::Endpoint, 5)],
                    ..NOTHING
                },
            },
            Choice {
                label: "Accept the risk",
                text: "You accept the risk. The risk accepts you back.",
                effect: Effect {
                    posture: &[(Area::Perimeter, -15)],
                    ..NOTHING
                },
            },
        ],
    },
    Event {
        text: "Your payroll vendor was breached. Your data was in there somewhere.",
        effect: Effect {
            trust: -2,
            intruder: true,
            ..NOTHING
        },
        choices: &[],
    },
    Event {
        text: "The CEO announced the IPO date on social media with three rocket ships. Attackers took note.",
        effect: Effect {
            brand: 3,
            threat: 10,
            ..NOTHING
        },
        choices: &[],
    },
    Event {
        text: "The CIO wants to migrate the data center before the IPO. What could go wrong?",
        effect: NOTHING,
        choices: &[
            Choice {
                label: "Go along with it",
                text: "The migration begins. Firewall rules are \"temporarily\" wide open.",
                effect: Effect {
                    trust: 3,
                    posture: &[
                        (Area::Identity, -5),
                        (Area::Endpoint, -5),
                        (Area::People, -5),
                        (Area::Perimeter, -5),
                        (Area::Resilience, -5),
                        (Area::Detection, -5),
                    ],
                    ..NOTHING
                },
            },
            Choice {
                label: "Push back",
                text: "You push back. The CIO is now \"disappointed\" in you.",
                effect: Effect {
                    trust: -5,
                    ..NOTHING
                },
            },
        ],
    },
    Event {
        text: "The coffee machine is broken. Smoke is involved.",
        effect: NOTHING,
        choices: &[
            Choice {
                label: "Buy a new one ($2K)",
                text: "A shiny new espresso machine. Morale is caffeinated.",
                effect: Effect {
                    budget: -2_000,
                    ..NOTHING
                },
            },
            Choice {
                label: "Live without it",
                text: "No machine. The remaining coffee was used to put out the fire.",
                effect: Effect {
                    coffee: -100,
                    ..NOTHING
                },
            },
        ],
    },
    Event {
        text: "Holiday week. Half the company is out. The attackers are not.",
        effect: Effect {
            burnout: -10,
            holiday: 7,
            ..NOTHING
        },
        choices: &[],
    },
    Event {
        text: "The board approves an extra $100K for security. Someone read the news.",
        effect: Effect {
            budget: 100_000,
            ..NOTHING
        },
        choices: &[],
    },
    Event {
        text: "A grateful employee brought donuts for the SOC.",
        effect: Effect {
            burnout: -5,
            ..NOTHING
        },
        choices: &[],
    },
    Event {
        text: "Marketing renamed the SOC the \"Cyber Fortress\". Nothing else changed.",
        effect: NOTHING,
        choices: &[],
    },
];
