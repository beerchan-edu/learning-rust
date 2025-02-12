enum RuneType{
    Elemental(ElementalRune),
    Special(SpecialRune),
}

enum SpecialRune{
    Frog,
    Unknown
}
enum ElementalRune {
    Fire,
    Water,
    Air,
    Earth,
}

impl RuneType {
    fn find_rune_power (rune: &Option<RuneType>) -> String { 
        match rune.as_ref().unwrap_or(&RuneType::Special(SpecialRune::Unknown)) {
            RuneType::Elemental(ElementalRune::Fire) => String::from("Burns enemies to ashes!"),
            RuneType::Elemental(ElementalRune::Water) => String::from("Heals allies and restores mana."),
            RuneType::Elemental(ElementalRune::Air) => String::from("Enhances speed and agility."),
            RuneType::Elemental(ElementalRune::Earth) => String::from("Provides a strong defensive barrier."),
            RuneType::Special(SpecialRune::Frog) => String::from("Summons wednesday, my dudes!"),
            RuneType::Special(SpecialRune::Unknown) => String::from("This rune has unknown power."),
        }
    }
}

impl RuneType{
    fn classify_rune (rune: &Option<RuneType>) {
        if let Some(RuneType::Elemental(_)) = rune {
            println!("This is an Elemental Rune!");
        }
        else if let Some(RuneType::Special(_)) = rune {
            println!("This is a Special Rune!");
        }
        else {
            println!("No rune found.");
        }
    }
}

pub fn mystic_rune_system() {
    let fire_rune = Some(RuneType::Elemental(ElementalRune::Fire));
    let water_rune = Some(RuneType::Elemental(ElementalRune::Water));
    let earth_rune = Some(RuneType::Elemental(ElementalRune::Earth));
    let air_rune = Some(RuneType::Elemental(ElementalRune::Air));
    let unknown_rune = Some(RuneType::Special(SpecialRune::Unknown));
    let frog_rune = Some(RuneType::Special(SpecialRune::Frog));
    let poo_rune: Option<RuneType> = None;

    println!("{}", RuneType::find_rune_power(&fire_rune));
    println!("{}", RuneType::find_rune_power(&water_rune));
    println!("{}", RuneType::find_rune_power(&air_rune));
    println!("{}", RuneType::find_rune_power(&earth_rune));
    println!("{}", RuneType::find_rune_power(&unknown_rune));
    println!("{}", RuneType::find_rune_power(&frog_rune));
    println!("{}", RuneType::find_rune_power(&poo_rune));

    RuneType::classify_rune(&poo_rune);
    RuneType::classify_rune(&frog_rune);
    RuneType::classify_rune(&air_rune);
}


// 🧙‍♂️ Task: The Mystic Rune System (Advanced Version)

// In the land of Eldoria, wizards use Mystic Runes to cast spells. Each rune has a special meaning and is classified into different rune types.
// Step 1: Define Nested Enums

// Create an enum hierarchy to classify runes:

//     Elemental Runes (ElementalRune): Fire, Water, Air, Earth.
//     Special Runes (SpecialRune): Frog, Unknown.
//     General Rune Type (RuneType), which wraps both categories.

// Step 2: Create find_rune_power

//     Takes an optional rune (Option<RuneType>).
//     Returns a String describing the rune’s magical effect:
//         Fire: "Burns enemies to ashes!"
//         Water: "Heals allies and restores mana."
//         Air: "Enhances speed and agility."
//         Earth: "Provides a strong defensive barrier."
//         Frog: "Summons Wednesday, my dudes!"
//         Unknown or None: "This rune has no power."

// Step 3: Create classify_rune

//     Takes an optional rune (Option<RuneType>).
//     Uses if let ... else to:
//         Print "This is an Elemental Rune!" if it belongs to ElementalRune.
//         Print "This is a Special Rune!" if it belongs to SpecialRune.
//         Print "No rune found." if None.

// Step 4: Test the System

//     In main, create a few runes, some wrapped in Some(), and one that is None.
//     Print their magical effects using find_rune_power.
//     Classify them using classify_rune.

// 📜 Rules:

// ✅ Use an ordinary enum (RuneType) for rune types.
// ✅ Work with Option<T> for handling rune values.
// ✅ Implement nested enums (ElementalRune, SpecialRune).
// ✅ Use if let ... else in classify_rune.
