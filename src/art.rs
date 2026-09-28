//! 24x16 pixel-art sprites for turn reports. Each character is one pixel;
//! `.` is transparent and the rest are palette colors mapped in `draw`.

pub type Sprite = [&'static str; 16];

pub const WIDTH: usize = 24;

pub const HUNT: Sprite = [
    "........................",
    ".......wwwwww...........",
    ".....ww......ww.........",
    "....w..........w........",
    "...w...r....r...w.......",
    "...w....r..r....w.......",
    "..w....rrrrrr....w......",
    "..w...r.rrrr.r...w......",
    "..w....rrrrrr....w......",
    "...w..r.rrrr.r..w.......",
    "...w............w.......",
    "....w..........wbb......",
    ".....ww......wwbbbb.....",
    ".......wwwwww...bbbbb...",
    "..................bbbbb.",
    "....................bb..",
];

pub const SPEND: Sprite = [
    "........................",
    ".........bbbbbb.........",
    "..........bbbb..........",
    "...........bb...........",
    ".........aaaaaa.........",
    ".......aaaaaaaaaa.......",
    "......aaaaaakaaaaa......",
    ".....aaaaakkkkkaaaa.....",
    ".....aaaakaakaaaaaa.....",
    "....aaaaaakkkkaaaaaa....",
    "....aaaaaaaakaakaaaa....",
    "....aaaaakkkkkaaaaaa....",
    ".....aaaaaakaaaaaaa.....",
    "......aaaaaaaaaaaa......",
    ".......aaaaaaaaaa.......",
    "........................",
];

pub const SLEEP: Sprite = [
    ".................wwww...",
    "...................w....",
    "...........www....w.....",
    ".............w...wwww...",
    "............w...........",
    "...........w............",
    "...........www..........",
    "bb......................",
    "bbwww...................",
    "bbwwwll.................",
    "bbwwlllccccccccccccccccc",
    "bbwwwllccccccccccccccccc",
    "bbbbbbbbbbbbbbbbbbbbbbbb",
    "bbbbbbbbbbbbbbbbbbbbbbbb",
    "bb....................bb",
    "bb....................bb",
];

pub const PATCH: Sprite = [
    "........................",
    ".dddddddddddd......ll.ll",
    ".dnnnnnnnnnnd......ll.ll",
    ".dnllllllgnnd......lllll",
    ".dnnnnnnnnnnd.......lll.",
    ".dnllllllgnnd......ll...",
    ".dnnnnnnnnnnd.....ll....",
    ".dnllllllrnnd....ll.....",
    ".dnnnnnnnnnnd...ll......",
    ".dnllllllgnnd..ll.......",
    ".dnnnnnnnnnnd.ll........",
    ".dnllllllgnndll.........",
    ".dnnnnnnnnnnd...........",
    ".dddddddddddd...........",
    "..d........d............",
    "........................",
];

pub const PHISH: Sprite = [
    "............l...........",
    "............l...........",
    "............l...........",
    "........l...l...........",
    "........ll..l...........",
    ".........llll...........",
    "..wwwwwwwwwwwwwwwwwwww..",
    "..wdwwwwwwwwwwwwwwwwdw..",
    "..wwdwwwwwwwwwwwwwwdww..",
    "..wwwdwwwwwwwwwwwwdwww..",
    "..wwwwddwwwwwwwwddwwww..",
    "..wwwwwwddwwwwddwwwwww..",
    "..wwwwwwwwddddwwwwwwww..",
    "..wwwwwwwwwwwwwwwwwwww..",
    "..wwwwwwwwwwwwwwwwwwww..",
    "........................",
];

pub const COFFEE: Sprite = [
    "........................",
    "........w...w...w.......",
    ".......w...w...w........",
    "........w...w...w.......",
    ".......w...w...w........",
    "........................",
    "....wwwwwwwwwwwwwwww....",
    "....wbbbbbbbbbbbbbbw....",
    "....wwwwwwwwwwwwwwwwww..",
    "....wwwwrrwwwwrrwwww..w.",
    "....wwwwrrwwwwrrwwww..w.",
    "....wwwwwwwwwwwwwwww..w.",
    "....wwwwwwrrrrwwwwwwww..",
    ".....wwwwwwwwwwwwwww....",
    "......wwwwwwwwwwwww.....",
    "...dddddddddddddddddd...",
];

pub const BLOCK: Sprite = [
    "........................",
    "............llllllll....",
    "..rr........bbblbbbl....",
    "..rr..rr....bbblbbbl....",
    "......rr....llllllll....",
    "............blbbblbb...g",
    "rr..........blbbblbb..g.",
    "rr...rr.....llllllllg.g.",
    ".....rr.....bbblbbbl.g..",
    "............bbblbbbl....",
    "...rr.......llllllll....",
    "...rr..rr...blbbblbb....",
    ".......rr...blbbblbb....",
    "............llllllll....",
    "............bbblbbbl....",
    "............bbblbbbl....",
];

pub const ISOLATE: Sprite = [
    "........................",
    "..dddddddddddddddddddd..",
    "..dkkkkkkkkkkkkkkkkkkd..",
    "..dkkkkkkkaaaakkkkkkkd..",
    "..dkkkkkkakkkkakkkkkkd..",
    "..dkkkkkkakkkkakkkkkkd..",
    "..dkkkkkaaaaaaaakkkkkd..",
    "..dkkkkkaaakkaaakkkkkd..",
    "..dkkkkkaaakkaaakkkkkd..",
    "..dkkkkkaaaaaaaakkkkkd..",
    "..dkkkkkkkkkkkkkkkkkkd..",
    "..dddddddddddddddddddd..",
    ".dllllllllllllllllllllld",
    "dllllllllllllllllllllld.",
    "dddddddddddddddddddddddd",
    "........................",
];

pub const UNPLUG: Sprite = [
    "........................",
    "...............y........",
    ".............y...y......",
    "..............y.........",
    "........................",
    "....llllll.......dddddd.",
    "....llllllll..y..dkkkkd.",
    "ddddllllll...y.y.dddddd.",
    "ddddllllll..y....dddddd.",
    "....llllllll..y..dkkkkd.",
    "....llllll.......dddddd.",
    "........................",
    ".............y..........",
    "...........y...y........",
    ".............y..........",
    "........................",
];

pub const PRESS: Sprite = [
    "..y..................y..",
    ".yyy.....lll........yyy.",
    "..y.....lllll........y..",
    "........lllll...........",
    ".........lll............",
    "..........d.............",
    "..........d.............",
    "....bbbbbbbbbbbbbbbb....",
    "....bbbbbbbbbbbbbbbb....",
    ".....bbbbbbbbbbbbbb.....",
    ".....bbbbbaaaabbbbb.....",
    ".....bbbbbaaaabbbbb.....",
    ".....bbbbbbbbbbbbbb.....",
    ".....bbbbbbbbbbbbbb.....",
    ".....bbbbbbbbbbbbbb.....",
    "....dddddddddddddddd....",
];

pub const SKULL: Sprite = [
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

pub const BOX: Sprite = [
    "........................",
    "..........g..g..........",
    "..........gggg..........",
    "...........gg...........",
    "..........rrrr..........",
    "..llll....rrrr....ww....",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    ".baaaaaaaaaaaaaaaaaaaab.",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbbbbbbbbbb.",
    "........................",
];

pub const TOMBSTONE: Sprite = [
    "........................",
    "........llllllll........",
    ".......llllllllll.......",
    "......llllllllllll......",
    "......llllllllllll......",
    "......llkkllklkkll......",
    "......llklklklklkl......",
    "......llkkllklkkll......",
    "......llklklklklll......",
    "......llklklklklll......",
    "......llllllllllll......",
    "......llllllllllll......",
    "......llllllllllll......",
    "..G.G.llllllllllll.G.G..",
    "GGGGGGGGGGGGGGGGGGGGGGGG",
    "........................",
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Pixel characters the renderer in `draw` knows how to color.
    const PALETTE: &str = ".kndlwgGcarby";

    #[test]
    fn sprites_are_full_width_and_use_the_palette() {
        for sprite in [
            HUNT, SPEND, SLEEP, PATCH, PHISH, COFFEE, BLOCK, ISOLATE, UNPLUG, PRESS, SKULL, BOX,
            TOMBSTONE,
        ] {
            for row in sprite {
                assert_eq!(row.len(), WIDTH, "row {row:?}");
                assert!(row.chars().all(|c| PALETTE.contains(c)), "row {row:?}");
            }
        }
    }
}
