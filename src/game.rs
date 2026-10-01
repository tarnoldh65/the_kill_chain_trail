use std::fmt;

/// Fatigue every team member gains each stage regardless of the choice.
const BASE_FATIGUE: i32 = 3;
/// Extra fatigue when the coffee runs out.
const NO_COFFEE_FATIGUE: i32 = 15;
/// Containment each remaining analyst adds per stage.
const ANALYST_CONTAINMENT: i32 = 2;
/// Trust below this gets the CISO fired.
const FIRING_TRUST: i32 = 30;
/// Budget below this gets the Manager made redundant.
const REDUNDANCY_BUDGET: i32 = 20;

/// Pots of coffee the break room can hold.
pub const COFFEE_CAPACITY: i32 = 36;
/// Cost of sending the intern across the street for coffee.
pub const COFFEE_RUN_COST: i32 = 10;
/// Pots of coffee the intern brings back.
const COFFEE_RUN_POTS: i32 = 12;

/// What happens to an intern who loses to traffic, rotated by stage.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Reconnaissance,
    Weaponization,
    Delivery,
    Exploitation,
    Installation,
    CommandAndControl,
    ActionsOnObjectives,
}

impl Stage {
    pub const ALL: [Stage; 7] = [
        Self::Reconnaissance,
        Self::Weaponization,
        Self::Delivery,
        Self::Exploitation,
        Self::Installation,
        Self::CommandAndControl,
        Self::ActionsOnObjectives,
    ];

    fn next(self) -> Self {
        Self::ALL[(self as usize + 1).min(Self::ALL.len() - 1)]
    }

    /// Containment the team must reach by the end of this stage to stay ahead of the attacker.
    pub fn target(self) -> i32 {
        12 * (self as i32 + 1)
    }

    pub fn choices(self) -> [Choice; 6] {
        match self {
            Self::Reconnaissance => RECONNAISSANCE,
            Self::Weaponization => WEAPONIZATION,
            Self::Delivery => DELIVERY,
            Self::Exploitation => EXPLOITATION,
            Self::Installation => INSTALLATION,
            Self::CommandAndControl => COMMAND_AND_CONTROL,
            Self::ActionsOnObjectives => ACTIONS_ON_OBJECTIVES,
        }
    }

    pub fn events(self) -> [Event; 4] {
        match self {
            Self::Reconnaissance => RECONNAISSANCE_EVENTS,
            Self::Weaponization => WEAPONIZATION_EVENTS,
            Self::Delivery => DELIVERY_EVENTS,
            Self::Exploitation => EXPLOITATION_EVENTS,
            Self::Installation => INSTALLATION_EVENTS,
            Self::CommandAndControl => COMMAND_AND_CONTROL_EVENTS,
            Self::ActionsOnObjectives => ACTIONS_ON_OBJECTIVES_EVENTS,
        }
    }
}

/// A well-mixed pseudo-random number from a seed and a salt (the murmur3 finalizer).
fn roll(seed: u32, salt: u32) -> u32 {
    let mut x = seed ^ salt.wrapping_mul(0x9E37_79B9);
    x ^= x >> 16;
    x = x.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2_AE35);
    x ^ x >> 16
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Reconnaissance => "Reconnaissance",
            Self::Weaponization => "Weaponization",
            Self::Delivery => "Delivery",
            Self::Exploitation => "Exploitation",
            Self::Installation => "Installation",
            Self::CommandAndControl => "Command and Control (C2)",
            Self::ActionsOnObjectives => "Actions on Objectives",
        })
    }
}

/// What a choice looks like in its report illustration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scene {
    Hunt,
    Spend,
    Sleep,
    Patch,
    Phish,
    Coffee,
    Block,
    Isolate,
    Unplug,
    Press,
}

/// The worst thing that happened on a turn, ordered by severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Setback {
    Behind,
    Fired,
    Departed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Choice {
    pub label: &'static str,
    pub text: &'static str,
    pub scene: Scene,
    pub cost: i32,
    pub coffee: i32,
    pub fatigue: i32,
    pub containment: i32,
    pub trust: i32,
    pub brand: i32,
}

const NONE: Choice = Choice {
    label: "",
    text: "",
    scene: Scene::Hunt,
    cost: 0,
    coffee: 0,
    fatigue: 0,
    containment: 0,
    trust: 0,
    brand: 0,
};

const RECONNAISSANCE: [Choice; 6] = [
    Choice {
        label: "Hunt for scanning activity",
        text: "The team combs the firewall logs and finds a port scan from a server in a country nobody can pronounce.",
        scene: Scene::Hunt,
        cost: 10,
        fatigue: 10,
        containment: 8,
        ..NONE
    },
    Choice {
        label: "Buy a threat intel feed",
        text: "The vendor promised AI-powered, blockchain-ready threat intel. It is a CSV file. It is useful anyway.",
        scene: Scene::Spend,
        cost: 30,
        containment: 7,
        trust: 5,
        ..NONE
    },
    Choice {
        label: "Ignore the noise and rest up",
        text: "Everyone goes home early. The attacker, sadly, does not.",
        scene: Scene::Sleep,
        fatigue: -12,
        trust: -5,
        ..NONE
    },
    Choice {
        label: "Scan your own perimeter first",
        text: "You find 14 forgotten servers, a printer from 2008, and a Minecraft server in Finance.",
        scene: Scene::Hunt,
        cost: 5,
        fatigue: 8,
        containment: 6,
        ..NONE
    },
    Choice {
        label: "Set up a honeypot",
        text: "The attacker falls for a decoy file named passwords_FINAL_v2.xlsx. So does the CFO.",
        scene: Scene::Spend,
        cost: 20,
        fatigue: 4,
        containment: 8,
        ..NONE
    },
    Choice {
        label: "Ask the CIO what they think",
        text: "The CIO suggests moving everything to the cloud. The meeting runs 90 minutes.",
        scene: Scene::Press,
        containment: 2,
        trust: 5,
        ..NONE
    },
];

const WEAPONIZATION: [Choice; 6] = [
    Choice {
        label: "Patch the internet-facing servers",
        text: "Servers are patched overnight. Three of them now need a reboot nobody approved.",
        scene: Scene::Patch,
        cost: 5,
        fatigue: 12,
        containment: 8,
        ..NONE
    },
    Choice {
        label: "Run a phishing awareness drill",
        text: "Staff click the fake phish at a record 42% rate. The CEO clicked twice.",
        scene: Scene::Phish,
        cost: 15,
        fatigue: 3,
        containment: 5,
        brand: 5,
        ..NONE
    },
    Choice {
        label: "Order pizza and restock coffee",
        text: "Pizza arrives and morale improves. Someone ordered pineapple on half of it and a feud begins.",
        scene: Scene::Coffee,
        cost: 10,
        coffee: 12,
        fatigue: -8,
        containment: 1,
        ..NONE
    },
    Choice {
        label: "Update the antivirus signatures",
        text: "Signatures are updated. The antivirus can now detect 2011 with great confidence.",
        scene: Scene::Patch,
        cost: 5,
        fatigue: 6,
        containment: 6,
        ..NONE
    },
    Choice {
        label: "Brief the board on the threat",
        text: "The board nods along and asks if this is related to the blockchain.",
        scene: Scene::Press,
        fatigue: 2,
        containment: 3,
        trust: 8,
        ..NONE
    },
    Choice {
        label: "Buy everyone energy drinks",
        text: "Analysts can now hear colors. Productivity is up. Heart rates are concerning.",
        scene: Scene::Coffee,
        cost: 5,
        coffee: 6,
        fatigue: -6,
        containment: 3,
        ..NONE
    },
];

const DELIVERY: [Choice; 6] = [
    Choice {
        label: "Quarantine the phishing emails",
        text: "Hundreds of \"Urgent Invoice\" emails vanish. Accounts payable finds the silence suspicious.",
        scene: Scene::Phish,
        cost: 5,
        fatigue: 10,
        containment: 8,
        ..NONE
    },
    Choice {
        label: "Hire an incident response firm",
        text: "Consultants arrive in matching fleece vests and bill you for the parking.",
        scene: Scene::Spend,
        cost: 40,
        containment: 10,
        trust: 5,
        ..NONE
    },
    Choice {
        label: "Send the team home to sleep",
        text: "The team sleeps for the first time in days. The phishing emails keep arriving.",
        scene: Scene::Sleep,
        fatigue: -20,
        trust: -10,
        ..NONE
    },
    Choice {
        label: "Block suspicious attachments",
        text: "All .zip files are blocked. Marketing can no longer send their 4GB brochure.",
        scene: Scene::Block,
        cost: 5,
        fatigue: 8,
        containment: 7,
        brand: -3,
        ..NONE
    },
    Choice {
        label: "Turn on MFA for everyone",
        text: "MFA is on. The CEO approves 47 push prompts by accident, then complains about MFA.",
        scene: Scene::Isolate,
        cost: 15,
        fatigue: 10,
        containment: 10,
        brand: -5,
        ..NONE
    },
    Choice {
        label: "Tell users to think before clicking",
        text: "An all-staff email goes out. It is flagged as phishing and nobody reads it.",
        scene: Scene::Phish,
        containment: 2,
        trust: -3,
        ..NONE
    },
];

const EXPLOITATION: [Choice; 6] = [
    Choice {
        label: "Isolate the phished laptops",
        text: "Infected laptops are yanked off the network, including the one running the CFO's slideshow.",
        scene: Scene::Isolate,
        cost: 10,
        fatigue: 12,
        containment: 9,
        brand: -5,
        ..NONE
    },
    Choice {
        label: "Deploy emergency EDR everywhere",
        text: "EDR lands on every endpoint and immediately flags the printer as a nation-state actor.",
        scene: Scene::Spend,
        cost: 35,
        fatigue: 5,
        containment: 10,
        ..NONE
    },
    Choice {
        label: "Coffee run and a nap rotation",
        text: "Fresh coffee arrives. Naps are scheduled in 20 minute shifts under desks.",
        scene: Scene::Coffee,
        cost: 10,
        coffee: 12,
        fatigue: -10,
        containment: 2,
        ..NONE
    },
    Choice {
        label: "Emergency patch the exploited bug",
        text: "The vendor patch arrives with a 600-page readme. It installs on the third try.",
        scene: Scene::Patch,
        cost: 10,
        fatigue: 12,
        containment: 10,
        ..NONE
    },
    Choice {
        label: "Disable the vulnerable service",
        text: "The service is off. So is the customer portal, which needed it for reasons nobody remembers.",
        scene: Scene::Unplug,
        fatigue: 6,
        containment: 8,
        brand: -8,
        ..NONE
    },
    Choice {
        label: "Hold an all-hands war room",
        text: "Forty people join the call. Six talk. One is on mute explaining the fix to nobody.",
        scene: Scene::Press,
        cost: 5,
        fatigue: 5,
        containment: 6,
        trust: 3,
        ..NONE
    },
];

const INSTALLATION: [Choice; 6] = [
    Choice {
        label: "Reimage the infected servers",
        text: "Infected servers are wiped and rebuilt. Nobody backed up the wiki. Nobody notices.",
        scene: Scene::Patch,
        cost: 10,
        fatigue: 15,
        containment: 10,
        brand: -5,
        ..NONE
    },
    Choice {
        label: "Hunt for persistence with the IR firm",
        text: "The IR firm finds a backdoor, a rogue scheduled task, and a crypto miner from 2019.",
        scene: Scene::Hunt,
        cost: 30,
        fatigue: 5,
        containment: 9,
        ..NONE
    },
    Choice {
        label: "Blame the intern and go to bed",
        text: "The intern is blamed for everything. The intern started yesterday.",
        scene: Scene::Sleep,
        fatigue: -15,
        trust: -5,
        ..NONE
    },
    Choice {
        label: "Rotate every service account",
        text: "Service accounts are rotated. Three apps break, including the one that orders coffee.",
        scene: Scene::Isolate,
        cost: 5,
        coffee: -3,
        fatigue: 8,
        containment: 10,
        ..NONE
    },
    Choice {
        label: "Restore from backups",
        text: "Backups are restored. They were last tested in 2017, but somehow they work.",
        scene: Scene::Patch,
        cost: 15,
        fatigue: 6,
        containment: 8,
        ..NONE
    },
    Choice {
        label: "Pay an MSSP to babysit overnight",
        text: "The managed provider watches your alerts all night and sends a 90-page PDF at dawn.",
        scene: Scene::Spend,
        cost: 25,
        fatigue: -8,
        containment: 6,
        ..NONE
    },
];

const COMMAND_AND_CONTROL: [Choice; 6] = [
    Choice {
        label: "Block the C2 servers at the firewall",
        text: "Firewall rules cut off the attacker's servers. They spin up new ones, but it slows them down.",
        scene: Scene::Block,
        cost: 15,
        fatigue: 10,
        containment: 9,
        ..NONE
    },
    Choice {
        label: "Sinkhole the attacker's domains",
        text: "The attacker's domains now point at you. Their malware phones home to your SOC instead.",
        scene: Scene::Block,
        cost: 30,
        fatigue: 5,
        containment: 10,
        trust: 5,
        ..NONE
    },
    Choice {
        label: "Pull the internet connection",
        text: "The internet is cut. The attacker and every customer are locked out equally.",
        scene: Scene::Unplug,
        fatigue: 5,
        containment: 14,
        brand: -15,
        ..NONE
    },
    Choice {
        label: "Watch the C2 traffic quietly",
        text: "You watch the attacker work. They type slower than your analysts. Morale improves.",
        scene: Scene::Hunt,
        fatigue: 10,
        containment: 7,
        ..NONE
    },
    Choice {
        label: "Block DNS to brand-new domains",
        text: "Newly registered domains are blocked, including the CEO's pet startup.",
        scene: Scene::Block,
        cost: 10,
        fatigue: 6,
        containment: 9,
        brand: -3,
        ..NONE
    },
    Choice {
        label: "Call the FBI",
        text: "The FBI is very interested. They ask for the logs you deleted last spring.",
        scene: Scene::Press,
        fatigue: 3,
        containment: 5,
        trust: 5,
        ..NONE
    },
];

const ACTIONS_ON_OBJECTIVES: [Choice; 6] = [
    Choice {
        label: "Reset every domain password",
        text: "Every password is reset. The help desk queue hits 4,000 tickets before lunch.",
        scene: Scene::Isolate,
        cost: 5,
        fatigue: 15,
        containment: 12,
        brand: -10,
        ..NONE
    },
    Choice {
        label: "Segment the network",
        text: "The network is carved into zones. The attacker is trapped in a VLAN with the vending machines.",
        scene: Scene::Block,
        cost: 35,
        fatigue: 10,
        containment: 11,
        ..NONE
    },
    Choice {
        label: "Brief the press early",
        text: "Leadership gets ahead of the story. The headline reads \"Company Mostly Fine, Probably\".",
        scene: Scene::Press,
        cost: 10,
        trust: 10,
        brand: 15,
        ..NONE
    },
    Choice {
        label: "Encrypt the crown jewels",
        text: "The sensitive data is encrypted. The key is on a sticky note, but a different sticky note.",
        scene: Scene::Isolate,
        cost: 20,
        fatigue: 10,
        containment: 11,
        ..NONE
    },
    Choice {
        label: "Kill every active session",
        text: "Every session is terminated, including the attacker's and the CEO's call with investors.",
        scene: Scene::Unplug,
        fatigue: 15,
        containment: 11,
        brand: -6,
        ..NONE
    },
    Choice {
        label: "Offer the attacker a job",
        text: "The attacker declines, but refers a friend. HR calls it a pipeline win.",
        scene: Scene::Spend,
        cost: 15,
        containment: 8,
        trust: -5,
        brand: 5,
        ..NONE
    },
];

/// Something that happens alongside a decision, for better or worse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub text: &'static str,
    pub budget: i32,
    pub coffee: i32,
    pub fatigue: i32,
    pub containment: i32,
    pub trust: i32,
    pub brand: i32,
}

/// No event at all.
const CALM: Event = Event {
    text: "",
    budget: 0,
    coffee: 0,
    fatigue: 0,
    containment: 0,
    trust: 0,
    brand: 0,
};

const RECONNAISSANCE_EVENTS: [Event; 4] = [
    Event {
        text: "Meanwhile, a pen tester you forgot you hired files a report. It is surprisingly useful.",
        containment: 4,
        ..CALM
    },
    Event {
        text: "Meanwhile, the office Wi-Fi password appears on a sticky note in a public Instagram photo.",
        containment: -4,
        trust: -3,
        ..CALM
    },
    Event {
        text: "Meanwhile, nothing else happens. Everyone finds this deeply suspicious.",
        ..CALM
    },
    Event {
        text: "Meanwhile, the vending machine eats $5 of the budget. Nobody can prove it.",
        budget: -5,
        ..CALM
    },
];

const WEAPONIZATION_EVENTS: [Event; 4] = [
    Event {
        text: "Meanwhile, a researcher tweets about the attacker's malware. Your SOC reads it first.",
        containment: 4,
        ..CALM
    },
    Event {
        text: "Meanwhile, Facilities schedules a fire drill during the patch window.",
        fatigue: 5,
        ..CALM
    },
    Event {
        text: "Meanwhile, Finance approves a surprise budget top-up after a very scary slide deck.",
        budget: 15,
        ..CALM
    },
    Event {
        text: "Meanwhile, a calendar invite titled \"quick sync\" lasts two hours.",
        fatigue: 3,
        ..CALM
    },
];

const DELIVERY_EVENTS: [Event; 4] = [
    Event {
        text: "Meanwhile, a user forwards the phish to all staff asking \"is this legit?\"",
        containment: -5,
        ..CALM
    },
    Event {
        text: "Meanwhile, the spam filter catches a wave of phish on its own. Nobody is more surprised than the vendor.",
        containment: 5,
        ..CALM
    },
    Event {
        text: "Meanwhile, someone microwaves fish in the break room. Morale plummets.",
        fatigue: 5,
        ..CALM
    },
    Event {
        text: "Meanwhile, a customer praises your response on social media. It was an accident, but you take it.",
        brand: 5,
        ..CALM
    },
];

const EXPLOITATION_EVENTS: [Event; 4] = [
    Event {
        text: "Meanwhile, the coffee machine catches fire. Facilities says it is \"not a priority\".",
        coffee: -6,
        ..CALM
    },
    Event {
        text: "Meanwhile, the exploit crashes on your ancient servers. Legacy tech saves the day.",
        containment: 5,
        ..CALM
    },
    Event {
        text: "Meanwhile, the CEO asks for hourly updates, in person, with slides.",
        fatigue: 6,
        trust: 3,
        ..CALM
    },
    Event {
        text: "Meanwhile, a vendor's free \"AI SOC\" trial flags itself as malware.",
        ..CALM
    },
];

const INSTALLATION_EVENTS: [Event; 4] = [
    Event {
        text: "Meanwhile, an analyst finds the attacker's to-do list. It is better organized than yours.",
        containment: 5,
        ..CALM
    },
    Event {
        text: "Meanwhile, Windows decides now is the time for updates. Every machine reboots.",
        fatigue: 5,
        containment: -3,
        ..CALM
    },
    Event {
        text: "Meanwhile, Legal asks everyone to preserve evidence and to please stop deleting things.",
        trust: 3,
        ..CALM
    },
    Event {
        text: "Meanwhile, a board member's nephew offers to help because he is \"good with computers\".",
        trust: -3,
        ..CALM
    },
];

const COMMAND_AND_CONTROL_EVENTS: [Event; 4] = [
    Event {
        text: "Meanwhile, the attacker's server goes down for maintenance. Even hackers have change windows.",
        containment: 5,
        ..CALM
    },
    Event {
        text: "Meanwhile, the attacker starts tunneling over DNS. Your entire DNS team is one guy named Gary.",
        containment: -5,
        ..CALM
    },
    Event {
        text: "Meanwhile, a pizza shop mistakes the SOC for a party and delivers 20 free pizzas.",
        fatigue: -5,
        ..CALM
    },
    Event {
        text: "Meanwhile, a reporter calls for comment. PR says \"no comment\" with great confidence.",
        brand: -4,
        ..CALM
    },
];

const ACTIONS_ON_OBJECTIVES_EVENTS: [Event; 4] = [
    Event {
        text: "Meanwhile, the attacker posts a ransom note in Comic Sans. The board is more offended by the font.",
        trust: -3,
        ..CALM
    },
    Event {
        text: "Meanwhile, cyber insurance agrees to cover part of the bill after only 40 forms.",
        budget: 20,
        ..CALM
    },
    Event {
        text: "Meanwhile, an analyst spots data leaving at 3 AM and pulls the plug just in time.",
        containment: 6,
        ..CALM
    },
    Event {
        text: "Meanwhile, the attacker leaks the org chart. Everyone learns their manager's real title.",
        brand: -5,
        ..CALM
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Analyst,
    Manager,
    Ciso,
    Cio,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Analyst => "Analyst",
            Self::Manager => "Manager",
            Self::Ciso => "CISO",
            Self::Cio => "CIO",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamMember {
    pub name: &'static str,
    pub role: Role,
    pub burnout: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Contained,
    Breached,
    TeamCollapsed,
    Fired,
    Bankrupt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub company: String,
    pub lead: String,
    pub stage: Stage,
    pub budget: i32,
    pub coffee: i32,
    pub trust: i32,
    pub brand: i32,
    pub containment: i32,
    pub team: Vec<TeamMember>,
    pub log: Vec<String>,
    pub outcome: Option<Outcome>,
    /// Whether the intern has already been sent for coffee this stage.
    pub coffee_run_made: bool,
    /// Picks which choices are offered and which events happen.
    pub seed: u32,
}

impl GameState {
    pub fn new(company: &str, lead: &str, seed: u32) -> Self {
        let member = |name, role, burnout| TeamMember {
            name,
            role,
            burnout,
        };
        Self {
            company: company.to_string(),
            lead: lead.to_string(),
            stage: Stage::Reconnaissance,
            budget: 150,
            coffee: 24,
            trust: 60,
            brand: 60,
            containment: 0,
            team: vec![
                member("Maya", Role::Analyst, 40),
                member("Dev", Role::Analyst, 30),
                member("Sam", Role::Analyst, 20),
                member("Jules", Role::Manager, 30),
                member("Ravi", Role::Ciso, 25),
                member("Dana", Role::Cio, 10),
            ],
            log: vec![format!(
                "Suspicious traffic is hitting {company}. {lead} takes command of the SOC."
            )],
            outcome: None,
            coffee_run_made: false,
            seed,
        }
    }

    /// The three choices offered at this stage, picked by the seed.
    pub fn options(&self) -> [Choice; 3] {
        let mut pool = self.stage.choices();
        for i in 0..3 {
            let salt = self.stage as u32 * 8 + i as u32;
            let j = i + roll(self.seed, salt) as usize % (pool.len() - i);
            pool.swap(i, j);
        }
        [pool[0], pool[1], pool[2]]
    }

    pub fn analysts(&self) -> i32 {
        self.team.iter().filter(|m| m.role == Role::Analyst).count() as i32
    }

    pub fn can_afford(&self, choice: &Choice) -> bool {
        choice.cost <= self.budget
    }

    pub fn coffee_full(&self) -> bool {
        self.coffee >= COFFEE_CAPACITY
    }

    pub fn can_send_intern(&self) -> bool {
        self.outcome.is_none()
            && !self.coffee_run_made
            && !self.coffee_full()
            && self.budget >= COFFEE_RUN_COST
    }

    /// Pays for a coffee run; the intern still has to survive the street.
    pub fn send_intern(&mut self) {
        self.budget -= COFFEE_RUN_COST;
        self.coffee_run_made = true;
    }

    pub fn intern_returns(&mut self, survived: bool) {
        if survived {
            self.coffee = (self.coffee + COFFEE_RUN_POTS).min(COFFEE_CAPACITY);
            self.log.push(format!(
                "The intern made it back with {COFFEE_RUN_POTS} pots of coffee and only minor tire marks."
            ));
        } else {
            self.log
                .push(INTERN_FATES[self.stage as usize % INTERN_FATES.len()].to_string());
        }
        self.check_bankrupt();
    }

    /// Ends the game when none of the offered choices is affordable.
    fn check_bankrupt(&mut self) {
        if self.outcome.is_none() && !self.options().iter().any(|c| self.can_afford(c)) {
            self.outcome = Some(Outcome::Bankrupt);
        }
    }

    /// Plays a choice alongside a random event for this stage and returns the worst setback.
    pub fn play(&mut self, choice: &Choice) -> Option<Setback> {
        let events = self.stage.events();
        let salt = [
            self.stage as i32,
            self.budget,
            self.containment,
            self.coffee,
        ]
        .iter()
        .fold(0u32, |hash, &v| {
            hash.wrapping_mul(31).wrapping_add(v as u32)
        });
        let event = events[roll(self.seed, salt) as usize % events.len()];
        self.resolve(choice, &event)
    }

    fn resolve(&mut self, choice: &Choice, event: &Event) -> Option<Setback> {
        let mut setback = None;
        self.log.push(format!("{}: {}.", self.stage, choice.label));
        if !event.text.is_empty() {
            self.log.push(event.text.to_string());
        }

        self.budget = (self.budget - choice.cost + event.budget).max(0);
        self.coffee = (self.coffee + choice.coffee + event.coffee).min(COFFEE_CAPACITY)
            - self.team.len() as i32;
        self.trust = (self.trust + choice.trust + event.trust).clamp(0, 100);
        self.brand = (self.brand + choice.brand + event.brand).clamp(0, 100);
        self.containment = (self.containment
            + choice.containment
            + event.containment
            + ANALYST_CONTAINMENT * self.analysts())
        .clamp(0, 100);

        let mut fatigue = BASE_FATIGUE + choice.fatigue + event.fatigue;
        if self.coffee < 0 {
            self.coffee = 0;
            fatigue += NO_COFFEE_FATIGUE;
            self.log
                .push("The coffee ran out. Tempers are short.".to_string());
        }
        for member in &mut self.team {
            member.burnout = (member.burnout + fatigue).max(0);
        }

        if self.containment < self.stage.target() {
            setback = Some(Setback::Behind);
            self.trust = (self.trust - 15).max(0);
            self.brand = (self.brand - 10).max(0);
            self.log.push(
                "The attackers are ahead. The board and customers are losing patience.".to_string(),
            );
        }

        let mut exit = self.stage as usize;
        let log = &mut self.log;
        self.team.retain(|m| {
            if m.burnout >= 100 {
                let reason = BURNOUT_EXITS[exit % BURNOUT_EXITS.len()];
                log.push(format!("{} the {} {reason}", m.name, m.role));
                exit += 1;
                setback = Some(Setback::Departed);
            }
            m.burnout < 100
        });

        if self.budget < REDUNDANCY_BUDGET
            && let Some(i) = self.team.iter().position(|m| m.role == Role::Manager)
        {
            let manager = self.team.remove(i);
            setback = setback.max(Some(Setback::Fired));
            self.log.push(format!(
                "Finance made {} the Manager redundant to save money. The consultants who recommended it billed $80k.",
                manager.name
            ));
        }

        if self.trust < FIRING_TRUST
            && let Some(i) = self.team.iter().position(|m| m.role == Role::Ciso)
        {
            let ciso = self.team.remove(i);
            setback = setback.max(Some(Setback::Fired));
            self.log.push(format!(
                "The board fired {} the CISO over the handling of the incident.",
                ciso.name
            ));
        }

        self.outcome = if self.trust == 0 || self.brand == 0 {
            Some(Outcome::Fired)
        } else if self.analysts() == 0 {
            Some(Outcome::TeamCollapsed)
        } else if self.stage == Stage::ActionsOnObjectives {
            Some(if self.containment >= self.stage.target() {
                Outcome::Contained
            } else {
                Outcome::Breached
            })
        } else {
            None
        };

        if self.outcome.is_none() {
            self.stage = self.stage.next();
            self.coffee_run_made = false;
            self.check_bankrupt();
        }
        setback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::OnceLock;

    /// Games played per seed in the balance tests.
    const SEEDS: u32 = 40;

    fn game() -> GameState {
        GameState::new("Acme", "Alex", 1)
    }

    /// A new game at `stage` with the first seed whose offered choices pass `wanted`.
    fn game_offering(stage: Stage, wanted: impl Fn(&[Choice; 3]) -> bool) -> GameState {
        (0..)
            .map(|seed| GameState {
                stage,
                ..GameState::new("Acme", "Alex", seed)
            })
            .find(|game| wanted(&game.options()))
            .unwrap()
    }

    /// Plays the given choice index at each stage with no events.
    fn play_path(path: &[usize]) -> GameState {
        let mut game = game();
        for &i in path {
            let choice = game.stage.choices()[i];
            game.resolve(&choice, &CALM);
        }
        game
    }

    /// Plays every affordable offered path, with events, for many seeds and
    /// returns each path's choice labels with its outcome.
    fn all_endings() -> &'static [(Vec<&'static str>, Outcome)] {
        static ENDINGS: OnceLock<Vec<(Vec<&'static str>, Outcome)>> = OnceLock::new();
        ENDINGS.get_or_init(|| {
            let mut endings = Vec::new();
            let mut pending: Vec<_> = (0..SEEDS)
                .map(|seed| (GameState::new("Acme", "Alex", seed), Vec::new()))
                .collect();
            while let Some((game, path)) = pending.pop() {
                match game.outcome {
                    Some(outcome) => endings.push((path, outcome)),
                    None => {
                        for choice in game.options().iter().filter(|c| game.can_afford(c)) {
                            let mut next = game.clone();
                            next.play(choice);
                            pending.push((next, [path.as_slice(), &[choice.label]].concat()));
                        }
                    }
                }
            }
            endings
        })
    }

    #[test]
    fn new_game_starts_at_reconnaissance_with_full_roster() {
        let game = game();

        assert_eq!(game.stage, Stage::Reconnaissance);
        assert_eq!(game.analysts(), 3);
        assert!(game.team.iter().any(|m| m.role == Role::Ciso));
        assert!(game.team.iter().any(|m| m.role == Role::Cio));
        assert!(game.log[0].contains("Acme") && game.log[0].contains("Alex"));
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn each_stage_has_distinct_choices() {
        let labels: Vec<_> = Stage::ALL
            .iter()
            .flat_map(|s| s.choices())
            .map(|c| c.label)
            .collect();

        for (i, label) in labels.iter().enumerate() {
            assert!(!labels[i + 1..].contains(label), "duplicate choice {label}");
        }
    }

    #[test]
    fn playing_a_choice_applies_its_effects_and_advances() {
        let mut game = game();

        game.resolve(&RECONNAISSANCE[1], &CALM);

        assert_eq!(game.budget, 120);
        assert_eq!(game.coffee, 18);
        assert_eq!(game.trust, 65);
        assert_eq!(game.containment, 13);
        assert_eq!(game.team[0].burnout, 40 + BASE_FATIGUE);
        assert_eq!(game.stage, Stage::Weaponization);
    }

    #[test]
    fn falling_behind_the_attacker_costs_trust_and_brand() {
        let mut game = game();

        game.resolve(&RECONNAISSANCE[2], &CALM);

        assert_eq!(game.containment, 6);
        assert_eq!(game.trust, 40);
        assert_eq!(game.brand, 50);
    }

    #[test]
    fn choices_over_budget_are_unaffordable() {
        let mut game = game();
        game.budget = 20;

        assert!(game.can_afford(&RECONNAISSANCE[0]));
        assert!(!game.can_afford(&RECONNAISSANCE[1]));
    }

    #[test]
    fn running_out_of_coffee_adds_fatigue() {
        let mut game = game();
        game.coffee = 0;

        game.resolve(&RECONNAISSANCE[0], &CALM);

        assert_eq!(game.coffee, 0);
        assert_eq!(
            game.team[0].burnout,
            40 + BASE_FATIGUE + 10 + NO_COFFEE_FATIGUE
        );
        assert!(game.log.iter().any(|e| e.contains("coffee ran out")));
    }

    #[test]
    fn burned_out_members_quit() {
        let mut game = game();
        game.team[0].burnout = 95;

        game.resolve(&RECONNAISSANCE[0], &CALM);

        assert_eq!(game.analysts(), 2);
        assert!(
            game.log
                .iter()
                .any(|e| e == "Maya the Analyst burned out and quit to open a llama sanctuary.")
        );
    }

    #[test]
    fn departures_rotate_through_burnout_exits() {
        let mut game = game();
        game.stage = Stage::Weaponization;
        game.team[0].burnout = 95;
        game.team[1].burnout = 95;

        game.resolve(&WEAPONIZATION[0], &CALM);

        assert_eq!(game.analysts(), 1);
        assert!(
            game.log
                .iter()
                .any(|e| e.starts_with("Maya the Analyst had a heart attack"))
        );
        assert!(
            game.log
                .iter()
                .any(|e| e.starts_with("Dev the Analyst had a stroke"))
        );
    }

    #[test]
    fn low_budget_makes_the_manager_redundant() {
        let mut game = game();
        game.budget = 25;

        game.resolve(&RECONNAISSANCE[0], &CALM);

        assert!(game.team.iter().all(|m| m.role != Role::Manager));
        assert!(
            game.log
                .iter()
                .any(|e| e.contains("made Jules the Manager redundant"))
        );
    }

    #[test]
    fn play_reports_the_worst_setback() {
        let mut game = game();
        assert_eq!(game.clone().resolve(&RECONNAISSANCE[0], &CALM), None);
        assert_eq!(
            game.clone().resolve(&RECONNAISSANCE[2], &CALM),
            Some(Setback::Behind)
        );

        game.trust = 20;
        game.team[0].burnout = 95;
        assert_eq!(
            game.resolve(&RECONNAISSANCE[0], &CALM),
            Some(Setback::Departed)
        );
    }

    #[test]
    fn options_are_three_different_choices_from_the_stage() {
        for seed in 0..SEEDS {
            for stage in Stage::ALL {
                let game = GameState {
                    stage,
                    ..GameState::new("Acme", "Alex", seed)
                };
                let options = game.options();

                assert!(options.iter().all(|c| stage.choices().contains(c)));
                assert!(options[0] != options[1] && options[1] != options[2]);
                assert!(options[0] != options[2]);
                assert_eq!(options, game.options(), "options must be stable");
            }
        }
    }

    #[test]
    fn every_choice_and_event_turns_up_across_seeds() {
        for stage in Stage::ALL {
            let mut offered = Vec::new();
            let mut happened = Vec::new();
            for seed in 0..SEEDS {
                for budget in [100, 150] {
                    let mut game = GameState {
                        stage,
                        budget,
                        ..GameState::new("Acme", "Alex", seed)
                    };
                    offered.extend(game.options());
                    let choice = game.options()[0];
                    game.play(&choice);
                    happened.extend(
                        game.log
                            .iter()
                            .filter(|e| e.starts_with("Meanwhile"))
                            .cloned(),
                    );
                }
            }

            for choice in stage.choices() {
                assert!(
                    offered.contains(&choice),
                    "{} is never offered",
                    choice.label
                );
            }
            for event in stage.events() {
                assert!(
                    happened.iter().any(|e| e == event.text),
                    "{} never happens",
                    event.text
                );
            }
        }
    }

    #[test]
    fn events_apply_their_effects_and_are_logged() {
        let mut game = game();
        let windfall = Event {
            text: "Meanwhile, a test happens.",
            budget: 15,
            containment: 5,
            ..CALM
        };

        game.resolve(&RECONNAISSANCE[1], &windfall);

        assert_eq!(game.budget, 150 - 30 + 15);
        assert_eq!(game.containment, 13 + 5);
        assert_eq!(game.log[2], "Meanwhile, a test happens.");
    }

    #[test]
    fn every_choice_has_report_text() {
        for choice in Stage::ALL.iter().flat_map(|s| s.choices()) {
            assert!(!choice.text.is_empty(), "{} has no text", choice.label);
        }
    }

    #[test]
    fn a_coffee_run_costs_money_once_per_stage() {
        let mut game = game();

        game.send_intern();

        assert_eq!(game.budget, 150 - COFFEE_RUN_COST);
        assert!(!game.can_send_intern());
        game.resolve(&RECONNAISSANCE[0], &CALM);
        assert!(game.can_send_intern());
        game.budget = COFFEE_RUN_COST - 1;
        assert!(!game.can_send_intern());
    }

    #[test]
    fn a_surviving_intern_restocks_the_coffee() {
        let mut game = game();

        game.intern_returns(true);

        assert_eq!(game.coffee, 24 + COFFEE_RUN_POTS);
        assert!(game.log.last().unwrap().contains("made it back"));
    }

    #[test]
    fn coffee_never_exceeds_capacity() {
        let mut game = game();
        game.coffee = COFFEE_CAPACITY - 2;

        game.intern_returns(true);
        assert_eq!(game.coffee, COFFEE_CAPACITY);

        game.resolve(&WEAPONIZATION[2], &CALM);
        assert_eq!(game.coffee, COFFEE_CAPACITY - 6);
    }

    #[test]
    fn the_intern_is_not_sent_when_coffee_is_full() {
        let mut game = game();
        game.coffee = COFFEE_CAPACITY;

        assert!(!game.can_send_intern());
        game.coffee -= 1;
        assert!(game.can_send_intern());
    }

    #[test]
    fn a_flattened_intern_brings_no_coffee() {
        let mut game = game();

        game.intern_returns(false);

        assert_eq!(game.coffee, 24);
        assert!(game.log.last().unwrap().contains("delivery truck"));
    }

    #[test]
    fn low_trust_gets_the_ciso_fired() {
        let mut game = game();
        game.trust = 20;

        game.resolve(&RECONNAISSANCE[0], &CALM);

        assert!(game.team.iter().all(|m| m.role != Role::Ciso));
        assert!(game.log.iter().any(|e| e.contains("fired Ravi the CISO")));
    }

    #[test]
    fn losing_analysts_slows_containment() {
        let mut game = game();
        game.team.retain(|m| m.name != "Maya");

        game.resolve(&RECONNAISSANCE[0], &CALM);

        assert_eq!(game.containment, 8 + 2 * ANALYST_CONTAINMENT);
    }

    #[test]
    fn actions_on_objectives_is_played_before_the_game_ends() {
        let game = play_path(&[2, 0, 1, 2, 0, 1]);

        assert_eq!(game.stage, Stage::ActionsOnObjectives);
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn balanced_play_contains_the_attack() {
        let game = play_path(&[2, 0, 1, 2, 0, 1, 1]);

        assert_eq!(game.outcome, Some(Outcome::Contained));
    }

    #[test]
    fn resting_through_the_incident_gets_the_lead_fired() {
        let game = play_path(&[2, 2, 2]);

        assert_eq!(game.outcome, Some(Outcome::Fired));
    }

    #[test]
    fn pushing_the_team_too_hard_collapses_it() {
        let game = play_path(&[0, 0, 0, 0, 0]);

        assert_eq!(game.outcome, Some(Outcome::TeamCollapsed));
    }

    #[test]
    fn running_out_of_money_without_a_free_choice_is_bankruptcy() {
        let mut game = game_offering(Stage::Weaponization, |o| o.iter().all(|c| c.cost > 4));
        game.stage = Stage::Reconnaissance;
        game.budget = 4;

        game.resolve(&RECONNAISSANCE[2], &CALM);

        assert_eq!(game.stage, Stage::Weaponization);
        assert_eq!(game.outcome, Some(Outcome::Bankrupt));
    }

    #[test]
    fn a_free_choice_keeps_a_broke_team_playing() {
        let mut game = game_offering(Stage::Delivery, |o| o.iter().any(|c| c.cost == 0));
        game.stage = Stage::Weaponization;
        game.budget = 5;

        game.resolve(&WEAPONIZATION[0], &CALM);

        assert_eq!(game.stage, Stage::Delivery);
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn a_coffee_run_that_empties_the_budget_ends_the_game() {
        let mut game = game_offering(Stage::Weaponization, |o| o.iter().all(|c| c.cost > 0));
        game.budget = COFFEE_RUN_COST;

        game.send_intern();
        game.intern_returns(true);

        assert_eq!(game.outcome, Some(Outcome::Bankrupt));
    }

    #[test]
    fn every_outcome_is_reachable() {
        let outcomes: Vec<_> = all_endings().iter().map(|(_, o)| *o).collect();

        for outcome in [
            Outcome::Contained,
            Outcome::Breached,
            Outcome::TeamCollapsed,
            Outcome::Fired,
            Outcome::Bankrupt,
        ] {
            assert!(outcomes.contains(&outcome), "{outcome:?} is unreachable");
        }
    }

    #[test]
    fn roughly_a_third_of_all_paths_win() {
        let endings = all_endings();
        let wins = endings
            .iter()
            .filter(|(_, o)| *o == Outcome::Contained)
            .count();
        let rate = wins as f32 / endings.len() as f32;

        assert!((0.25..=0.40).contains(&rate), "win rate {rate}");
    }

    #[test]
    fn every_choice_is_part_of_some_winning_path() {
        let endings = all_endings();

        for choice in Stage::ALL.iter().flat_map(|s| s.choices()) {
            assert!(
                endings
                    .iter()
                    .any(|(path, o)| *o == Outcome::Contained && path.contains(&choice.label)),
                "{} never wins",
                choice.label
            );
        }
    }
}
