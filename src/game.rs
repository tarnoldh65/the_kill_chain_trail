use std::fmt;

/// Fatigue every team member gains each stage regardless of the choice.
const BASE_FATIGUE: i32 = 4;
/// Extra fatigue when the coffee runs out.
const NO_COFFEE_FATIGUE: i32 = 15;
/// Containment each remaining analyst adds per stage.
const ANALYST_CONTAINMENT: i32 = 2;
/// Trust below this gets the CISO fired.
const FIRING_TRUST: i32 = 30;
/// Budget below this gets the Manager made redundant.
const REDUNDANCY_BUDGET: i32 = 20;

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

    pub fn choices(self) -> [Choice; 3] {
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

const RECONNAISSANCE: [Choice; 3] = [
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
];

const WEAPONIZATION: [Choice; 3] = [
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
];

const DELIVERY: [Choice; 3] = [
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
];

const EXPLOITATION: [Choice; 3] = [
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
];

const INSTALLATION: [Choice; 3] = [
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
];

const COMMAND_AND_CONTROL: [Choice; 3] = [
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
];

const ACTIONS_ON_OBJECTIVES: [Choice; 3] = [
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
}

impl GameState {
    pub fn new(company: &str, lead: &str) -> Self {
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
        }
    }

    pub fn analysts(&self) -> i32 {
        self.team.iter().filter(|m| m.role == Role::Analyst).count() as i32
    }

    pub fn can_afford(&self, choice: &Choice) -> bool {
        choice.cost <= self.budget
    }

    /// Plays a choice and returns the worst setback it caused.
    pub fn play(&mut self, choice: &Choice) -> Option<Setback> {
        let mut setback = None;
        self.log.push(format!("{}: {}.", self.stage, choice.label));

        self.budget -= choice.cost;
        self.coffee += choice.coffee - self.team.len() as i32;
        self.trust = (self.trust + choice.trust).clamp(0, 100);
        self.brand = (self.brand + choice.brand).clamp(0, 100);
        self.containment =
            (self.containment + choice.containment + ANALYST_CONTAINMENT * self.analysts())
                .min(100);

        let mut fatigue = BASE_FATIGUE + choice.fatigue;
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
        }
        setback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> GameState {
        GameState::new("Acme", "Alex")
    }

    /// Plays the given choice index at each stage.
    fn play_path(path: &[usize]) -> GameState {
        let mut game = game();
        for &i in path {
            let choice = game.stage.choices()[i];
            game.play(&choice);
        }
        game
    }

    /// Plays every affordable choice path and returns each path with its outcome.
    fn all_endings() -> Vec<(Vec<usize>, Outcome)> {
        let mut endings = Vec::new();
        let mut pending = vec![(game(), Vec::new())];
        while let Some((game, path)) = pending.pop() {
            match game.outcome {
                Some(outcome) => endings.push((path, outcome)),
                None => {
                    for (i, choice) in game.stage.choices().iter().enumerate() {
                        if game.can_afford(choice) {
                            let mut next = game.clone();
                            next.play(choice);
                            pending.push((next, [path.as_slice(), &[i]].concat()));
                        }
                    }
                }
            }
        }
        endings
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

        game.play(&RECONNAISSANCE[1]);

        assert_eq!(game.budget, 120);
        assert_eq!(game.coffee, 18);
        assert_eq!(game.trust, 65);
        assert_eq!(game.containment, 13);
        assert_eq!(game.team[0].burnout, 44);
        assert_eq!(game.stage, Stage::Weaponization);
    }

    #[test]
    fn falling_behind_the_attacker_costs_trust_and_brand() {
        let mut game = game();

        game.play(&RECONNAISSANCE[2]);

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

        game.play(&RECONNAISSANCE[0]);

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

        game.play(&RECONNAISSANCE[0]);

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

        game.play(&WEAPONIZATION[0]);

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

        game.play(&RECONNAISSANCE[0]);

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
        assert_eq!(game.clone().play(&RECONNAISSANCE[0]), None);
        assert_eq!(game.clone().play(&RECONNAISSANCE[2]), Some(Setback::Behind));

        game.trust = 20;
        game.team[0].burnout = 95;
        assert_eq!(game.play(&RECONNAISSANCE[0]), Some(Setback::Departed));
    }

    #[test]
    fn every_choice_has_report_text() {
        for choice in Stage::ALL.iter().flat_map(|s| s.choices()) {
            assert!(!choice.text.is_empty(), "{} has no text", choice.label);
        }
    }

    #[test]
    fn low_trust_gets_the_ciso_fired() {
        let mut game = game();
        game.trust = 20;

        game.play(&RECONNAISSANCE[0]);

        assert!(game.team.iter().all(|m| m.role != Role::Ciso));
        assert!(game.log.iter().any(|e| e.contains("fired Ravi the CISO")));
    }

    #[test]
    fn losing_analysts_slows_containment() {
        let mut game = game();
        game.team.retain(|m| m.name != "Maya");

        game.play(&RECONNAISSANCE[0]);

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
    fn every_outcome_is_reachable() {
        let outcomes: Vec<_> = all_endings().into_iter().map(|(_, o)| o).collect();

        for outcome in [
            Outcome::Contained,
            Outcome::Breached,
            Outcome::TeamCollapsed,
            Outcome::Fired,
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

        for (stage, _) in Stage::ALL.iter().enumerate() {
            for choice in 0..3 {
                assert!(
                    endings
                        .iter()
                        .any(|(path, o)| *o == Outcome::Contained && path[stage] == choice),
                    "choice {choice} at stage {stage} never wins"
                );
            }
        }
    }
}
