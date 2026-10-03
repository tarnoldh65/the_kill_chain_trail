use std::fmt;

use crate::attack::{Actor, Alert, Campaign, Condition, End, Response, Stage};
use crate::conference::{CONFERENCE_DAYS, Card, LEAD_CARDS, TICKET, Track};
use crate::events::{Choice, EVENTS, Effect, PET_PROJECT};
use crate::landmarks::{Landmark, Odds};
use crate::operations::{OPERATIONS, Operation};

/// The day the IPO bell is scheduled to ring, 26 weeks after day 1.
pub const IPO_DAY: u32 = 182;
/// How far each delay pushes the IPO back.
const DELAY_DAYS: u32 = 14;
/// Total delay the board tolerates before pulling the IPO.
const MAX_DELAY: u32 = 42;
/// Extra daily odds of new attackers, in thousandths, once the S-1 is public.
const FLIP_THREAT: u32 = 20;
/// Days between the board's funding releases, starting from day 1.
pub const FUNDING_INTERVAL: u32 = 42;
/// Trust at which the board releases its full quarterly grant.
const FUNDING_TRUST: i64 = 60;
/// What each analyst's conference expertise adds to their area.
const EXPERTISE_BONUS: i32 = 10;
/// What a specialist adds to their area, by level.
const JUNIOR_SKILL: i32 = 15;
const SENIOR_SKILL: i32 = 25;
/// Extra daily burnout for analysts left holding the fort during the conference.
const STAY_HOME_BURNOUT: i32 = 2;
/// Percent of list price through Procurement with the conference badge-scan discount.
const DISCOUNT_PERCENT: i64 = 80;
/// Posture below this is a weak area on the pen test report.
const WEAK_POSTURE: i32 = 40;

/// Pots of coffee the break room can hold.
pub const COFFEE_CAPACITY: i32 = 36;
/// Cost in dollars of sending the intern across the street for coffee.
pub const COFFEE_RUN_COST: i64 = 10;
/// Pots of coffee in a case from Procurement.
const CASE_POTS: i32 = 12;
/// Pots the coffee subscription delivers every Monday, and what it costs each week.
const SUBSCRIPTION_POTS: i32 = 18;
const SUBSCRIPTION_FEE: i64 = 2_000;
/// Pots of fancy coffee the intern brings back, and the burnout each analyst sheds.
const FANCY_POTS: i32 = 6;
const FANCY_RELIEF: i32 = 8;
/// Extra burnout everyone gains each day the coffee is out.
const NO_COFFEE_BURNOUT: i32 = 1;
/// Burnout a junior analyst sheds each day before their share of the workload.
const JUNIOR_REST: i32 = 1;
/// Seniors handle the same workload with less wear.
const SENIOR_REST: i32 = 2;
/// Weekly pay for each analyst level.
const JUNIOR_SALARY: i64 = 4_000;
const SENIOR_SALARY: i64 = 8_000;
/// Condition every deployed tool loses each Monday without maintenance.
const TOOL_WEAR: i32 = 8;
/// Condition below which a category's maintenance action is offered.
const MAINTAIN_BELOW: i32 = 80;
/// Days an operation's boost lasts.
const BOOST_DAYS: u32 = 28;
/// Days a search for a new analyst takes.
const SEARCH_DAYS: u32 = 7;
/// Most analysts the SOC has desks for.
pub const MAX_ANALYSTS: i32 = 8;
/// Brand loyalty below this drags the valuation down every Monday.
const BRAND_DRAG_LEVEL: i32 = 50;
/// Valuation below this percentage of the base valuation pulls the IPO.
const PULLED_PERCENT: i64 = 40;

/// Most attacker campaigns running at once.
const MAX_CAMPAIGNS: usize = 3;
/// Base days an alert investigation takes.
const INVESTIGATION_DAYS: u32 = 3;
/// What the incident response firm bills per call.
pub const IR_FEE: i64 = 25_000;
/// Weekly legal fees while regulators are asking questions.
const LEGAL_FEES: i64 = 10_000;

/// What happens to an intern who loses to traffic, rotated by week.
const INTERN_FATES: [&str; 4] = [
    "The intern was flattened by a delivery truck. The $10 and the coffee order are gone.",
    "The intern became a hood ornament. Legal says interns are unpaid, so this is technically fine.",
    "The intern was hit by a rideshare driver who then rated them one star. No coffee today.",
    "The intern crossed on \"Don't Walk\". Traffic did not stop. HR is already posting the job ad.",
];

/// Ways a burned out team member leaves, rotated so departures read differently.
const BURNOUT_EXITS: [&str; 5] = [
    "burned out and quit to open a llama sanctuary.",
    "had a heart attack when the CFO asked for \"just a quick update\". Doctors prescribed zero email.",
    "had a stroke after triaging 40,000 alerts. Doctors blame the \"informational\" ones.",
    "has died of dysentery. The break room fridge is now a crime scene.",
    "rage-quit and changed their LinkedIn headline to \"Goat Farmer\".",
];

/// A well-mixed pseudo-random number from a seed and a salt (the murmur3 finalizer).
fn roll(seed: u32, salt: u32) -> u32 {
    let mut x = seed ^ salt.wrapping_mul(0x9E37_79B9);
    x ^= x >> 16;
    x = x.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2_AE35);
    x ^ x >> 16
}

/// How a fired analyst takes the news; `{name}` is the analyst.
const LETTING_GO: [&str; 8] = [
    "{name} says \"You can't fire me, I quit!\" and storms out. HR is still deciding which happened.",
    "You tell {name} \"It's not you, it's me.\" {name} says \"No, it's definitely you\" and leaves.",
    "{name} takes the news calmly, then takes the good stapler.",
    "{name} replies-all to the whole company with a farewell essay. Legal is reading it now.",
    "{name} leaves a sticky note on their monitor: \"Please water my plant.\"",
    "{name} hands back the badge and asks if they can still come to the holiday party.",
    "{name} was already updating their resume during the meeting. They had three offers by lunch.",
    "Security walks {name} out with a box of stickers. {name} insists on keeping the stickers.",
];

/// What crosses your mind before letting someone go, shown on the confirmation.
pub const SECOND_THOUGHTS: [&str; 6] = [
    "They still owe you twenty dollars.",
    "They did bring donuts that one time.",
    "They know where all the logs are buried.",
    "HR will want a form for this. In triplicate.",
    "Their desk plant has never looked healthier.",
    "Their farewell email will be long. Very long.",
];

/// Formats dollars the way the game shows money: $10, $18K, $1.5M, $600M, $1.20B.
pub fn money(dollars: i64) -> String {
    let d = dollars as f64;
    match dollars.abs() {
        1_000_000_000.. => format!("${:.2}B", d / 1e9),
        10_000_000.. => format!("${}M", dollars / 1_000_000),
        1_000_000.. => format!("${:.1}M", d / 1e6),
        1_000.. => format!("${}K", dollars / 1_000),
        _ => format!("${dollars}"),
    }
}

/// The kind of company being protected, the game's "banker, carpenter, farmer" choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Fintech,
    Healthtech,
    Gaming,
}

impl Profile {
    pub const ALL: [Profile; 3] = [Self::Fintech, Self::Healthtech, Self::Gaming];

    pub fn budget(self) -> i64 {
        match self {
            Self::Fintech => 600_000,
            Self::Healthtech => 450_000,
            Self::Gaming => 300_000,
        }
    }

    /// What the board releases every six weeks at trust 60.
    pub fn quarterly(self) -> i64 {
        match self {
            Self::Fintech => 180_000,
            Self::Healthtech => 160_000,
            Self::Gaming => 120_000,
        }
    }

    pub fn valuation(self) -> i64 {
        match self {
            Self::Fintech => 1_500_000_000,
            Self::Healthtech => 1_000_000_000,
            Self::Gaming => 600_000_000,
        }
    }

    /// Multiplies the final score, rewarding the harder profiles.
    pub fn multiplier(self) -> i64 {
        match self {
            Self::Fintech => 1,
            Self::Healthtech => 2,
            Self::Gaming => 3,
        }
    }

    /// How likely each actor in `Actor::ALL` is to pick on this company.
    fn threats(self) -> [u32; 6] {
        match self {
            Self::Fintech => [5, 3, 2, 1, 2, 1],
            Self::Healthtech => [3, 5, 1, 1, 1, 2],
            Self::Gaming => [2, 2, 2, 5, 1, 1],
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            Self::Fintech => "Deep pockets. Money attracts money-motivated attackers.",
            Self::Healthtech => "Patient records everywhere. PII breaches hurt the most.",
            Self::Gaming => "Shoestring budget, angry teenagers, and DDoS.",
        }
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Fintech => "Fintech",
            Self::Healthtech => "Healthtech",
            Self::Gaming => "Gaming startup",
        })
    }
}

/// Everything for sale through Procurement, in shop order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Junior,
    Senior,
    PasswordManager,
    MfaTokens,
    Pam,
    Antivirus,
    Allowlisting,
    Edr,
    Posters,
    EmailGateway,
    TrainingPlatform,
    VulnScanner,
    Waf,
    Asm,
    Runbooks,
    Backups,
    DrSite,
    LogCollection,
    Siem,
    Mdr,
    IrRetainer,
    Insurance,
    Coffee,
    CoffeeSubscription,
}

impl Item {
    /// Staff, then three tools per defense area from cheapest to premium, then services.
    pub const ALL: [Item; 24] = [
        Self::Junior,
        Self::Senior,
        Self::PasswordManager,
        Self::MfaTokens,
        Self::Pam,
        Self::Antivirus,
        Self::Allowlisting,
        Self::Edr,
        Self::Posters,
        Self::EmailGateway,
        Self::TrainingPlatform,
        Self::VulnScanner,
        Self::Waf,
        Self::Asm,
        Self::Runbooks,
        Self::Backups,
        Self::DrSite,
        Self::LogCollection,
        Self::Siem,
        Self::Mdr,
        Self::IrRetainer,
        Self::Insurance,
        Self::Coffee,
        Self::CoffeeSubscription,
    ];

    /// One-time cost; for analysts this is the recruiter's fee.
    pub fn price(self) -> i64 {
        match self {
            Self::Junior => 5_000,
            Self::Senior => 10_000,
            Self::PasswordManager => 10_000,
            Self::MfaTokens => 20_000,
            Self::Pam => 70_000,
            Self::Antivirus => 10_000,
            Self::Allowlisting => 35_000,
            Self::Edr => 60_000,
            Self::Posters => 5_000,
            Self::EmailGateway => 25_000,
            Self::TrainingPlatform => 45_000,
            Self::VulnScanner => 15_000,
            Self::Waf => 40_000,
            Self::Asm => 60_000,
            Self::Runbooks => 8_000,
            Self::Backups => 30_000,
            Self::DrSite => 80_000,
            Self::LogCollection => 15_000,
            Self::Siem => 80_000,
            Self::Mdr => 100_000,
            Self::IrRetainer => 50_000,
            Self::Insurance => 40_000,
            Self::Coffee => 100,
            Self::CoffeeSubscription => 0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Junior => "Hire a junior analyst",
            Self::Senior => "Hire a senior analyst",
            Self::PasswordManager => "Password manager",
            Self::MfaTokens => "MFA tokens",
            Self::Pam => "Privileged access management",
            Self::Antivirus => "Antivirus suite",
            Self::Allowlisting => "Application allowlisting",
            Self::Edr => "EDR",
            Self::Posters => "Security awareness posters",
            Self::EmailGateway => "Email security gateway",
            Self::TrainingPlatform => "Security training platform",
            Self::VulnScanner => "Vulnerability scanner",
            Self::Waf => "DDoS protection and WAF",
            Self::Asm => "Attack surface management",
            Self::Runbooks => "Incident runbooks",
            Self::Backups => "Immutable backups",
            Self::DrSite => "Disaster recovery site",
            Self::LogCollection => "Log collection",
            Self::Siem => "SIEM",
            Self::Mdr => "Managed detection service",
            Self::IrRetainer => "Incident response retainer",
            Self::Insurance => "Cyber insurance",
            Self::Coffee => "A case of coffee (12 pots)",
            Self::CoffeeSubscription => "Coffee subscription",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Junior => "Cheap and eager. Slower, and burns out faster.",
            Self::Senior => "Expensive. Faster at everything and harder to burn out.",
            Self::PasswordManager => "Fewer sticky notes with passwords. Must be deployed.",
            Self::MfaTokens => "Protects identities. Employees will complain. Must be deployed.",
            Self::Pam => "Locks down the admin accounts attackers want. Must be deployed.",
            Self::Antivirus => "Catches the obvious malware. Must be deployed.",
            Self::Allowlisting => "Only approved software runs. Must be deployed.",
            Self::Edr => "Endpoint protection and detection. Must be deployed.",
            Self::Posters => "\"Think before you click\" on every wall. Must be deployed.",
            Self::EmailGateway => "Stops phishing at the door. Must be deployed.",
            Self::TrainingPlatform => "Monthly lessons nobody can skip. Must be deployed.",
            Self::VulnScanner => "Finds the holes before attackers do. Must be deployed.",
            Self::Waf => "Shields the perimeter from floods. Must be deployed.",
            Self::Asm => "Maps everything you forgot was online. Must be deployed.",
            Self::Runbooks => "Step-by-step plans for the worst day. Must be deployed.",
            Self::Backups => "Ransomware's worst enemy. Must be deployed.",
            Self::DrSite => "A whole second data center on standby. Must be deployed.",
            Self::LogCollection => "Gathers the logs in one place. Must be deployed.",
            Self::Siem => "Detection across the board. Must be deployed.",
            Self::Mdr => "Experts watching your alerts around the clock. Must be deployed.",
            Self::IrRetainer => "Lets you call in an incident response firm.",
            Self::Insurance => "Pays for part of the damage when things go wrong.",
            Self::Coffee => "Fuel. The break room holds 36 pots.",
            Self::CoffeeSubscription => "18 pots every Monday. Enough unless you crunch.",
        }
    }

    /// The messages a finished rollout of this tool picks from.
    pub fn rollout(self) -> [&'static str; 3] {
        match self {
            Self::PasswordManager => [
                "Password manager deployed. Marketing still keeps the master password on a sticky note.",
                "Password manager is live. Forty-one accounts used \"Summer2024!\". Now they don't.",
                "The password manager rollout is done. The vault is full; the monitor bezels are empty.",
            ],
            Self::MfaTokens => [
                "MFA is enforced everywhere. The help desk is drowning in \"I lost my phone\" tickets.",
                "MFA is live. The CFO approved 37 push prompts at 2 AM \"to make them stop.\"",
                "MFA is enforced. Sales says it adds four seconds to every login and wants a meeting about it.",
            ],
            Self::Pam => [
                "PAM is deployed. Admins must now check out their own keys like library books.",
                "PAM is live. The shared \"admin/admin\" account has been put down gently.",
                "Privileged access is locked down. The DBA calls it \"a hostage situation.\"",
            ],
            Self::Antivirus => [
                "Antivirus is on. It quarantined the CEO's solitaire and nothing else.",
                "Antivirus deployed. Every laptop fan is now spinning at full speed, all the time.",
                "Antivirus is running. It caught one virus from 2009 and is very proud of it.",
            ],
            Self::Allowlisting => [
                "Allowlisting is on. Engineering filed 212 exception requests before lunch.",
                "Only approved software runs now. Somebody's crypto miner has gone quiet.",
                "Allowlisting is deployed. The intern's \"productivity toolbar\" did not make the list.",
            ],
            Self::Edr => [
                "EDR is deployed. Every laptop now reports everything, mostly Chrome tabs.",
                "EDR is live. It immediately flagged IT's own admin scripts as malicious.",
                "EDR rollout complete. You can see every process on every machine. You wish you couldn't.",
            ],
            Self::Posters => [
                "Posters are up. Someone drew a mustache on the phishing hook.",
                "\"Think Before You Click\" now hangs in every break room. Nobody looked up from their phone.",
                "The posters are up. The one in the kitchen is already used as a coaster.",
            ],
            Self::EmailGateway => [
                "Email gateway is live. It blocked 4,000 phishes and the CEO's daughter's wedding invite.",
                "Email gateway deployed. The Nigerian prince will have to find another way in.",
                "Email gateway is on. Accounting wants to know where their \"invoices\" went.",
            ],
            Self::TrainingPlatform => [
                "Training platform launched. Employees now play the training videos on mute at 2x speed.",
                "Security training is live. The completion rate is 100%; the attention rate is unknown.",
                "Training is mandatory now. Someone already found the \"skip to quiz\" trick.",
            ],
            Self::VulnScanner => [
                "The vuln scanner is running. It found 9,000 issues and a printer from 1998.",
                "Vuln scanner deployed. The first report is 600 pages long. Nobody has read page 2.",
                "Scanning is live. It turns out the lobby TV has an open SSH port.",
            ],
            Self::Waf => [
                "The WAF is up. It blocked the attackers and, briefly, the checkout page.",
                "WAF deployed. The bots knock politely now and get turned away.",
                "The WAF is live. Marketing's landing page was flagged as SQL injection. It was close.",
            ],
            Self::Asm => [
                "ASM is deployed. It found three websites nobody remembers building.",
                "Attack surface mapped. There is a staging server named \"DO-NOT-USE-final2.\"",
                "ASM is live. It found a marketing microsite from 2017 still collecting emails.",
            ],
            Self::Runbooks => [
                "Runbooks are published. Step one in every runbook is \"Don't panic.\"",
                "Runbooks are done. They are 80 pages long and will be printed during the outage.",
                "Incident runbooks are live. The binder is labeled \"OPEN IN CASE OF FIRE.\"",
            ],
            Self::Backups => [
                "Backups are running. For the first time, someone actually tested a restore.",
                "Backups are live. The tapes are in a fireproof safe. The safe's combination is on a sticky note.",
                "Backups deployed. Ransomware gangs now have to work for their money.",
            ],
            Self::DrSite => [
                "The DR site is online. Nobody is sure which data center is \"primary\" anymore.",
                "Disaster recovery site is ready. Now you can lose two data centers instead of one.",
                "The DR site is up. It costs a fortune and hopefully does nothing at all.",
            ],
            Self::LogCollection => [
                "Logs are flowing into one place. It is mostly printer errors.",
                "Log collection is live. The storage bill arrived before the first useful alert.",
                "All the logs are centralized. You now know exactly how often the CEO forgets their password.",
            ],
            Self::Siem => [
                "The SIEM is live. It generated 10,000 alerts in its first hour.",
                "SIEM deployed. The dashboard is beautiful. Nobody knows what it means.",
                "The SIEM is up. The first correlation rule caught your own team's lunch orders.",
            ],
            Self::Mdr => [
                "MDR is watching. Someone in another time zone now judges your alerts.",
                "MDR is live. They already called at 3 AM about the vending machine.",
                "MDR is deployed. The experts are watching, and they have opinions.",
            ],
            _ => unreachable!("only tools are deployed"),
        }
    }

    /// What this costs every Monday: an analyst's salary or the coffee subscription.
    pub fn weekly(self) -> i64 {
        match self {
            Self::Junior => JUNIOR_SALARY,
            Self::Senior => SENIOR_SALARY,
            Self::CoffeeSubscription => SUBSCRIPTION_FEE,
            _ => 0,
        }
    }

    /// The defense area a tool belongs to; `None` for staff and services.
    pub fn category(self) -> Option<Area> {
        match self {
            Self::PasswordManager | Self::MfaTokens | Self::Pam => Some(Area::Identity),
            Self::Antivirus | Self::Allowlisting | Self::Edr => Some(Area::Endpoint),
            Self::Posters | Self::EmailGateway | Self::TrainingPlatform => Some(Area::People),
            Self::VulnScanner | Self::Waf | Self::Asm => Some(Area::Perimeter),
            Self::Runbooks | Self::Backups | Self::DrSite => Some(Area::Resilience),
            Self::LogCollection | Self::Siem | Self::Mdr => Some(Area::Detection),
            _ => None,
        }
    }

    /// Posture each tool adds once deployed; nothing for analysts, services, or coffee.
    pub fn boosts(self) -> &'static [(Area, i32)] {
        match self {
            Self::PasswordManager => &[(Area::Identity, 12)],
            Self::MfaTokens => &[(Area::Identity, 30)],
            Self::Pam => &[(Area::Identity, 30), (Area::Detection, 15)],
            Self::Antivirus => &[(Area::Endpoint, 12)],
            Self::Allowlisting => &[(Area::Endpoint, 25)],
            Self::Edr => &[(Area::Endpoint, 30), (Area::Detection, 15)],
            Self::Posters => &[(Area::People, 10)],
            Self::EmailGateway => &[(Area::People, 25)],
            Self::TrainingPlatform => &[(Area::People, 30)],
            Self::VulnScanner => &[(Area::Perimeter, 12), (Area::Endpoint, 5)],
            Self::Waf => &[(Area::Perimeter, 30)],
            Self::Asm => &[(Area::Perimeter, 30), (Area::Detection, 10)],
            Self::Runbooks => &[(Area::Resilience, 10)],
            Self::Backups => &[(Area::Resilience, 25)],
            Self::DrSite => &[(Area::Resilience, 35)],
            Self::LogCollection => &[(Area::Detection, 12)],
            Self::Siem => &[(Area::Detection, 30)],
            Self::Mdr => &[(Area::Detection, 35)],
            _ => &[],
        }
    }

    /// The operation this tool unlocks once deployed, if any.
    pub fn operation(self) -> Option<&'static Operation> {
        OPERATIONS.iter().find(|o| o.tool == self)
    }

    /// Analysts and coffee can be bought again; tools and services cannot.
    fn once(self) -> bool {
        !matches!(self, Self::Junior | Self::Senior | Self::Coffee)
    }
}

/// The sections of the action list, in the order they appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// Clearing what an incident left behind.
    Recovery,
    /// Rolling out tools.
    Implementation,
    /// Keeping tools at full strength.
    Maintenance,
    /// Hands-on security work.
    Operations,
    /// Looking after the team and leadership.
    Management,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Recovery => "Recovery",
            Self::Implementation => "Implementation",
            Self::Maintenance => "Maintenance",
            Self::Operations => "Operations",
            Self::Management => "Management",
        })
    }
}

/// Each area on its own, for naming the area a maintenance action restores.
const AREA_TAGS: [(Area, i32); 6] = [
    (Area::Identity, 0),
    (Area::Endpoint, 0),
    (Area::People, 0),
    (Area::Perimeter, 0),
    (Area::Resilience, 0),
    (Area::Detection, 0),
];

/// A deployed tool and how well maintained it is, from 0 to 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tool {
    pub item: Item,
    pub condition: i32,
}

impl Tool {
    pub fn new(item: Item) -> Self {
        Self {
            item,
            condition: 100,
        }
    }

    /// How the tool's condition reads on the defenses screen.
    pub fn state(self) -> &'static str {
        match self.condition {
            80.. => "fresh",
            50.. => "aging",
            _ => "stale",
        }
    }
}

/// Something the SOC spends days (and sometimes money) on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Rolls out an owned tool from Procurement.
    Deploy(Item),
    /// Runs the operation a deployed tool unlocks.
    Operate(Item),
    DayOff,
    Offsite,
    BriefLeadership,
    /// Restores every deployed tool in an area to full condition.
    Maintain(Area),
    /// Ends a lingering condition left by an incident.
    Clear(Condition),
}

impl Action {
    pub fn label(self) -> String {
        match self {
            Self::Deploy(Item::MfaTokens) => "Enforce MFA everywhere".to_string(),
            Self::Deploy(item) => format!("Deploy {}", item.label()),
            Self::Operate(item) => item.operation().unwrap().label.to_string(),
            Self::DayOff => "Give everyone the day off".to_string(),
            Self::Offsite => "Team offsite".to_string(),
            Self::BriefLeadership => "Brief leadership".to_string(),
            Self::Maintain(area) => match area {
                Area::Identity => "Run an access review",
                Area::Endpoint => "Update endpoint policies",
                Area::People => "Refresh email filters",
                Area::Perimeter => "Update firewall and WAF rules",
                Area::Resilience => "Rehearse disaster recovery",
                Area::Detection => "Tune detections",
            }
            .to_string(),
            Self::Clear(condition) => condition.cure().to_string(),
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Deploy(Item::MfaTokens) => "Big identity boost. Employees will complain.",
            Self::Deploy(_) => "Turns the tool on so it actually helps.",
            Self::Operate(item) => item.operation().unwrap().description,
            Self::DayOff => "Everyone recovers. Nobody is watching for a day.",
            Self::Offsite => "A week of trust falls. Large burnout recovery.",
            Self::BriefLeadership => "Tell the board what the SOC is doing.",
            Self::Maintain(_) => "Brings this area's tools back to full strength.",
            Self::Clear(condition) => condition.effect(),
        }
    }

    pub fn cost(self) -> i64 {
        match self {
            Self::Operate(item) => item.operation().unwrap().cost,
            Self::Offsite => 20_000,
            Self::Clear(Condition::RegulatorInquiry) => 20_000,
            Self::Clear(Condition::Downtime) => 30_000,
            _ => 0,
        }
    }

    pub fn kind(self) -> Kind {
        match self {
            Self::Clear(_) => Kind::Recovery,
            Self::Deploy(_) => Kind::Implementation,
            Self::Maintain(_) => Kind::Maintenance,
            Self::Operate(_) => Kind::Operations,
            Self::DayOff | Self::Offsite | Self::BriefLeadership => Kind::Management,
        }
    }

    /// The areas an action helps, for the IMPROVES strip: a tool's boosts, the area a
    /// maintenance action restores, or an operation's boosts.
    pub fn improves(self) -> &'static [(Area, i32)] {
        match self {
            Self::Deploy(item) => item.boosts(),
            Self::Maintain(area) => std::slice::from_ref(&AREA_TAGS[area as usize]),
            Self::Operate(item) => item.operation().unwrap().boosts,
            _ => &[],
        }
    }

    /// Days at Steady tempo with no seniors.
    fn base_days(self) -> u32 {
        match self {
            Self::Deploy(item) => match item {
                Item::Posters => 1,
                Item::PasswordManager | Item::Antivirus | Item::VulnScanner | Item::Runbooks => 2,
                Item::EmailGateway | Item::LogCollection => 3,
                Item::Waf | Item::Mdr => 4,
                Item::MfaTokens | Item::TrainingPlatform | Item::Backups => 5,
                Item::Allowlisting | Item::Asm => 6,
                Item::Edr => 7,
                Item::Pam => 8,
                _ => 10,
            },
            Self::Operate(item) => item.operation().unwrap().days,
            Self::Maintain(_) => 2,
            Self::DayOff | Self::BriefLeadership => 1,
            Self::Offsite => 5,
            Self::Clear(Condition::SystemsDown) => 7,
            Self::Clear(Condition::RegulatorInquiry) => 5,
            Self::Clear(Condition::LeakyRoadmap | Condition::Paranoia) => 4,
            Self::Clear(Condition::Downtime) => 2,
            Self::Clear(Condition::PersistentAccess) => 10,
        }
    }

    /// Whether tempo and seniors leave the duration alone.
    fn fixed(self) -> bool {
        matches!(self, Self::DayOff | Self::Offsite | Self::BriefLeadership)
    }

    /// Burnout everyone sheds each day instead of working, for rest actions.
    fn rest(self) -> Option<i32> {
        match self {
            Self::DayOff => Some(25),
            Self::Offsite => Some(10),
            _ => None,
        }
    }
}

/// A finished operation's lift to one area, until the day it wears off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boost {
    pub item: Item,
    pub area: Area,
    pub amount: i32,
    pub until: u32,
}

/// What the player can see of the company, to describe what an outcome changed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Glance {
    valuation: i64,
    trust: i32,
    brand: i32,
    budget: i64,
    coffee: i32,
    /// Average analyst burnout.
    burnout: i32,
    /// Each area's level, shown only as better or worse.
    levels: [i32; 6],
    ipo_day: u32,
    team: Vec<String>,
    conditions: Vec<Condition>,
}

/// The action in progress and how long it has left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub action: Action,
    pub days_left: u32,
    /// Average analyst burnout when the action started, to show what resting did.
    start_burnout: i32,
}

/// A week-long search for a new analyst, running in the background.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Search {
    pub name: String,
    /// Procurement's `Junior` or `Senior` listing.
    pub level: Item,
    pub days_left: u32,
}

/// An alert being looked into in the background while other work goes on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Investigation {
    /// Index into the game's alerts.
    pub alert: usize,
    pub days_left: u32,
}

/// A landmark the SOC has reached and has not moved on from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stop {
    pub landmark: Landmark,
    /// Whether the team has already rested at this fort.
    pub rested: bool,
    /// Whether the conference is over (or skipped), opening the fort.
    pub attended: bool,
}

/// A random event waiting for a choice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingEvent {
    /// Index into `EVENTS`.
    pub index: usize,
    /// Team index of the analyst the event names.
    pub patient: usize,
}

/// Ways to answer an alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reply {
    Investigate,
    CallIr,
    Ignore,
}

/// How hard the SOC is working, the game's "pace" setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tempo {
    Relaxed,
    Steady,
    Crunch,
}

impl Tempo {
    pub fn next(self) -> Self {
        match self {
            Self::Relaxed => Self::Steady,
            Self::Steady => Self::Crunch,
            Self::Crunch => Self::Relaxed,
        }
    }

    /// Burnout the analysts share each day.
    fn workload(self) -> i32 {
        match self {
            Self::Relaxed => 2,
            Self::Steady => 6,
            Self::Crunch => 12,
        }
    }

    /// Percent of an action's base duration at this tempo.
    fn duration_percent(self) -> u32 {
        match self {
            Self::Relaxed => 150,
            Self::Steady => 100,
            Self::Crunch => 67,
        }
    }

    /// Pots of coffee the SOC drinks each day.
    fn coffee(self) -> i32 {
        match self {
            Self::Relaxed => 1,
            Self::Steady => 2,
            Self::Crunch => 4,
        }
    }
}

impl fmt::Display for Tempo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Relaxed => "Relaxed",
            Self::Steady => "Steady",
            Self::Crunch => "Crunch",
        })
    }
}

/// Hidden security posture areas, shown to the player only on the pen test report. Their
/// levels come from deployed tools and the analysts on the team.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    Identity,
    Endpoint,
    People,
    Perimeter,
    Resilience,
    Detection,
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Identity => "Identity",
            Self::Endpoint => "Endpoint",
            Self::People => "People",
            Self::Perimeter => "Perimeter",
            Self::Resilience => "Resilience",
            Self::Detection => "Detection",
        })
    }
}

impl Area {
    /// The messages finished maintenance on this area picks from.
    pub fn upkeep(self) -> [&'static str; 3] {
        match self {
            Self::Identity => [
                "Access review done. Fourteen former employees still had accounts. Not anymore.",
                "Access review done. The intern from two summers ago was still a domain admin.",
                "Access review complete. Three service accounts belonged to nobody. They belong to the void now.",
            ],
            Self::Endpoint => [
                "Endpoint policies updated. Only one laptop bricked itself.",
                "Endpoint policies refreshed. Someone's \"work laptop\" was mostly running Minecraft.",
                "Endpoint policies pushed. 98% of laptops complied. The other 2% are in a drawer.",
            ],
            Self::People => [
                "Email filters refreshed. The CEO's newsletter is no longer flagged as spam.",
                "Email filters updated. The \"URGENT WIRE TRANSFER\" crowd is back in the junk folder.",
                "Filters refreshed. HR's benefits email finally reaches people's inboxes.",
            ],
            Self::Perimeter => [
                "Firewall and WAF rules updated. Someone finally closed port 23.",
                "Firewall rules cleaned up. Rule 1 was \"allow any any.\" It had a comment: \"temp.\"",
                "WAF rules updated. The bots are trying a new trick. So are you.",
            ],
            Self::Resilience => [
                "Disaster recovery rehearsed. Everything came back, eventually.",
                "DR drill complete. The restore worked. The person who knew how is on vacation.",
                "Disaster recovery rehearsed. Only the coffee machine failed to come back.",
            ],
            Self::Detection => [
                "Detections tuned. The SIEM cries wolf less often.",
                "Detections tuned. 400 alerts a day down to 40. 39 are still the printer.",
                "Detection rules tuned. The alert for \"user logged in\" has been retired with honors.",
            ],
        }
    }

    /// The attackers this area slows down.
    pub fn stops(self) -> Vec<Actor> {
        Actor::ALL
            .into_iter()
            .filter(|actor| actor.defenses().contains(&self))
            .collect()
    }

    pub const ALL: [Area; 6] = [
        Self::Identity,
        Self::Endpoint,
        Self::People,
        Self::Perimeter,
        Self::Resilience,
        Self::Detection,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Junior,
    Senior,
}

impl Role {
    /// What a specialist at this level adds to their area.
    fn skill(self) -> i32 {
        match self {
            Self::Junior => JUNIOR_SKILL,
            Self::Senior => SENIOR_SKILL,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Junior => "Junior Analyst",
            Self::Senior => "Senior Analyst",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamMember {
    pub name: String,
    pub role: Role,
    pub burnout: i32,
    /// Weekly pay from the SOC budget.
    pub salary: i64,
    /// The posture area this analyst specializes in.
    pub specialty: Option<Area>,
    /// A posture area this analyst picked up at the conference.
    pub expertise: Option<Area>,
    /// The track they are on while away at the conference.
    pub track: Option<Track>,
}

impl TeamMember {
    /// How a departure message ends: specialty and expertise leave with the person.
    fn farewell(&self) -> String {
        let specialty = self
            .specialty
            .map(|area| format!(" Their {area} specialty went with them."));
        let expertise = self
            .expertise
            .map(|area| format!(" They took their {area} expertise with them."));
        specialty.into_iter().chain(expertise).collect()
    }
}

/// One line of the game log and the day it happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub day: u32,
    pub text: String,
}

/// Lets a log entry be read as its text.
impl std::ops::Deref for Entry {
    type Target = str;

    fn deref(&self) -> &str {
        &self.text
    }
}

/// A card shown to the player: the outcome of something they did, or a conference report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Revealed {
    pub title: String,
    pub text: String,
    pub effect: String,
    /// The attendee's track, or `None` for the lead's card.
    pub track: Option<Track>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Ipo,
    Pulled,
    ShutDown,
    Fired,
    TeamCollapsed,
    Bankrupt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub company: String,
    pub lead: String,
    pub profile: Profile,
    /// Day 1 is a Monday; the game ends as an IPO on `ipo_day`.
    pub day: u32,
    /// `IPO_DAY` plus any delays.
    pub ipo_day: u32,
    /// When each landmark in `Landmark::ALL` comes up, moved back by delays.
    pub landmark_days: [u32; Landmark::ALL.len()],
    /// Index of the next landmark to reach.
    pub next_landmark: usize,
    pub stop: Option<Stop>,
    /// Total days the IPO has slipped.
    delay: u32,
    /// Incidents left out of the S-1, waiting to surface.
    hidden: u32,
    /// The pen test's grades and the day they were given, kept for the defenses screen.
    pub pen_test: Option<(u32, [(Area, char); 6])>,
    /// Whether a leak has already forced an amended S-1 (it only happens once).
    amended: bool,
    /// Days left at the conference.
    pub conference_days: u32,
    /// Conference cards waiting to be revealed.
    pub cards: Vec<Revealed>,
    /// Badge-scan discount at Procurement during this conference.
    discount: bool,
    /// A senior analyst from the after-party, hired with no fee.
    free_senior: bool,
    pub base_valuation: i64,
    pub valuation: i64,
    pub trust: i32,
    pub brand: i32,
    pub budget: i64,
    pub coffee: i32,
    pub tempo: Tempo,
    /// Added to each area's level, indexed by `Area`, so tests can set posture directly.
    #[cfg(test)]
    bonus: [i32; Area::ALL.len()],
    pub team: Vec<TeamMember>,
    /// Tools and services bought through Procurement. Tools need deploying before they help.
    pub owned: Vec<Item>,
    pub deployed: Vec<Tool>,
    /// Operation boosts still in effect.
    pub boosts: Vec<Boost>,
    /// Attackers whose next incident does half the damage, one charge each.
    pub braced: Vec<Actor>,
    pub task: Option<Task>,
    pub investigation: Option<Investigation>,
    /// Analysts being recruited; they join when their search ends.
    pub searches: Vec<Search>,
    pub conditions: Vec<Condition>,
    /// Hidden from the player until the after-action report.
    campaigns: Vec<Campaign>,
    alerts: Vec<Alert>,
    /// The alert waiting for a reply, as an index into `alerts`.
    alert: Option<usize>,
    /// The incident waiting for a response.
    pub incident: Option<Actor>,
    pub event: Option<PendingEvent>,
    /// Extra daily odds of new attackers, in thousandths, from publicity.
    threat: u32,
    /// Days of thin holiday coverage left.
    holiday: u32,
    pub log: Vec<Entry>,
    pub outcome: Option<Outcome>,
    /// Whether the intern has already been sent for coffee this week.
    pub coffee_run_made: bool,
    /// Drives every random outcome, so a seed always plays out the same way.
    pub seed: u32,
    /// How many random numbers this game has drawn.
    rolls: u32,
}

impl GameState {
    /// A new game with no analysts yet; hire them through Procurement.
    pub fn new(company: &str, lead: &str, profile: Profile, seed: u32) -> Self {
        Self {
            company: company.to_string(),
            lead: lead.to_string(),
            profile,
            day: 1,
            ipo_day: IPO_DAY,
            landmark_days: Landmark::ALL.map(Landmark::day),
            next_landmark: 0,
            stop: None,
            delay: 0,
            hidden: 0,
            pen_test: None,
            amended: false,
            conference_days: 0,
            cards: Vec::new(),
            discount: false,
            free_senior: false,
            base_valuation: profile.valuation(),
            valuation: profile.valuation(),
            trust: 60,
            brand: 60,
            budget: profile.budget(),
            coffee: 12,
            tempo: Tempo::Steady,
            #[cfg(test)]
            bonus: [0; Area::ALL.len()],
            team: Vec::new(),
            owned: Vec::new(),
            deployed: Vec::new(),
            boosts: Vec::new(),
            braced: Vec::new(),
            task: None,
            investigation: None,
            searches: Vec::new(),
            conditions: Vec::new(),
            campaigns: Vec::new(),
            alerts: Vec::new(),
            alert: None,
            incident: None,
            event: None,
            threat: 0,
            holiday: 0,
            log: vec![Entry {
                day: 1,
                text: format!(
                    "{company} goes public in 26 weeks. {lead} takes command of the SOC."
                ),
            }],
            outcome: None,
            coffee_run_made: false,
            seed,
            rolls: 0,
        }
    }

    /// Whether an analyst can be let go: here, not the last one, with a week's salary
    /// on hand for severance.
    pub fn can_fire(&self, index: usize) -> bool {
        self.team
            .get(index)
            .is_some_and(|m| m.track.is_none() && self.analysts() > 1 && self.affords(m.salary))
    }

    /// Lets an analyst go with a week's salary as severance.
    pub fn fire(&mut self, index: usize) {
        let before = self.glance();
        let analyst = self.team.remove(index);
        self.budget -= analyst.salary;
        let exit = LETTING_GO[self.chance() as usize % LETTING_GO.len()];
        let message = exit.replace("{name}", &analyst.name) + &analyst.farewell();
        self.note(message.clone());
        self.card("Let go", &message, &before);
    }

    /// Adds a line to the log, stamped with today's date.
    fn note(&mut self, text: impl Into<String>) {
        self.log.push(Entry {
            day: self.day,
            text: text.into(),
        });
    }

    /// The next number in this game's random sequence, from 0 to 999.
    fn chance(&mut self) -> u32 {
        self.rolls += 1;
        roll(self.seed, self.rolls) % 1000
    }

    fn seniors(&self) -> u32 {
        self.team.iter().filter(|m| m.role == Role::Senior).count() as u32
    }

    /// Whether the budget covers a cost; free things are always possible, even in debt.
    fn affords(&self, cost: i64) -> bool {
        cost == 0 || cost <= self.budget
    }

    /// What an item costs right now, after any conference perks.
    pub fn price(&self, item: Item) -> i64 {
        if item == Item::Senior && self.free_senior {
            0
        } else if self.discount {
            item.price() * DISCOUNT_PERCENT / 100
        } else {
            item.price()
        }
    }

    pub fn can_buy(&self, item: Item) -> bool {
        self.affords(self.price(item))
            && !(item.once() && self.owned.contains(&item))
            && match item {
                Item::Junior | Item::Senior => {
                    self.analysts() + (self.searches.len() as i32) < MAX_ANALYSTS
                        && !self.conditions.contains(&Condition::Paranoia)
                }
                Item::Coffee => !self.coffee_full(),
                _ => true,
            }
    }

    /// Buys a tool, service, or coffee. Analysts are hired by name with `hire`.
    pub fn buy(&mut self, item: Item) {
        self.budget -= self.price(item);
        match item {
            Item::Coffee => self.coffee = (self.coffee + CASE_POTS).min(COFFEE_CAPACITY),
            Item::Junior | Item::Senior => unreachable!("analysts are hired by name"),
            _ => self.owned.push(item),
        }
    }

    /// Hires an analyst from Procurement's `Junior` or `Senior` listing.
    /// Hires an analyst: at once on day 1 and at the conference job fair, otherwise after
    /// a week-long search. The recruiter's fee is paid now either way.
    pub fn hire(&mut self, item: Item, name: &str) {
        self.budget -= self.price(item);
        if item == Item::Senior {
            self.free_senior = false;
        }
        if self.hires_now() {
            self.add_analyst(item, name);
        } else {
            self.searches.push(Search {
                name: name.to_string(),
                level: item,
                days_left: SEARCH_DAYS,
            });
            self.note(format!(
                "The search for a new analyst begins. {name} should start in a week."
            ));
        }
    }

    /// Whether hiring is instant: on day 1, or at the conference job fair.
    pub fn hires_now(&self) -> bool {
        self.day == 1
            || self
                .stop
                .as_ref()
                .is_some_and(|s| s.landmark == Landmark::Conference && s.attended)
    }

    /// Counts down every search and welcomes the analysts whose searches end today.
    fn recruit(&mut self) -> bool {
        for search in &mut self.searches {
            search.days_left -= 1;
        }
        let (done, waiting) = self.searches.drain(..).partition(|s| s.days_left == 0);
        self.searches = waiting;
        let found = !done.is_empty();
        for search in done {
            let before = self.glance();
            self.add_analyst(search.level, &search.name);
            let message = format!(
                "{} the {} joins the SOC.",
                search.name,
                self.team.last().unwrap().role
            );
            self.note(message.clone());
            self.card("New hire", &message, &before);
        }
        found
    }

    fn add_analyst(&mut self, item: Item, name: &str) {
        let role = match item {
            Item::Junior => Role::Junior,
            Item::Senior => Role::Senior,
            _ => unreachable!("only analysts are hired"),
        };
        self.team.push(TeamMember {
            name: name.to_string(),
            role,
            burnout: 0,
            salary: item.weekly(),
            specialty: None,
            expertise: None,
            track: None,
        });
    }

    pub fn is_deployed(&self, item: Item) -> bool {
        self.deployed.iter().any(|t| t.item == item)
    }

    /// Deployed tools by their condition, the specialty and expertise of analysts on the
    /// team, and operation boosts. Nothing else holds posture, so nothing else drifts.
    fn level(&self, area: Area) -> i32 {
        let skill: i32 = self
            .team
            .iter()
            .map(|m| {
                let specialty = if m.specialty == Some(area) {
                    m.role.skill()
                } else {
                    0
                };
                let expertise = if m.expertise == Some(area) {
                    EXPERTISE_BONUS
                } else {
                    0
                };
                specialty + expertise
            })
            .sum();
        let tools: i32 = self
            .deployed
            .iter()
            .flat_map(|t| {
                t.item
                    .boosts()
                    .iter()
                    .map(move |&(a, b)| (a, b * t.condition / 100))
            })
            .filter(|&(a, _)| a == area)
            .map(|(_, b)| b)
            .sum();
        let boosts: i32 = self
            .boosts
            .iter()
            .filter(|b| b.area == area)
            .map(|b| b.amount)
            .sum();
        #[cfg(test)]
        let tools = tools + self.bonus[area as usize];
        (tools + skill + boosts).min(100)
    }

    /// Actions the SOC can start now, grouped by kind: prerequisites met, affordable, and
    /// nothing else under way.
    pub fn actions(&self) -> Vec<Action> {
        if self.task.is_some() || self.outcome.is_some() {
            return Vec::new();
        }
        let cures = self.conditions.iter().map(|&c| Action::Clear(c));
        if self.conditions.contains(&Condition::SystemsDown) {
            return cures
                .chain([Action::DayOff, Action::BriefLeadership])
                .filter(|action| self.affords(action.cost()))
                .collect();
        }
        let deployable = self
            .owned
            .iter()
            .filter(|item| item.category().is_some())
            .filter(|&&item| !self.is_deployed(item))
            .map(|&item| Action::Deploy(item));
        let upkeep = Area::ALL
            .into_iter()
            .filter(|&area| {
                self.deployed
                    .iter()
                    .any(|t| t.item.category() == Some(area) && t.condition < MAINTAIN_BELOW)
            })
            .map(Action::Maintain);
        let operations = Item::ALL
            .into_iter()
            .filter(|&item| item.operation().is_some() && self.is_deployed(item))
            .map(Action::Operate);
        cures
            .chain(deployable)
            .chain(upkeep)
            .chain(operations)
            .chain([Action::DayOff, Action::Offsite, Action::BriefLeadership])
            .filter(|action| self.affords(action.cost()))
            .collect()
    }

    /// Days an action takes now: tempo stretches or shrinks it, and every two seniors save a day.
    pub fn duration(&self, action: Action) -> u32 {
        if action.fixed() {
            return action.base_days();
        }
        self.scaled(action.base_days())
    }

    fn scaled(&self, base_days: u32) -> u32 {
        let days = (base_days * self.tempo.duration_percent()).div_ceil(100);
        days.saturating_sub(self.seniors() / 2).max(1)
    }

    /// Pays for an action and puts it under way.
    pub fn start(&mut self, action: Action) {
        self.budget -= action.cost();
        let days = self.duration(action);
        self.note(format!("Started: {} ({days} days).", action.label()));
        self.task = Some(Task {
            action,
            days_left: days,
            start_burnout: self.glance().burnout,
        });
    }

    fn glance(&self) -> Glance {
        Glance {
            valuation: self.valuation,
            trust: self.trust,
            brand: self.brand,
            budget: self.budget,
            coffee: self.coffee,
            burnout: self.team.iter().map(|m| m.burnout).sum::<i32>()
                / self.team.len().max(1) as i32,
            levels: Area::ALL.map(|a| self.level(a)),
            ipo_day: self.ipo_day,
            team: self.team.iter().map(|m| m.name.clone()).collect(),
            conditions: self.conditions.clone(),
        }
    }

    /// Describes what visibly changed since `before`, like "Trust +2. Resilience improved."
    fn changes(&self, before: &Glance) -> String {
        let after = self.glance();
        let signed = |change: i64| {
            if change > 0 {
                format!("+{change}")
            } else {
                change.to_string()
            }
        };
        let mut parts = Vec::new();
        let valuation = after.valuation - before.valuation;
        if valuation != 0 {
            let sign = if valuation > 0 { "+" } else { "-" };
            parts.push(format!("Valuation {sign}{}", money(valuation.abs())));
        }
        for (label, old, new) in [
            ("Trust", before.trust, after.trust),
            ("Brand", before.brand, after.brand),
            ("Team burnout", before.burnout, after.burnout),
            ("Coffee", before.coffee, after.coffee),
        ] {
            if new != old {
                parts.push(format!("{label} {}", signed((new - old) as i64)));
            }
        }
        let budget = after.budget - before.budget;
        if budget != 0 {
            let sign = if budget > 0 { "+" } else { "-" };
            parts.push(format!("Budget {sign}{}", money(budget.abs())));
        }
        for (i, area) in Area::ALL.iter().enumerate() {
            match after.levels[i].cmp(&before.levels[i]) {
                std::cmp::Ordering::Greater => parts.push(format!("{area} improved")),
                std::cmp::Ordering::Less => parts.push(format!("{area} weakened")),
                std::cmp::Ordering::Equal => {}
            }
        }
        if after.ipo_day > before.ipo_day {
            parts.push(format!("IPO delayed to day {}", after.ipo_day));
        }
        for name in after.team.iter().filter(|n| !before.team.contains(n)) {
            parts.push(format!("{name} joins the team"));
        }
        for name in before.team.iter().filter(|n| !after.team.contains(n)) {
            parts.push(format!("{name} leaves the team"));
        }
        for condition in after
            .conditions
            .iter()
            .filter(|c| !before.conditions.contains(c))
        {
            parts.push(format!("{} begins", condition.tag()));
        }
        for condition in before
            .conditions
            .iter()
            .filter(|c| !after.conditions.contains(c))
        {
            parts.push(format!("{} cleared", condition.tag()));
        }
        if parts.is_empty() {
            "No visible change.".to_string()
        } else {
            parts.join(". ") + "."
        }
    }

    /// Queues a result card for something that has just been logged.
    fn card(&mut self, title: &str, text: &str, before: &Glance) {
        let effect = self.changes(before);
        self.cards.push(Revealed {
            title: title.to_uppercase(),
            text: text.to_string(),
            effect,
            track: None,
        });
    }

    /// Adds to the condition of every deployed tool in an area.
    fn wear(&mut self, area: Area, change: i32) {
        for tool in &mut self.deployed {
            if tool.item.category() == Some(area) {
                tool.condition = (tool.condition + change).clamp(0, 100);
            }
        }
    }

    /// Applies a finished action's effects.
    fn finish(&mut self, task: Task) {
        let action = task.action;
        let mut before = self.glance();
        if action.rest().is_some() {
            before.burnout = task.start_burnout;
        }
        let message = match task.action {
            Action::Deploy(item) => {
                self.deployed.push(Tool::new(item));
                if item == Item::MfaTokens {
                    self.trust -= 3;
                }
                item.rollout()[self.chance() as usize % 3].to_string()
            }
            Action::Operate(item) => self.operate(item),
            Action::DayOff => "The team is back from a day off, slightly less haunted.".to_string(),
            Action::Offsite => {
                "The team offsite is over. Trust falls were had. Nobody was dropped.".to_string()
            }
            Action::BriefLeadership => {
                self.trust += 4;
                "You briefed leadership. The board nodded at all the right moments.".to_string()
            }
            Action::Maintain(area) => {
                self.wear(area, 100);
                area.upkeep()[self.chance() as usize % 3].to_string()
            }
            Action::Clear(condition) => {
                self.conditions.retain(|&c| c != condition);
                format!("{} is done. Things are back to normal.", condition.cure())
            }
        };
        self.trust = self.trust.clamp(0, 100);
        self.note(message.clone());
        self.card(&action.label(), &message, &before);
        // A third of briefings end with the CIO pitching a pet project.
        if action == Action::BriefLeadership && self.chance() < 333 {
            self.trigger(PET_PROJECT);
        }
    }

    /// Finishes a tool's operation: boosts, evictions, and braces scaled by the tool's
    /// condition, plus its effects or, sometimes, its mishap.
    fn operate(&mut self, item: Item) -> String {
        let op = item.operation().unwrap();
        let condition = self
            .deployed
            .iter()
            .find(|t| t.item == item)
            .map_or(0, |t| t.condition);
        self.boosts.retain(|b| b.item != item);
        for &(area, amount) in op.boosts {
            self.boosts.push(Boost {
                item,
                area,
                amount: amount * condition / 100,
                until: self.day + BOOST_DAYS,
            });
        }
        for &actor in op.braces {
            if !self.braced.contains(&actor) {
                self.braced.push(actor);
            }
        }
        self.apply(&op.effect, usize::MAX);
        let mut message = match op.mishap {
            Some((odds, effect, text)) if self.chance() < odds => {
                self.apply(&effect, usize::MAX);
                text.to_string()
            }
            _ => op.results[self.chance() as usize % 3].to_string(),
        };
        if !op.evicts.is_empty() {
            let found = self.hunt(op.evicts, op.odds * condition as u32 / 100);
            message += &if found.is_empty() {
                " They found nothing. Either you are clean or they are good.".to_string()
            } else {
                format!(" They found and evicted: {}.", found.join(", "))
            };
        }
        message
    }

    /// Drops the operation boosts that have run out, saying which ones faded.
    fn expire(&mut self) {
        let day = self.day;
        let mut faded: Vec<Item> = Vec::new();
        for boost in self.boosts.iter().filter(|b| b.until <= day) {
            if !faded.contains(&boost.item) {
                faded.push(boost.item);
            }
        }
        self.boosts.retain(|b| b.until > day);
        for item in faded {
            self.note(item.operation().unwrap().fades);
        }
    }

    /// Sets off a random event: applied at once, or left waiting for a choice.
    fn trigger(&mut self, index: usize) {
        if self.team.is_empty() {
            return;
        }
        let patient = self.chance() as usize % self.team.len();
        let event = EVENTS[index];
        self.note(event.text.replace("{name}", &self.team[patient].name));
        if event.choices.is_empty() {
            self.apply(&event.effect, patient);
        } else {
            self.event = Some(PendingEvent { index, patient });
        }
    }

    fn apply(&mut self, effect: &Effect, patient: usize) {
        self.trust = (self.trust + effect.trust).clamp(0, 100);
        self.brand = (self.brand + effect.brand).clamp(0, 100);
        self.budget += effect.budget;
        self.valuation += self.base_valuation * effect.valuation / 1000;
        for member in self.team.iter_mut() {
            member.burnout = (member.burnout + effect.burnout).max(0);
        }
        if let Some(patient) = self.team.get_mut(patient) {
            patient.burnout = (patient.burnout + effect.patient).max(0);
        }
        for &(area, change) in effect.wear {
            self.wear(area, change);
        }
        self.coffee = (self.coffee + effect.coffee).clamp(0, COFFEE_CAPACITY);
        self.threat += effect.threat;
        self.holiday = self.holiday.max(effect.holiday);
        if effect.delay {
            self.slip();
        }
        if effect.intruder {
            self.campaigns.push(Campaign {
                actor: Actor::Apt,
                start: self.day,
                stage: Stage::InitialAccess,
                seen: false,
                end: None,
            });
        }
    }

    /// Pushes the IPO and every landmark still ahead back two weeks, or pulls the IPO.
    fn slip(&mut self) {
        self.delay += DELAY_DAYS;
        if self.delay > MAX_DELAY {
            self.note(
                "Too many delays. The board pulls the IPO \"until market conditions improve\"."
                    .to_string(),
            );
            self.outcome = Some(Outcome::Pulled);
            return;
        }
        self.ipo_day += DELAY_DAYS;
        for day in &mut self.landmark_days[self.next_landmark..] {
            *day += DELAY_DAYS;
        }
        self.valuation -= self.base_valuation * 20 / 1000;
        self.trust = (self.trust - 5).max(0);
        self.note(format!(
            "The IPO slips two weeks, to day {}. Investors grumble.",
            self.ipo_day
        ));
    }

    fn incidents(&self) -> u32 {
        self.campaigns
            .iter()
            .filter(|c| c.end == Some(End::Succeeded))
            .count() as u32
    }

    /// Chance in thousandths of making it across a river.
    pub fn crossing_odds(&self, odds: &Odds) -> u32 {
        let average = Area::ALL.iter().map(|&a| self.level(a)).sum::<i32>() as u32 / 6;
        (odds.base + odds.posture * average / 100)
            .saturating_sub(odds.per_incident * self.incidents())
            .min(1000)
    }

    /// Letter grades for each posture area, as the pen testers see it.
    pub fn report_card(&self) -> [(Area, char); 6] {
        Area::ALL.map(|area| {
            let grade = match self.level(area) {
                80.. => 'A',
                60.. => 'B',
                40.. => 'C',
                20.. => 'D',
                _ => 'F',
            };
            (area, grade)
        })
    }

    /// A hidden S-1 incident comes out half the time it gets a chance to.
    fn surface(&mut self) {
        if self.hidden == 0 || self.chance() >= 500 {
            return;
        }
        self.valuation -= self.base_valuation * 60 * self.hidden as i64 / 1000;
        self.trust = (self.trust - 10).max(0);
        self.hidden = 0;
        self.note(
            "A journalist found the incidents you left out of the S-1. The SEC would like a word."
                .to_string(),
        );
    }

    /// Reaches the next landmark if today is its day.
    fn arrive(&mut self) -> bool {
        if self.landmark_days.get(self.next_landmark) != Some(&self.day) {
            return false;
        }
        let landmark = Landmark::ALL[self.next_landmark];
        self.next_landmark += 1;
        self.note(format!("You have reached the {landmark}."));
        match landmark {
            Landmark::PenTest => self.pen_test = Some((self.day, self.report_card())),
            Landmark::Flip => {
                self.threat += FLIP_THREAT;
                self.surface();
            }
            Landmark::Roadshow => {
                self.surface();
                if self.conditions.contains(&Condition::SystemsDown) {
                    self.note("You cannot pitch investors while systems are down.".to_string());
                    self.slip();
                }
            }
            _ => {}
        }
        if self.outcome.is_none() {
            self.stop = Some(Stop {
                landmark,
                rested: false,
                attended: false,
            });
        }
        true
    }

    pub fn can_cross(&self, index: usize) -> bool {
        self.stop
            .as_ref()
            .and_then(|s| s.landmark.crossings().get(index))
            .is_some_and(|c| self.affords(c.cost))
    }

    /// Crosses the river at the current landmark one of its ways.
    pub fn cross(&mut self, index: usize) {
        let landmark = self.stop.take().expect("a river to cross").landmark;
        let crossing = landmark.crossings()[index];
        let before = self.glance();
        self.budget -= crossing.cost;
        let (text, effect) = if self.chance() < self.crossing_odds(&crossing.odds) {
            crossing.success
        } else {
            crossing.failure
        };
        self.note(text.to_string());
        self.apply(&effect, usize::MAX);
        match landmark {
            Landmark::S1Filing if index == 0 => {
                self.valuation -= self.base_valuation * 15 * self.incidents() as i64 / 1000;
            }
            Landmark::S1Filing => self.hidden = self.incidents(),
            Landmark::PenTest => {
                let weak = Area::ALL
                    .iter()
                    .filter(|&&a| self.level(a) < WEAK_POSTURE)
                    .count() as i64;
                let cost = self.base_valuation * 10 * weak / 1000 / (index as i64 + 1);
                if cost > 0 {
                    self.valuation -= cost;
                    self.note(format!(
                        "{weak} weak areas in the report cost {} in valuation.",
                        money(cost)
                    ));
                }
            }
            _ => {}
        }
        self.card(&landmark.to_string(), text, &before);
        self.check_outcome();
    }

    /// Rests the team at a fort, once per visit, without time passing.
    pub fn rest_at_fort(&mut self) {
        let Some(stop) = &mut self.stop else {
            return;
        };
        if stop.rested {
            return;
        }
        stop.rested = true;
        for member in &mut self.team {
            member.burnout = (member.burnout - 15).max(0);
        }
        self.note("The team rests. Nobody checks chat for a whole afternoon.".to_string());
    }

    pub fn leave_fort(&mut self) {
        self.stop = None;
        self.discount = false;
        self.free_senior = false;
    }

    /// Ticket and travel for the lead plus `analysts` attendees.
    pub fn conference_cost(&self, analysts: usize) -> i64 {
        TICKET * (analysts as i64 + 1)
    }

    /// Opens the conference fort without anyone going.
    pub fn skip_conference(&mut self) {
        if let Some(stop) = &mut self.stop {
            stop.attended = true;
        }
    }

    /// Sends the lead and the picked analysts (team index and track) to the conference.
    pub fn attend(&mut self, picks: &[(usize, Track)]) {
        self.budget -= self.conference_cost(picks.len());
        for &(i, track) in picks {
            self.team[i].track = Some(track);
        }
        self.conference_days = CONFERENCE_DAYS;
        self.stop = None;
        self.note(format!(
            "You and {} analysts head to the conference. The rest hold the fort.",
            picks.len()
        ));
    }

    /// Counts down the conference and brings everyone back with their cards on the last day.
    fn conferencing(&mut self) -> bool {
        if self.conference_days == 0 {
            return false;
        }
        self.conference_days -= 1;
        if self.conference_days > 0 {
            return false;
        }
        self.return_from_conference();
        true
    }

    fn return_from_conference(&mut self) {
        let mut used: Vec<&str> = Vec::new();
        for i in 0..self.team.len() {
            let Some(track) = self.team[i].track.take() else {
                continue;
            };
            let mut pool: Vec<Card> = track
                .cards()
                .iter()
                .filter(|c| !used.contains(&c.text))
                .copied()
                .collect();
            if pool.is_empty() {
                pool = Track::ALL
                    .iter()
                    .flat_map(|t| t.cards())
                    .filter(|c| !used.contains(&c.text))
                    .copied()
                    .collect();
            }
            let card = pool[self.chance() as usize % pool.len()];
            used.push(card.text);
            let member = &mut self.team[i];
            member.burnout = (member.burnout - card.relief).max(0);
            if card.expertise.is_some() {
                member.expertise = card.expertise;
            }
            let effect = match card.expertise {
                Some(area) => format!("{area} expertise, burnout -{}", card.relief),
                None => format!("Burnout -{}", card.relief),
            };
            let text = card.text.replace("{name}", &member.name);
            self.note(text.clone());
            self.cards.push(Revealed {
                title: "CONFERENCE REPORT".to_string(),
                text,
                effect,
                track: Some(track),
            });
        }

        let lead = LEAD_CARDS[self.chance() as usize % LEAD_CARDS.len()];
        self.trust = (self.trust + lead.trust).clamp(0, 100);
        self.discount = lead.discount;
        self.free_senior = lead.recruit;
        let effect = if lead.trust != 0 {
            format!("Trust +{}", lead.trust)
        } else if lead.discount {
            "20% off through Procurement".to_string()
        } else if lead.recruit {
            "A senior hire with no fee through Procurement".to_string()
        } else {
            "No effect".to_string()
        };
        self.note(lead.text.to_string());
        self.cards.push(Revealed {
            title: "CONFERENCE REPORT".to_string(),
            text: lead.text.to_string(),
            effect,
            track: None,
        });
        self.stop = Some(Stop {
            landmark: Landmark::Conference,
            rested: false,
            attended: true,
        });
    }

    /// Shows the next conference card, removing it from the queue.
    pub fn reveal_next(&mut self) {
        if !self.cards.is_empty() {
            self.cards.remove(0);
        }
    }

    /// The choices for the event waiting on the player, if any.
    pub fn event_choices(&self) -> &'static [Choice] {
        self.event.as_ref().map_or(&[], |e| EVENTS[e.index].choices)
    }

    pub fn choose(&mut self, index: usize) {
        let event = self.event.take().expect("an event to choose for");
        let choice = EVENTS[event.index].choices[index];
        let before = self.glance();
        self.apply(&choice.effect, event.patient);
        self.note(choice.text.to_string());
        self.card(choice.label, choice.text, &before);
        self.check_outcome();
    }

    /// A random event on about one day in forty, when nothing else is waiting.
    fn maybe_event(&mut self) -> bool {
        let busy = self.alert.is_some() || self.incident.is_some() || self.event.is_some();
        if busy || self.analysts() == 0 || self.chance() >= 25 {
            return false;
        }
        let index = self.chance() as usize % EVENTS.len();
        self.trigger(index);
        true
    }

    fn spawn_odds(&self) -> u32 {
        15 + 45 * self.day / IPO_DAY + self.threat
    }

    /// Counts down the action in progress and finishes it on its last day.
    fn progress(&mut self) -> bool {
        let Some(task) = &mut self.task else {
            return false;
        };
        task.days_left -= 1;
        if task.days_left > 0 {
            return false;
        }
        let task = self.task.take().unwrap();
        self.finish(task);
        true
    }

    /// Chance in thousandths that a hunt or investigation finds a real attacker.
    fn find_odds(&self, base: u32) -> u32 {
        (base + 15 * self.seniors()).min(90) * 10
    }

    /// Looks for running campaigns by `actors`, evicting each one found; returns who.
    fn hunt(&mut self, actors: &[Actor], base: u32) -> Vec<String> {
        let mut found = Vec::new();
        for i in 0..self.campaigns.len() {
            let campaign = &self.campaigns[i];
            if campaign.end.is_none()
                && actors.contains(&campaign.actor)
                && self.chance() < self.find_odds(base)
            {
                let campaign = &mut self.campaigns[i];
                campaign.end = Some(End::Evicted);
                campaign.seen = true;
                found.push(campaign.actor.to_string());
            }
        }
        found
    }

    /// The alert waiting for a reply, if any.
    pub fn pending_alert(&self) -> Option<&str> {
        self.alert.map(|i| self.alerts[i].text)
    }

    pub fn can_reply(&self, reply: Reply) -> bool {
        match reply {
            Reply::Investigate => self.investigation.is_none(),
            Reply::CallIr => self.owned.contains(&Item::IrRetainer) && self.budget >= IR_FEE,
            Reply::Ignore => true,
        }
    }

    pub fn investigation_days(&self) -> u32 {
        self.scaled(INVESTIGATION_DAYS)
    }

    pub fn reply(&mut self, reply: Reply) {
        let Some(alert) = self.alert.take() else {
            return;
        };
        let before = self.glance();
        let message = match reply {
            Reply::Investigate => {
                let days = self.investigation_days();
                self.investigation = Some(Investigation {
                    alert,
                    days_left: days,
                });
                format!("The team starts digging into it ({days} days).")
            }
            Reply::CallIr => {
                self.budget -= IR_FEE;
                if self.chance() < 950 && self.evict(alert) {
                    format!(
                        "The IR firm found {} inside and threw them out.",
                        self.alert_actor(alert)
                    )
                } else {
                    format!("The IR firm billed {} and found nothing.", money(IR_FEE))
                }
            }
            Reply::Ignore => "You ignore it. Probably nothing.".to_string(),
        };
        if reply == Reply::CallIr {
            self.card("Incident response firm", &message, &before);
        }
        self.note(message);
    }

    fn alert_actor(&self, alert: usize) -> Actor {
        let campaign = self.alerts[alert].campaign.expect("a real alert");
        self.campaigns[campaign].actor
    }

    /// Ends the campaign behind an alert, if it is real and still running.
    fn evict(&mut self, alert: usize) -> bool {
        match self.alerts[alert].campaign {
            Some(i) if self.campaigns[i].end.is_none() => {
                self.campaigns[i].end = Some(End::Evicted);
                true
            }
            _ => false,
        }
    }

    /// Counts down the investigation and reports what it found on its last day.
    fn investigate(&mut self) -> bool {
        let Some(investigation) = &mut self.investigation else {
            return false;
        };
        investigation.days_left -= 1;
        if investigation.days_left > 0 {
            return false;
        }
        let alert = self.investigation.take().unwrap().alert;
        let before = self.glance();
        let message = if self.chance() < self.find_odds(50) && self.evict(alert) {
            format!(
                "Investigation complete: it was a {}. They have been evicted.",
                self.alert_actor(alert)
            )
        } else {
            "Investigation complete: nothing conclusive.".to_string()
        };
        self.note(message.clone());
        self.card("Investigation", &message, &before);
        true
    }

    /// Whether a response is possible: backups deployed if needed, and the money for it.
    pub fn can_respond(&self, response: &Response) -> bool {
        (!response.needs_backups || self.is_deployed(Item::Backups))
            && self.affords(self.incident_cost(response))
    }

    /// What a response costs after insurance pays its half.
    pub fn incident_cost(&self, response: &Response) -> i64 {
        if self.owned.contains(&Item::Insurance) {
            response.budget / 2
        } else {
            response.budget
        }
    }

    /// Answers the pending incident with one of its responses; Resilience softens the damage.
    pub fn respond(&mut self, index: usize) {
        let actor = self.incident.take().expect("an incident to respond to");
        let response = actor.responses()[index];
        let before = self.glance();
        let mut scale = 200 - self.level(Area::Resilience) as i64;
        let mut text = response.text.to_string();
        if let Some(i) = self.braced.iter().position(|&a| a == actor) {
            self.braced.remove(i);
            scale /= 2;
            text += " The drills paid off: the damage is half what it could have been.";
        }
        self.valuation -= self.base_valuation * response.valuation * scale / 200_000;
        self.trust = (self.trust + response.trust * scale as i32 / 200).clamp(0, 100);
        self.brand = (self.brand + response.brand * scale as i32 / 200).clamp(0, 100);
        self.budget -= self.incident_cost(&response);
        let condition = actor.condition();
        if response.lingers && !self.conditions.contains(&condition) {
            self.conditions.push(condition);
        }
        self.note(text.clone());
        let flipped = self.next_landmark > Landmark::Flip as usize;
        if flipped && !self.amended && matches!(actor, Actor::DataThief | Actor::Insider) {
            self.amended = true;
            self.note("The leak forces an amended S-1.".to_string());
            self.slip();
        }
        self.card(response.label, &text, &before);
        self.check_outcome();
    }

    /// Effective Detection: lower while the team is resting.
    fn watch(&self) -> i32 {
        let mut detection = self.level(Area::Detection);
        if self.holiday > 0 {
            detection /= 2;
        }
        let analysts = self.analysts();
        if self.conference_days > 0 && analysts > 0 {
            let home = self.team.iter().filter(|m| m.track.is_none()).count() as i32;
            detection = detection * home / analysts;
        }
        match self.task.as_ref().map(|t| t.action) {
            Some(Action::DayOff) => detection / 4,
            Some(Action::Offsite) => detection / 2,
            _ => detection,
        }
    }

    /// Average posture across the areas that defend against an actor.
    fn defense(&self, actor: Actor) -> u32 {
        let areas = actor.defenses();
        let total: i32 = areas.iter().map(|&a| self.level(a)).sum();
        (total / areas.len() as i32) as u32
    }

    fn pick_actor(&mut self) -> Actor {
        let weights = self.profile.threats();
        let mut pick = self.chance() % weights.iter().sum::<u32>();
        for (actor, weight) in Actor::ALL.into_iter().zip(weights) {
            if pick < weight {
                return actor;
            }
            pick -= weight;
        }
        unreachable!("the pick is below the total weight")
    }

    /// Raises an alert unless one is already waiting; `campaign` is `None` for a false alarm.
    fn raise(&mut self, campaign: Option<usize>) -> bool {
        if self.alert.is_some() {
            return false;
        }
        let actor = match campaign {
            Some(i) => self.campaigns[i].actor,
            None => Actor::ALL[self.chance() as usize % Actor::ALL.len()],
        };
        let text = actor.alerts()[self.chance() as usize % 2];
        self.alerts.push(Alert {
            day: self.day,
            text,
            campaign,
        });
        self.alert = Some(self.alerts.len() - 1);
        self.note(format!("ALERT: {text}"));
        true
    }

    /// One day of attacker activity: new campaigns, kill chain steps, and false alarms.
    fn attack(&mut self) -> bool {
        let mut stop = false;
        let active = self.campaigns.iter().filter(|c| c.end.is_none()).count();
        if active < MAX_CAMPAIGNS && self.chance() < self.spawn_odds() {
            let actor = self.pick_actor();
            let stage = if self.conditions.contains(&Condition::PersistentAccess) {
                Stage::Foothold
            } else {
                actor.first_stage()
            };
            self.campaigns.push(Campaign {
                actor,
                start: self.day,
                stage,
                seen: false,
                end: None,
            });
        }

        for i in 0..self.campaigns.len() {
            if self.campaigns[i].end.is_some() {
                continue;
            }
            let actor = self.campaigns[i].actor;
            let defense = self.defense(actor);
            let roll = self.chance();
            if roll < 20 + 50 * (100 - defense) / 100 {
                let next = self.campaigns[i].stage.next();
                if next == Stage::Objective {
                    // One incident at a time; the attacker waits for the next opening.
                    if self.incident.is_some() {
                        continue;
                    }
                    self.campaigns[i].stage = next;
                    self.campaigns[i].end = Some(End::Succeeded);
                    self.incident = Some(actor);
                    self.note(format!("INCIDENT: {}", actor.incident()));
                    stop = true;
                } else {
                    self.campaigns[i].stage = next;
                    let detect = 10 + self.watch() as u32 * 7 / 10;
                    if self.chance() < detect * 10 && self.raise(Some(i)) {
                        self.campaigns[i].seen = true;
                        stop = true;
                    }
                }
            } else if roll >= 1000 - defense / 4 {
                self.campaigns[i].end = Some(End::GaveUp);
            }
        }

        // A well-kept SIEM is quiet; a neglected one cries wolf more than no SIEM at all.
        let noise = match self.deployed.iter().find(|t| t.item == Item::Siem) {
            None => 30,
            Some(siem) => 15 + 35 * (100 - siem.condition) as u32 / 100,
        };
        if self.chance() < noise {
            stop |= self.raise(None);
        }
        stop
    }

    /// Daily damage from lingering conditions.
    fn linger(&mut self) {
        self.holiday = self.holiday.saturating_sub(1);
        if self.conditions.contains(&Condition::SystemsDown) {
            self.brand -= 1;
        }
        if self.conditions.contains(&Condition::Downtime) {
            self.brand -= 2;
        }
        self.brand = self.brand.max(0);
    }

    /// Raises a false alarm, for testing screens that wait on an alert.
    #[cfg(test)]
    pub fn raise_false_alarm(&mut self) {
        self.raise(None);
    }

    /// What is helping an area: deployed tools, tools still waiting to be deployed, and
    /// analysts with expertise in it.
    pub fn defenders(&self, area: Area) -> Vec<String> {
        let tools = self
            .owned
            .iter()
            .filter(|item| item.boosts().iter().any(|&(a, _)| a == area))
            .map(
                |&item| match self.deployed.iter().find(|t| t.item == item) {
                    Some(tool) => format!("{} ({})", item.label(), tool.state()),
                    None => format!("{} (not deployed)", item.label()),
                },
            );
        let specialists = self
            .team
            .iter()
            .filter(|m| m.specialty == Some(area))
            .map(|m| format!("{}'s specialty", m.name));
        let experts = self
            .team
            .iter()
            .filter(|m| m.expertise == Some(area))
            .map(|m| format!("{}'s expertise", m.name));
        tools.chain(specialists).chain(experts).collect()
    }

    /// What really happened, shown only once the game is over.
    pub fn after_action(&self) -> Vec<String> {
        if self.outcome.is_none() {
            return Vec::new();
        }
        if self.campaigns.is_empty() && self.alerts.is_empty() {
            return vec!["No attackers came calling. Suspiciously lucky.".to_string()];
        }
        let campaigns = self.campaigns.iter().map(|c| {
            let end = match c.end {
                None => "still at it",
                Some(End::GaveUp) => "gave up",
                Some(End::Evicted) => "evicted",
                Some(End::Succeeded) => "succeeded",
            };
            let seen = if c.seen { "seen" } else { "unseen" };
            format!(
                "Day {}: {} reached {}, {seen}, {end}.",
                c.start, c.actor, c.stage
            )
        });
        let alerts = self.alerts.iter().map(|a| {
            let kind = if a.campaign.is_some() {
                "REAL "
            } else {
                "FALSE"
            };
            format!("Day {} {kind} {}", a.day, a.text)
        });
        let none = |empty: bool, text: &str| empty.then(|| text.to_string());
        std::iter::once("ATTACKERS".to_string())
            .chain(campaigns)
            .chain(none(
                self.campaigns.is_empty(),
                "None. Every alert was noise.",
            ))
            .chain([String::new(), "ALERTS".to_string()])
            .chain(alerts)
            .chain(none(self.alerts.is_empty(), "None. They were never seen."))
            .collect()
    }

    pub fn analysts(&self) -> i32 {
        self.team.len() as i32
    }

    /// Days since the last Monday: 0 is Monday.
    pub fn weekday(&self) -> u32 {
        (self.day - 1) % 7
    }

    /// The board's grant at today's trust: the full quarterly amount at trust 60.
    pub fn grant(&self) -> i64 {
        self.profile.quarterly() * self.trust as i64 / FUNDING_TRUST
    }

    /// The board's funding days still ahead, before IPO day.
    fn funding_days(&self) -> impl Iterator<Item = u32> {
        let (day, ipo_day) = (self.day, self.ipo_day);
        (1..)
            .map(|n| 1 + n * FUNDING_INTERVAL)
            .take_while(move |&d| d < ipo_day)
            .filter(move |&d| d > day)
    }

    /// The next day the board releases funding, if one comes before the IPO.
    pub fn next_funding(&self) -> Option<u32> {
        self.funding_days().next()
    }

    fn fund(&mut self) {
        let before = self.glance();
        let grant = self.grant();
        self.budget += grant;
        let message = format!(
            "The board releases {} for the quarter. Trust at {} earned {}% of the grant.",
            money(grant),
            self.trust,
            self.trust as i64 * 100 / FUNDING_TRUST
        );
        self.note(message.clone());
        self.card("Board funding", &message, &before);
    }

    pub fn days_to_ipo(&self) -> u32 {
        self.ipo_day - self.day
    }

    pub fn payroll(&self) -> i64 {
        self.team.iter().map(|m| m.salary).sum()
    }

    /// Everything that comes out of the budget each Monday: payroll and the coffee subscription.
    pub fn weekly_costs(&self) -> i64 {
        self.payroll() + self.owned.iter().map(|i| i.weekly()).sum::<i64>()
    }

    /// The final score, only for reaching the IPO: valuation in millions times the profile multiplier.
    pub fn score(&self) -> Option<i64> {
        (self.outcome == Some(Outcome::Ipo))
            .then_some(self.valuation / 1_000_000 * self.profile.multiplier())
    }

    pub fn coffee_full(&self) -> bool {
        self.coffee >= COFFEE_CAPACITY
    }

    pub fn can_send_intern(&self) -> bool {
        self.outcome.is_none() && !self.coffee_run_made && self.budget >= COFFEE_RUN_COST
    }

    /// Pays for a coffee run; the intern still has to survive the street.
    pub fn send_intern(&mut self) {
        self.budget -= COFFEE_RUN_COST;
        self.coffee_run_made = true;
    }

    /// The intern's fancy coffee: a few pots and a lift for every analyst, if they survive.
    pub fn intern_returns(&mut self, survived: bool) {
        let before = self.glance();
        let message = if survived {
            self.coffee = (self.coffee + FANCY_POTS).min(COFFEE_CAPACITY);
            for member in self.team.iter_mut() {
                member.burnout = (member.burnout - FANCY_RELIEF).max(0);
            }
            "The intern made it back with oat milk lattes for everyone and only minor tire marks."
                .to_string()
        } else {
            let week = (self.day - 1) / 7;
            INTERN_FATES[week as usize % INTERN_FATES.len()].to_string()
        };
        self.note(message.clone());
        self.card("Coffee run", &message, &before);
    }

    /// Advances one day and returns whether something happened that should stop the clock.
    pub fn advance(&mut self) -> bool {
        self.day += 1;
        let mut stop = false;
        if self.weekday() == 0 {
            stop |= self.monday();
        }
        if self.outcome.is_none() {
            stop |= self.drink_coffee();
            stop |= self.work();
            stop |= self.progress();
            stop |= self.investigate();
            stop |= self.recruit();
            stop |= self.conferencing();
            self.expire();
            self.linger();
            stop |= self.attack();
            stop |= self.maybe_event();
            self.check_outcome();
        }
        if self.outcome.is_none() {
            stop |= self.arrive();
        }
        stop || self.outcome.is_some()
    }

    /// Weekly upkeep: tool wear, brand drag, and payroll.
    fn monday(&mut self) -> bool {
        self.coffee_run_made = false;
        if (self.day - 1).is_multiple_of(FUNDING_INTERVAL) {
            self.fund();
        }
        for tool in &mut self.deployed {
            tool.condition = (tool.condition - TOOL_WEAR).max(0);
        }
        if self.brand < BRAND_DRAG_LEVEL {
            let drag = self.base_valuation * (BRAND_DRAG_LEVEL - self.brand) as i64 / 1000;
            self.valuation -= drag;
            self.note(format!(
                "Weak brand loyalty knocked {} off the valuation.",
                money(drag)
            ));
        }

        if self.conditions.contains(&Condition::RegulatorInquiry) {
            self.budget -= LEGAL_FEES;
            self.note(format!(
                "The regulators' questions cost {} in legal fees this week.",
                money(LEGAL_FEES)
            ));
        }
        if self.conditions.contains(&Condition::LeakyRoadmap) {
            let leak = self.base_valuation * 5 / 1000;
            self.valuation -= leak;
            self.note(format!(
                "More of the roadmap leaked. The valuation slips {}.",
                money(leak)
            ));
        }

        let mut stop = false;
        while self.payroll() > self.budget && self.analysts() > 1 {
            let (i, _) = self
                .team
                .iter()
                .enumerate()
                .max_by_key(|(_, m)| m.salary)
                .unwrap();
            let analyst = self.team.remove(i);
            self.note(format!(
                "Payroll came up short. {} the {} was laid off and walked out holding a cardboard box.{}",
                analyst.name,
                analyst.role,
                analyst.farewell()
            ));
            stop = true;
        }
        if self.payroll() > self.budget {
            self.outcome = Some(Outcome::Bankrupt);
            return true;
        }
        self.budget -= self.payroll();
        self.note(format!("Payday: {} in salaries.", money(self.payroll())));
        if self.owned.contains(&Item::CoffeeSubscription) {
            if self.budget >= SUBSCRIPTION_FEE {
                self.budget -= SUBSCRIPTION_FEE;
                self.coffee = (self.coffee + SUBSCRIPTION_POTS).min(COFFEE_CAPACITY);
                self.note(format!(
                    "The coffee subscription delivered {SUBSCRIPTION_POTS} pots for {}.",
                    money(SUBSCRIPTION_FEE)
                ));
            } else {
                self.note("The coffee subscription payment bounced. No delivery this week.");
            }
        }
        stop
    }

    fn drink_coffee(&mut self) -> bool {
        let had_coffee = self.coffee > 0;
        self.coffee = (self.coffee - self.tempo.coffee()).max(0);
        if self.coffee > 0 {
            return false;
        }
        for member in &mut self.team {
            member.burnout += NO_COFFEE_BURNOUT;
        }
        if had_coffee {
            self.note("The coffee ran out. Tempers are short.".to_string());
        }
        had_coffee
    }

    /// Analysts share the day's workload, or everyone rests; anyone fully burned out leaves.
    fn work(&mut self) -> bool {
        let analysts = self.analysts();
        if let Some(rest) = self.task.as_ref().and_then(|t| t.action.rest()) {
            for member in &mut self.team {
                member.burnout = (member.burnout - rest).max(0);
            }
        } else if analysts > 0 {
            // Analysts away at the conference leave the workload to the ones at home.
            let home = self.team.iter().filter(|m| m.track.is_none()).count() as i32;
            let paranoia = self.conditions.contains(&Condition::Paranoia) as i32;
            let stay_home = if self.conference_days > 0 {
                STAY_HOME_BURNOUT
            } else {
                0
            };
            // Weekends are quieter: half the workload on Saturday and Sunday.
            let weekend = if self.weekday() >= 5 { 2 } else { 1 };
            let share = self.tempo.workload() / weekend / home.max(1) + paranoia + stay_home;
            for member in self.team.iter_mut().filter(|m| m.track.is_none()) {
                let rest = match member.role {
                    Role::Junior => JUNIOR_REST,
                    Role::Senior => SENIOR_REST,
                };
                member.burnout = (member.burnout + share - rest).max(0);
            }
        }

        let mut exit = self.day as usize;
        let (log, day) = (&mut self.log, self.day);
        let before = self.team.len();
        self.team.retain(|m| {
            if m.burnout >= 100 {
                let reason = BURNOUT_EXITS[exit % BURNOUT_EXITS.len()];
                log.push(Entry {
                    day,
                    text: format!("{} the {} {reason}{}", m.name, m.role, m.farewell()),
                });
                exit += 1;
            }
            m.burnout < 100
        });
        self.team.len() < before
    }

    fn check_outcome(&mut self) {
        if self.outcome == Some(Outcome::Pulled) {
            return;
        }
        self.outcome = if self.trust <= 0 {
            Some(Outcome::Fired)
        } else if self.brand <= 0 {
            Some(Outcome::ShutDown)
        } else if self.valuation < self.base_valuation * PULLED_PERCENT / 100 {
            Some(Outcome::Pulled)
        } else if self.analysts() == 0 {
            Some(Outcome::TeamCollapsed)
        } else if self.day >= self.ipo_day {
            Some(Outcome::Ipo)
        } else {
            None
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(name: &str, role: Role, burnout: i32, salary: i64) -> TeamMember {
        TeamMember {
            name: name.to_string(),
            role,
            burnout,
            salary,
            specialty: None,
            expertise: None,
            track: None,
        }
    }

    /// A Healthtech game with three junior analysts already hired.
    fn game() -> GameState {
        GameState {
            budget: 500_000,
            coffee: 24,
            team: vec![
                member("Maya", Role::Junior, 20, 6_000),
                member("Dev", Role::Junior, 15, 6_000),
                member("Sam", Role::Junior, 10, 6_000),
            ],
            ..GameState::new("Acme", "Alex", Profile::Healthtech, 1)
        }
    }

    /// Advances until the given day, keeping the coffee topped up.
    fn advance_to(game: &mut GameState, day: u32) {
        while game.day < day && game.outcome.is_none() {
            game.coffee = COFFEE_CAPACITY;
            game.advance();
        }
    }

    /// Each analyst's burnout change and the coffee drunk over one quiet day.
    fn one_day(tempo: Tempo, analysts: usize) -> (i32, i32) {
        let mut game = game();
        game.day = 2;
        game.tempo = tempo;
        game.coffee = COFFEE_CAPACITY;
        game.team = (0..analysts)
            .map(|_| member("A", Role::Junior, 50, 0))
            .collect();
        game.advance();
        (game.team[0].burnout - 50, COFFEE_CAPACITY - game.coffee)
    }

    #[test]
    fn new_game_starts_on_day_one_with_leadership_and_no_analysts() {
        let game = GameState::new("Acme", "Alex", Profile::Fintech, 1);

        assert_eq!(game.day, 1);
        assert_eq!(game.weekday(), 0);
        assert_eq!(game.days_to_ipo(), IPO_DAY - 1);
        assert!(game.team.is_empty(), "hire analysts through Procurement");
        assert!(game.owned.is_empty());
        assert!(game.log[0].contains("Acme") && game.log[0].contains("Alex"));
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn each_profile_sets_its_budget_valuation_and_multiplier() {
        let expected = [
            (Profile::Fintech, 600_000, 1_500_000_000, 1),
            (Profile::Healthtech, 450_000, 1_000_000_000, 2),
            (Profile::Gaming, 300_000, 600_000_000, 3),
        ];
        for (profile, budget, valuation, multiplier) in expected {
            let mut game = GameState::new("Acme", "Alex", profile, 1);

            assert_eq!(game.budget, budget);
            assert_eq!(
                (game.base_valuation, game.valuation),
                (valuation, valuation)
            );

            game.outcome = Some(Outcome::Ipo);
            assert_eq!(game.score(), Some(valuation / 1_000_000 * multiplier));
        }
    }

    #[test]
    fn purchases_cannot_exceed_the_budget() {
        let mut game = game();
        for item in Item::ALL.into_iter().filter(|i| i.price() > 0) {
            game.budget = item.price() - 1;
            assert!(!game.can_buy(item), "{item:?}");
            game.budget = item.price();
            assert!(game.can_buy(item), "{item:?}");
        }
    }

    #[test]
    fn tools_and_services_are_bought_once_and_kept() {
        let mut game = game();
        for item in &Item::ALL[2..10] {
            game.buy(*item);
            assert!(!game.can_buy(*item), "{item:?}");
        }

        assert_eq!(game.owned, &Item::ALL[2..10]);
        let total: i64 = Item::ALL[2..10].iter().map(|i| i.price()).sum();
        assert_eq!(game.budget, 500_000 - total);
    }

    #[test]
    fn bought_coffee_stops_at_capacity() {
        let mut game = game();
        game.coffee = COFFEE_CAPACITY - 5;

        assert!(game.can_buy(Item::Coffee));
        game.buy(Item::Coffee);

        assert_eq!(game.coffee, COFFEE_CAPACITY);
        assert!(!game.can_buy(Item::Coffee));
    }

    #[test]
    fn hired_analysts_keep_their_names_levels_and_salaries() {
        let mut game = GameState::new("Acme", "Alex", Profile::Gaming, 1);

        game.hire(Item::Junior, "Priya");
        game.hire(Item::Senior, "Marcus");

        assert_eq!(game.analysts(), 2);
        assert_eq!(
            game.team[0],
            member("Priya", Role::Junior, 0, JUNIOR_SALARY)
        );
        assert_eq!(
            game.team[1],
            member("Marcus", Role::Senior, 0, SENIOR_SALARY)
        );
        assert_eq!(game.payroll(), JUNIOR_SALARY + SENIOR_SALARY);
        let start = Profile::Gaming.budget();
        assert_eq!(
            game.budget,
            start - Item::Junior.price() - Item::Senior.price()
        );

        game.day = 7;
        game.advance();
        assert_eq!(
            game.budget,
            start - Item::Junior.price() - Item::Senior.price() - game.payroll()
        );
    }

    #[test]
    fn the_soc_has_desks_for_a_limited_number_of_analysts() {
        let mut game = GameState::new("Acme", "Alex", Profile::Fintech, 1);
        for _ in 0..MAX_ANALYSTS {
            game.hire(Item::Junior, "A");
        }

        assert!(!game.can_buy(Item::Junior));
        assert!(!game.can_buy(Item::Senior));
    }

    #[test]
    fn owned_tools_do_not_change_posture() {
        let mut game = game();

        for item in &Item::ALL[2..10] {
            game.buy(*item);
        }

        assert_eq!(Area::ALL.map(|a| game.level(a)), [0; 6]);
    }

    #[test]
    fn seniors_burn_out_slower_than_juniors() {
        let mut game = game();
        game.day = 2;
        game.team = vec![
            member("Jo", Role::Junior, 50, 0),
            member("Sr", Role::Senior, 50, 0),
        ];

        game.advance();

        assert!(game.team[1].burnout < game.team[0].burnout);
    }

    #[test]
    fn advancing_moves_exactly_one_day() {
        let mut game = game();

        game.advance();
        assert_eq!(game.day, 2);
        game.advance();
        assert_eq!(game.day, 3);
    }

    #[test]
    fn a_quiet_game_ends_as_an_ipo_on_ipo_day() {
        let mut game = game();
        game.tempo = Tempo::Relaxed;

        advance_to(&mut game, IPO_DAY - 1);
        assert_eq!(game.outcome, None);
        assert_eq!(game.score(), None);

        assert!(game.advance());
        assert_eq!(game.day, IPO_DAY);
        assert_eq!(game.outcome, Some(Outcome::Ipo));
        assert_eq!(game.score(), Some(1_000 * 2));
    }

    #[test]
    fn payroll_is_taken_only_on_mondays() {
        let mut game = game();
        let mut paydays = Vec::new();

        while game.day < 22 {
            let budget = game.budget;
            game.advance();
            if game.budget != budget {
                paydays.push(game.day);
                assert_eq!(budget - game.budget, 18_000);
            }
        }

        assert_eq!(paydays, [8, 15, 22]);
        assert!(
            game.log
                .iter()
                .any(|e| e.text == "Payday: $18K in salaries.")
        );
    }

    #[test]
    fn a_short_budget_lays_off_the_most_expensive_analyst() {
        let mut game = game();
        game.day = 7;
        game.team[1].salary = 9_000;
        game.budget = 13_000;

        assert!(game.advance());

        assert_eq!(game.analysts(), 2);
        assert!(game.team.iter().all(|m| m.name != "Dev"));
        assert_eq!(game.budget, 1_000);
        assert!(
            game.log
                .iter()
                .any(|e| e.contains("Dev the Junior Analyst was laid off"))
        );
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn a_budget_that_cannot_pay_one_analyst_is_bankrupt() {
        let mut game = game();
        game.day = 7;
        game.budget = 5_000;

        assert!(game.advance());

        assert_eq!(game.analysts(), 1);
        assert_eq!(game.outcome, Some(Outcome::Bankrupt));
        assert_eq!(game.score(), None);
    }

    #[test]
    fn harder_tempos_burn_out_analysts_and_drink_coffee_faster() {
        let (relaxed, steady, crunch) = (
            one_day(Tempo::Relaxed, 3),
            one_day(Tempo::Steady, 3),
            one_day(Tempo::Crunch, 3),
        );

        assert!(relaxed.0 < 0, "Relaxed recovers");
        assert!(relaxed.0 < steady.0 && steady.0 < crunch.0);
        assert!(relaxed.1 < steady.1 && steady.1 < crunch.1);
    }

    #[test]
    fn more_analysts_share_the_burnout() {
        for tempo in [Tempo::Steady, Tempo::Crunch] {
            assert!(one_day(tempo, 6).0 < one_day(tempo, 3).0);
        }
    }

    #[test]
    fn tempo_cycles_through_all_three() {
        assert_eq!(Tempo::Relaxed.next(), Tempo::Steady);
        assert_eq!(Tempo::Steady.next(), Tempo::Crunch);
        assert_eq!(Tempo::Crunch.next(), Tempo::Relaxed);
    }

    #[test]
    fn running_out_of_coffee_burns_everyone_and_stops_the_clock() {
        let mut game = game();
        game.coffee = 1;

        assert!(game.advance());
        assert_eq!(game.coffee, 0);
        assert_eq!(
            game.team[2].burnout,
            10 + NO_COFFEE_BURNOUT + 1,
            "and a Steady day"
        );
        assert!(game.log.iter().any(|e| e.contains("coffee ran out")));

        assert!(!game.advance(), "only running out stops the clock");
        assert_eq!(game.team[2].burnout, 10 + 2 * NO_COFFEE_BURNOUT + 2);
    }

    #[test]
    fn posture_starts_at_zero_and_drifts_only_through_tool_wear() {
        let mut game = game();
        assert_eq!(Area::ALL.map(|a| game.level(a)), [0; 6]);

        game.team[0].specialty = Some(Area::Identity);
        game.team[1].expertise = Some(Area::People);
        game.deployed.push(Tool::new(Item::Waf));
        let levels = Area::ALL.map(|a| game.level(a));
        assert_eq!(levels, [JUNIOR_SKILL, 0, EXPERTISE_BONUS, 30, 0, 0]);

        advance_to(&mut game, 8);
        assert_eq!(
            Area::ALL.map(|a| game.level(a)),
            [
                JUNIOR_SKILL,
                0,
                EXPERTISE_BONUS,
                30 * (100 - TOOL_WEAR) / 100,
                0,
                0
            ],
            "only the tool wore"
        );
    }

    #[test]
    fn weak_brand_drags_the_valuation_on_mondays() {
        let mut game = game();
        game.brand = 30;

        advance_to(&mut game, 7);
        assert_eq!(game.valuation, 1_000_000_000);
        advance_to(&mut game, 8);

        assert_eq!(game.valuation, 1_000_000_000 - 20_000_000);
        assert!(game.log.iter().any(|e| e.contains("knocked $20M off")));
    }

    #[test]
    fn burned_out_members_leave_with_a_message() {
        let mut game = game();
        game.day = 4;
        game.team[0].burnout = 99;

        assert!(game.advance());

        assert_eq!(game.analysts(), 2);
        assert!(
            game.log.iter().any(|e| e.text
                == "Maya the Junior Analyst burned out and quit to open a llama sanctuary.")
        );
    }

    #[test]
    fn departures_rotate_through_burnout_exits() {
        let mut game = game();
        game.team[0].burnout = 99;
        game.team[1].burnout = 99;

        game.advance();

        assert!(
            game.log
                .iter()
                .any(|e| e.starts_with("Maya the Junior Analyst had a stroke"))
        );
        assert!(
            game.log
                .iter()
                .any(|e| e.starts_with("Dev the Junior Analyst has died of dysentery"))
        );
    }

    #[test]
    fn each_ending_has_its_trigger_and_only_an_ipo_scores() {
        for outcome in [
            Outcome::Fired,
            Outcome::ShutDown,
            Outcome::Pulled,
            Outcome::TeamCollapsed,
            Outcome::Ipo,
        ] {
            let mut game = game();
            match outcome {
                Outcome::Fired => game.trust = 0,
                Outcome::ShutDown => game.brand = 0,
                Outcome::Pulled => game.valuation = 399_999_999,
                Outcome::TeamCollapsed => game.team.iter_mut().for_each(|m| m.burnout = 99),
                Outcome::Ipo => game.day = IPO_DAY - 1,
                Outcome::Bankrupt => unreachable!("tested with payroll"),
            }

            assert!(game.advance());

            assert_eq!(game.outcome, Some(outcome));
            assert_eq!(game.score().is_some(), outcome == Outcome::Ipo);
        }
    }

    #[test]
    fn a_coffee_run_costs_money_once_per_week() {
        let mut game = game();

        game.send_intern();

        assert_eq!(game.budget, 500_000 - COFFEE_RUN_COST);
        assert!(!game.can_send_intern());
        advance_to(&mut game, 7);
        game.coffee = 0;
        assert!(!game.can_send_intern());
        advance_to(&mut game, 8);
        game.coffee = 0;
        assert!(game.can_send_intern());
        game.budget = COFFEE_RUN_COST - 1;
        assert!(!game.can_send_intern());
    }

    #[test]
    fn a_surviving_intern_restocks_the_coffee_up_to_capacity() {
        let mut game = game();

        game.intern_returns(true);
        assert_eq!(game.coffee, 24 + FANCY_POTS);
        assert_eq!(
            game.team[0].burnout,
            20 - FANCY_RELIEF,
            "lattes lift the analysts"
        );
        assert_eq!(game.cards.last().unwrap().title, "COFFEE RUN");
        assert!(game.log.last().unwrap().contains("made it back"));

        game.intern_returns(true);
        assert_eq!(game.coffee, COFFEE_CAPACITY);
    }

    #[test]
    fn the_intern_goes_even_when_the_break_room_is_full() {
        let mut game = game();
        game.coffee = COFFEE_CAPACITY;

        assert!(game.can_send_intern(), "fancy coffee is about morale");
    }

    #[test]
    fn the_coffee_subscription_delivers_every_monday_for_a_weekly_fee() {
        let mut without = game();
        without.day = 7;
        without.coffee = 2;
        without.advance();
        assert_eq!(without.coffee, 0, "no delivery without the subscription");

        let mut game = game();
        assert_eq!(game.price(Item::CoffeeSubscription), 0, "nothing up front");
        game.buy(Item::CoffeeSubscription);
        assert!(!game.can_buy(Item::CoffeeSubscription), "bought once");
        assert_eq!(game.budget, 500_000);
        assert_eq!(game.weekly_costs(), 18_000 + SUBSCRIPTION_FEE);
        assert!(
            !game
                .actions()
                .contains(&Action::Deploy(Item::CoffeeSubscription)),
            "nothing to deploy"
        );
        game.day = 7;
        game.coffee = 2;
        game.advance();
        assert_eq!(
            game.coffee,
            2 + SUBSCRIPTION_POTS - 2,
            "delivered, then a Steady day"
        );
        assert_eq!(game.budget, 500_000 - 18_000 - SUBSCRIPTION_FEE);
        assert!(
            game.log
                .iter()
                .any(|e| e.contains("coffee subscription delivered"))
        );

        game.day = 14;
        game.coffee = COFFEE_CAPACITY - 4;
        game.advance();
        assert_eq!(game.coffee, COFFEE_CAPACITY - 2, "never past capacity");

        game.day = 21;
        game.coffee = 2;
        game.budget = 18_000 + SUBSCRIPTION_FEE - 1;
        game.advance();
        assert_eq!(game.coffee, 0, "no money, no delivery");
        assert!(game.log.iter().any(|e| e.contains("bounced")));
    }

    #[test]
    fn a_flattened_intern_brings_no_coffee() {
        let mut game = game();

        game.intern_returns(false);

        assert_eq!(game.coffee, 24);
        assert!(game.log.last().unwrap().contains("delivery truck"));
    }

    /// Starts an action and advances until it finishes, keeping the coffee topped up.
    const PATCH: Action = Action::Operate(Item::VulnScanner);
    const PHISH: Action = Action::Operate(Item::TrainingPlatform);
    const TABLETOP: Action = Action::Operate(Item::Runbooks);
    const HUNT: Action = Action::Operate(Item::Siem);

    /// `game()` with the tools behind the patch sprint, phishing simulation, and tabletop.
    fn equipped() -> GameState {
        GameState {
            deployed: [Item::VulnScanner, Item::TrainingPlatform, Item::Runbooks]
                .map(Tool::new)
                .to_vec(),
            ..game()
        }
    }

    fn run(game: &mut GameState, action: Action) {
        game.start(action);
        while game.task.is_some() {
            game.coffee = COFFEE_CAPACITY;
            game.advance();
        }
    }

    #[test]
    fn only_owned_undeployed_tools_can_be_deployed() {
        let mut game = game();
        assert!(
            !game
                .actions()
                .iter()
                .any(|a| matches!(a, Action::Deploy(_)))
        );

        game.owned = vec![Item::Siem, Item::IrRetainer, Item::Waf, Item::Insurance];
        let deploys: Vec<_> = game
            .actions()
            .into_iter()
            .filter(|a| matches!(a, Action::Deploy(_)))
            .collect();
        assert_eq!(
            deploys,
            [Action::Deploy(Item::Siem), Action::Deploy(Item::Waf)]
        );

        run(&mut game, Action::Deploy(Item::Siem));
        assert!(!game.actions().contains(&Action::Deploy(Item::Siem)));
        assert_eq!(game.deployed, [Tool::new(Item::Siem)]);
    }

    #[test]
    fn enforcing_mfa_annoys_employees() {
        let mut game = game();
        game.owned.push(Item::MfaTokens);

        run(&mut game, Action::Deploy(Item::MfaTokens));

        assert_eq!(game.trust, 60 - 3);
    }

    #[test]
    fn each_tool_rollout_picks_one_of_its_own_messages() {
        for item in Item::ALL.into_iter().filter(|i| i.category().is_some()) {
            let mut game = game();
            game.owned.push(item);
            run(&mut game, Action::Deploy(item));
            assert!(
                item.rollout()
                    .contains(&game.log.last().unwrap().text.as_str())
            );
        }
    }

    #[test]
    fn each_action_applies_its_cost_duration_and_effects() {
        let mut game = equipped();

        let start = game.clone();
        run(&mut game, PATCH);
        assert_eq!(game.day, start.day + 5);
        assert_eq!(game.level(Area::Perimeter), 12 + 15);
        assert_eq!(game.level(Area::Endpoint), 5 + 10);
        assert!(game.team[0].burnout > start.team[0].burnout + 4, "tiring");

        let mut game = start.clone();
        run(&mut game, PHISH);
        assert_eq!(game.day, start.day + 4);
        assert_eq!(game.budget, start.budget - 5_000);
        assert_eq!(game.level(Area::People), 30 + 20);

        let mut game = start.clone();
        run(&mut game, TABLETOP);
        assert_eq!(game.day, start.day + 2);
        assert_eq!(game.level(Area::Resilience), 10 + 10);
        assert_eq!(game.trust, 62);

        let mut game = start.clone();
        run(&mut game, Action::BriefLeadership);
        assert_eq!(game.day, start.day + 1);
        assert_eq!(game.trust, 64);

        let mut game = start.clone();
        run(&mut game, Action::DayOff);
        assert_eq!(game.day, start.day + 1);
        assert_eq!(game.team[0].burnout, 0);
        assert!(game.log.last().unwrap().contains("day off"));

        let mut game = start.clone();
        game.team[0].burnout = 90;
        run(&mut game, Action::Offsite);
        assert_eq!(game.day, start.day + 5);
        assert_eq!(game.budget, start.budget - 20_000);
        assert_eq!(game.team[0].burnout, 40);
    }

    #[test]
    fn phishing_simulations_sometimes_upset_a_vp() {
        let mut outcomes = Vec::new();
        for seed in 0..30 {
            let mut game = GameState { seed, ..equipped() };
            run(&mut game, PHISH);
            outcomes.push(game.trust);
        }

        assert!(outcomes.contains(&60) && outcomes.contains(&57));
    }

    #[test]
    fn hiring_after_day_one_starts_a_week_long_search() {
        let mut game = game();
        game.day = 2;

        game.hire(Item::Senior, "Priya");
        assert_eq!(game.budget, 500_000 - Item::Senior.price(), "fee paid now");
        assert_eq!(game.analysts(), 3);
        assert_eq!(game.searches[0].days_left, 7);

        for _ in 0..6 {
            game.coffee = COFFEE_CAPACITY;
            assert!(!game.recruit());
        }
        assert!(game.recruit(), "the new hire stops the clock");
        assert!(game.searches.is_empty());
        assert_eq!(
            game.team.last().unwrap(),
            &member("Priya", Role::Senior, 0, Item::Senior.weekly())
        );
        let card = game.cards.last().unwrap();
        assert_eq!(card.title, "NEW HIRE");
        assert!(card.effect.contains("Priya joins the team"));
    }

    #[test]
    fn hiring_is_instant_on_day_one_and_at_the_conference_job_fair() {
        let mut game = game();
        game.hire(Item::Junior, "Ann");
        assert_eq!(game.analysts(), 4);

        let mut fair = at(Landmark::Conference, 1, 30);
        fair.stop.as_mut().unwrap().attended = true;
        fair.hire(Item::Junior, "Bo");
        assert_eq!(fair.analysts(), 4);
        assert!(fair.searches.is_empty());
    }

    #[test]
    fn searches_count_against_the_desks_and_paranoia_stops_hiring() {
        let mut busy = game();
        busy.day = 2;
        for i in 0..5 {
            busy.hire(Item::Junior, &format!("A{i}"));
        }
        assert_eq!(busy.searches.len(), 5);
        assert!(!busy.can_buy(Item::Junior), "every desk is spoken for");

        let mut paranoid = game();
        paranoid.conditions.push(Condition::Paranoia);
        assert!(!paranoid.can_buy(Item::Junior));
        assert!(!paranoid.can_buy(Item::Senior));
    }

    #[test]
    fn unaffordable_actions_and_unmet_prerequisites_are_not_offered() {
        let mut game = equipped();
        assert!(!game.actions().contains(&Action::Deploy(Item::Backups)));
        game.owned.push(Item::Backups);
        assert!(game.actions().contains(&Action::Deploy(Item::Backups)));

        game.budget = 4_999;
        let offered = game.actions();
        assert!(!offered.contains(&PHISH));
        assert!(!offered.contains(&Action::Offsite));
        assert!(offered.contains(&PATCH));
    }

    #[test]
    fn nothing_else_starts_while_an_action_is_under_way() {
        let mut game = equipped();

        game.start(PATCH);

        assert!(game.actions().is_empty());
    }

    #[test]
    fn tempo_and_seniors_change_how_long_actions_take() {
        let mut game = equipped();
        assert_eq!(game.duration(PATCH), 5);

        game.tempo = Tempo::Relaxed;
        assert_eq!(game.duration(PATCH), 8);
        game.tempo = Tempo::Crunch;
        assert_eq!(game.duration(PATCH), 4);

        game.tempo = Tempo::Steady;
        game.team.push(member("S1", Role::Senior, 0, 0));
        assert_eq!(game.duration(PATCH), 5, "one senior is not enough");
        game.team.push(member("S2", Role::Senior, 0, 0));
        assert_eq!(game.duration(PATCH), 4);

        game.tempo = Tempo::Crunch;
        game.team.push(member("S3", Role::Senior, 0, 0));
        game.team.push(member("S4", Role::Senior, 0, 0));
        assert_eq!(game.duration(TABLETOP), 1, "never under a day");
        assert_eq!(game.duration(Action::DayOff), 1, "fixed");
    }

    #[test]
    fn an_interrupted_action_resumes_where_it_left_off() {
        let mut game = equipped();
        game.coffee = 2;
        game.start(PATCH);

        assert!(game.advance(), "running out of coffee stops the clock");
        assert_eq!(game.task.as_ref().unwrap().days_left, 4);

        assert!(!game.advance());
        assert!(!game.advance());
        assert!(!game.advance());
        assert!(game.advance(), "finishing stops the clock");
        assert_eq!(game.task, None);
        assert_eq!(game.day, 6);
    }

    #[test]
    fn days_off_replace_the_workload() {
        let mut game = game();
        game.start(Action::DayOff);

        game.advance();

        assert_eq!(game.team[1].burnout, 0);
    }

    /// Plants a hidden campaign and returns its index.
    fn plant(game: &mut GameState, actor: Actor, stage: Stage) -> usize {
        game.campaigns.push(Campaign {
            actor,
            start: game.day,
            stage,
            seen: false,
            end: None,
        });
        game.campaigns.len() - 1
    }

    /// Plants a pending alert, real if it names a campaign.
    fn plant_alert(game: &mut GameState, campaign: Option<usize>) {
        game.alerts.push(Alert {
            day: game.day,
            text: "Something odd.",
            campaign,
        });
        game.alert = Some(game.alerts.len() - 1);
    }

    /// Plays `days` days, ignoring alerts and taking the first possible incident response.
    fn idle(game: &mut GameState, days: u32, incidents: &mut Vec<Actor>) {
        for _ in 0..days {
            if game.outcome.is_some() {
                return;
            }
            game.coffee = COFFEE_CAPACITY;
            game.budget = 500_000;
            game.trust = 60;
            game.brand = 60;
            game.valuation = game.base_valuation;
            for member in &mut game.team {
                member.burnout = 0;
            }
            game.advance();
            game.reply(Reply::Ignore);
            if game.event.is_some() {
                game.choose(0);
            }
            pass_landmark(game);
            if let Some(actor) = game.incident {
                incidents.push(actor);
                let i = actor
                    .responses()
                    .iter()
                    .position(|r| game.can_respond(r))
                    .unwrap();
                game.respond(i);
            }
        }
    }

    /// Leaves a fort or crosses a river the first affordable way.
    fn pass_landmark(game: &mut GameState) {
        let Some(stop) = &game.stop else {
            return;
        };
        if stop.landmark.is_fort() {
            game.leave_fort();
        } else {
            let way = (0..).find(|&i| game.can_cross(i)).unwrap();
            game.cross(way);
        }
    }

    /// A game stopped at `landmark`, as if it had just arrived.
    fn at(landmark: Landmark, seed: u32, posture: i32) -> GameState {
        let index = Landmark::ALL.iter().position(|&l| l == landmark).unwrap();
        GameState {
            seed,
            bonus: [posture; 6],
            day: landmark.day(),
            next_landmark: index + 1,
            stop: Some(Stop {
                landmark,
                rested: false,
                attended: false,
            }),
            ..game()
        }
    }

    fn with_posture(seed: u32, value: i32) -> GameState {
        GameState {
            seed,
            bonus: [value; 6],
            ..game()
        }
    }

    #[test]
    fn each_profile_draws_mostly_its_favorite_attackers() {
        for (profile, favorite) in [
            (Profile::Fintech, Actor::Ransomware),
            (Profile::Healthtech, Actor::DataThief),
            (Profile::Gaming, Actor::Hacktivists),
        ] {
            let mut game = GameState::new("Acme", "Alex", profile, 7);
            let mut counts = [0; 6];
            for _ in 0..3000 {
                let actor = game.pick_actor();
                counts[Actor::ALL.iter().position(|a| *a == actor).unwrap()] += 1;
            }
            let top = Actor::ALL[(0..6).max_by_key(|&i| counts[i]).unwrap()];

            assert_eq!(top, favorite, "{profile}: {counts:?}");
            assert!(counts.iter().all(|&c| c > 0), "everyone shows up");
        }
    }

    #[test]
    fn higher_posture_slows_attackers_down() {
        let progress = |posture| -> usize {
            (0..60)
                .map(|seed| {
                    let mut game = with_posture(seed, posture);
                    let i = plant(&mut game, Actor::DataThief, Stage::Reconnaissance);
                    for _ in 0..20 {
                        game.attack();
                        game.alert = None;
                    }
                    game.campaigns[i].stage as usize
                })
                .sum()
        };

        assert!(
            progress(90) < progress(10),
            "{} vs {}",
            progress(90),
            progress(10)
        );
    }

    #[test]
    fn strong_defenses_make_attackers_give_up() {
        let gave_up = (0..60)
            .filter(|&seed| {
                let mut game = with_posture(seed, 100);
                let i = plant(&mut game, Actor::Hacktivists, Stage::Reconnaissance);
                for _ in 0..60 {
                    game.attack();
                    game.alert = None;
                }
                game.campaigns[i].end == Some(End::GaveUp)
            })
            .count();

        assert!(gave_up > 0);
    }

    #[test]
    fn better_detection_sees_more_real_attacks() {
        let real_alerts = |detection| -> usize {
            (0..40)
                .map(|seed| {
                    let mut game = with_posture(seed, 20);
                    game.bonus[Area::Detection as usize] = detection;
                    idle(&mut game, 120, &mut Vec::new());
                    game.alerts.iter().filter(|a| a.campaign.is_some()).count()
                })
                .sum()
        };

        assert!(real_alerts(90) > real_alerts(0));
    }

    #[test]
    fn a_neglected_siem_cries_wolf_more_often() {
        let false_alarms = |maintained: bool| -> usize {
            (0..40)
                .map(|seed| {
                    let mut game = with_posture(seed, 100);
                    game.deployed.push(Tool::new(Item::Siem));
                    for _ in 0..120 {
                        if !maintained {
                            game.deployed[0].condition = 0;
                        }
                        game.coffee = COFFEE_CAPACITY;
                        game.team.iter_mut().for_each(|m| m.burnout = 0);
                        game.advance();
                        game.reply(Reply::Ignore);
                        game.deployed[0].condition = game.deployed[0]
                            .condition
                            .max(if maintained { 100 } else { 0 });
                    }
                    game.alerts.iter().filter(|a| a.campaign.is_none()).count()
                })
                .sum()
        };

        assert!(false_alarms(true) < false_alarms(false));
    }

    #[test]
    fn alerts_stop_the_clock_and_wait_for_a_reply() {
        let mut game = game();
        let i = plant(&mut game, Actor::Ransomware, Stage::Reconnaissance);
        plant_alert(&mut game, Some(i));

        assert_eq!(game.pending_alert(), Some("Something odd."));
        game.reply(Reply::Ignore);
        assert_eq!(game.pending_alert(), None);
        assert_eq!(game.campaigns[i].end, None, "ignoring does nothing");
    }

    #[test]
    fn investigating_a_real_alert_can_evict_the_attacker() {
        let evicted = (0..40)
            .filter(|&seed| {
                let mut game = GameState { seed, ..game() };
                let i = plant(&mut game, Actor::Espionage, Stage::InitialAccess);
                plant_alert(&mut game, Some(i));
                game.reply(Reply::Investigate);
                while game.investigation.is_some() {
                    game.campaigns[i].stage = Stage::InitialAccess;
                    game.advance();
                    game.reply(Reply::Ignore);
                }
                game.campaigns[i].end == Some(End::Evicted)
            })
            .count();

        assert!(evicted > 10, "{evicted} of 40");
    }

    #[test]
    fn investigating_a_false_alarm_only_costs_days() {
        let mut game = game();
        plant_alert(&mut game, None);
        let days = game.investigation_days();

        game.reply(Reply::Investigate);
        assert!(!game.can_reply(Reply::Investigate), "one at a time");
        let start = game.day;
        while game.investigation.is_some() {
            game.advance();
            game.reply(Reply::Ignore);
        }

        assert_eq!(game.day, start + days);
        assert!(game.log.iter().any(|e| e.contains("nothing conclusive")));
    }

    #[test]
    fn seniors_investigate_faster() {
        let mut game = game();
        assert_eq!(game.investigation_days(), 3);
        game.team.push(member("S1", Role::Senior, 0, 0));
        game.team.push(member("S2", Role::Senior, 0, 0));
        assert_eq!(game.investigation_days(), 2);
    }

    #[test]
    fn the_ir_firm_needs_a_retainer_and_bills_per_call() {
        let mut unretained = game();
        plant_alert(&mut unretained, None);
        assert!(!unretained.can_reply(Reply::CallIr));

        let evicted = (0..20)
            .filter(|&seed| {
                let mut game = GameState { seed, ..game() };
                game.owned.push(Item::IrRetainer);
                let i = plant(&mut game, Actor::Apt, Stage::Foothold);
                plant_alert(&mut game, Some(i));
                assert!(game.can_reply(Reply::CallIr));
                game.reply(Reply::CallIr);
                assert_eq!(game.budget, 500_000 - IR_FEE);
                game.campaigns[i].end == Some(End::Evicted)
                    && game.log.last().unwrap().contains("Nation-state APT")
            })
            .count();

        assert!(evicted >= 17, "{evicted} of 20");
    }

    #[test]
    fn an_attacker_reaching_its_objective_is_an_incident() {
        let mut game = with_posture(1, 0);
        let i = plant(&mut game, Actor::Hacktivists, Stage::Foothold);
        while game.incident.is_none() {
            game.attack();
            game.alert = None;
        }

        assert_eq!(game.incident, Some(Actor::Hacktivists));
        assert_eq!(game.campaigns[i].end, Some(End::Succeeded));
        assert!(game.log.last().unwrap().starts_with("INCIDENT:"));
    }

    #[test]
    fn every_incident_type_happens_across_seeds() {
        let mut incidents = Vec::new();
        for seed in 0..30 {
            for profile in Profile::ALL {
                let mut game = GameState {
                    seed,
                    profile,
                    bonus: [0; 6],
                    ..game()
                };
                idle(&mut game, IPO_DAY, &mut incidents);
            }
        }

        for actor in Actor::ALL {
            assert!(incidents.contains(&actor), "{actor} never succeeds");
        }
    }

    #[test]
    fn every_response_applies_its_effects_and_condition() {
        for actor in Actor::ALL {
            for (i, response) in actor.responses().iter().enumerate() {
                let mut game = game();
                game.deployed.push(Tool::new(Item::Backups));
                game.incident = Some(actor);
                assert!(game.can_respond(response), "{}", response.label);

                // Fresh backups' Resilience 25 scales damage to 175/200.
                assert_eq!(game.level(Area::Resilience), 25);
                game.respond(i);
                let scale = 175;
                assert_eq!(
                    game.valuation,
                    1_000_000_000 - 1_000_000_000 * response.valuation * scale / 200_000
                );
                assert_eq!(game.trust, 60 + response.trust * scale as i32 / 200);
                assert_eq!(game.brand, 60 + response.brand * scale as i32 / 200);
                assert_eq!(game.budget, 500_000 - response.budget);
                assert_eq!(game.incident, None);
                assert_eq!(
                    game.conditions.contains(&actor.condition()),
                    response.lingers,
                    "{}",
                    response.label
                );
                assert_eq!(game.log.last().unwrap().text, response.text);
            }
        }
    }

    #[test]
    fn restoring_from_backups_needs_deployed_backups() {
        let mut game = game();
        let restore = &Actor::Ransomware.responses()[0];

        assert!(!game.can_respond(restore));
        game.owned.push(Item::Backups);
        assert!(!game.can_respond(restore));
        game.deployed.push(Tool::new(Item::Backups));
        assert!(game.can_respond(restore));
    }

    #[test]
    fn every_condition_has_a_cure_that_clears_it() {
        for actor in Actor::ALL {
            let condition = actor.condition();
            let mut game = game();
            game.conditions.push(condition);

            assert_eq!(game.actions()[0], Action::Clear(condition));
            run(&mut game, Action::Clear(condition));

            assert!(game.conditions.is_empty(), "{condition:?}");
            assert!(game.log.last().unwrap().contains(condition.cure()));
        }
    }

    #[test]
    fn conditions_change_what_the_soc_can_do() {
        let mut down = game();
        down.conditions.push(Condition::SystemsDown);
        assert_eq!(
            down.actions(),
            [
                Action::Clear(Condition::SystemsDown),
                Action::DayOff,
                Action::BriefLeadership
            ]
        );

        let mut paranoid = game();
        paranoid.conditions.push(Condition::Paranoia);
        assert!(!paranoid.can_buy(Item::Junior), "nobody can be recruited");
    }

    #[test]
    fn conditions_do_damage_while_they_last() {
        let mut hurting = game();
        hurting.day = 6;
        hurting.conditions = vec![
            Condition::RegulatorInquiry,
            Condition::LeakyRoadmap,
            Condition::Downtime,
        ];
        advance_to(&mut hurting, 8);

        assert_eq!(hurting.budget, 500_000 - LEGAL_FEES - 18_000);
        assert_eq!(hurting.valuation, 1_000_000_000 - 5_000_000);
        assert!(hurting.brand <= 60 - 4, "two days of downtime");

        let mut infested = game();
        infested.conditions.push(Condition::PersistentAccess);
        infested.seed = (0..)
            .find(|&seed| {
                let mut g = GameState {
                    seed,
                    ..infested.clone()
                };
                g.attack();
                !g.campaigns.is_empty()
            })
            .unwrap();
        infested.attack();
        assert!(infested.campaigns[0].stage >= Stage::Foothold);

        let mut calm = game();
        let mut paranoid = game();
        paranoid.conditions.push(Condition::Paranoia);
        calm.advance();
        paranoid.advance();
        assert!(paranoid.team[0].burnout > calm.team[0].burnout);
    }

    #[test]
    fn hunting_finds_more_with_seniors_and_a_siem() {
        let found = |seniors: usize, condition: i32| -> usize {
            (0..60)
                .map(|seed| {
                    let mut game = GameState { seed, ..game() };
                    game.team
                        .extend((0..seniors).map(|_| member("S", Role::Senior, 0, 0)));
                    game.deployed.push(Tool {
                        item: Item::Siem,
                        condition,
                    });
                    let i = plant(&mut game, Actor::Apt, Stage::Foothold);
                    game.operate(Item::Siem);
                    (game.campaigns[i].end == Some(End::Evicted)) as usize
                })
                .sum()
        };

        assert!(found(0, 20) < found(0, 100));
        assert!(found(0, 100) < found(4, 100));
    }

    #[test]
    fn only_some_tools_unlock_an_operation_and_only_once_deployed() {
        let tools: Vec<Item> = OPERATIONS.iter().map(|o| o.tool).collect();
        assert_eq!(
            tools,
            [
                Item::PasswordManager,
                Item::Pam,
                Item::Edr,
                Item::TrainingPlatform,
                Item::VulnScanner,
                Item::Asm,
                Item::Runbooks,
                Item::Backups,
                Item::DrSite,
                Item::LogCollection,
                Item::Siem,
            ]
        );
        for item in tools {
            let mut game = game();
            game.owned.push(item);
            assert!(!game.actions().contains(&Action::Operate(item)), "{item:?}");
            game.deployed.push(Tool::new(item));
            assert!(game.actions().contains(&Action::Operate(item)), "{item:?}");
        }
    }

    #[test]
    fn every_operation_reads_well() {
        for op in &OPERATIONS {
            assert!(!op.label.is_empty() && !op.description.is_empty());
            assert!(op.days > 0, "{}", op.label);
            let [a, b, c] = op.results;
            assert!(a != b && b != c && a != c, "{}", op.label);
            assert_eq!(op.boosts.is_empty(), op.fades.is_empty(), "{}", op.label);
            assert_eq!(op.evicts.is_empty(), op.odds == 0, "{}", op.label);
        }
    }

    #[test]
    fn operations_boost_by_the_tools_condition() {
        for op in &OPERATIONS {
            let mut game = game();
            game.deployed.push(Tool {
                item: op.tool,
                condition: 60,
            });
            let before = Area::ALL.map(|a| game.level(a));

            game.operate(op.tool);

            for area in Area::ALL {
                let boost: i32 = op
                    .boosts
                    .iter()
                    .filter(|&&(a, _)| a == area)
                    .map(|&(_, b)| b * 60 / 100)
                    .sum();
                assert_eq!(
                    game.level(area),
                    before[area as usize] + boost,
                    "{} {area}",
                    op.label
                );
            }
        }
    }

    #[test]
    fn boosts_wear_off_after_four_weeks_and_rerunning_refreshes_them() {
        let mut game = equipped();
        game.operate(Item::VulnScanner);
        game.operate(Item::VulnScanner);
        assert_eq!(game.boosts.len(), 2, "refreshed, not stacked");
        assert!(game.boosts.iter().all(|b| b.until == 1 + BOOST_DAYS));

        advance_to(&mut game, BOOST_DAYS);
        assert_eq!(game.boosts.len(), 2);
        advance_to(&mut game, 1 + BOOST_DAYS);
        assert!(game.boosts.is_empty());
        let fades = Item::VulnScanner.operation().unwrap().fades;
        assert_eq!(game.log.iter().filter(|e| e.text == fades).count(), 1);
    }

    #[test]
    fn a_brace_halves_one_matching_incident() {
        let hit = |game: &mut GameState, actor: Actor| {
            let before = game.valuation;
            game.incident = Some(actor);
            game.respond(0);
            before - game.valuation
        };
        let mut game = game();
        game.operate(Item::Backups);
        assert_eq!(game.braced, [Actor::Ransomware]);

        let full = hit(&mut game.clone(), Actor::DataThief);
        let thief = hit(&mut game, Actor::DataThief);
        assert_eq!(thief, full, "a different attacker leaves the brace alone");
        assert_eq!(game.braced, [Actor::Ransomware]);

        let full = hit(
            &mut GameState {
                braced: Vec::new(),
                ..game.clone()
            },
            Actor::Ransomware,
        );
        let braced = hit(&mut game, Actor::Ransomware);
        assert_eq!(braced, full / 2);
        assert!(game.log.last().unwrap().contains("drills paid off"));
        assert!(game.braced.is_empty(), "used up");
        assert_eq!(hit(&mut game, Actor::Ransomware), full);
    }

    #[test]
    fn a_failover_test_braces_for_everyone_once() {
        let mut game = game();
        game.operate(Item::DrSite);
        game.operate(Item::DrSite);

        assert_eq!(game.braced, Actor::ALL);
    }

    #[test]
    fn targeted_evictions_only_hit_their_attackers() {
        let mut evicted = [0; 2];
        for seed in 0..40 {
            let mut game = GameState { seed, ..game() };
            game.deployed.push(Tool::new(Item::Pam));
            let ransomware = plant(&mut game, Actor::Ransomware, Stage::Foothold);
            let thief = plant(&mut game, Actor::DataThief, Stage::Foothold);

            game.operate(Item::Pam);

            for (count, i) in evicted.iter_mut().zip([ransomware, thief]) {
                *count += (game.campaigns[i].end == Some(End::Evicted)) as u32;
            }
        }

        assert_eq!(evicted[0], 0, "PAM never finds ransomware");
        assert!(evicted[1] > 10, "{}", evicted[1]);
    }

    #[test]
    fn a_fruitless_hunt_says_so() {
        let mut game = game();
        game.deployed.push(Tool::new(Item::Siem));

        run(&mut game, HUNT);

        assert!(game.log.iter().any(|e| e.contains("found nothing")));
    }

    #[test]
    fn specialists_add_skill_by_level_and_take_it_with_them() {
        let mut game = game();
        game.team.push(member("Ana", Role::Senior, 0, 8_000));
        game.team[0].specialty = Some(Area::Identity);
        assert_eq!(game.level(Area::Identity), JUNIOR_SKILL);
        game.team[3].specialty = Some(Area::Identity);
        assert_eq!(game.level(Area::Identity), JUNIOR_SKILL + SENIOR_SKILL);
        assert_eq!(
            game.defenders(Area::Identity),
            ["Maya's specialty", "Ana's specialty"]
        );

        game.fire(0);

        assert_eq!(game.level(Area::Identity), SENIOR_SKILL);
        assert!(
            game.log
                .last()
                .unwrap()
                .ends_with(" Their Identity specialty went with them.")
        );
    }

    #[test]
    fn event_wear_hits_only_its_areas_and_stays_in_range() {
        let mut game = GameState {
            deployed: worn_tools(),
            ..game()
        };
        game.deployed[3].condition = 95;
        let wear = |game: &mut GameState, wear: &'static [(Area, i32)]| {
            let effect = Effect {
                wear,
                ..crate::events::NOTHING
            };
            game.apply(&effect, usize::MAX);
            game.deployed
                .iter()
                .map(|t| t.condition)
                .collect::<Vec<_>>()
        };

        assert_eq!(
            wear(&mut game, &[(Area::Perimeter, 20)]),
            [50, 50, 50, 100, 50, 50]
        );
        assert_eq!(
            wear(&mut game, &[(Area::Perimeter, -200)]),
            [50, 50, 50, 0, 50, 50]
        );
    }

    /// The pen test grade for one area with `items` deployed at 88% condition and an
    /// optional specialist.
    fn grade(area: Area, items: &[Item], specialist: Option<Role>) -> char {
        let mut game = game();
        game.deployed = items
            .iter()
            .map(|&item| Tool {
                item,
                condition: 88,
            })
            .collect();
        if let Some(role) = specialist {
            game.team.push(TeamMember {
                specialty: Some(area),
                ..member("Spec", role, 0, 0)
            });
        }
        game.report_card()[area as usize].1
    }

    #[test]
    fn a_strong_build_earns_good_grades() {
        assert_eq!(grade(Area::Identity, &[], None), 'F');
        assert_eq!(
            grade(Area::Identity, &[Item::MfaTokens], Some(Role::Junior)),
            'C'
        );
        assert_eq!(
            grade(
                Area::Identity,
                &[Item::MfaTokens, Item::Pam],
                Some(Role::Senior)
            ),
            'B'
        );
        for area in Area::ALL {
            let tools: Vec<Item> = Item::ALL
                .into_iter()
                .filter(|i| i.category() == Some(area))
                .collect();
            assert_eq!(grade(area, &tools, Some(Role::Senior)), 'A', "{area}");
        }
    }

    #[test]
    fn pricier_tools_never_give_less() {
        for area in Area::ALL {
            let tiers: Vec<Item> = Item::ALL
                .into_iter()
                .filter(|i| i.category() == Some(area))
                .collect();
            let own = |item: Item| {
                item.boosts()
                    .iter()
                    .filter(|&&(a, _)| a == area)
                    .map(|&(_, b)| b)
                    .sum::<i32>()
            };
            let total = |item: Item| item.boosts().iter().map(|&(_, b)| b).sum::<i32>();
            for pair in tiers.windows(2) {
                assert!(pair[0].price() < pair[1].price());
                assert!(own(pair[0]) <= own(pair[1]), "{:?}", pair[1]);
                assert!(total(pair[0]) < total(pair[1]), "{:?}", pair[1]);
            }
        }
    }

    #[test]
    fn tools_wear_each_monday_and_count_by_their_condition() {
        let mut game = game();
        game.deployed.push(Tool::new(Item::Waf));
        assert_eq!(game.level(Area::Perimeter), 30);

        game.day = 7;
        game.advance();
        assert_eq!(game.deployed[0].condition, 100 - TOOL_WEAR);
        assert_eq!(game.level(Area::Perimeter), 30 * (100 - TOOL_WEAR) / 100);

        game.deployed[0].condition = 50;
        assert_eq!(game.level(Area::Perimeter), 15);
        game.deployed[0].condition = 0;
        assert_eq!(game.level(Area::Perimeter), 0);
    }

    #[test]
    fn pen_test_grades_count_a_tools_condition() {
        let mut game = game();
        game.bonus = [50; 6];
        game.deployed.push(Tool::new(Item::Waf));
        assert_eq!(game.report_card()[3], (Area::Perimeter, 'A'));

        game.deployed[0].condition = 0;
        assert_eq!(game.report_card()[3], (Area::Perimeter, 'C'));
    }

    #[test]
    fn maintenance_is_offered_when_needed_and_restores_the_whole_area() {
        let mut game = game();
        game.deployed = vec![
            Tool::new(Item::Backups),
            Tool::new(Item::Runbooks),
            Tool::new(Item::Siem),
        ];
        let maintain = Action::Maintain(Area::Resilience);
        assert!(!game.actions().contains(&maintain), "all fresh");

        game.deployed[0].condition = 79;
        game.deployed[1].condition = 40;
        game.deployed[2].condition = 40;
        assert!(game.actions().contains(&maintain));
        assert!(game.actions().contains(&Action::Maintain(Area::Detection)));
        assert!(!game.actions().contains(&Action::Maintain(Area::Identity)));

        run(&mut game, maintain);

        assert_eq!(game.deployed[0].condition, 100);
        assert_eq!(game.deployed[1].condition, 100);
        assert_eq!(game.deployed[2].condition, 40, "other areas untouched");
        assert!(
            Area::Resilience
                .upkeep()
                .contains(&game.log.last().unwrap().text.as_str())
        );
        assert!(
            game.cards
                .last()
                .unwrap()
                .effect
                .contains("Resilience improved")
        );
    }

    #[test]
    fn resting_lowers_the_watch() {
        let mut game = game();
        game.bonus = [60; 6];
        let normal = game.watch();

        game.start(Action::Offsite);
        let offsite = game.watch();
        game.task = None;
        game.start(Action::DayOff);
        let day_off = game.watch();

        assert!(day_off < offsite && offsite < normal);
    }

    #[test]
    fn resilience_and_insurance_soften_incidents() {
        let damage = |resilience: i32, insured: bool| {
            let mut game = game();
            game.bonus[Area::Resilience as usize] = resilience;
            if insured {
                game.owned.push(Item::Insurance);
            }
            game.incident = Some(Actor::Ransomware);
            game.respond(1);
            (
                1_000_000_000 - game.valuation,
                60 - game.trust,
                500_000 - game.budget,
            )
        };

        let (soft, hard) = (damage(100, false), damage(0, false));
        assert!(soft.0 < hard.0 && soft.1 < hard.1);
        assert_eq!(damage(0, true).2, hard.2 / 2);
    }

    #[test]
    fn the_after_action_report_waits_for_the_end_and_tells_all() {
        let mut game = game();
        let i = plant(&mut game, Actor::Insider, Stage::Foothold);
        game.campaigns[i].seen = true;
        plant_alert(&mut game, Some(i));
        plant_alert(&mut game, None);
        assert!(game.after_action().is_empty());

        game.outcome = Some(Outcome::Fired);
        let report = game.after_action();

        assert!(report.contains(
            &"Day 1: Malicious insider reached Foothold, seen, still at it.".to_string()
        ));
        assert!(report.contains(&"Day 1 REAL  Something odd.".to_string()));
        assert!(report.contains(&"Day 1 FALSE Something odd.".to_string()));
    }

    #[test]
    fn a_quiet_game_says_so_in_the_report() {
        let mut game = game();
        game.outcome = Some(Outcome::Ipo);

        assert_eq!(game.after_action().len(), 1);
    }

    /// One cheap tool per area at half condition, for events to wear.
    fn worn_tools() -> Vec<Tool> {
        [
            Item::PasswordManager,
            Item::Antivirus,
            Item::Posters,
            Item::VulnScanner,
            Item::Runbooks,
            Item::LogCollection,
        ]
        .map(|item| Tool {
            item,
            condition: 50,
        })
        .to_vec()
    }

    /// Checks that `after` is `before` with `effect` applied, naming `patient`.
    fn assert_effect(before: &GameState, after: &GameState, effect: &Effect, patient: usize) {
        assert_eq!(after.trust, (before.trust + effect.trust).clamp(0, 100));
        assert_eq!(after.brand, (before.brand + effect.brand).clamp(0, 100));
        assert_eq!(after.budget, before.budget + effect.budget);
        assert_eq!(
            after.valuation,
            before.valuation + before.base_valuation * effect.valuation / 1000
        );
        for (i, (old, new)) in before.team.iter().zip(&after.team).enumerate() {
            let mut burnout = old.burnout;
            burnout = (burnout + effect.burnout).max(0);
            if i == patient {
                burnout = (burnout + effect.patient).max(0);
            }
            assert_eq!(new.burnout, burnout, "{}", old.name);
        }
        for (old, new) in before.deployed.iter().zip(&after.deployed) {
            let change: i32 = effect
                .wear
                .iter()
                .filter(|&&(area, _)| old.item.category() == Some(area))
                .map(|&(_, change)| change)
                .sum();
            assert_eq!(new.condition, (old.condition + change).clamp(0, 100));
        }
        assert_eq!(
            after.coffee,
            (before.coffee + effect.coffee).clamp(0, COFFEE_CAPACITY)
        );
        assert_eq!(after.threat, before.threat + effect.threat);
        assert_eq!(after.holiday, before.holiday.max(effect.holiday));
        assert_eq!(
            after.campaigns.len(),
            before.campaigns.len() + effect.intruder as usize
        );
    }

    #[test]
    fn every_event_happens_across_seeds_and_stops_the_clock() {
        let mut seen = [false; EVENTS.len()];
        for seed in 0..30 {
            let mut game = GameState { seed, ..game() };
            while game.outcome.is_none() {
                game.coffee = COFFEE_CAPACITY;
                game.budget = 500_000;
                game.trust = 60;
                game.brand = 60;
                game.team.iter_mut().for_each(|m| m.burnout = 0);
                let entries = game.log.len();
                let stopped = game.advance();
                for (i, event) in EVENTS.iter().enumerate() {
                    let tail = event.text.split("{name}").last().unwrap();
                    if game.log[entries..].iter().any(|e| e.ends_with(tail)) {
                        seen[i] = true;
                        assert!(stopped, "{}", event.text);
                    }
                }
                game.reply(Reply::Ignore);
                if game.event.is_some() {
                    game.choose(0);
                }
                if let Some(actor) = game.incident {
                    let i = (0..actor.responses().len())
                        .find(|&i| game.can_respond(&actor.responses()[i]))
                        .unwrap();
                    game.respond(i);
                }
            }
        }

        for (i, event) in EVENTS.iter().enumerate() {
            assert!(seen[i], "never happens: {}", event.text);
        }
    }

    #[test]
    fn events_without_choices_apply_their_effects_at_once() {
        for (i, event) in EVENTS.iter().enumerate() {
            if !event.choices.is_empty() {
                continue;
            }
            let before = GameState {
                deployed: worn_tools(),
                ..game()
            };
            let mut after = before.clone();

            after.trigger(i);

            assert_eq!(after.event, None);
            assert_effect(&before, &after, &event.effect, usize::MAX);
            assert_eq!(after.log.last().unwrap().text, event.text);
        }
    }

    #[test]
    fn every_event_choice_applies_its_effects() {
        for (i, event) in EVENTS.iter().enumerate() {
            for (j, choice) in event.choices.iter().enumerate() {
                let mut before = GameState {
                    deployed: worn_tools(),
                    ..game()
                };
                before.trigger(i);
                let pending = before.event.clone().expect("waiting for a choice");
                assert_eq!(before.event_choices(), event.choices);
                assert!(!before.log.last().unwrap().contains("{name}"));
                let mut after = before.clone();

                after.choose(j);

                assert_eq!(after.event, None);
                assert_effect(&before, &after, &choice.effect, pending.patient);
                assert_eq!(after.log.last().unwrap().text, choice.text);
            }
        }
    }

    #[test]
    fn the_flu_names_one_of_the_analysts() {
        let mut game = game();

        game.trigger(0);

        let patient = &game.team[game.event.as_ref().unwrap().patient];
        assert!(game.log.last().unwrap().starts_with(&patient.name));
    }

    #[test]
    fn briefings_sometimes_end_with_a_pet_project() {
        let projects = (0..30)
            .filter(|&seed| {
                let mut game = GameState { seed, ..game() };
                game.start(Action::BriefLeadership);
                game.progress();
                game.event.as_ref().map(|e| e.index) == Some(PET_PROJECT)
            })
            .count();

        assert!(projects > 0 && projects < 30, "{projects} of 30");
    }

    #[test]
    fn publicity_holidays_and_breached_vendors_help_the_attackers() {
        let mut game = game();
        game.bonus = [60; 6];
        let (odds, watch) = (game.spawn_odds(), game.watch());

        game.apply(
            &Effect {
                threat: 10,
                holiday: 7,
                intruder: true,
                ..crate::events::NOTHING
            },
            0,
        );

        assert_eq!(game.spawn_odds(), odds + 10);
        assert!(game.watch() < watch);
        assert_eq!(game.campaigns[0].actor, Actor::Apt);
        assert_eq!(game.campaigns[0].stage, Stage::InitialAccess);
        advance_to(&mut game, 8);
        assert_eq!(game.holiday, 0, "the holiday ends");
    }

    #[test]
    fn every_landmark_comes_up_on_its_day() {
        let mut game = game();
        let mut arrivals = Vec::new();
        while game.outcome.is_none() {
            game.coffee = COFFEE_CAPACITY;
            game.bonus = [100; 6];
            game.team.iter_mut().for_each(|m| m.burnout = 0);
            game.advance();
            game.alert = None;
            game.event = None;
            game.incident = None;
            if let Some(stop) = &game.stop {
                arrivals.push((stop.landmark, game.day));
                pass_landmark(&mut game);
            }
        }

        let expected: Vec<_> = Landmark::ALL.iter().map(|&l| (l, l.day())).collect();
        assert_eq!(arrivals, expected);
        assert_eq!(game.outcome, Some(Outcome::Ipo));
    }

    #[test]
    fn every_way_across_every_river_can_succeed_or_fail() {
        for landmark in Landmark::ALL {
            for (i, crossing) in landmark.crossings().iter().enumerate() {
                let (mut succeeded, mut failed) = (false, false);
                for seed in 0..60 {
                    for posture in [0, 100] {
                        let mut game = at(landmark, seed, posture);
                        game.cross(i);
                        assert_eq!(game.stop, None);
                        let log = &game.log;
                        succeeded |= log.iter().any(|e| e.text == crossing.success.0);
                        failed |= log.iter().any(|e| e.text == crossing.failure.0);
                    }
                }

                assert!(succeeded, "{landmark}: {} never succeeds", crossing.label);
                let certain = crossing.odds == crate::landmarks::CERTAIN;
                assert_eq!(failed, !certain, "{landmark}: {}", crossing.label);
            }
        }
    }

    #[test]
    fn crossings_cost_money_up_front() {
        let mut game = at(Landmark::Audit, 1, 30);
        game.budget = 59_999;
        assert!(!game.can_cross(1));
        game.budget = 60_000;
        assert!(game.can_cross(1));

        game.cross(1);

        assert_eq!(game.budget, 0, "consultants paid");
    }

    #[test]
    fn incidents_make_the_board_harder_to_please() {
        let mut game = at(Landmark::BoardBriefing, 1, 50);
        let odds = crate::landmarks::Landmark::BoardBriefing.crossings()[0].odds;
        let calm = game.crossing_odds(&odds);

        let i = plant(&mut game, Actor::DataThief, Stage::Objective);
        game.campaigns[i].end = Some(End::Succeeded);

        assert_eq!(game.crossing_odds(&odds), calm - 150);
    }

    #[test]
    fn failing_the_audit_delays_the_ipo_and_later_landmarks() {
        let mut game = (0..)
            .map(|seed| at(Landmark::Audit, seed, 0))
            .find(|g| {
                let mut g = g.clone();
                g.cross(0);
                g.ipo_day > IPO_DAY
            })
            .unwrap();
        let before = game.clone();

        game.cross(0);

        assert_eq!(game.ipo_day, IPO_DAY + 14);
        assert_eq!(game.landmark_days[..3], before.landmark_days[..3]);
        for i in 3..Landmark::ALL.len() {
            assert_eq!(game.landmark_days[i], before.landmark_days[i] + 14);
        }
        assert_eq!(game.valuation, before.valuation - 20_000_000);
        assert_eq!(game.trust, before.trust - 5 - 5, "audit and delay");
        assert_eq!(game.days_to_ipo(), IPO_DAY + 14 - game.day);
    }

    #[test]
    fn a_delayed_ipo_rings_on_its_new_day() {
        let mut game = game();
        game.slip();
        game.day = IPO_DAY;

        game.advance();

        assert_eq!(game.outcome, None);
        advance_to(&mut game, IPO_DAY + 14);
        assert_eq!(game.outcome, Some(Outcome::Ipo));
    }

    #[test]
    fn leaks_after_the_flip_force_an_amended_s1() {
        for (next, delayed) in [(Landmark::Flip as usize, false), (6, true)] {
            for actor in [Actor::DataThief, Actor::Insider] {
                let mut game = GameState {
                    next_landmark: next,
                    ..game()
                };
                game.incident = Some(actor);

                game.respond(0);

                assert_eq!(game.ipo_day > IPO_DAY, delayed, "{actor} at {next}");
            }
        }

        let mut game = GameState {
            next_landmark: 6,
            ..game()
        };
        game.incident = Some(Actor::Hacktivists);
        game.respond(0);
        assert_eq!(game.ipo_day, IPO_DAY);
    }

    #[test]
    fn systems_down_at_the_roadshow_delays_the_ipo() {
        let mut game = game();
        game.next_landmark = 6;
        game.day = Landmark::Roadshow.day() - 1;
        game.conditions.push(Condition::SystemsDown);

        assert!(game.advance());

        assert_eq!(game.ipo_day, IPO_DAY + 14);
        assert_eq!(game.stop.as_ref().unwrap().landmark, Landmark::Roadshow);
    }

    #[test]
    fn more_than_six_weeks_of_delays_pulls_the_ipo() {
        let mut game = game();
        for _ in 0..3 {
            game.slip();
        }
        assert_eq!(game.outcome, None);
        assert_eq!(game.ipo_day, IPO_DAY + 42);

        game.slip();

        assert_eq!(game.outcome, Some(Outcome::Pulled));
        assert_eq!(game.score(), None);
        game.advance();
        assert_eq!(game.outcome, Some(Outcome::Pulled));
    }

    #[test]
    fn the_pen_test_grades_hidden_posture() {
        let mut game = game();
        game.bonus = [85, 65, 45, 25, 5, 100];

        let grades: Vec<char> = game.report_card().iter().map(|(_, g)| *g).collect();

        assert_eq!(grades, ['A', 'B', 'C', 'D', 'F', 'A']);
        assert_eq!(game.report_card()[0].0, Area::Identity);
    }

    #[test]
    fn weak_areas_on_the_pen_test_cost_valuation() {
        let mut game = at(Landmark::PenTest, 1, 50);
        game.bonus[0] = 10;
        game.bonus[1] = 39;
        let mut softened = game.clone();

        game.cross(0);
        softened.cross(1);

        assert_eq!(game.valuation, 1_000_000_000 - 20_000_000);
        assert_eq!(softened.valuation, 1_000_000_000 - 10_000_000);
        assert_eq!(softened.budget, 500_000 - 40_000);
    }

    #[test]
    fn hiding_incidents_from_the_s1_can_cost_more_later() {
        let with_incidents = |seed| {
            let mut game = at(Landmark::S1Filing, seed, 50);
            for _ in 0..2 {
                let i = plant(&mut game, Actor::Ransomware, Stage::Objective);
                game.campaigns[i].end = Some(End::Succeeded);
            }
            game
        };

        let mut honest = with_incidents(1);
        honest.cross(0);
        let disclosed = 1_000_000_000 - honest.valuation;
        assert_eq!(disclosed, 30_000_000);

        let (mut surfaced, mut stayed_hidden) = (false, false);
        for seed in 0..20 {
            let mut game = with_incidents(seed);
            game.cross(1);
            assert_eq!(game.valuation, 1_000_000_000, "nothing up front");
            game.day = Landmark::Flip.day() - 1;
            game.next_landmark = 5;
            game.advance();
            if game.valuation < 1_000_000_000 - disclosed {
                surfaced = true;
                assert!(game.log.iter().any(|e| e.contains("journalist")));
            } else {
                stayed_hidden = true;
            }
        }
        assert!(surfaced && stayed_hidden);
    }

    #[test]
    fn the_flip_raises_the_threat_but_adds_no_budget() {
        let mut game = game();
        game.day = Landmark::Flip.day() - 1;
        game.next_landmark = 5;
        let (budget, odds) = (game.budget, game.spawn_odds());

        game.advance();

        assert_eq!(game.stop.as_ref().unwrap().landmark, Landmark::Flip);
        assert_eq!(game.budget, budget - 18_000, "only Monday payroll");
        assert!(game.spawn_odds() >= odds + FLIP_THREAT);
    }

    #[test]
    fn the_board_funds_every_sixth_monday_by_trust() {
        let mut calendar = game();
        let mut funded = Vec::new();
        while calendar.outcome.is_none() {
            calendar.coffee = COFFEE_CAPACITY;
            calendar.team.iter_mut().for_each(|m| m.burnout = 0);
            calendar.trust = 60;
            calendar.budget = 500_000;
            calendar.advance();
            if calendar.budget > 500_000 {
                funded.push(calendar.day);
            }
        }
        assert_eq!(funded, [43, 85, 127, 169]);

        for (trust, grant) in [(60, 160_000), (30, 80_000), (90, 240_000)] {
            let mut game = GameState { trust, ..game() };
            game.day = 42;
            game.advance();
            assert_eq!(game.budget, 500_000 - 18_000 + grant, "trust {trust}");
            assert_eq!(game.cards.last().unwrap().title, "BOARD FUNDING");
        }
    }

    #[test]
    fn each_profile_has_its_quarterly_grant() {
        for (profile, grant) in [
            (Profile::Fintech, 180_000),
            (Profile::Healthtech, 160_000),
            (Profile::Gaming, 120_000),
        ] {
            let game = GameState::new("Acme", "Alex", profile, 1);
            assert_eq!(game.grant(), grant, "{profile}");
        }
    }

    #[test]
    fn funding_keeps_coming_through_ipo_delays() {
        let mut game = game();
        assert_eq!(game.next_funding(), Some(43));

        for _ in 0..3 {
            game.slip();
        }
        game.day = 170;
        assert_eq!(game.ipo_day, IPO_DAY + 42);
        assert_eq!(
            game.next_funding(),
            Some(211),
            "a fifth grant before the late IPO"
        );
    }

    #[test]
    fn forts_offer_one_rest_and_let_you_move_on() {
        let mut game = at(Landmark::Conference, 1, 30);

        game.rest_at_fort();
        assert_eq!(game.team[0].burnout, 5);
        game.rest_at_fort();
        assert_eq!(game.team[0].burnout, 5, "once per visit");

        game.leave_fort();
        assert_eq!(game.stop, None);
    }

    /// Sends `picks` to the conference and plays until everyone is back.
    fn conference(seed: u32, picks: &[(usize, Track)]) -> GameState {
        let mut game = at(Landmark::Conference, seed, 30);
        game.attend(picks);
        while game.conference_days > 0 {
            game.coffee = COFFEE_CAPACITY;
            game.advance();
        }
        game
    }

    fn all_cards() -> Vec<Card> {
        Track::ALL.iter().flat_map(|t| t.cards()).copied().collect()
    }

    #[test]
    fn everyone_who_goes_pays_once_including_the_lead() {
        let mut game = at(Landmark::Conference, 1, 30);
        assert_eq!(game.conference_cost(0), TICKET);
        assert_eq!(game.conference_cost(2), 3 * TICKET);

        game.attend(&[(0, Track::Talks), (1, Track::Expo)]);

        assert_eq!(game.budget, 500_000 - 3 * TICKET);
        assert_eq!(game.stop, None, "away at the conference");
        assert_eq!(game.conference_days, CONFERENCE_DAYS);
    }

    #[test]
    fn the_conference_takes_three_days_and_reopens_the_fort() {
        let start = at(Landmark::Conference, 1, 30).day;
        let game = conference(1, &[(0, Track::Hallway)]);

        assert_eq!(game.day, start + CONFERENCE_DAYS);
        let stop = game.stop.as_ref().unwrap();
        assert_eq!(stop.landmark, Landmark::Conference);
        assert!(stop.attended);
        assert_eq!(game.cards.len(), 2, "one analyst card and the lead's");
        assert!(
            game.team.iter().all(|m| m.track.is_none()),
            "everyone is back"
        );
    }

    #[test]
    fn conference_cards_never_lower_posture() {
        for card in all_cards() {
            assert!(card.relief > 0, "{}", card.text);
            let mut game = game();
            let before = Area::ALL.map(|a| game.level(a));
            game.team[0].expertise = card.expertise;

            for (area, old) in Area::ALL.iter().zip(before) {
                assert!(game.level(*area) >= old, "{}", card.text);
            }
        }
    }

    #[test]
    fn every_card_turns_up_and_never_twice_in_one_conference() {
        let mut seen = Vec::new();
        for seed in 0..40 {
            for track in Track::ALL {
                let mut game = at(Landmark::Conference, seed, 30);
                game.team
                    .extend((0..5).map(|i| member(&format!("A{i}"), Role::Junior, 0, 0)));
                let picks: Vec<_> = (0..game.team.len()).map(|i| (i, track)).collect();
                game.attend(&picks);
                while game.conference_days > 0 {
                    game.advance();
                }

                let texts: Vec<_> = game.cards.iter().map(|c| c.text.clone()).collect();
                for (i, text) in texts.iter().enumerate() {
                    assert!(!texts[i + 1..].contains(text), "duplicate: {text}");
                }
                seen.extend(texts);
            }
        }

        for card in all_cards() {
            let tail = card.text.split("{name}").last().unwrap();
            assert!(
                seen.iter().any(|t| t.ends_with(tail)),
                "never drawn: {}",
                card.text
            );
        }
        for lead in LEAD_CARDS {
            assert!(
                seen.iter().any(|t| t == lead.text),
                "never drawn: {}",
                lead.text
            );
        }
    }

    #[test]
    fn expertise_lasts_while_the_analyst_stays() {
        let mut game = game();
        game.team[0].expertise = Some(Area::Perimeter);
        assert_eq!(game.level(Area::Perimeter), EXPERTISE_BONUS);
        advance_to(&mut game, 9);
        assert_eq!(game.level(Area::Perimeter), EXPERTISE_BONUS, "no decay");

        game.team[0].burnout = 99;
        game.advance();
        assert_eq!(game.level(Area::Perimeter), 0);
        assert!(
            game.log.iter().any(|e| e.contains("Maya")
                && e.ends_with("They took their Perimeter expertise with them."))
        );
    }

    #[test]
    fn attending_analysts_come_back_with_their_card() {
        let game = conference(3, &[(0, Track::Talks)]);
        let card = &game.cards[0];

        assert!(card.text.starts_with("Maya"));
        assert!(card.effect.ends_with("expertise, burnout -5"));
        assert!(
            game.team[0].expertise.is_some(),
            "every talk teaches something"
        );
        assert_eq!(game.team[0].burnout, 20 - 5);
    }

    #[test]
    fn the_watch_thins_and_the_home_team_works_harder_during_the_conference() {
        let mut game = game();
        game.team.push(member("Ann", Role::Junior, 20, 0));
        let normal = game.watch();
        let mut stayed = game.clone();
        stayed.day = 2;
        stayed.advance();

        game.day = 2;
        game.attend(&[(0, Track::Hallway), (1, Track::Hallway)]);
        assert_eq!(game.watch(), normal * 2 / 4);
        game.advance();

        assert_eq!(game.team[0].burnout, 20, "away analysts do not work");
        assert!(game.team[2].burnout > stayed.team[2].burnout);
    }

    #[test]
    fn every_lead_card_pays_off_as_promised() {
        for (i, lead) in LEAD_CARDS.iter().enumerate() {
            let mut game = (0..)
                .map(|seed| conference(seed, &[]))
                .find(|g| g.cards[0].text == lead.text)
                .unwrap();
            let trust_before = 60;
            assert_eq!(game.trust, trust_before + lead.trust, "card {i}");

            let edr = if lead.discount { 48_000 } else { 60_000 };
            assert_eq!(game.price(Item::Edr), edr, "card {i}");
            let fee = match (lead.recruit, lead.discount) {
                (true, _) => 0,
                (false, true) => Item::Senior.price() * 80 / 100,
                (false, false) => Item::Senior.price(),
            };
            assert_eq!(game.price(Item::Senior), fee, "card {i}");

            let budget = game.budget;
            game.hire(Item::Senior, "Priya");
            assert_eq!(game.budget, budget - fee);
            assert_ne!(game.price(Item::Senior), 0, "only one free hire");

            game.leave_fort();
            assert_eq!(game.price(Item::Edr), 60_000, "perks end with the visit");
        }
    }

    #[test]
    fn skipping_the_conference_opens_the_fort() {
        let mut game = at(Landmark::Conference, 1, 30);

        game.skip_conference();

        assert!(game.stop.as_ref().unwrap().attended);
        assert_eq!(game.budget, 500_000);
    }

    #[test]
    fn cards_are_revealed_one_at_a_time() {
        let mut game = conference(1, &[(0, Track::Villages)]);

        game.reveal_next();
        assert_eq!(game.cards.len(), 1);
        game.reveal_next();
        assert!(game.cards.is_empty());
    }

    #[test]
    fn free_choices_stay_open_when_the_budget_is_negative() {
        let mut game = equipped();
        game.budget = -5_000;

        assert!(game.actions().contains(&PATCH));
        assert!(!game.actions().contains(&PHISH));
        game.incident = Some(Actor::Ransomware);
        assert!(game.can_respond(&Actor::Ransomware.responses()[2]));
        assert!(!game.can_respond(&Actor::Ransomware.responses()[1]));
        game.incident = None;
        let mut river = at(Landmark::Audit, 1, 30);
        river.budget = -5_000;
        assert!(river.can_cross(0) && !river.can_cross(1));
    }

    #[test]
    fn log_entries_carry_their_day() {
        let mut game = game();
        advance_to(&mut game, 8);

        let payday = game.log.iter().find(|e| e.starts_with("Payday")).unwrap();
        assert_eq!(payday.day, 8);
        assert_eq!(game.log[0].day, 1);
    }

    #[test]
    fn result_cards_describe_what_visibly_changed() {
        let mut tabletop = game();
        tabletop.deployed.push(Tool::new(Item::Runbooks));
        run(&mut tabletop, TABLETOP);
        let card = tabletop.cards.last().unwrap();
        assert_eq!(card.title, "TABLETOP EXERCISE");
        assert_eq!(card.effect, "Trust +2. Resilience improved.");
        assert!(tabletop.log.iter().any(|e| e.text == card.text));

        let mut rested = game();
        rested.team.iter_mut().for_each(|m| m.burnout = 50);
        run(&mut rested, Action::DayOff);
        assert!(
            rested
                .cards
                .last()
                .unwrap()
                .effect
                .contains("Team burnout -")
        );

        let mut hit = game();
        hit.incident = Some(Actor::Ransomware);
        hit.respond(1);
        let card = hit.cards.last().unwrap();
        assert_eq!(card.title, "PAY THE RANSOM ($150K)");
        assert!(card.effect.contains("Budget -$150K"));
    }

    #[test]
    fn hidden_posture_only_reads_as_better_or_worse() {
        let mut game = game();
        let before = game.glance();
        game.bonus[Area::Identity as usize] += 7;
        game.bonus[Area::Perimeter as usize] -= 3;

        assert_eq!(
            game.changes(&before),
            "Identity improved. Perimeter weakened."
        );
        assert_eq!(game.clone().changes(&game.glance()), "No visible change.");
    }

    #[test]
    fn firing_an_analyst_costs_a_week_of_salary_and_saves_payroll() {
        let mut game = game();
        game.team[1].expertise = Some(Area::Endpoint);
        let level = game.level(Area::Endpoint);

        game.fire(1);

        assert_eq!(game.analysts(), 2);
        assert_eq!(game.budget, 500_000 - 6_000, "a week of severance");
        assert_eq!(game.payroll(), 12_000);
        assert_eq!(game.level(Area::Endpoint), level - EXPERTISE_BONUS);
        let card = game.cards.last().unwrap();
        assert_eq!(card.title, "LET GO");
        assert!(card.effect.contains("Dev leaves the team"));
        assert!(game.log.last().unwrap().contains("Endpoint expertise"));
    }

    #[test]
    fn fired_analysts_go_out_in_different_ways() {
        let exits: Vec<String> = (0..100)
            .map(|seed| {
                let mut game = GameState { seed, ..game() };
                game.fire(1);
                game.log.last().unwrap().text.clone()
            })
            .collect();

        for exit in &exits {
            assert!(exit.contains("Dev"), "{exit}");
            assert!(!exit.contains("{name}"), "{exit}");
        }
        for template in LETTING_GO {
            let tail = template.split("{name}").last().unwrap();
            assert!(exits.iter().any(|e| e.ends_with(tail)), "never: {template}");
        }
    }

    #[test]
    fn only_present_analysts_can_be_fired_and_never_the_last() {
        let mut game = game();
        assert!(game.can_fire(0));
        assert!(!game.can_fire(3), "leadership is not yours to fire");
        assert!(!game.can_fire(99));

        game.team[0].track = Some(Track::Talks);
        assert!(!game.can_fire(0), "away at the conference");
        game.team[0].track = None;

        game.budget = 5_999;
        assert!(!game.can_fire(0), "no money for severance");
        game.budget = 500_000;

        game.fire(0);
        game.fire(0);
        assert_eq!(game.analysts(), 1);
        assert!(!game.can_fire(0), "someone has to watch the alerts");
    }

    #[test]
    fn every_area_can_be_improved_and_stops_someone() {
        for area in Area::ALL {
            let improved_by = |boosts: &[(Area, i32)]| boosts.iter().any(|&(a, _)| a == area);
            assert!(
                Item::ALL.iter().any(|i| improved_by(i.boosts())),
                "no tool for {area}"
            );
            assert!(!area.stops().is_empty(), "{area} stops nobody");
        }
        assert_eq!(Area::Perimeter.stops(), [Actor::Hacktivists]);
    }

    #[test]
    fn the_pen_test_grades_are_kept_from_that_day() {
        let mut game = game();
        game.bonus = [85, 65, 45, 25, 5, 100];
        game.day = Landmark::PenTest.day() - 1;
        game.next_landmark = 4;

        game.advance();
        game.bonus = [0; 6];

        let (day, grades) = game.pen_test.unwrap();
        assert_eq!(day, Landmark::PenTest.day());
        assert_eq!(grades[0], (Area::Identity, 'A'));
        assert_eq!(grades[4], (Area::Resilience, 'F'));
    }

    #[test]
    fn defenders_list_tools_and_experts_for_an_area() {
        let mut game = game();
        assert!(game.defenders(Area::Endpoint).is_empty());

        game.owned.push(Item::Edr);
        game.team[0].expertise = Some(Area::Endpoint);
        assert_eq!(
            game.defenders(Area::Endpoint),
            ["EDR (not deployed)", "Maya's expertise"]
        );
        game.deployed.push(Tool::new(Item::Edr));
        assert_eq!(game.defenders(Area::Detection), ["EDR (fresh)"]);
        game.deployed[0].condition = 60;
        assert_eq!(game.defenders(Area::Detection), ["EDR (aging)"]);
        game.deployed[0].condition = 30;
        assert_eq!(game.defenders(Area::Detection), ["EDR (stale)"]);
    }

    #[test]
    fn every_defense_has_tools_at_different_prices() {
        for area in Area::ALL {
            let tools: Vec<Item> = Item::ALL
                .into_iter()
                .filter(|i| i.category() == Some(area))
                .collect();
            assert!(tools.len() >= 2, "{area}");
            let mut prices: Vec<i64> = tools.iter().map(|t| t.price()).collect();
            prices.dedup();
            assert_eq!(prices.len(), tools.len(), "{area} prices differ");
            for tool in tools {
                assert_eq!(
                    tool.boosts()[0].0,
                    area,
                    "{tool:?} improves its own area first"
                );
            }
        }
        for item in Item::ALL {
            assert_eq!(
                item.category().is_some(),
                !item.boosts().is_empty(),
                "{item:?}"
            );
        }
    }

    #[test]
    fn deploying_any_tool_applies_exactly_its_boosts() {
        for tool in Item::ALL.into_iter().filter(|i| i.category().is_some()) {
            let mut game = game();
            game.owned.push(tool);
            let mut control = game.clone();

            run(&mut game, Action::Deploy(tool));
            advance_to(&mut control, game.day);

            for area in Area::ALL {
                let boost: i32 = tool
                    .boosts()
                    .iter()
                    .filter(|(a, _)| *a == area)
                    .map(|(_, b)| b)
                    .sum();
                assert_eq!(
                    game.level(area),
                    (control.level(area) + boost).min(100),
                    "{tool:?} {area}"
                );
            }
        }
    }

    #[test]
    fn actions_are_listed_by_kind() {
        let mut game = game();
        game.owned.push(Item::Edr);
        game.deployed.push(Tool {
            item: Item::VulnScanner,
            condition: 50,
        });
        game.conditions.push(Condition::Downtime);

        let kinds: Vec<Kind> = game.actions().iter().map(|a| a.kind()).collect();

        assert_eq!(kinds.first(), Some(&Kind::Recovery));
        assert!(kinds.windows(2).all(|w| w[0] <= w[1]), "{kinds:?}");
        for kind in [
            Kind::Implementation,
            Kind::Maintenance,
            Kind::Operations,
            Kind::Management,
        ] {
            assert!(kinds.contains(&kind), "{kind}");
        }
    }

    #[test]
    fn money_is_shown_in_short_units() {
        assert_eq!(money(10), "$10");
        assert_eq!(money(18_000), "$18K");
        assert_eq!(money(1_500_000), "$1.5M");
        assert_eq!(money(600_000_000), "$600M");
        assert_eq!(money(1_200_000_000), "$1.20B");
    }
}
