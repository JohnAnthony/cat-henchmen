use dice;
use std::fmt;

pub struct Henchman {
    skills: &'static str,
    personality: &'static str,
    payment: &'static str,
    revenge: &'static str,

    ac: u64,
    hp: u64,

    weapons: &'static str,
}

const SKILLS: &'static [&'static str; 10] = &[
    "Storm trooper: Violence +1 to hit and damage",
    "Meat Shield: AC+1",
    "Scout: 2/6 chance to detect traps",
    "Skirmisher: Throws spears and then runs away. Also carries a bow",
    "Ambusher: They hide then jump out and backstab opponents",
    "Know-it-all: Languages and obscure knowledge is their thing",
    "Mule: Carry stuff, break down doors (automatically succeeds)",
    "Performer: They can dance, play an instrument and carry a tune",
    "Nothing/Useless: Although this may not be immediately noticeable",
    "Really useless: But skilled at getting other henchmen to do their tasks"
];

const PERSONALITIES: &'static [&'static str; 10] = &[
    "Arrogant",
    "Snivelling and cowardly",
    "Pervert",
    "Obsessed with money",
    "Drunk or drugged up",
    "Workshy",
    "Thrill seeker",
    "Psychotic",
    "Damaged",
    "Schizophrenic",
];

const PAYMENTS: &'static [&'static str; 10] = &[
    "An equal share of the loot",
    "A flat fee. d10 x 5 per day",
    "A magic item",
    "Enough treasure to never work again",
    "Food and lodging",
    "Beer and partying after a successful adventure",
    "Bragging rights",
    "Revenge",
    "With attention",
    "Don't know you tell me?",
];

const REVENGES: &'static [&'static str; 10] = &[
    "Damage reputation",
    "Hurt loved ones",
    "Immediate violence",
    "Backstab",
    "Run away",
    "Theft",
    "Go fetch the PC's enemies",
    "Unleash hell",
    "Long-term annoyance",
    "Double nasty. Roll two times",
];

const WEAPONS: &'static [&'static str; 10] = &[
    "Club (1d6)",
    "Sling (1d6) and Dagger (1d4)",
    "Five throwing daggers (1d6)",
    "Staff (1d6)",
    "Shortsword (1d6) and dagger (1d4)",
    "A rock (1d3 thrown or as melee)",
    "Longsword (1d8)",
    "Short bow (1d6) and dagger (1d4)",
    "A dagger (1d4)",
    "No weapon (1d2)",
];

impl Henchman {
    pub fn new() -> Henchman {
        let skill_roll = dice::d(1, 10);

        let ac;
        if skill_roll == 2 {
            ac = 13;
        } else {
            ac = 12;
        }

        Henchman {
            skills: SKILLS[(skill_roll - 1) as usize],
            personality: PERSONALITIES[(dice::d(1, 10) - 1) as usize],
            payment: PAYMENTS[(dice::d(1, 10) - 1) as usize],
            revenge: REVENGES[(dice::d(1, 10) - 1) as usize],

            ac: ac,
            hp: dice::d(1, 8),

            weapons: WEAPONS[(dice::d(1, 10) - 1) as usize],
        }
    }
}

impl fmt::Display for Henchman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"SKILLS:      {}
PERSONALITY: {}
PAYMENT:     {}
REVENGE:     {}

AC:          {}
HP:          {}

WEAPONS:     {}"#,
            self.skills,
            self.personality,
            self.payment,
            self.revenge,
            self.ac,
            self.hp,
            self.weapons
        )
    }
}
