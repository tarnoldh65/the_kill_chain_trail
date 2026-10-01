//! The security conference minigame: who goes, which track they pick, and what they bring back.

use std::fmt;

use crate::game::Area;

/// Ticket and travel for each person who goes, including the incident lead.
pub const TICKET: i64 = 6_000;
/// Days the conference takes on the clock.
pub const CONFERENCE_DAYS: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Track {
    Talks,
    Villages,
    Expo,
    Hallway,
}

impl Track {
    pub const ALL: [Track; 4] = [Self::Talks, Self::Villages, Self::Expo, Self::Hallway];

    pub fn blurb(self) -> &'static str {
        match self {
            Self::Talks => "Usually expertise. Small burnout relief.",
            Self::Villages => "Hands-on expertise or pure fun. Some relief.",
            Self::Expo => "Tool expertise and swag. Small relief.",
            Self::Hallway => "Almost always pure fun. Big relief.",
        }
    }

    pub fn cards(self) -> &'static [Card] {
        match self {
            Self::Talks => &TALKS,
            Self::Villages => &VILLAGES,
            Self::Expo => &EXPO,
            Self::Hallway => &HALLWAY,
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Talks => "Talks",
            Self::Villages => "Villages",
            Self::Expo => "Expo floor",
            Self::Hallway => "Hallway track and parties",
        })
    }
}

/// What an analyst brings back. `{name}` is the attendee. Cards never lower posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub text: &'static str,
    pub expertise: Option<Area>,
    /// Burnout the attendee sheds.
    pub relief: i32,
}

const TALKS: [Card; 4] = [
    Card {
        text: "{name} sat through a three-hour talk on passwordless login and will not stop saying \"FIDO2\".",
        expertise: Some(Area::Identity),
        relief: 5,
    },
    Card {
        text: "{name} attended \"Your Logs Are Lying To You\" and now trusts nobody, including the logs.",
        expertise: Some(Area::Detection),
        relief: 5,
    },
    Card {
        text: "{name} fell asleep in a keynote and woke up in a ransomware recovery workshop. Lucky.",
        expertise: Some(Area::Resilience),
        relief: 5,
    },
    Card {
        text: "{name} saw one talk on cloud firewalls and now draws network diagrams on every napkin.",
        expertise: Some(Area::Perimeter),
        relief: 5,
    },
];

const VILLAGES: [Card; 4] = [
    Card {
        text: "{name} spent the whole time at the Lockpick Village. They learned nothing useful but have never been happier.",
        expertise: None,
        relief: 30,
    },
    Card {
        text: "{name} won the Social Engineering Village by talking the hotel into the keynote speaker's room key.",
        expertise: Some(Area::People),
        relief: 15,
    },
    Card {
        text: "{name} took apart a laptop at the Hardware Village and only had two screws left over.",
        expertise: Some(Area::Endpoint),
        relief: 15,
    },
    Card {
        text: "{name} lost a capture-the-flag to a 14-year-old and learned a lot about humility and log files.",
        expertise: Some(Area::Detection),
        relief: 15,
    },
];

const EXPO: [Card; 4] = [
    Card {
        text: "{name} attended one vendor session on WAFs and is now the company's self-declared WAF expert.",
        expertise: Some(Area::Perimeter),
        relief: 5,
    },
    Card {
        text: "{name} collected 47 vendor t-shirts and will not need to do laundry until the IPO.",
        expertise: None,
        relief: 15,
    },
    Card {
        text: "{name} got a free SIEM tote bag and, somehow, also learned to write detection rules.",
        expertise: Some(Area::Detection),
        relief: 5,
    },
    Card {
        text: "{name} sat through a backup demo for the free espresso and accidentally learned how immutable storage works.",
        expertise: Some(Area::Resilience),
        relief: 5,
    },
];

const HALLWAY: [Card; 4] = [
    Card {
        text: "{name} skipped every talk for the hallway track. Three new friends, zero notes.",
        expertise: None,
        relief: 30,
    },
    Card {
        text: "{name} discovered the hotel pool. The pool is now their favorite security control.",
        expertise: None,
        relief: 30,
    },
    Card {
        text: "{name} sang karaoke with three CISOs at the vendor party. No regrets, no memories.",
        expertise: None,
        relief: 30,
    },
    Card {
        text: "{name} wandered into a closed-door CISO roundtable and nobody noticed. They learned a lot and loved the free lunch.",
        expertise: Some(Area::Identity),
        relief: 30,
    },
];

/// What the incident lead brings back; the lead has no burnout, so these affect the company.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeadCard {
    pub text: &'static str,
    pub trust: i32,
    /// 20% off at this conference's Vendor Hall.
    pub discount: bool,
    /// A senior analyst waits at the job fair with no hiring fee.
    pub recruit: bool,
}

pub const LEAD_CARDS: [LeadCard; 4] = [
    LeadCard {
        text: "You gave a lightning talk about your SOC. The CISO saw it on LinkedIn.",
        trust: 5,
        discount: false,
        recruit: false,
    },
    LeadCard {
        text: "You let a vendor scan your badge. You now get 40 emails a day and 20% off at the Vendor Hall.",
        trust: 0,
        discount: true,
        recruit: false,
    },
    LeadCard {
        text: "You met a senior analyst at the after-party who wants a job. No recruiter fee needed.",
        trust: 0,
        discount: false,
        recruit: true,
    },
    LeadCard {
        text: "You spent the whole conference answering chat from your hotel room. The team is grateful you were reachable.",
        trust: 0,
        discount: false,
        recruit: false,
    },
];
