// 📓 Simple Note-Taking Application
// This application allows users to add, view, and delete notes with timestamps and IDs.

use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

//  Main function to run the application
fn main() {
    loop {
        println!("\n📋 Menu");
        println!("1 = Add Note");
        println!("2 = View Notes");
        println!("3 = Clear Notes");
        println!("4 = Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");

        match choice.trim() {
            "1" => add_note(),
            "2" => display_saved_notes(),
            "3" => clear_or_delete_notes(),
            "4" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("⚠️ Invalid choice. Try again."),
        }
    }
}

//  Write a note to the file with timestamp and ID
fn write_note(note: &str) {
    let timestamp = Local::now().format("%A, %d %B %Y at %I:%M %p").to_string();
    let next_id = get_next_id("notes.txt");

    // Open the file in append mode and write the note
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("Can't open file");

    writeln!(
        file,
        "✅ ID:{} on  📅{}\n📝 Note:\n{} \n",
        next_id, timestamp, note
    )
    .expect("Failed to write to file");

    println!("✅ Note saved with ID {} on {}.", next_id, timestamp);
}

// ✍️ Add a new note
fn add_note() {
    println!("Enter your note:");
    let mut note = String::new();
    io::stdin()
        .read_line(&mut note)
        .expect("Failed to read name");
    let note = note.trim();

    write_note(&format!("{}", note.trim()));
}

// 📖 Display all notes
fn display_saved_notes() {
    match File::open("notes.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");
            if contents.trim().is_empty() {
                println!("📭 No notes found.");
            } else {
                println!("\n📓 Saved Notes:\n{}", contents);
            }
        }
        Err(_) => println!("🚫 Couldn't open the file."),
    }
}

// 🧹 Clear notes or delete by ID
fn clear_or_delete_notes() {
    println!("\n🧹 Options:");
    println!("1 = Delete by ID");
    println!("2 = Clear All");
    println!("Enter your choice:");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read option");

    match option.trim() {
        "1" => delete_note_by_id(),
        "2" => {
            File::create("notes.txt").expect("Failed to clear file");
            println!("🧼 All notes cleared.");
        }
        _ => println!("⚠️ Invalid option."),
    }
}

// 🗑️ Delete specific note by ID
fn delete_note_by_id() {
    println!("Enter ID to delete:");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read ID");

    let target_id = id_input.trim();

    let file = match File::open("notes.txt") {
        Ok(f) => f,
        Err(_) => {
            println!("🚫 Couldn't open file.");
            return;
        }
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut new_lines = Vec::new();
    let mut skip = false;

    for line in &lines {
        if line.starts_with(&format!("ID: {}", target_id)) {
            skip = true;
        }

        if skip && line == "---" {
            skip = false;
            continue; // skip the separator too
        }

        if !skip {
            new_lines.push(line.to_string());
        }
    }

    if new_lines.len() == lines.len() {
        println!("❌ ID {} not found.", target_id);
    } else {
        let mut file = File::create("notes.txt").expect("Failed to overwrite file");
        for line in new_lines {
            writeln!(file, "{}", line).expect("Failed to write to file");
        }
        println!("✅ Note with ID {} deleted.", target_id);
    }
}

// 🧮 Count existing IDs
fn get_next_id(filename: &str) -> u32 {
    let file =
        File::open(filename).unwrap_or_else(|_| File::create(filename).expect("Can't create file"));
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter(|line| line.as_ref().map_or(false, |l| l.starts_with("✅ ID:")))
        .count() as u32
        + 1
}
