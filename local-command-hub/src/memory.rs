use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn score(task: &str) -> u8 {
    let urgent = ["urgent", "fix", "asap", "deadline", "important"];

    for word in urgent {
        if task.to_lowercase().contains(word) {
            return 3;
        }
    }

    if task.contains("#idea") {
        return 2;
    }

    1
}

pub fn add_memory(raw: String) {
    let mut task = raw.clone();
    let mut tag = "#general".to_string();

    if let Some(pos) = raw.find('#') {
        task = raw[..pos].trim().to_string();
        tag = format!("#{}", &raw[pos + 1..]);
    }

    let sc = score(&raw);
    let ts = now();

    let entry = format!(
        "{} | {} | {} | score:{}",
        ts, task, tag, sc
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("brain.db")
        .unwrap();

    writeln!(file, "{}", entry).unwrap();

    println!("✓ memory stored");

    if sc == 3 {
        println!("⚠ marked as high priority");
    }
}

pub fn view_memory() {
    let content = read_to_string("brain.db")
        .unwrap_or(String::new());

    println!("\n=========== MEMORY (INTELLIGENT VIEW) ===========");

    let mut lines: Vec<&str> = content.lines().collect();

    // smarter ordering: newest first + high priority boost
    lines.sort_by(|a, b| b.cmp(a));

    for line in &lines {
        if line.contains("score:3") {
            println!("🔥 {}", line);
        } else if line.contains("score:2") {
            println!("💡 {}", line);
        } else {
            println!("🧠 {}", line);
        }
    }

    println!("================================================\n");

    println!("🤖 Insight: prioritize 🔥 items first.");
}

pub fn search_memory(query: String) {
    let content = read_to_string("brain.db")
        .unwrap_or(String::new());

    println!("\n========= SEARCH =========");

    let mut found = false;

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            println!("• {}", line);
            found = true;
        }
    }

    if !found {
        println!("No matching memory found.");
    }

    println!("==========================\n");
}