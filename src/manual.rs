//! The instruction manual, written like the printed booklets that came in the box.
//! In a page's paragraphs, `# ` starts a subheading and an empty string is a blank line.

use crate::attack::Condition;

pub struct Page {
    pub title: &'static str,
    pub paragraphs: Vec<String>,
}

fn page(title: &'static str, paragraphs: &[&str]) -> Page {
    Page {
        title,
        paragraphs: paragraphs.iter().map(|p| p.to_string()).collect(),
    }
}

pub fn pages() -> Vec<Page> {
    vec![
        page(
            "INSTRUCTION MANUAL",
            &[
                "Congratulations on your purchase of THE KILL CHAIN TRAIL, the security operations simulation that asks the question: can one SOC make it to the IPO?",
                "",
                "# CONTENTS",
                "  2. The Story So Far",
                "  3. Your Mission",
                "  4. Getting Started",
                "  5. Controls",
                "  6. The Day Menu",
                "  7. Your Team",
                "  8. Money",
                "  9. Your Defenses",
                " 10. Actions",
                " 11. Attackers, Alerts, and Incidents",
                " 12. Lasting Conditions",
                " 13. Landmarks",
                " 14. Odds and Ends",
                " 15. A Few Words of Advice",
                "",
                "Please read this manual completely before playing. Or at least before your first ransomware incident.",
            ],
        ),
        page(
            "THE STORY SO FAR",
            &[
                "Your company has spent years building a product people love, a customer list investors drool over, and exactly zero security.",
                "",
                "Now the bankers have set a date. In 26 weeks the company rings the opening bell and goes public. Last Tuesday a competitor made the evening news for leaking four million customer records, and the board finally asked the question nobody wanted to answer: \"Wait, who is in charge of security here?\"",
                "",
                "Nobody was. So they hired you.",
                "",
                "As incident lead, you have a budget, an empty room with a coffee machine, and six months to stand up a security operations center (SOC) from scratch. Somewhere out there, ransomware gangs, data thieves, spies, hacktivists, nation-states, and the occasional disgruntled insider have already noticed the IPO announcement.",
                "",
                "Good luck. The bell rings whether you are ready or not.",
            ],
        ),
        page(
            "YOUR MISSION",
            &[
                "Reach IPO day with the highest valuation you can.",
                "",
                "Your score is the final valuation in millions, multiplied by your company profile's score multiplier. The ten best IPOs are kept on the title screen. Only an IPO earns a score.",
                "",
                "# WAYS TO LOSE",
                "IPO PULLED: the valuation falls below 40% of where it started, or the IPO is delayed more than six weeks.",
                "SHUT DOWN: brand loyalty falls to zero and the customers leave.",
                "FIRED: corporate trust falls to zero and security walks you out.",
                "TEAM COLLAPSED: every analyst has quit, burned out, or been let go.",
                "BANKRUPT: payroll comes due and the SOC cannot pay even one analyst.",
                "",
                "When the game ends, the after-action report shows what the attackers were really doing all along. It is usually humbling.",
            ],
        ),
        page(
            "GETTING STARTED",
            &[
                "Name your company and yourself, then choose a profile:",
                "",
                "# FINTECH",
                "$600K to start, $1.50B valuation, score x1. Deep pockets attract money-hungry attackers.",
                "# HEALTHTECH",
                "$450K to start, $1.00B valuation, score x2. Data thieves love patient records.",
                "# GAMING STARTUP",
                "$300K to start, $600M valuation, score x3. A shoestring budget, angry teenagers, and DDoS.",
                "",
                "You then arrive at PROCUREMENT with your starting budget. Hire at least one analyst (two or three is wiser), pick up a few tools, and do not forget the coffee subscription. Candidates you do not hire on day 1 take other jobs; after that, POST JOB OPENINGS to find more.",
            ],
        ),
        page(
            "CONTROLS",
            &[
                "THE KILL CHAIN TRAIL is played entirely from the keyboard.",
                "",
                "# EVERYWHERE",
                "NUMBER KEYS pick a numbered option.",
                "ENTER confirms, dismisses a card, or closes a screen.",
                "UP and DOWN move through lists.",
                "LEFT and RIGHT switch Procurement tabs, log pages, and manual pages.",
                "",
                "# DURING PLAY",
                "ANY KEY stops the clock while days are passing.",
                "L opens the full log. D opens your defenses. M opens this manual.",
                "T on the action list changes the tempo.",
                "F on the team screen lets an analyst go; Y confirms.",
                "",
                "# ON THE TITLE SCREEN",
                "1 to 3 pick a music track, 4 turns music off, M opens this manual, and ENTER begins.",
                "",
                "# THE COFFEE RUN",
                "ARROW KEYS guide the intern across four lanes of traffic and back. Look both ways.",
            ],
        ),
        page(
            "THE DAY MENU",
            &[
                "Whenever the clock stops, you choose what happens next:",
                "",
                "1 CONTINUE lets the days pass. Something will interrupt you soon enough.",
                "2 TAKE AN ACTION puts the team to work for a few days.",
                "3 CHECK THE TEAM shows salaries and burnout, and lets you fire people.",
                "4 SET THE TEMPO between Relaxed, Steady, and Crunch.",
                "5 SEND THE INTERN for fancy coffee, once a week.",
                "6 GO THROUGH PROCUREMENT to buy, hire, and subscribe.",
                "",
                "# TEMPO",
                "RELAXED: actions take longer, the team recovers, and coffee lasts.",
                "STEADY: the normal pace. Burnout creeps up slowly.",
                "CRUNCH: actions finish fast, burnout climbs fast, and the coffee vanishes.",
                "",
                "The timeline at the top shows today, every landmark, and IPO day. Weekends are quieter for the team. Attackers do not take weekends.",
            ],
        ),
        page(
            "YOUR TEAM",
            &[
                "# ANALYSTS",
                "Juniors start at $4K a week, seniors at $8K; each extra proficiency or a tool specialty adds $1K. Seniors work faster and burn out slower, and every two seniors shave a day off most actions. You have desks for eight.",
                "",
                "# RESUMES",
                "Each proficiency adds to a defense: 15 for a junior, 25 for a senior. A tool specialist deploys, maintains, and operates that tool in half the time. LEFT/RIGHT cycle through the resumes, ENTER hires, B goes back.",
                "",
                "# MORALE",
                "Every analyst has a burnout level: GOOD, FAIR, POOR, or VERY POOR. Workload is shared, so more analysts means less burnout each. Crunch, incidents, and empty coffee pots make it worse. At full burnout an analyst leaves, usually dramatically.",
                "",
                "# HIRING AND FIRING",
                "Post job openings ($10K) and resumes arrive two weeks later. Review them in Procurement within a week; once you leave Procurement, the rest take other jobs. The conference job fair offers a few strong candidates. Firing costs a week of salary; you cannot fire your last analyst or someone away at the conference.",
                "",
                "# COFFEE",
                "The break room holds 36 pots. The team drinks 1 a day at Relaxed, 2 at Steady, and 4 at Crunch. With no coffee, everyone's burnout rises every day. The coffee subscription delivers 18 pots every Monday for $2K. Once a week the intern can brave traffic for fancy coffee: 6 pots and a burnout break for every analyst.",
            ],
        ),
        page(
            "MONEY",
            &[
                "# THE BUDGET",
                "Salaries and the coffee subscription come out every Monday. If payroll comes up short, the most expensive analyst is laid off. If even one analyst cannot be paid, the SOC is bankrupt.",
                "",
                "# BOARD FUNDING",
                "Every sixth Monday (days 43, 85, 127, and 169) the board releases more money. How much depends on corporate trust: at trust 60 you get the full grant (Fintech $180K, Healthtech $160K, Gaming startup $120K), at 30 half, at 90 one and a half. SOC STATUS shows the next funding day.",
                "",
                "# WHERE IT GOES",
                "Tools are bought once. Services, like the incident response retainer and cyber insurance, are bought once and pay off later. Actions are mostly free, but offsites, phishing simulations, and incident responses can cost real money.",
                "",
                "Keep an eye on the weekly costs in Procurement. A shiny tool is no use if you cannot pay the people who run it.",
            ],
        ),
        page(
            "YOUR DEFENSES",
            &[
                "Your security is measured in six areas. You never see the numbers, but press D to see what each one stops and what is helping it.",
                "",
                "IDENTITY slows data thieves, spies, and insiders.",
                "ENDPOINT slows ransomware gangs.",
                "PEOPLE slows ransomware gangs and spies.",
                "PERIMETER slows hacktivists.",
                "RESILIENCE slows ransomware and softens every incident.",
                "DETECTION spots attackers early and slows data thieves, nation-states, and insiders.",
                "",
                "# TOOLS",
                "Procurement sells three tools per area, from cheap to premium; + to +++ shows how much each helps. A tool does nothing until an action deploys it. Deployed tools lose strength every week and show as FRESH, AGING, or STALE; maintain them to bring them back.",
                "",
                "# BUILDING UP",
                "Every area starts at nothing. Deployed tools, analysts proficient in it or with conference expertise, and recent operations raise it. Nothing else drifts, but weekly wear and some events weaken your tools.",
                "",
                "# THE PEN TEST",
                "On day 113 the penetration testers grade every area from A to F. The grades stay on the defenses screen.",
            ],
        ),
        page(
            "ACTIONS",
            &[
                "Actions take days. The clock keeps running while the team works, and an interruption only pauses them. Only one action runs at a time.",
                "",
                "# RECOVERY",
                "Clears what an incident left behind, such as restoring operations or answering the regulators.",
                "# IMPLEMENTATION",
                "Deploys the tools you bought. Until then they are shelfware.",
                "# MAINTENANCE",
                "Restores an area's tools to full strength once one starts aging.",
                "# OPERATIONS",
                "Some deployed tools unlock a longer job: a patch sprint, a threat hunt, a backup drill. They boost an area for four weeks, evict attackers you never saw, or brace you for the next incident. Stale tools give weaker results.",
                "# MANAGEMENT",
                "Days off and offsites for the team, and briefings to keep leadership on your side. Briefings sometimes give the CIO ideas.",
                "",
                "LEFT/RIGHT switch between these tabs. The IMPROVES row shows how much an action helps each area, from + to +++. When an action finishes, a card shows what visibly changed.",
            ],
        ),
        page(
            "ATTACKERS, ALERTS, AND INCIDENTS",
            &[
                "Attackers work in the dark. Each one creeps along the kill chain from reconnaissance to its objective, and you never see where they are.",
                "",
                "# ALERTS",
                "When your detection catches a step, an ALERT stops the clock. Some alerts are false alarms; a neglected SIEM cries wolf more often. You can:",
                "  INVESTIGATE: a few days of digging that may evict the attacker. It runs in the background alongside your action.",
                "  CALL THE IR FIRM: $25K and almost certain, if you bought the retainer.",
                "  IGNORE IT: free, and sometimes the right call.",
                "",
                "# INCIDENTS",
                "When an attacker reaches its objective, you get an INCIDENT and must choose a response. Responses trade valuation, trust, brand, and money. Strong resilience softens the damage; cyber insurance pays half the bill. Some responses leave a lasting condition behind.",
            ],
        ),
        page("LASTING CONDITIONS", &[]),
        page(
            "LANDMARKS",
            &[
                "Landmarks wait on the timeline. Forts let you rest; rivers must be crossed.",
                "",
                "WEEK 4, BOARD BRIEFING: pitch the board. Honesty works if your numbers are good.",
                "WEEK 7, SECURITY CONFERENCE: choose who goes and which track they attend. They return with stories, and sometimes expertise.",
                "WEEK 10, SOC 2 AUDIT: fail it and the IPO slips.",
                "WEEK 13, S-1 FILING: disclose past incidents now, or hope nobody finds out later.",
                "WEEK 16, PEN TEST: your defenses get graded. Weak areas cost valuation.",
                "WEEK 19, PUBLIC S-1: the world knows about the IPO, attackers included.",
                "WEEK 23, ROADSHOW: protect the executives on the road.",
                "WEEK 26, IPO DAY: ring the bell.",
                "",
                "# DELAYS",
                "A failed audit, a leak after the public S-1, or systems down at the roadshow pushes the IPO back two weeks. More than six weeks of delays and the board pulls the IPO.",
            ],
        ),
        page(
            "ODDS AND ENDS",
            &[
                "# THE METERS",
                "VALUATION is your score. Incidents, failed landmarks, and weak brand loyalty eat at it.",
                "TRUST is leadership's confidence in you. It decides board funding, and at zero you are fired.",
                "BRAND is the public's confidence. Below 50 it drags the valuation down every week.",
                "",
                "# RANDOM EVENTS",
                "The flu, zero-days, cloud outages, broken coffee machines, and the CEO posting the IPO date online. Some ask you to choose.",
                "",
                "# THE LOG",
                "Everything that happens is written down. Press L to page through it by day.",
                "",
                "# THE CONFERENCE",
                "Tickets are $6K each, and you always go. Analysts pick a track: talks for expertise, villages for hands-on skills or fun, the expo floor for swag, the hallway track for pure rest. While they are away, the rest of the team holds the fort.",
            ],
        ),
        page(
            "A FEW WORDS OF ADVICE",
            &[
                "Buy the coffee subscription. Your analysts will thank you. Your former analysts would have thanked you.",
                "",
                "Deploy what you buy. A tool in a box protects nobody.",
                "",
                "Maintain what you deploy. Attackers love a stale firewall rule.",
                "",
                "Investigate alerts, but not all of them. The SIEM will cry wolf, especially if you neglect it.",
                "",
                "Trust is money. The board funds the SOC it believes in.",
                "",
                "Rest your team before they rest permanently.",
                "",
                "And remember: the attackers only have to be right once. You have to be right until the bell rings.",
            ],
        ),
    ]
    .into_iter()
    .map(|page| {
        if page.title == "LASTING CONDITIONS" {
            conditions()
        } else {
            page
        }
    })
    .collect()
}

/// The conditions page, written from the game's own data so it never goes stale.
fn conditions() -> Page {
    let mut paragraphs = vec![
        "Some incident responses leave a condition behind. Each shows as a red tag under the timeline and adds a Recovery action that clears it.".to_string(),
        String::new(),
    ];
    for condition in Condition::ALL {
        paragraphs.push(format!("# {}", condition.tag()));
        paragraphs.push(format!(
            "{} Cure: {}.",
            condition.effect(),
            condition.cure()
        ));
    }
    Page {
        title: "LASTING CONDITIONS",
        paragraphs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::wrap;

    /// Characters and lines that fit on a page of the manual on screen.
    const WIDTH: usize = 68;
    const LINES: usize = 32;

    #[test]
    fn every_page_fits_on_screen() {
        for page in pages() {
            let lines: usize = page.paragraphs.iter().map(|p| wrap(p, WIDTH).len()).sum();
            assert!(lines <= LINES, "{} has {lines} lines", page.title);
            assert!(!page.paragraphs.is_empty(), "{} is empty", page.title);
        }
    }

    #[test]
    fn the_contents_list_every_page() {
        let pages = pages();
        for (i, page) in pages.iter().enumerate().skip(1) {
            let entry = format!("{}. ", i + 1);
            assert!(
                pages[0]
                    .paragraphs
                    .iter()
                    .any(|p| p.trim_start().starts_with(&entry)
                        && p.to_uppercase().ends_with(page.title)),
                "contents missing {}",
                page.title
            );
        }
    }

    #[test]
    fn every_condition_is_explained() {
        let pages = pages();
        let conditions = pages
            .iter()
            .find(|p| p.title == "LASTING CONDITIONS")
            .unwrap();
        for condition in Condition::ALL {
            assert!(
                conditions
                    .paragraphs
                    .iter()
                    .any(|p| p == &format!("# {}", condition.tag())),
                "{}",
                condition.tag()
            );
        }
    }
}
