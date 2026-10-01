use std::fmt;

/// The day the IPO bell rings, 26 weeks after day 1.
pub const IPO_DAY: u32 = 182;

/// Pots of coffee the break room can hold.
pub const COFFEE_CAPACITY: i32 = 36;
/// Cost in dollars of sending the intern across the street for coffee.
pub const COFFEE_RUN_COST: i64 = 10;
/// Pots of coffee the intern brings back, and in a case from the Vendor Hall.
const COFFEE_RUN_POTS: i32 = 12;
/// Extra burnout everyone gains each day the coffee is out.
const NO_COFFEE_BURNOUT: i32 = 2;
/// Burnout a junior analyst sheds each day before their share of the workload.
const JUNIOR_REST: i32 = 1;
/// Seniors handle the same workload with less wear.
const SENIOR_REST: i32 = 2;
/// Weekly pay for each analyst level.
const JUNIOR_SALARY: i64 = 4_000;
const SENIOR_SALARY: i64 = 8_000;
/// Most analysts the SOC has desks for.
pub const MAX_ANALYSTS: i32 = 8;
/// Weeks of payroll between day 1 and IPO day.
pub const PAYDAYS: i64 = 25;
/// Trust below this gets the CISO fired.
const FIRING_TRUST: i32 = 30;
/// Budget in dollars below which the SOC Manager is made redundant.
const REDUNDANCY_BUDGET: i64 = 50_000;
/// Posture each area loses every Monday as systems drift.
const POSTURE_DECAY: i32 = 2;
/// Brand loyalty below this drags the valuation down every Monday.
const BRAND_DRAG_LEVEL: i32 = 50;
/// Valuation below this percentage of the base valuation pulls the IPO.
const PULLED_PERCENT: i64 = 40;

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
            Self::Fintech => 900_000,
            Self::Healthtech => 700_000,
            Self::Gaming => 500_000,
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

/// Everything for sale at the Vendor Hall, in shop order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Junior,
    Senior,
    Edr,
    Siem,
    MfaTokens,
    EmailGateway,
    Waf,
    Backups,
    IrRetainer,
    Insurance,
    Coffee,
}

impl Item {
    pub const ALL: [Item; 11] = [
        Self::Junior,
        Self::Senior,
        Self::Edr,
        Self::Siem,
        Self::MfaTokens,
        Self::EmailGateway,
        Self::Waf,
        Self::Backups,
        Self::IrRetainer,
        Self::Insurance,
        Self::Coffee,
    ];

    /// One-time cost; for analysts this is the recruiter's fee.
    pub fn price(self) -> i64 {
        match self {
            Self::Junior => 5_000,
            Self::Senior => 10_000,
            Self::Edr => 60_000,
            Self::Siem => 80_000,
            Self::MfaTokens => 20_000,
            Self::EmailGateway => 25_000,
            Self::Waf => 40_000,
            Self::Backups => 30_000,
            Self::IrRetainer => 50_000,
            Self::Insurance => 40_000,
            Self::Coffee => 100,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Junior => "Hire a junior analyst",
            Self::Senior => "Hire a senior analyst",
            Self::Edr => "EDR",
            Self::Siem => "SIEM",
            Self::MfaTokens => "MFA tokens",
            Self::EmailGateway => "Email security gateway",
            Self::Waf => "DDoS protection and WAF",
            Self::Backups => "Immutable backups",
            Self::IrRetainer => "Incident response retainer",
            Self::Insurance => "Cyber insurance",
            Self::Coffee => "A case of coffee (12 pots)",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Junior => "Cheap and eager. Slower, and burns out faster.",
            Self::Senior => "Expensive. Faster at everything and harder to burn out.",
            Self::Edr => "Endpoint protection and detection. Must be deployed to help.",
            Self::Siem => "Detection across the board. Must be deployed to help.",
            Self::MfaTokens => "Protects identities. Must be deployed to help.",
            Self::EmailGateway => "Stops phishing at the door. Must be deployed to help.",
            Self::Waf => "Shields the perimeter. Must be deployed to help.",
            Self::Backups => "Ransomware's worst enemy. Must be deployed to help.",
            Self::IrRetainer => "Lets you call in an incident response firm.",
            Self::Insurance => "Pays for part of the damage when things go wrong.",
            Self::Coffee => "Fuel. The break room holds 36 pots.",
        }
    }

    /// Weekly pay for an analyst hired from this listing; zero for everything else.
    pub fn salary(self) -> i64 {
        match self {
            Self::Junior => JUNIOR_SALARY,
            Self::Senior => SENIOR_SALARY,
            _ => 0,
        }
    }

    /// Analysts and coffee can be bought again; tools and services cannot.
    fn once(self) -> bool {
        !matches!(self, Self::Junior | Self::Senior | Self::Coffee)
    }
}

/// Something the SOC spends days (and sometimes money) on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Rolls out an owned tool from the Vendor Hall.
    Deploy(Item),
    PhishingSim,
    PatchSprint,
    Tabletop,
    BackupTest,
    /// Recruits an analyst of the Vendor Hall's `Junior` or `Senior` level.
    Recruit(Item),
    DayOff,
    Offsite,
    BriefLeadership,
}

impl Action {
    pub fn label(self) -> String {
        match self {
            Self::Deploy(Item::MfaTokens) => "Enforce MFA everywhere".to_string(),
            Self::Deploy(item) => format!("Deploy {}", item.label()),
            Self::PhishingSim => "Run a phishing simulation".to_string(),
            Self::PatchSprint => "Patch sprint".to_string(),
            Self::Tabletop => "Tabletop exercise".to_string(),
            Self::BackupTest => "Backup restore test".to_string(),
            Self::Recruit(Item::Senior) => "Recruit a senior analyst".to_string(),
            Self::Recruit(_) => "Recruit a junior analyst".to_string(),
            Self::DayOff => "Give everyone the day off".to_string(),
            Self::Offsite => "Team offsite".to_string(),
            Self::BriefLeadership => "Brief leadership".to_string(),
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Deploy(Item::MfaTokens) => "Big identity boost. Employees will complain.",
            Self::Deploy(_) => "Turns the tool on so it actually helps.",
            Self::PhishingSim => "Teaches people to spot phishing. Executives hate it.",
            Self::PatchSprint => "Hardens endpoints and the perimeter. Tiring.",
            Self::Tabletop => "Practice for the worst day. Leadership likes it.",
            Self::BackupTest => "Proves the backups actually restore.",
            Self::Recruit(_) => "Find and hire a new analyst. Takes a week.",
            Self::DayOff => "Everyone recovers. Nobody is watching for a day.",
            Self::Offsite => "A week of trust falls. Large burnout recovery.",
            Self::BriefLeadership => "Tell the board what the SOC is doing.",
        }
    }

    pub fn cost(self) -> i64 {
        match self {
            Self::PhishingSim => 5_000,
            Self::Offsite => 20_000,
            Self::Recruit(level) => level.price(),
            _ => 0,
        }
    }

    /// Days at Steady tempo with no seniors.
    fn base_days(self) -> u32 {
        match self {
            Self::Deploy(Item::Edr) => 7,
            Self::Deploy(Item::Siem) => 10,
            Self::Deploy(Item::EmailGateway) => 3,
            Self::Deploy(Item::Waf) => 4,
            Self::Deploy(_) => 5,
            Self::PhishingSim | Self::BackupTest => 2,
            Self::PatchSprint => 4,
            Self::Tabletop | Self::DayOff | Self::BriefLeadership => 1,
            Self::Recruit(_) => 7,
            Self::Offsite => 5,
        }
    }

    /// Whether tempo and seniors leave the duration alone.
    fn fixed(self) -> bool {
        matches!(
            self,
            Self::Recruit(_) | Self::DayOff | Self::Offsite | Self::BriefLeadership
        )
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

/// The action in progress and how long it has left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub action: Action,
    pub days_left: u32,
    /// The new analyst's name, for `Recruit`.
    pub name: String,
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

/// Hidden security posture areas, never shown to the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    Identity,
    Endpoint,
    People,
    Perimeter,
    Resilience,
    Detection,
}

impl Area {
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
    Manager,
    Ciso,
    Cio,
}

impl Role {
    pub fn is_analyst(self) -> bool {
        matches!(self, Self::Junior | Self::Senior)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Junior => "Junior Analyst",
            Self::Senior => "Senior Analyst",
            Self::Manager => "Manager",
            Self::Ciso => "CISO",
            Self::Cio => "CIO",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamMember {
    pub name: String,
    pub role: Role,
    pub burnout: i32,
    /// Weekly pay from the SOC budget; leadership is paid by the company.
    pub salary: i64,
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
    /// Day 1 is a Monday; the game ends as an IPO on `IPO_DAY`.
    pub day: u32,
    pub base_valuation: i64,
    pub valuation: i64,
    pub trust: i32,
    pub brand: i32,
    pub budget: i64,
    pub coffee: i32,
    pub tempo: Tempo,
    /// Indexed by `Area`; kept private so it can never be drawn.
    posture: [i32; Area::ALL.len()],
    pub team: Vec<TeamMember>,
    /// Tools and services bought at the Vendor Hall. Tools need deploying before they help.
    pub owned: Vec<Item>,
    pub deployed: Vec<Item>,
    pub task: Option<Task>,
    pub log: Vec<String>,
    pub outcome: Option<Outcome>,
    /// Whether the intern has already been sent for coffee this week.
    pub coffee_run_made: bool,
    /// Drives every random outcome, so a seed always plays out the same way.
    pub seed: u32,
}

impl GameState {
    /// A new game with leadership in place and no analysts yet; hire them at the Vendor Hall.
    pub fn new(company: &str, lead: &str, profile: Profile, seed: u32) -> Self {
        let leader = |name: &str, role| TeamMember {
            name: name.to_string(),
            role,
            burnout: 10,
            salary: 0,
        };
        Self {
            company: company.to_string(),
            lead: lead.to_string(),
            profile,
            day: 1,
            base_valuation: profile.valuation(),
            valuation: profile.valuation(),
            trust: 60,
            brand: 60,
            budget: profile.budget(),
            coffee: 12,
            tempo: Tempo::Steady,
            posture: [30; Area::ALL.len()],
            team: vec![
                leader("Jules", Role::Manager),
                leader("Ravi", Role::Ciso),
                leader("Dana", Role::Cio),
            ],
            owned: Vec::new(),
            deployed: Vec::new(),
            task: None,
            log: vec![format!(
                "{company} goes public in 26 weeks. {lead} takes command of the SOC."
            )],
            outcome: None,
            coffee_run_made: false,
            seed,
        }
    }

    pub fn can_buy(&self, item: Item) -> bool {
        item.price() <= self.budget
            && !(item.once() && self.owned.contains(&item))
            && match item {
                Item::Junior | Item::Senior => self.analysts() < MAX_ANALYSTS,
                Item::Coffee => !self.coffee_full(),
                _ => true,
            }
    }

    /// Buys a tool, service, or coffee. Analysts are hired by name with `hire`.
    pub fn buy(&mut self, item: Item) {
        self.budget -= item.price();
        match item {
            Item::Coffee => self.coffee = (self.coffee + COFFEE_RUN_POTS).min(COFFEE_CAPACITY),
            Item::Junior | Item::Senior => unreachable!("analysts are hired by name"),
            _ => self.owned.push(item),
        }
    }

    /// Hires an analyst from the Vendor Hall's `Junior` or `Senior` listing.
    pub fn hire(&mut self, item: Item, name: &str) {
        let role = match item {
            Item::Junior => Role::Junior,
            Item::Senior => Role::Senior,
            _ => unreachable!("only analysts are hired"),
        };
        self.budget -= item.price();
        self.team.push(TeamMember {
            name: name.to_string(),
            role,
            burnout: 0,
            salary: item.salary(),
        });
    }

    /// Actions the SOC can start now: prerequisites met, affordable, and nothing else under way.
    pub fn actions(&self) -> Vec<Action> {
        if self.task.is_some() || self.outcome.is_some() {
            return Vec::new();
        }
        let deployable = self
            .owned
            .iter()
            .filter(|item| !matches!(item, Item::IrRetainer | Item::Insurance))
            .filter(|item| !self.deployed.contains(item))
            .map(|&item| Action::Deploy(item));
        let backup_test = self
            .deployed
            .contains(&Item::Backups)
            .then_some(Action::BackupTest);
        let recruits = (self.analysts() < MAX_ANALYSTS)
            .then_some([Action::Recruit(Item::Junior), Action::Recruit(Item::Senior)])
            .into_iter()
            .flatten();
        deployable
            .chain([Action::PhishingSim, Action::PatchSprint, Action::Tabletop])
            .chain(backup_test)
            .chain(recruits)
            .chain([Action::DayOff, Action::Offsite, Action::BriefLeadership])
            .filter(|action| action.cost() <= self.budget)
            .collect()
    }

    /// Days an action takes now: tempo stretches or shrinks it, and every two seniors save a day.
    pub fn duration(&self, action: Action) -> u32 {
        if action.fixed() {
            return action.base_days();
        }
        let seniors = self.team.iter().filter(|m| m.role == Role::Senior).count() as u32;
        let days = (action.base_days() * self.tempo.duration_percent()).div_ceil(100);
        days.saturating_sub(seniors / 2).max(1)
    }

    /// Pays for an action and puts it under way; `name` names a recruit.
    pub fn start(&mut self, action: Action, name: &str) {
        self.budget -= action.cost();
        let days = self.duration(action);
        self.log
            .push(format!("Started: {} ({days} days).", action.label()));
        self.task = Some(Task {
            action,
            days_left: days,
            name: name.to_string(),
        });
    }

    fn boost(&mut self, area: Area, amount: i32) {
        let value = &mut self.posture[area as usize];
        *value = (*value + amount).clamp(0, 100);
    }

    /// Applies a finished action's effects.
    fn finish(&mut self, task: Task) {
        let message = match task.action {
            Action::Deploy(item) => {
                self.deployed.push(item);
                match item {
                    Item::Edr => {
                        self.boost(Area::Endpoint, 25);
                        self.boost(Area::Detection, 10);
                    }
                    Item::Siem => self.boost(Area::Detection, 30),
                    Item::MfaTokens => {
                        self.boost(Area::Identity, 30);
                        self.trust -= 3;
                    }
                    Item::EmailGateway => self.boost(Area::People, 20),
                    Item::Waf => self.boost(Area::Perimeter, 30),
                    Item::Backups => self.boost(Area::Resilience, 25),
                    _ => unreachable!("only tools are deployed"),
                }
                if item == Item::MfaTokens {
                    "MFA is enforced everywhere. The help desk is drowning in \"I lost my phone\" tickets.".to_string()
                } else {
                    format!(
                        "The {} rollout is finished. It is actually turned on.",
                        item.label()
                    )
                }
            }
            Action::PhishingSim => {
                self.boost(Area::People, 8);
                if roll(self.seed, self.day).is_multiple_of(3) {
                    self.trust -= 3;
                    "The VP of Sales clicked the test phish and is furious about being \"tricked\"."
                        .to_string()
                } else {
                    "Phishing simulation done. 23% clicked. One person replied with their password."
                        .to_string()
                }
            }
            Action::PatchSprint => {
                self.boost(Area::Endpoint, 8);
                self.boost(Area::Perimeter, 5);
                for member in self.team.iter_mut().filter(|m| m.role.is_analyst()) {
                    member.burnout += 8;
                }
                "Patch sprint complete. 1,200 patches applied, 3 servers rebooted unexpectedly."
                    .to_string()
            }
            Action::Tabletop => {
                self.boost(Area::Resilience, 6);
                self.trust += 2;
                "The tabletop exercise went well. The CEO learned what ransomware is.".to_string()
            }
            Action::BackupTest => {
                self.boost(Area::Resilience, 6);
                "Backup restore test passed. Someone finally knows where the backups are."
                    .to_string()
            }
            Action::Recruit(level) => {
                self.hire(level, &task.name);
                // The recruiter's fee was paid when the search started.
                self.budget += level.price();
                let role = self.team.last().unwrap().role;
                format!("{} the {role} joins the SOC.", task.name)
            }
            Action::DayOff => "The team is back from a day off, slightly less haunted.".to_string(),
            Action::Offsite => {
                "The team offsite is over. Trust falls were had. Nobody was dropped.".to_string()
            }
            Action::BriefLeadership => {
                self.trust += 4;
                "You briefed leadership. The board nodded at all the right moments.".to_string()
            }
        };
        self.trust = self.trust.clamp(0, 100);
        self.log.push(message);
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

    pub fn analysts(&self) -> i32 {
        self.team.iter().filter(|m| m.role.is_analyst()).count() as i32
    }

    /// Days since the last Monday: 0 is Monday.
    pub fn weekday(&self) -> u32 {
        (self.day - 1) % 7
    }

    pub fn days_to_ipo(&self) -> u32 {
        IPO_DAY - self.day
    }

    pub fn payroll(&self) -> i64 {
        self.team.iter().map(|m| m.salary).sum()
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
            let week = (self.day - 1) / 7;
            self.log
                .push(INTERN_FATES[week as usize % INTERN_FATES.len()].to_string());
        }
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
            stop |= self.leadership_changes();
            self.check_outcome();
        }
        stop || self.outcome.is_some()
    }

    /// Weekly upkeep: posture drift, brand drag, and payroll.
    fn monday(&mut self) -> bool {
        self.coffee_run_made = false;
        for value in &mut self.posture {
            *value = (*value - POSTURE_DECAY).clamp(0, 100);
        }
        if self.brand < BRAND_DRAG_LEVEL {
            let drag = self.base_valuation * (BRAND_DRAG_LEVEL - self.brand) as i64 / 1000;
            self.valuation -= drag;
            self.log.push(format!(
                "Weak brand loyalty knocked {} off the valuation.",
                money(drag)
            ));
        }

        let mut stop = false;
        while self.payroll() > self.budget && self.analysts() > 1 {
            let (i, _) = self
                .team
                .iter()
                .enumerate()
                .filter(|(_, m)| m.role.is_analyst())
                .max_by_key(|(_, m)| m.salary)
                .unwrap();
            let analyst = self.team.remove(i);
            self.log.push(format!(
                "Payroll came up short. {} the {} was laid off and walked out holding a cardboard box.",
                analyst.name, analyst.role
            ));
            stop = true;
        }
        if self.payroll() > self.budget {
            self.outcome = Some(Outcome::Bankrupt);
            return true;
        }
        self.budget -= self.payroll();
        self.log
            .push(format!("Payday: {} in salaries.", money(self.payroll())));
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
            self.log
                .push("The coffee ran out. Tempers are short.".to_string());
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
            let share = self.tempo.workload() / analysts;
            for member in self.team.iter_mut() {
                let rest = match member.role {
                    Role::Junior => JUNIOR_REST,
                    Role::Senior => SENIOR_REST,
                    _ => continue,
                };
                member.burnout = (member.burnout + share - rest).max(0);
            }
        }

        let mut exit = self.day as usize;
        let log = &mut self.log;
        let before = self.team.len();
        self.team.retain(|m| {
            if m.burnout >= 100 {
                let reason = BURNOUT_EXITS[exit % BURNOUT_EXITS.len()];
                log.push(format!("{} the {} {reason}", m.name, m.role));
                exit += 1;
            }
            m.burnout < 100
        });
        self.team.len() < before
    }

    fn leadership_changes(&mut self) -> bool {
        let mut changed = false;
        if self.budget < REDUNDANCY_BUDGET
            && let Some(i) = self.team.iter().position(|m| m.role == Role::Manager)
        {
            let manager = self.team.remove(i);
            self.log.push(format!(
                "Finance made {} the Manager redundant to save money. The consultants who recommended it billed $80K.",
                manager.name
            ));
            changed = true;
        }
        if self.trust < FIRING_TRUST
            && let Some(i) = self.team.iter().position(|m| m.role == Role::Ciso)
        {
            let ciso = self.team.remove(i);
            self.log.push(format!(
                "The board fired {} the CISO over the SOC's performance.",
                ciso.name
            ));
            changed = true;
        }
        changed
    }

    fn check_outcome(&mut self) {
        self.outcome = if self.trust <= 0 {
            Some(Outcome::Fired)
        } else if self.brand <= 0 {
            Some(Outcome::ShutDown)
        } else if self.valuation < self.base_valuation * PULLED_PERCENT / 100 {
            Some(Outcome::Pulled)
        } else if self.analysts() == 0 {
            Some(Outcome::TeamCollapsed)
        } else if self.day >= IPO_DAY {
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
                member("Jules", Role::Manager, 20, 0),
                member("Ravi", Role::Ciso, 15, 0),
                member("Dana", Role::Cio, 10, 0),
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
        assert_eq!(game.analysts(), 0);
        assert!(game.team.iter().any(|m| m.role == Role::Manager));
        assert!(game.team.iter().any(|m| m.role == Role::Ciso));
        assert!(game.team.iter().any(|m| m.role == Role::Cio));
        assert!(game.owned.is_empty());
        assert!(game.log[0].contains("Acme") && game.log[0].contains("Alex"));
        assert_eq!(game.outcome, None);
    }

    #[test]
    fn each_profile_sets_its_budget_valuation_and_multiplier() {
        let expected = [
            (Profile::Fintech, 900_000, 1_500_000_000, 1),
            (Profile::Healthtech, 700_000, 1_000_000_000, 2),
            (Profile::Gaming, 500_000, 600_000_000, 3),
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
        for item in Item::ALL {
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
            game.team[3],
            member("Priya", Role::Junior, 0, JUNIOR_SALARY)
        );
        assert_eq!(
            game.team[4],
            member("Marcus", Role::Senior, 0, SENIOR_SALARY)
        );
        assert_eq!(game.payroll(), JUNIOR_SALARY + SENIOR_SALARY);
        assert_eq!(
            game.budget,
            500_000 - Item::Junior.price() - Item::Senior.price()
        );

        game.day = 7;
        game.advance();
        assert_eq!(
            game.budget,
            500_000 - Item::Junior.price() - Item::Senior.price() - game.payroll()
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
        let posture = game.posture;

        for item in &Item::ALL[2..10] {
            game.buy(*item);
        }

        assert_eq!(game.posture, posture);
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
        assert!(game.log.iter().any(|e| e == "Payday: $18K in salaries."));
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
        assert_eq!(game.team[5].burnout, 10 + NO_COFFEE_BURNOUT);
        assert!(game.log.iter().any(|e| e.contains("coffee ran out")));

        assert!(!game.advance(), "only running out stops the clock");
        assert_eq!(game.team[5].burnout, 10 + 2 * NO_COFFEE_BURNOUT);
    }

    #[test]
    fn posture_decays_every_monday_and_stays_in_range() {
        let mut game = game();

        advance_to(&mut game, 7);
        assert_eq!(game.posture, [30; 6]);
        advance_to(&mut game, 8);
        assert_eq!(game.posture, [30 - POSTURE_DECAY; 6]);

        game.tempo = Tempo::Relaxed;
        while game.outcome.is_none() {
            game.coffee = COFFEE_CAPACITY;
            game.advance();
            assert!(game.posture.iter().all(|p| (0..=100).contains(p)));
        }
        assert_eq!(game.posture, [0; 6]);
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
            game.log
                .iter()
                .any(|e| e
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
    fn low_budget_makes_the_manager_redundant() {
        let mut game = game();
        game.budget = REDUNDANCY_BUDGET - 1;

        assert!(game.advance());

        assert!(game.team.iter().all(|m| m.role != Role::Manager));
        assert!(
            game.log
                .iter()
                .any(|e| e.contains("made Jules the Manager redundant"))
        );
    }

    #[test]
    fn low_trust_gets_the_ciso_fired() {
        let mut game = game();
        game.trust = FIRING_TRUST - 1;

        assert!(game.advance());

        assert!(game.team.iter().all(|m| m.role != Role::Ciso));
        assert!(game.log.iter().any(|e| e.contains("fired Ravi the CISO")));
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
        assert_eq!(game.coffee, 24 + COFFEE_RUN_POTS);
        assert!(game.log.last().unwrap().contains("made it back"));

        game.intern_returns(true);
        assert_eq!(game.coffee, COFFEE_CAPACITY);
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

    /// Starts an action and advances until it finishes, keeping the coffee topped up.
    fn run(game: &mut GameState, action: Action) {
        game.start(action, "Priya");
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
        assert_eq!(game.deployed, [Item::Siem]);
    }

    #[test]
    fn deploying_each_tool_raises_its_posture_areas() {
        let expected: [(Item, &[(Area, i32)]); 6] = [
            (Item::Edr, &[(Area::Endpoint, 25), (Area::Detection, 10)]),
            (Item::Siem, &[(Area::Detection, 30)]),
            (Item::MfaTokens, &[(Area::Identity, 30)]),
            (Item::EmailGateway, &[(Area::People, 20)]),
            (Item::Waf, &[(Area::Perimeter, 30)]),
            (Item::Backups, &[(Area::Resilience, 25)]),
        ];
        for (tool, boosts) in expected {
            let mut game = game();
            game.owned.push(tool);
            let mut control = game.clone();

            run(&mut game, Action::Deploy(tool));
            advance_to(&mut control, game.day);

            for area in Area::ALL {
                let boost = boosts
                    .iter()
                    .find(|(a, _)| *a == area)
                    .map_or(0, |(_, b)| *b);
                assert_eq!(
                    game.posture[area as usize],
                    control.posture[area as usize] + boost,
                    "{tool:?} {area:?}"
                );
            }
        }
    }

    #[test]
    fn enforcing_mfa_annoys_employees() {
        let mut game = game();
        game.owned.push(Item::MfaTokens);

        run(&mut game, Action::Deploy(Item::MfaTokens));

        assert_eq!(game.trust, 60 - 3);
        assert!(game.log.last().unwrap().contains("I lost my phone"));
    }

    #[test]
    fn each_action_applies_its_cost_duration_and_effects() {
        let mut game = game();
        game.deployed.push(Item::Backups);
        let posture = |g: &GameState, area: Area| g.posture[area as usize];

        let start = game.clone();
        run(&mut game, Action::PatchSprint);
        assert_eq!(game.day, start.day + 4);
        assert_eq!(posture(&game, Area::Endpoint), 38);
        assert_eq!(posture(&game, Area::Perimeter), 35);
        assert!(game.team[0].burnout > start.team[0].burnout + 4, "tiring");

        let mut game = start.clone();
        run(&mut game, Action::PhishingSim);
        assert_eq!(game.day, start.day + 2);
        assert_eq!(game.budget, start.budget - 5_000);
        assert_eq!(posture(&game, Area::People), 38);

        let mut game = start.clone();
        run(&mut game, Action::Tabletop);
        assert_eq!(game.day, start.day + 1);
        assert_eq!(posture(&game, Area::Resilience), 36);
        assert_eq!(game.trust, 62);

        let mut game = start.clone();
        run(&mut game, Action::BackupTest);
        assert_eq!(game.day, start.day + 2);
        assert_eq!(posture(&game, Area::Resilience), 36);

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
            let mut game = GameState { seed, ..game() };
            run(&mut game, Action::PhishingSim);
            outcomes.push(game.trust);
        }

        assert!(outcomes.contains(&60) && outcomes.contains(&57));
    }

    #[test]
    fn a_recruited_analyst_is_named_and_joins_after_a_week() {
        let mut game = game();

        game.start(Action::Recruit(Item::Senior), "Priya");
        assert_eq!(game.budget, 500_000 - Item::Senior.price());
        assert_eq!(game.analysts(), 3);

        while game.task.is_some() {
            game.advance();
        }
        assert_eq!(game.day, 8);
        assert_eq!(game.analysts(), 4);
        assert_eq!(
            game.team.last().unwrap(),
            &member("Priya", Role::Senior, 0, Item::Senior.salary())
        );
        assert_eq!(game.payroll(), 18_000 + Item::Senior.salary());
        assert_eq!(
            game.budget,
            500_000 - Item::Senior.price() - 18_000,
            "fee paid once, new hire not yet on payroll"
        );
        assert!(game.log.last().unwrap() == "Priya the Senior Analyst joins the SOC.");
    }

    #[test]
    fn unaffordable_actions_and_unmet_prerequisites_are_not_offered() {
        let mut game = game();
        assert!(!game.actions().contains(&Action::BackupTest));
        game.owned.push(Item::Backups);
        assert!(
            !game.actions().contains(&Action::BackupTest),
            "owned but not deployed"
        );
        game.deployed.push(Item::Backups);
        assert!(game.actions().contains(&Action::BackupTest));

        game.budget = 4_999;
        let offered = game.actions();
        assert!(!offered.contains(&Action::PhishingSim));
        assert!(!offered.contains(&Action::Offsite));
        assert!(!offered.contains(&Action::Recruit(Item::Junior)));
        assert!(offered.contains(&Action::PatchSprint));

        game.team
            .extend((0..5).map(|_| member("A", Role::Junior, 0, 0)));
        game.budget = 500_000;
        assert!(
            !game.actions().contains(&Action::Recruit(Item::Junior)),
            "no desks"
        );
    }

    #[test]
    fn nothing_else_starts_while_an_action_is_under_way() {
        let mut game = game();

        game.start(Action::PatchSprint, "");

        assert!(game.actions().is_empty());
    }

    #[test]
    fn tempo_and_seniors_change_how_long_actions_take() {
        let mut game = game();
        let sprint = Action::PatchSprint;
        assert_eq!(game.duration(sprint), 4);

        game.tempo = Tempo::Relaxed;
        assert_eq!(game.duration(sprint), 6);
        game.tempo = Tempo::Crunch;
        assert_eq!(game.duration(sprint), 3);

        game.tempo = Tempo::Steady;
        game.team.push(member("S1", Role::Senior, 0, 0));
        assert_eq!(game.duration(sprint), 4, "one senior is not enough");
        game.team.push(member("S2", Role::Senior, 0, 0));
        assert_eq!(game.duration(sprint), 3);

        game.tempo = Tempo::Crunch;
        assert_eq!(game.duration(Action::Tabletop), 1, "never under a day");
        assert_eq!(game.duration(Action::Recruit(Item::Junior)), 7, "fixed");
    }

    #[test]
    fn an_interrupted_action_resumes_where_it_left_off() {
        let mut game = game();
        game.coffee = 2;
        game.start(Action::PatchSprint, "");

        assert!(game.advance(), "running out of coffee stops the clock");
        assert_eq!(game.task.as_ref().unwrap().days_left, 3);

        assert!(!game.advance());
        assert!(!game.advance());
        assert!(game.advance(), "finishing stops the clock");
        assert_eq!(game.task, None);
        assert_eq!(game.day, 5);
    }

    #[test]
    fn days_off_replace_the_workload() {
        let mut game = game();
        game.start(Action::DayOff, "");

        game.advance();

        assert_eq!(game.team[3].burnout, 0, "leadership rests too");
        assert_eq!(game.team[1].burnout, 0);
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
