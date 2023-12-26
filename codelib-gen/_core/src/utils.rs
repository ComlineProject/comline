// Standard Uses

// Crate Uses

// External Uses


const GENERATION_NOTE: &str = " Generated with Comline compiler and code generator\n
 Advisory: This file is hashed and should not be manually edited, the runtime will complain
 and might not even run if so (this is to be decided yet)
";

pub fn generation_note(prefix: &str) -> String {
    let mut note = String::new();

    for line in GENERATION_NOTE.lines() {
        note += prefix; note += line;
        note += "\n";
    }

    note
}

