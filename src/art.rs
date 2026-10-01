//! Pixel-art sprites. Each character is one pixel; `.` is transparent and the
//! rest are palette colors mapped in `draw`.

use crate::attack::Actor;
use crate::conference::Track;

pub type Sprite = &'static [&'static str];

/// Ransomware: the skull and crossbones of every ransom note.
const RANSOM: Sprite = &[
    "........................",
    ".......gggggggggg.......",
    "......gggggggggggg......",
    ".....gggggggggggggg.....",
    ".....gggggggggggggg.....",
    ".....ggrrrggggrrrgg.....",
    ".....ggrrrggggrrrgg.....",
    ".....ggrrrggggrrrgg.....",
    ".....gggggggkkggggg.....",
    "......ggggggkkgggg......",
    ".......gggggggggg.......",
    "........gkgkgkgkg.......",
    "........gkgkgkgkg.......",
    ".........ggggggg........",
    "........................",
    "........................",
];

/// A data breach: a database spilling records.
const LEAK: Sprite = &[
    "........................",
    ".....cccccccccccc.......",
    "....cnnnnnnnnnnnnc......",
    "....ccccccccccccccc.....",
    "....cnnnnnnnnnnnnnc.....",
    "....ccccccccccccccc.....",
    "....cnnnnnnnnnnnnnc.....",
    "....ccccccccccccccc.....",
    "....cnnnnnnnnnnnnnc.....",
    "....ccccccccccccccc.....",
    "............r...........",
    "...........rwr..........",
    "............r....w.w....",
    "......w.w..w.w.......w..",
    "....w......w...w..w.....",
    "..w....w........w....w..",
];

/// Espionage: a spy peeking over the product roadmap.
const SPY: Sprite = &[
    "........................",
    "........dddddddd........",
    "........dddddddd........",
    ".....dddddddddddddd.....",
    "........wwwwwwww........",
    ".......wkkwwwwkkw.......",
    ".......wkkwwwwkkw.......",
    "........wwwwwwww........",
    "...bbbbbbbbbbbbbbbbbb...",
    "...bwwwwwwwwwwwwwwwwb...",
    "...bwkkkkkkkwwwwwwwwb...",
    "...bwwwwwwwwwwwwwwwwb...",
    "...bwkkkkwwwwwgggwwwb...",
    "...bwwwwwwwwwwgggwwwb...",
    "...bwkkkkkkkkwwwwwwwb...",
    "...bbbbbbbbbbbbbbbbbb...",
];

/// Hacktivists: a website flooded off the air.
const FLOOD: Sprite = &[
    "a.......a.......a.......",
    ".a.......a.......a......",
    "..a.......a.......a.....",
    "...dddddddddddddddddd...",
    "...dkkkkkkkkkkkkkkkkd...",
    "...dkrkkkkkkkkkkkkrkd...",
    "...dkkrkkkkkkkkkkrkkd...",
    "...dkkkrkkkkkkkkrkkkd...",
    "...dkkkkrkkkkkkrkkkkd...",
    "...dkkkkkrkkkkrkkkkkd...",
    "...dkkkkkkrrrrkkkkkkd...",
    "...dkkkkkkrrrrkkkkkkd...",
    "...dddddddddddddddddd...",
    ".........dddddd.........",
    "......dddddddddddd......",
    "........................",
];

/// Nation-state APT: eyes in the dark of your own network.
const LURKER: Sprite = &[
    "nnnnnnnnnnnnnnnnnnnnnnnn",
    "nnnnnnnnnnnnnnnnnnnnnnnn",
    "nndddnnnnnnnnnnnnnndddnn",
    "nndgdnnnnnnnnnnnnnndgdnn",
    "nndddnnnnnnnnnnnnnndddnn",
    "nnnnnnnnnnnnnnnnnnnnnnnn",
    "nnnnnnyyyynnnnyyyynnnnnn",
    "nnnnnyykkyynnyykkyynnnnn",
    "nnnnnnyyyynnnnyyyynnnnnn",
    "nnnnnnnnnnnnnnnnnnnnnnnn",
    "nndddnnnnnnnnnnnnnndddnn",
    "nndgdnnnnnnnnnnnnnndrdnn",
    "nndddnnnnnnnnnnnnnndddnn",
    "nnnnnnnnnnnnnnnnnnnnnnnn",
    "nnnnnnnnnnnnnnnnnnnnnnnn",
    "nnnnnnnnnnnnnnnnnnnnnnnn",
];

/// Insider leak: the draft S-1 slipping out in an envelope.
const ENVELOPE: Sprite = &[
    "........................",
    "........wwwwwwwwww......",
    "........wkkkkkkkkw......",
    "........wwwwwwwwww......",
    "........wkkkkkkkww......",
    "...lllllllllllllllllll..",
    "...llwwlllllllllllwwll..",
    "...llllwwlllllllwwllll..",
    "...llllllwwlllwwllllll..",
    "...llllllllwwwllllllll..",
    "...lllllllllllllllllll..",
    "...lllllllllllllrrrrll..",
    "...lllllllllllllrllrll..",
    "...lllllllllllllrrrrll..",
    "...lllllllllllllllllll..",
    "........................",
];

/// Talks: a speaker at the podium.
const PODIUM: Sprite = &[
    "........................",
    "..........www...........",
    ".........wwwww..........",
    "..........www...........",
    ".........ccccc..d.......",
    "........ccccccc.d.......",
    "........cc.c.cc.d.......",
    "......bbbbbbbbbbbbb.....",
    "......bbbbbbbbbbbbb.....",
    ".......bbbbyyybbbb......",
    ".......bbbbyyybbbb......",
    ".......bbbbbbbbbbb......",
    ".......bbbbbbbbbbb......",
    ".......bbbbbbbbbbb......",
    ".....bbbbbbbbbbbbbbb....",
    "........................",
];

/// Villages: a padlock and its pick.
const LOCK: Sprite = &[
    "........................",
    ".........llllll.........",
    "........ll....ll........",
    ".......ll......ll.......",
    ".......ll......ll.......",
    ".......ll......ll.......",
    ".....yyyyyyyyyyyyyy.....",
    ".....yyyyyyyyyyyyyy.....",
    ".....yyyyyykkyyyyyy.....",
    ".....yyyyykkkkyyyyy.....",
    ".....yyyyyykkllllllllll.",
    ".....yyyyyykkyyyyyyy..l.",
    ".....yyyyyyyyyyyyyy.....",
    ".....yyyyyyyyyyyyyy.....",
    "........................",
    "........................",
];

/// Expo floor: a vendor booth giving away t-shirts.
const BOOTH: Sprite = &[
    "........................",
    "..rrrrrrrrrrrrrrrrrrrr..",
    "..rwwrwwrwwrwwrwwrwwrr..",
    "..rrrrrrrrrrrrrrrrrrrr..",
    "..d..................d..",
    "..d...cc.cc..gg.gg...d..",
    "..d...ccccc..ggggg...d..",
    "..d....ccc....ggg....d..",
    "..d....ccc....ggg....d..",
    "..d..................d..",
    "..llllllllllllllllllll..",
    "..lwwwwwwwwwwwwwwwwwwl..",
    "..lwwaawwaawwaawwaawwl..",
    "..lwwwwwwwwwwwwwwwwwwl..",
    "..llllllllllllllllllll..",
    "........................",
];

/// Hallway track and parties: drinks and a karaoke mic.
const PARTY: Sprite = &[
    "...a........y........a..",
    "........a.......y.......",
    "..y.......lll.......a...",
    "..........lll...........",
    "...........d............",
    "...........d....wwwww...",
    "..wwwww....d....waaaw...",
    "..wcccw....d.....waw....",
    "..wcccw....d.....waw....",
    "...wcw.....d......w.....",
    "...wcw.....d......w.....",
    "....w......d......w.....",
    "....w......d....wwwww...",
    "..wwwww....d............",
    "...........d............",
    "........ddddddd.........",
];

/// Timeline icon for a fort: a flag over a wall with a gate.
pub const FORT: Sprite = &[
    "...ra...", "...rrr..", "...a....", "l.l.l.l.", "llllllll", "lllkklll", "lllkklll", "llllllll",
];

/// Timeline icon for a river crossing.
pub const RIVER: Sprite = &[
    "........", ".cc..cc.", "c..cc..c", "........", ".cc..cc.", "c..cc..c", "........", "........",
];

/// The IPO bell at the end of the timeline.
pub const BELL: Sprite = &[
    "...yy...", "..yyyy..", ".yyyyyy.", ".yyyyyy.", ".yyyyyy.", "yyyyyyyy", "...bb...", "........",
];

/// The timeline marker: an analyst pushing a server rack toward the IPO.
pub const MARKER: Sprite = &[
    "..w.........",
    ".www..lllll.",
    "..w...lglrl.",
    ".ccccclllll.",
    ".cc...lglll.",
    ".cc...lllll.",
    ".d.d..lllll.",
    ".d..d.d...d.",
];

pub fn incident(actor: Actor) -> Sprite {
    match actor {
        Actor::Ransomware => RANSOM,
        Actor::DataThief => LEAK,
        Actor::Espionage => SPY,
        Actor::Hacktivists => FLOOD,
        Actor::Apt => LURKER,
        Actor::Insider => ENVELOPE,
    }
}

pub fn track(track: Track) -> Sprite {
    match track {
        Track::Talks => PODIUM,
        Track::Villages => LOCK,
        Track::Expo => BOOTH,
        Track::Hallway => PARTY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pixel characters the renderer in `draw` knows how to color.
    const PALETTE: &str = ".kndlwgGcarby";

    fn assert_sprite(sprite: Sprite, width: usize, height: usize) {
        assert_eq!(sprite.len(), height, "{sprite:?}");
        for row in sprite {
            assert_eq!(row.len(), width, "row {row:?}");
            assert!(row.chars().all(|c| PALETTE.contains(c)), "row {row:?}");
        }
    }

    #[test]
    fn illustrations_are_24_by_16_in_the_palette() {
        for actor in Actor::ALL {
            assert_sprite(incident(actor), 24, 16);
        }
        for t in Track::ALL {
            assert_sprite(track(t), 24, 16);
        }
    }

    #[test]
    fn timeline_sprites_fit_the_bar_in_the_palette() {
        for sprite in [FORT, RIVER, BELL] {
            assert_sprite(sprite, 8, 8);
        }
        assert_sprite(MARKER, 12, 8);
    }

    #[test]
    fn every_incident_and_track_has_its_own_picture() {
        let pictures: Vec<Sprite> = Actor::ALL
            .map(incident)
            .into_iter()
            .chain(Track::ALL.map(track))
            .collect();
        for (i, picture) in pictures.iter().enumerate() {
            assert!(!pictures[i + 1..].contains(picture));
        }
    }
}
