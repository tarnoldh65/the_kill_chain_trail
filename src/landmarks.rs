//! Fixed stops on the road to the IPO: forts to rest and resupply, and rivers to cross.

use std::fmt;

use crate::events::{Effect, NOTHING};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Landmark {
    BoardBriefing,
    Conference,
    Audit,
    S1Filing,
    PenTest,
    Flip,
    Roadshow,
}

impl Landmark {
    pub const ALL: [Landmark; 7] = [
        Self::BoardBriefing,
        Self::Conference,
        Self::Audit,
        Self::S1Filing,
        Self::PenTest,
        Self::Flip,
        Self::Roadshow,
    ];

    /// The scheduled day: the Monday that starts the landmark's week.
    pub fn day(self) -> u32 {
        let week = match self {
            Self::BoardBriefing => 4,
            Self::Conference => 7,
            Self::Audit => 10,
            Self::S1Filing => 13,
            Self::PenTest => 16,
            Self::Flip => 19,
            Self::Roadshow => 23,
        };
        week * 7 + 1
    }

    /// Short name for the timeline.
    pub fn short(self) -> &'static str {
        match self {
            Self::BoardBriefing => "BOARD",
            Self::Conference => "CONF",
            Self::Audit => "AUDIT",
            Self::S1Filing => "S-1",
            Self::PenTest => "PENTEST",
            Self::Flip => "FLIP",
            Self::Roadshow => "ROADSHOW",
        }
    }

    pub fn intro(self) -> &'static str {
        match self {
            Self::BoardBriefing => {
                "The board wants a security briefing. How you pitch it decides how much they trust you."
            }
            Self::Conference => {
                "The security conference is in town. The Vendor Hall is open and recruiters are everywhere."
            }
            Self::Audit => {
                "The SOC 2 Type II auditors have arrived with clipboards. Fail, and the IPO slips."
            }
            Self::S1Filing => {
                "The lawyers are drafting the S-1. Past incidents belong in the risk factors. Or do they?"
            }
            Self::PenTest => {
                "The penetration testers are done. Here is how your security really looks."
            }
            Self::Flip => {
                "The S-1 is public. Everyone knows about the IPO now, including the attackers."
            }
            Self::Roadshow => "The executives hit the road to pitch investors. Hotel Wi-Fi awaits.",
        }
    }

    /// Forts stop for resupply and rest; every other landmark is a river to cross.
    pub fn is_fort(self) -> bool {
        matches!(self, Self::Conference | Self::Flip)
    }

    pub fn crossings(self) -> &'static [Crossing] {
        match self {
            Self::BoardBriefing => &BOARD,
            Self::Audit => &AUDIT,
            Self::S1Filing => &S1,
            Self::PenTest => &PENTEST,
            Self::Roadshow => &ROADSHOW,
            Self::Conference | Self::Flip => &[],
        }
    }
}

impl fmt::Display for Landmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::BoardBriefing => "Board Security Briefing",
            Self::Conference => "Security Conference",
            Self::Audit => "SOC 2 Type II Audit",
            Self::S1Filing => "Confidential S-1 Filing",
            Self::PenTest => "Third-Party Pen Test",
            Self::Flip => "Public S-1 Flip",
            Self::Roadshow => "IPO Roadshow",
        })
    }
}

/// Chance of success in thousandths: `base`, plus up to `posture` for perfect average
/// posture, minus `per_incident` for each incident so far.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Odds {
    pub base: u32,
    pub posture: u32,
    pub per_incident: u32,
}

/// Odds that never fail.
pub const CERTAIN: Odds = Odds {
    base: 1000,
    posture: 0,
    per_incident: 0,
};

/// One way across a river, with what happens either way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crossing {
    pub label: &'static str,
    /// Paid up front, success or not.
    pub cost: i64,
    pub odds: Odds,
    pub success: (&'static str, Effect),
    pub failure: (&'static str, Effect),
}

const BOARD: [Crossing; 3] = [
    Crossing {
        label: "Present the honest numbers",
        cost: 0,
        odds: Odds {
            base: 200,
            posture: 800,
            per_incident: 150,
        },
        success: (
            "The board appreciates the candor. They trust you more.",
            Effect {
                trust: 10,
                ..NOTHING
            },
        ),
        failure: (
            "The honest numbers are not good. The board is concerned.",
            Effect {
                trust: -10,
                ..NOTHING
            },
        ),
    },
    Crossing {
        label: "Show them a scary animated threat map",
        cost: 0,
        odds: Odds {
            base: 500,
            posture: 0,
            per_incident: 100,
        },
        success: (
            "The blinking map works. The board approves an extra $50K.",
            Effect {
                trust: 5,
                budget: 50_000,
                ..NOTHING
            },
        ),
        failure: (
            "A board member asks what the dots mean. Nobody knows.",
            Effect {
                trust: -5,
                ..NOTHING
            },
        ),
    },
    Crossing {
        label: "Blame the previous CISO",
        cost: 0,
        odds: Odds {
            base: 300,
            posture: 0,
            per_incident: 0,
        },
        success: (
            "The board nods. The previous CISO is a convenient villain.",
            Effect {
                trust: 3,
                ..NOTHING
            },
        ),
        failure: (
            "The previous CISO sits on the board now. Awkward.",
            Effect {
                trust: -15,
                ..NOTHING
            },
        ),
    },
];

const AUDIT_PASSED: (&str, Effect) = (
    "Clean audit. The auditors seem almost disappointed.",
    Effect {
        trust: 5,
        valuation: 20,
        ..NOTHING
    },
);

const AUDIT_FAILED: (&str, Effect) = (
    "The auditors found 14 findings. The IPO slips while you fix them.",
    Effect {
        trust: -5,
        delay: true,
        ..NOTHING
    },
);

const AUDIT: [Crossing; 3] = [
    Crossing {
        label: "Rely on your controls",
        cost: 0,
        odds: Odds {
            base: 100,
            posture: 900,
            per_incident: 0,
        },
        success: AUDIT_PASSED,
        failure: AUDIT_FAILED,
    },
    Crossing {
        label: "Hire audit consultants ($60K)",
        cost: 60_000,
        odds: Odds {
            base: 750,
            posture: 200,
            per_incident: 0,
        },
        success: AUDIT_PASSED,
        failure: AUDIT_FAILED,
    },
    Crossing {
        label: "Crunch to fix findings before they see them",
        cost: 0,
        odds: Odds {
            base: 400,
            posture: 500,
            per_incident: 0,
        },
        success: (
            "The crunch worked. A clean audit, and a very tired team.",
            Effect {
                trust: 5,
                valuation: 20,
                burnout: 20,
                ..NOTHING
            },
        ),
        failure: (
            "The crunch was not enough. The IPO slips and the team is fried.",
            Effect {
                trust: -5,
                burnout: 20,
                delay: true,
                ..NOTHING
            },
        ),
    },
];

/// The S-1 decision; the per-incident cost is handled by the game.
const S1: [Crossing; 2] = [
    Crossing {
        label: "Disclose every incident",
        cost: 0,
        odds: CERTAIN,
        success: ("The risk factors are honest. Investors shrug.", NOTHING),
        failure: ("", NOTHING),
    },
    Crossing {
        label: "Leave the incidents out",
        cost: 0,
        odds: CERTAIN,
        success: (
            "The risk factors are spotless. Legal looks nervous.",
            NOTHING,
        ),
        failure: ("", NOTHING),
    },
];

/// The pen test report; the cost of weak areas is handled by the game.
const PENTEST: [Crossing; 2] = [
    Crossing {
        label: "Take the report as written",
        cost: 0,
        odds: CERTAIN,
        success: ("The report goes to the board unedited.", NOTHING),
        failure: ("", NOTHING),
    },
    Crossing {
        label: "Pay to soften the report ($40K)",
        cost: 40_000,
        odds: CERTAIN,
        success: (
            "The testers rephrase \"critical\" as \"interesting\". Half the sting is gone.",
            NOTHING,
        ),
        failure: ("", NOTHING),
    },
];

const ROADSHOW: [Crossing; 3] = [
    Crossing {
        label: "Burner laptops and a security escort ($50K)",
        cost: 50_000,
        odds: Odds {
            base: 900,
            posture: 0,
            per_incident: 0,
        },
        success: ("The roadshow goes off without a hitch.", NOTHING),
        failure: (
            "Even the escort could not stop the CFO from joining \"Free Airport WiFi\".",
            Effect {
                valuation: -20,
                ..NOTHING
            },
        ),
    },
    Crossing {
        label: "Send a stern email about hotel Wi-Fi",
        cost: 0,
        odds: Odds {
            base: 300,
            posture: 500,
            per_incident: 0,
        },
        success: ("Everyone read the email. Even the CEO.", NOTHING),
        failure: (
            "The CFO's laptop was stolen from a hotel bar. It was not encrypted.",
            Effect {
                valuation: -40,
                trust: -5,
                ..NOTHING
            },
        ),
    },
    Crossing {
        label: "Hope for the best",
        cost: 0,
        odds: Odds {
            base: 200,
            posture: 300,
            per_incident: 0,
        },
        success: ("Nothing went wrong. Pure luck.", NOTHING),
        failure: (
            "The CEO wired $2M to a \"banker\" from a convincing email. Whaling season.",
            Effect {
                valuation: -60,
                trust: -10,
                ..NOTHING
            },
        ),
    },
];
