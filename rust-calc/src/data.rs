use strum_macros::EnumIter;
use strum_macros::{Display, EnumCount};
use strum_macros::{EnumString, FromRepr};
use wasm_bindgen::prelude::*;

#[derive(Debug, Display, EnumIter, EnumString, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub enum Ability {
    Torrent,
    #[strum(serialize = "Poison Point")]
    PoisonPoint,
    Overgrow,
    #[strum(serialize = "Clear Body")]
    ClearBody,
    #[strum(serialize = "Suction Cups")]
    SuctionCups,
    Swarm,
    Truant,
    Illuminate,
    Immunity,
    Sturdy,
    #[strum(serialize = "Water Absorb")]
    WaterAbsorb,
    #[strum(serialize = "Cloud Nine")]
    CloudNine,
    #[strum(serialize = "Sand Stream")]
    SandStream,
    #[strum(serialize = "Early Bird")]
    EarlyBird,
    Insomnia,
    #[strum(serialize = "Shell Armor")]
    ShellArmor,
    #[strum(serialize = "Cute Charm")]
    CuteCharm,
    #[strum(serialize = "Rain Dish")]
    RainDish,
    Chlorophyll,
    #[strum(serialize = "Sticky Hold")]
    StickyHold,
    #[strum(serialize = "Serene Grace")]
    SereneGrace,
    #[strum(serialize = "Thick Fat")]
    ThickFat,
    #[strum(serialize = "Liquid Ooze")]
    LiquidOoze,
    #[strum(serialize = "Marvel Scale")]
    MarvelScale,
    #[strum(serialize = "Run Away")]
    RunAway,
    Soundproof,
    #[strum(serialize = "Keen Eye")]
    KeenEye,
    #[strum(serialize = "Own Tempo")]
    OwnTempo,
    #[strum(serialize = "Inner Focus")]
    InnerFocus,
    Trace,
    Oblivious,
    #[strum(serialize = "Arena Trap")]
    ArenaTrap,
    #[strum(serialize = "Swift Swim")]
    SwiftSwim,
    #[strum(serialize = "Volt Absorb")]
    VoltAbsorb,
    #[strum(serialize = "Battle Armor")]
    BattleArmor,
    Static,
    #[strum(serialize = "Sand Veil")]
    SandVeil,
    #[strum(serialize = "Natural Cure")]
    NaturalCure,
    #[strum(serialize = "Effect Spore")]
    EffectSpore,
    #[strum(serialize = "Flame Body")]
    FlameBody,
    #[strum(serialize = "Flash Fire")]
    FlashFire,
    Guts,
    Intimidate,
    Damp,
    Lightningrod,
    Pressure,
    Levitate,
    Blaze,
    #[strum(serialize = "Pure Power")]
    PurePower,
    Stench,
    #[strum(serialize = "Water Veil")]
    WaterVeil,
    Synchronize,
    #[strum(serialize = "Rock Head")]
    RockHead,
}

#[derive(Debug, Display, EnumIter, EnumString, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub enum Item {
    BrightPowder,
    Charcoal,
    #[strum(serialize = "Cheri Berry")]
    CheriBerry,
    #[strum(serialize = "Chesto Berry")]
    ChestoBerry,
    #[strum(serialize = "Choice Band")]
    ChoiceBand,
    #[strum(serialize = "Focus Band")]
    FocusBand,
    #[strum(serialize = "King's Rock")]
    KingsRock,
    Leftovers,
    #[strum(serialize = "Leppa Berry")]
    LeppaBerry,
    #[strum(serialize = "Liechi Berry")]
    LiechiBerry,
    #[strum(serialize = "Lum Berry")]
    LumBerry,
    Magnet,
    #[strum(serialize = "Miracle Seed")]
    MiracleSeed,
    #[strum(serialize = "Mystic Water")]
    MysticWater,
    NeverMeltIce,
    #[strum(serialize = "Persim Berry")]
    PersimBerry,
    #[strum(serialize = "Petaya Berry")]
    PetayaBerry,
    #[strum(serialize = "Poison Barb")]
    PoisonBarb,
    #[strum(serialize = "Quick Claw")]
    QuickClaw,
    #[strum(serialize = "Rawst Berry")]
    RawstBerry,
    #[strum(serialize = "Salac Berry")]
    SalacBerry,
    #[strum(serialize = "Scope Lens")]
    ScopeLens,
    #[strum(serialize = "Sharp Beak")]
    SharpBeak,
    #[strum(serialize = "Shell Bell")]
    ShellBell,
    #[strum(serialize = "Sitrus Berry")]
    SitrusBerry,
    #[strum(serialize = "Soft Sand")]
    SoftSand,
    #[strum(serialize = "Thick Club")]
    ThickClub,
    TwistedSpoon,
    #[strum(serialize = "White Herb")]
    WhiteHerb,
}

#[derive(Debug, Display, EnumIter, EnumString, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub enum Move {
    #[strum(serialize = "Acid Armor")]
    AcidArmor,
    #[strum(serialize = "Aerial Ace")]
    AerialAce,
    Agility,
    #[strum(serialize = "Air Cutter")]
    AirCutter,
    Amnesia,
    AncientPower,
    Aromatherapy,
    Astonish,
    Attract,
    Barrier,
    #[strum(serialize = "Baton Pass")]
    BatonPass,
    #[strum(serialize = "Belly Drum")]
    BellyDrum,
    Bite,
    #[strum(serialize = "Blaze Kick")]
    BlazeKick,
    Blizzard,
    Block,
    #[strum(serialize = "Body Slam")]
    BodySlam,
    Bonemerang,
    Bounce,
    #[strum(serialize = "Brick Break")]
    BrickBreak,
    #[strum(serialize = "Bulk Up")]
    BulkUp,
    #[strum(serialize = "Calm Mind")]
    CalmMind,
    Charm,
    #[strum(serialize = "Confuse Ray")]
    ConfuseRay,
    #[strum(serialize = "Cosmic Power")]
    CosmicPower,
    Counter,
    #[strum(serialize = "Cross Chop")]
    CrossChop,
    Crunch,
    #[strum(serialize = "Crush Claw")]
    CrushClaw,
    Curse,
    #[strum(serialize = "Defense Curl")]
    DefenseCurl,
    #[strum(serialize = "Destiny Bond")]
    DestinyBond,
    Detect,
    Dig,
    Disable,
    Dive,
    #[strum(serialize = "Dizzy Punch")]
    DizzyPunch,
    #[strum(serialize = "Double Kick")]
    DoubleKick,
    #[strum(serialize = "Double Team")]
    DoubleTeam,
    #[strum(serialize = "Double-Edge")]
    DoubleEdge,
    #[strum(serialize = "Dragon Claw")]
    DragonClaw,
    #[strum(serialize = "Dragon Dance")]
    DragonDance,
    DragonBreath,
    #[strum(serialize = "Dream Eater")]
    DreamEater,
    #[strum(serialize = "Drill Peck")]
    DrillPeck,
    DynamicPunch,
    Earthquake,
    Encore,
    Endure,
    Explosion,
    Extrasensory,
    ExtremeSpeed,
    Facade,
    #[strum(serialize = "Faint Attack")]
    FaintAttack,
    #[strum(serialize = "Fake Out")]
    FakeOut,
    #[strum(serialize = "Fake Tears")]
    FakeTears,
    #[strum(serialize = "Fire Blast")]
    FireBlast,
    #[strum(serialize = "Fire Punch")]
    FirePunch,
    Fissure,
    Flail,
    Flamethrower,
    Flash,
    Fly,
    #[strum(serialize = "Focus Punch")]
    FocusPunch,
    #[strum(serialize = "Follow Me")]
    FollowMe,
    Frustration,
    #[strum(serialize = "Future Sight")]
    FutureSight,
    #[strum(serialize = "Giga Drain")]
    GigaDrain,
    GrassWhistle,
    Grudge,
    Hail,
    Headbutt,
    #[strum(serialize = "Heat Wave")]
    HeatWave,
    #[strum(serialize = "Hi Jump Kick")]
    HiJumpKick,
    #[strum(serialize = "Horn Drill")]
    HornDrill,
    #[strum(serialize = "Hydro Pump")]
    HydroPump,
    #[strum(serialize = "Hyper Beam")]
    HyperBeam,
    #[strum(serialize = "Hyper Voice")]
    HyperVoice,
    Hypnosis,
    #[strum(serialize = "Ice Beam")]
    IceBeam,
    #[strum(serialize = "Ice Punch")]
    IcePunch,
    #[strum(serialize = "Icy Wind")]
    IcyWind,
    Ingrain,
    #[strum(serialize = "Iron Tail")]
    IronTail,
    #[strum(serialize = "Knock Off")]
    KnockOff,
    #[strum(serialize = "Leaf Blade")]
    LeafBlade,
    #[strum(serialize = "Leech Seed")]
    LeechSeed,
    #[strum(serialize = "Light Screen")]
    LightScreen,
    #[strum(serialize = "Lovely Kiss")]
    LovelyKiss,
    #[strum(serialize = "Luster Purge")]
    LusterPurge,
    #[strum(serialize = "Mach Punch")]
    MachPunch,
    #[strum(serialize = "Magical Leaf")]
    MagicalLeaf,
    #[strum(serialize = "Mean Look")]
    MeanLook,
    #[strum(serialize = "Mega Kick")]
    MegaKick,
    Megahorn,
    Memento,
    #[strum(serialize = "Metal Claw")]
    MetalClaw,
    #[strum(serialize = "Meteor Mash")]
    MeteorMash,
    Metronome,
    #[strum(serialize = "Milk Drink")]
    MilkDrink,
    Minimize,
    #[strum(serialize = "Mirror Coat")]
    MirrorCoat,
    #[strum(serialize = "Mist Ball")]
    MistBall,
    Moonlight,
    #[strum(serialize = "Mud-Slap")]
    MudSlap,
    #[strum(serialize = "Night Shade")]
    NightShade,
    Nightmare,
    Outrage,
    Overheat,
    #[strum(serialize = "Pain Split")]
    PainSplit,
    #[strum(serialize = "Perish Song")]
    PerishSong,
    #[strum(serialize = "Petal Dance")]
    PetalDance,
    Protect,
    #[strum(serialize = "Psych Up")]
    PsychUp,
    Psychic,
    Pursuit,
    #[strum(serialize = "Quick Attack")]
    QuickAttack,
    #[strum(serialize = "Rain Dance")]
    RainDance,
    Recover,
    Reflect,
    Refresh,
    Rest,
    Return,
    Revenge,
    Reversal,
    Roar,
    #[strum(serialize = "Rock Slide")]
    RockSlide,
    #[strum(serialize = "Rock Tomb")]
    RockTomb,
    Rollout,
    Safeguard,
    #[strum(serialize = "Sand Tomb")]
    SandTomb,
    #[strum(serialize = "Sand-Attack")]
    SandAttack,
    Sandstorm,
    #[strum(serialize = "Scary Face")]
    ScaryFace,
    Screech,
    #[strum(serialize = "Secret Power")]
    SecretPower,
    #[strum(serialize = "Seismic Toss")]
    SeismicToss,
    #[strum(serialize = "Shadow Ball")]
    ShadowBall,
    #[strum(serialize = "Sheer Cold")]
    SheerCold,
    #[strum(serialize = "Signal Beam")]
    SignalBeam,
    #[strum(serialize = "Silver Wind")]
    SilverWind,
    Sing,
    #[strum(serialize = "Skill Swap")]
    SkillSwap,
    #[strum(serialize = "Sky Attack")]
    SkyAttack,
    #[strum(serialize = "Sky Uppercut")]
    SkyUppercut,
    Slash,
    #[strum(serialize = "Sleep Powder")]
    SleepPowder,
    #[strum(serialize = "Sleep Talk")]
    SleepTalk,
    #[strum(serialize = "Sludge Bomb")]
    SludgeBomb,
    SmellingSalt,
    SmokeScreen,
    Softboiled,
    SolarBeam,
    Spark,
    Spikes,
    Spore,
    #[strum(serialize = "Steel Wing")]
    SteelWing,
    #[strum(serialize = "Stun Spore")]
    StunSpore,
    Substitute,
    #[strum(serialize = "Sunny Day")]
    SunnyDay,
    Superpower,
    Surf,
    Swagger,
    #[strum(serialize = "Sweet Scent")]
    SweetScent,
    Swift,
    #[strum(serialize = "Swords Dance")]
    SwordsDance,
    Synthesis,
    Thrash,
    Thunder,
    #[strum(serialize = "Thunder Wave")]
    ThunderWave,
    ThunderPunch,
    Thunderbolt,
    Torment,
    Toxic,
    #[strum(serialize = "Tri Attack")]
    TriAttack,
    Trick,
    #[strum(serialize = "Water Pulse")]
    WaterPulse,
    #[strum(serialize = "Will-O-Wisp")]
    WillOWisp,
    Wish,
    Wrap,
    Yawn,
    #[strum(serialize = "Zap Cannon")]
    ZapCannon,
}

#[derive(Debug, Display, EnumIter, EnumString, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub enum Nature {
    Adamant,
    Bold,
    Brave,
    Calm,
    Careful,
    Docile,
    Hardy,
    Impish,
    Jolly,
    Modest,
    Naughty,
    Quiet,
    Quirky,
    Relaxed,
    Sassy,
    Serious,
    Timid,
}

#[derive(Debug, Display, EnumCount, EnumIter, EnumString, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[wasm_bindgen]
pub enum Species {
    Aerodactyl,
    Aggron,
    Alakazam,
    Altaria,
    Ampharos,
    Arcanine,
    Armaldo,
    Articuno,
    Blastoise,
    Blaziken,
    Blissey,
    Breloom,
    Charizard,
    Claydol,
    Clefable,
    Cradily,
    Crobat,
    Dewgong,
    Dodrio,
    Donphan,
    Dragonite,
    Dugtrio,
    Dusclops,
    Electabuzz,
    Electrode,
    Entei,
    Espeon,
    Exeggutor,
    Exploud,
    Fearow,
    Feraligatr,
    Flareon,
    Flygon,
    Forretress,
    Gardevoir,
    Gengar,
    Glalie,
    Golduck,
    Golem,
    Granbull,
    Gyarados,
    Hariyama,
    Heracross,
    Houndoom,
    Hypno,
    Jolteon,
    Jynx,
    Kangaskhan,
    Kingdra,
    Lanturn,
    Lapras,
    Latias,
    Latios,
    Ludicolo,
    Machamp,
    Magmar,
    Manectric,
    Marowak,
    Medicham,
    Meganium,
    Metagross,
    Milotic,
    Miltank,
    Misdreavus,
    Moltres,
    #[strum(serialize = "Mr. Mime")]
    MrMime,
    Muk,
    Nidoking,
    Nidoqueen,
    Ninetales,
    Porygon2,
    Quagsire,
    Raichu,
    Raikou,
    Rapidash,
    Regice,
    Regirock,
    Registeel,
    Rhydon,
    Salamence,
    Sceptile,
    Scizor,
    Shiftry,
    Shuckle,
    Skarmory,
    Slaking,
    Slowbro,
    Slowking,
    Snorlax,
    Starmie,
    Steelix,
    Suicune,
    Swampert,
    Tauros,
    Tentacruel,
    Typhlosion,
    Tyranitar,
    Umbreon,
    Ursaring,
    Vaporeon,
    Venusaur,
    Victreebel,
    Vileplume,
    Wailord,
    Walrein,
    Weezing,
    Whiscash,
    Xatu,
    Zapdos,
}

#[derive(
    Debug, Display, EnumIter, EnumString, EnumCount, FromRepr, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[wasm_bindgen]
pub enum Type {
    Normal = 0,
    Fire = 1,
    Water = 2,
    Electric = 3,
    Grass = 4,
    Ice = 5,
    Fighting = 6,
    Poison = 7,
    Ground = 8,
    Flying = 9,
    Psychic = 10,
    Bug = 11,
    Rock = 12,
    Ghost = 13,
    Dragon = 14,
    Dark = 15,
    Steel = 16,
    Typeless = 17,
}
