//! Operations: longer jobs a deployed tool unlocks, each with a payoff at the end.

use crate::attack::Actor;
use crate::events::{Effect, NOTHING};
use crate::game::{Area, Item};

/// What a tool-unlocked operation costs and does. Boosts and eviction odds scale with the
/// tool's condition when the operation finishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operation {
    pub tool: Item,
    pub label: &'static str,
    pub description: &'static str,
    /// Days at Steady tempo with no seniors.
    pub days: u32,
    pub cost: i64,
    /// Added to each area for a few weeks.
    pub boosts: &'static [(Area, i32)],
    /// Attackers whose campaigns it may end, and the base odds in percent.
    pub evicts: &'static [Actor],
    pub odds: u32,
    /// Attackers whose next incident does half the damage.
    pub braces: &'static [Actor],
    /// Applied every time it finishes.
    pub effect: Effect,
    /// Odds in thousandths that it goes wrong, what that does, and what it says instead.
    pub mishap: Option<(u32, Effect, &'static str)>,
    pub results: [&'static str; 3],
    /// Logged when its boosts wear off.
    pub fades: &'static str,
}

const NONE: Operation = Operation {
    tool: Item::Coffee,
    label: "",
    description: "",
    days: 0,
    cost: 0,
    boosts: &[],
    evicts: &[],
    odds: 0,
    braces: &[],
    effect: NOTHING,
    mishap: None,
    results: ["", "", ""],
    fades: "",
};

pub const OPERATIONS: [Operation; 11] = [
    Operation {
        tool: Item::PasswordManager,
        label: "Sticky note sweep",
        description: "Scold everyone with a password on their monitor. Helps Identity a while.",
        days: 3,
        boosts: &[(Area::Identity, 10)],
        mishap: Some((
            250,
            Effect {
                trust: -2,
                ..NOTHING
            },
            "You scolded the CFO for the sticky note on their monitor. The CFO remembers.",
        )),
        results: [
            "Sticky note sweep done. 212 passwords confiscated; three of them were \"password\".",
            "The sweep found passwords under keyboards, in drawers, and on one forearm.",
            "Sticky note sweep complete. Facilities wants to know why the monitors are so clean.",
        ],
        fades: "The sticky notes are creeping back onto the monitors.",
        ..NONE
    },
    Operation {
        tool: Item::Pam,
        label: "Rotate the keys",
        description: "Reset every admin credential. May evict data thieves and insiders.",
        days: 5,
        boosts: &[(Area::Identity, 10)],
        evicts: &[Actor::DataThief, Actor::Insider],
        odds: 50,
        results: [
            "Every admin key is rotated. Two scripts broke; one of them belonged to nobody.",
            "Keys rotated. The service account \"temp_2019\" has finally been retired.",
            "Keys rotated. The DBA had to ask for their own password back. Twice.",
        ],
        fades: "Old habits are back: admins are sharing keys over chat again.",
        ..NONE
    },
    Operation {
        tool: Item::Edr,
        label: "Compromise assessment",
        description: "Sweep every laptop for implants. May evict ransomware gangs and APTs.",
        days: 6,
        boosts: &[(Area::Endpoint, 10)],
        evicts: &[Actor::Ransomware, Actor::Apt],
        odds: 50,
        results: [
            "The compromise assessment combed every laptop. It found mostly browser toolbars.",
            "Every endpoint was swept. One was running a crypto miner and a fantasy league.",
            "Compromise assessment done. The CEO's laptop had 14 \"PDF converters\".",
        ],
        fades: "New laptops have shipped since the compromise assessment. Nobody checked them.",
        ..NONE
    },
    Operation {
        tool: Item::TrainingPlatform,
        label: "Run a phishing simulation",
        description: "Test everyone with a fake phish. Helps People a while. Executives hate it.",
        days: 4,
        cost: 5_000,
        boosts: &[(Area::People, 20)],
        mishap: Some((
            333,
            Effect {
                trust: -3,
                ..NOTHING
            },
            "The VP of Sales clicked the test phish and is furious about being \"tricked\".",
        )),
        results: [
            "Phishing simulation done. 23% clicked. One person replied with their password.",
            "Phishing simulation done. Clicks are down; reported phish are up, mostly newsletters.",
            "The test phish went out. Accounting forwarded it to everyone \"just to be safe.\"",
        ],
        fades: "The phishing simulation is a distant memory. People are clicking again.",
        ..NONE
    },
    Operation {
        tool: Item::VulnScanner,
        label: "Patch sprint",
        description: "Patch what the scanner found. Helps Endpoint and Perimeter a while. Tiring.",
        days: 5,
        boosts: &[(Area::Perimeter, 15), (Area::Endpoint, 10)],
        effect: Effect {
            burnout: 8,
            ..NOTHING
        },
        results: [
            "Patch sprint complete. 1,200 patches applied, 3 servers rebooted unexpectedly.",
            "Patch sprint done. The scanner's 600-page report is down to 590 pages.",
            "Everything is patched. The lobby TV's open SSH port is finally closed.",
        ],
        fades: "New vulnerabilities have piled up since the last patch sprint.",
        ..NONE
    },
    Operation {
        tool: Item::Asm,
        label: "Forgotten server cleanup",
        description: "Take down what nobody remembers building. May evict hacktivists.",
        days: 5,
        boosts: &[(Area::Perimeter, 10)],
        evicts: &[Actor::Hacktivists],
        odds: 50,
        results: [
            "Cleanup done. \"DO-NOT-USE-final2\" is unplugged. Someone was using it.",
            "Three forgotten websites are gone. One was still taking credit cards.",
            "The 2017 marketing microsite is finally offline. Marketing held a small funeral.",
        ],
        fades: "Someone spun up new servers since the cleanup. Nobody wrote them down.",
        ..NONE
    },
    Operation {
        tool: Item::Runbooks,
        label: "Tabletop exercise",
        description: "Walk leadership through the worst day. Helps Resilience a while.",
        days: 2,
        boosts: &[(Area::Resilience, 10)],
        effect: Effect {
            trust: 2,
            ..NOTHING
        },
        results: [
            "The tabletop exercise went well. The CEO learned what ransomware is.",
            "Tabletop done. Legal and PR agreed on one thing, which is a first.",
            "The tabletop ended early when the CFO asked if they could just pay the ransom.",
        ],
        fades: "The tabletop's lessons have faded. Nobody can find the binder.",
        ..NONE
    },
    Operation {
        tool: Item::Backups,
        label: "Backup recovery drill",
        description: "Actually restore something. Softens the next ransomware attack.",
        days: 4,
        boosts: &[(Area::Resilience, 10)],
        braces: &[Actor::Ransomware],
        results: [
            "The recovery drill worked. Everything came back, eventually.",
            "Restore drill complete. The backups were fine; the instructions were not.",
            "Backup drill done. You restored the finance share and found 2016's budget.",
        ],
        fades: "It has been a while since anyone tested a restore.",
        ..NONE
    },
    Operation {
        tool: Item::DrSite,
        label: "Failover test",
        description: "Fail over to the DR site and back. Softens the next incident of any kind.",
        days: 5,
        braces: &Actor::ALL,
        mishap: Some((
            250,
            Effect {
                brand: -3,
                ..NOTHING
            },
            "The failover blipped the website for an hour. Customers noticed.",
        )),
        results: [
            "Failover test complete. Nobody noticed, which was the point.",
            "The DR site took the load. The primary data center took the afternoon off.",
            "Failover worked. The CIO now calls the DR site \"primary\".",
        ],
        ..NONE
    },
    Operation {
        tool: Item::LogCollection,
        label: "Log review",
        description: "Someone finally reads the logs. Helps Detection a while. May spot an insider.",
        days: 3,
        boosts: &[(Area::Detection, 10)],
        evicts: &[Actor::Insider],
        odds: 25,
        results: [
            "Log review done. The printer logs in as admin. That's fine. Probably.",
            "Someone read all the logs. They will never be the same.",
            "Log review finished. Most of it was the CEO forgetting their password.",
        ],
        fades: "Nobody has read the logs in weeks. They are piling up.",
        ..NONE
    },
    Operation {
        tool: Item::Siem,
        label: "Threat hunt",
        description: "Look for attackers already inside. Better with seniors and a fresh SIEM.",
        days: 6,
        evicts: &Actor::ALL,
        odds: 50,
        results: [
            "The hunt team ran a week of SIEM queries.",
            "The hunters followed every odd login back to its source.",
            "The threat hunt checked every dark corner of the network.",
        ],
        ..NONE
    },
];
