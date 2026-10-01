//! The attacker's side: threat actors, their hidden kill chain progress, the
//! alerts they leave behind, and the incidents they cause when they win.

use std::fmt;

use crate::game::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    Ransomware,
    DataThief,
    Espionage,
    Hacktivists,
    Apt,
    Insider,
}

impl Actor {
    pub const ALL: [Actor; 6] = [
        Self::Ransomware,
        Self::DataThief,
        Self::Espionage,
        Self::Hacktivists,
        Self::Apt,
        Self::Insider,
    ];

    /// Posture areas that slow this actor down.
    pub fn defenses(self) -> &'static [Area] {
        match self {
            Self::Ransomware => &[Area::People, Area::Endpoint, Area::Resilience],
            Self::DataThief => &[Area::Identity, Area::Detection],
            Self::Espionage => &[Area::People, Area::Identity],
            Self::Hacktivists => &[Area::Perimeter],
            Self::Apt => &[Area::Detection],
            Self::Insider => &[Area::Identity, Area::Detection],
        }
    }

    /// Where a new campaign starts; insiders are already inside.
    pub fn first_stage(self) -> Stage {
        match self {
            Self::Insider => Stage::InitialAccess,
            _ => Stage::Reconnaissance,
        }
    }

    /// What the SOC might notice while this actor works.
    pub fn alerts(self) -> [&'static str; 2] {
        match self {
            Self::Ransomware => [
                "Someone ran PowerShell from a Word document in Accounting.",
                "A file server is renaming files faster than usual.",
            ],
            Self::DataThief => [
                "Impossible travel: the CFO logged in from Ohio and Romania 10 minutes apart.",
                "The customer database exported 2 GB to an unknown IP at 3 AM.",
            ],
            Self::Espionage => [
                "An engineer got a LinkedIn \"job offer\" with a macro-enabled attachment.",
                "A contractor laptop is browsing the product roadmap folder.",
            ],
            Self::Hacktivists => [
                "Port scans from 400 IPs are hitting the public website.",
                "Someone keeps trying to log into the website CMS as \"admin\".",
            ],
            Self::Apt => [
                "A signed update from a trusted vendor is making odd DNS requests.",
                "A domain controller is talking to a server in a country we don't do business in.",
            ],
            Self::Insider => [
                "A finance employee printed the draft S-1 three times.",
                "Someone forwarded the draft S-1 to a personal email address.",
            ],
        }
    }

    /// The headline when this actor reaches its objective.
    pub fn incident(self) -> &'static str {
        match self {
            Self::Ransomware => {
                "RANSOMWARE! The file servers are encrypted and the printers are printing ransom notes."
            }
            Self::DataThief => "Customer records are for sale on a dark web forum.",
            Self::Espionage => {
                "Your product roadmap just appeared in a competitor's launch announcement."
            }
            Self::Hacktivists => {
                "Hacktivists knocked the website offline and replaced it with a meme."
            }
            Self::Apt => {
                "A government agency calls: a nation-state has been inside your network for weeks."
            }
            Self::Insider => "Draft S-1 numbers leaked to a financial blog.",
        }
    }

    pub fn responses(self) -> &'static [Response] {
        match self {
            Self::Ransomware => &RANSOMWARE,
            Self::DataThief => &PII_BREACH,
            Self::Espionage => &IP_THEFT,
            Self::Hacktivists => &DDOS,
            Self::Apt => &APT_FOUND,
            Self::Insider => &INSIDER_LEAK,
        }
    }

    /// The condition this actor's incident leaves behind if the response lets it linger.
    pub fn condition(self) -> Condition {
        match self {
            Self::Ransomware => Condition::SystemsDown,
            Self::DataThief => Condition::RegulatorInquiry,
            Self::Espionage => Condition::LeakyRoadmap,
            Self::Hacktivists => Condition::Downtime,
            Self::Apt => Condition::PersistentAccess,
            Self::Insider => Condition::Paranoia,
        }
    }
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Ransomware => "Ransomware gang",
            Self::DataThief => "Data thief",
            Self::Espionage => "Competitor espionage",
            Self::Hacktivists => "Hacktivists",
            Self::Apt => "Nation-state APT",
            Self::Insider => "Malicious insider",
        })
    }
}

/// The kill chain, as seen from the attacker's side. Never shown before the game ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    Reconnaissance,
    InitialAccess,
    Foothold,
    Objective,
}

impl Stage {
    pub fn next(self) -> Self {
        match self {
            Self::Reconnaissance => Self::InitialAccess,
            Self::InitialAccess => Self::Foothold,
            Self::Foothold | Self::Objective => Self::Objective,
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Reconnaissance => "Reconnaissance",
            Self::InitialAccess => "Initial access",
            Self::Foothold => "Foothold",
            Self::Objective => "Objective",
        })
    }
}

/// How a campaign stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    GaveUp,
    Evicted,
    Succeeded,
}

/// One threat actor's attempt on the company.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Campaign {
    pub actor: Actor,
    pub start: u32,
    /// The furthest kill chain stage reached.
    pub stage: Stage,
    /// Whether any of its steps raised an alert.
    pub seen: bool,
    pub end: Option<End>,
}

/// Something the SOC noticed; `campaign` is `None` for a false positive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alert {
    pub day: u32,
    pub text: &'static str,
    pub campaign: Option<usize>,
}

/// One way to respond to an incident. Damage is scaled down by Resilience and insurance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Response {
    pub label: &'static str,
    pub text: &'static str,
    /// Valuation lost, in thousandths of the base valuation.
    pub valuation: i64,
    pub trust: i32,
    pub brand: i32,
    pub budget: i64,
    /// Whether the incident's condition sticks around afterward.
    pub lingers: bool,
    pub needs_backups: bool,
}

const RESPONSE: Response = Response {
    label: "",
    text: "",
    valuation: 0,
    trust: 0,
    brand: 0,
    budget: 0,
    lingers: true,
    needs_backups: false,
};

const RANSOMWARE: [Response; 3] = [
    Response {
        label: "Restore from backups",
        text: "You restore from backups. Systems stay down until the restore is done.",
        valuation: 20,
        trust: -3,
        brand: -5,
        needs_backups: true,
        ..RESPONSE
    },
    Response {
        label: "Pay the ransom ($150K)",
        text: "You pay. The decryptor works, mostly. The board is not amused.",
        valuation: 20,
        trust: -15,
        brand: -5,
        budget: 150_000,
        lingers: false,
        ..RESPONSE
    },
    Response {
        label: "Wipe and rebuild everything",
        text: "You wipe it all. Systems stay down until the rebuild is done.",
        valuation: 60,
        trust: -5,
        brand: -10,
        ..RESPONSE
    },
];

const PII_BREACH: [Response; 3] = [
    Response {
        label: "Disclose now",
        text: "You disclose immediately. Regulators open an inquiry anyway.",
        valuation: 40,
        brand: -10,
        ..RESPONSE
    },
    Response {
        label: "Call outside counsel ($80K)",
        text: "Outside counsel manages the fallout for a hefty fee. Regulators still call.",
        valuation: 25,
        trust: -3,
        brand: -8,
        budget: 80_000,
        ..RESPONSE
    },
    Response {
        label: "Delay disclosure",
        text: "You delay disclosure. Legal is sweating. So is the board.",
        valuation: 10,
        trust: -15,
        brand: -5,
        ..RESPONSE
    },
];

const IP_THEFT: [Response; 2] = [
    Response {
        label: "Sue them ($100K)",
        text: "The lawyers are unleashed. A court order plugs the leak.",
        valuation: 20,
        brand: -3,
        budget: 100_000,
        lingers: false,
        ..RESPONSE
    },
    Response {
        label: "Quiet remediation",
        text: "You quietly patch things up. The roadmap may still be leaking.",
        valuation: 40,
        ..RESPONSE
    },
];

const DDOS: [Response; 2] = [
    Response {
        label: "Buy emergency mitigation ($50K)",
        text: "A scrubbing service soaks up the flood. The meme lives on in screenshots.",
        valuation: 5,
        brand: -5,
        budget: 50_000,
        lingers: false,
        ..RESPONSE
    },
    Response {
        label: "Wait it out",
        text: "You wait it out. Customers are not waiting.",
        valuation: 15,
        brand: -10,
        ..RESPONSE
    },
];

const APT_FOUND: [Response; 2] = [
    Response {
        label: "Full eviction now ($100K)",
        text: "Specialists tear the intruders out by the roots.",
        valuation: 20,
        trust: -5,
        budget: 100_000,
        lingers: false,
        ..RESPONSE
    },
    Response {
        label: "Partial cleanup",
        text: "You clean up what you can find. They know other ways in.",
        valuation: 10,
        ..RESPONSE
    },
];

const INSIDER_LEAK: [Response; 2] = [
    Response {
        label: "Fire the suspect",
        text: "You fire the suspect. Everyone else updates their resumes.",
        valuation: 30,
        trust: -5,
        brand: -5,
        lingers: false,
        ..RESPONSE
    },
    Response {
        label: "Investigate quietly",
        text: "You start a quiet investigation. Everyone suspects everyone.",
        valuation: 20,
        ..RESPONSE
    },
];

/// What a successful attack leaves behind until a specific action clears it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    SystemsDown,
    RegulatorInquiry,
    LeakyRoadmap,
    Downtime,
    PersistentAccess,
    Paranoia,
}

impl Condition {
    pub const ALL: [Condition; 6] = [
        Self::SystemsDown,
        Self::RegulatorInquiry,
        Self::LeakyRoadmap,
        Self::Downtime,
        Self::PersistentAccess,
        Self::Paranoia,
    ];

    /// Status tag shown on screen.
    pub fn tag(self) -> &'static str {
        match self {
            Self::SystemsDown => "SYSTEMS DOWN",
            Self::RegulatorInquiry => "REGULATORS",
            Self::LeakyRoadmap => "LEAKY ROADMAP",
            Self::Downtime => "DOWNTIME",
            Self::PersistentAccess => "INTRUDERS",
            Self::Paranoia => "PARANOIA",
        }
    }

    /// The action that clears this condition.
    pub fn cure(self) -> &'static str {
        match self {
            Self::SystemsDown => "Restore operations",
            Self::RegulatorInquiry => "Answer the regulators",
            Self::LeakyRoadmap => "Lock down the roadmap",
            Self::Downtime => "Mitigate the DDoS",
            Self::PersistentAccess => "Evict the intruders",
            Self::Paranoia => "Run an insider investigation",
        }
    }

    pub fn effect(self) -> &'static str {
        match self {
            Self::SystemsDown => "Most actions are impossible and the brand suffers daily.",
            Self::RegulatorInquiry => "Legal fees of $10K every Monday.",
            Self::LeakyRoadmap => "The valuation slips every Monday.",
            Self::Downtime => "Brand loyalty drops every day.",
            Self::PersistentAccess => "New attackers start with a foothold.",
            Self::Paranoia => "Burnout rises faster and nobody can be recruited.",
        }
    }
}
