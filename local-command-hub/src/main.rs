use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time::Duration};

mod assistant;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn score(task: &str) -> u8 {
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

fn summary(content: &str) {
    let total = content.lines().count();
    let urgent = content.matches("score:3").count();

    println!("\n========== SYSTEM SUMMARY ==========");
    println!("Stored memories : {}", total);
    println!("Urgent tasks    : {}", urgent);

    if urgent > 0 {
        println!("⚠ Attention required.");
    }

    println!("====================================\n");
}

fn boot_screen() {
    println!("====================================");
    println!("        COGNITIVE OS v1.0");
    println!("====================================");
    println!("Booting memory engine...");
    println!("Loading assistant layer...");
    println!("Initializing local cognition...");
    println!("System online.\n");
}

/* ================================
   PHASE 13 — AUTONOMOUS LOOP
   ================================ */

fn autonomous_brain_loop() {
    loop {
        thread::sleep(Duration::from_secs(10));

        let content = read_to_string("brain.db").unwrap_or(String::new());

        if content.trim().is_empty() {
            continue;
        }

        println!("\n🧠 [AUTONOMOUS CYCLE] Cognitive scan...");

        assistant::cognitive_state_report(&content);
        assistant::predict_next_action(&content);
        assistant::auto_suggest(&content);

        assistant::memory_compression(&content);
        assistant::habit_detection(&content);
        assistant::daily_brain_report(&content);

        // PHASE 14
        assistant::memory_evolution(&content);
    }
}

fn help_menu() {
    println!("\n=========== COMMANDS ===========");
    println!("hub add <task>");
    println!("hub view");
    println!("hub search <word>");
    println!("hub summary");
    println!("hub reflect");
    println!("hub state");
    println!("hub compress");
    println!("hub clear");
    println!("hub help");
    println!("hub exit");
    println!("================================\n");
}

/* ================================
   PHASE 12 PIPELINE
   ================================ */

fn run_phase_12(content: &str) {
    assistant::memory_compression(content);
    assistant::self_writing_logs(content);
    assistant::habit_detection(content);
    assistant::daily_brain_report(content);
    assistant::autonomous_goal_generation(content);
}

/* ================================
   PHASE 14 PIPELINE
   ================================ */

fn run_phase_14(content: &str) {
    assistant::memory_evolution(content);
}

fn main() {
    boot_screen();

    // PHASE 13 BACKGROUND BRAIN
    thread::spawn(|| {
        autonomous_brain_loop();
    });

    loop {
        print!("hub> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "hub help" {
            help_menu();
        }

        /* =========================
           ADD PIPELINE (CORE BRAIN)
           ========================= */
        else if input.starts_with("hub add ") {
            let raw = input.replace("hub add ", "");

            let mut task = raw.clone();
            let mut tag = "#general".to_string();

            if let Some(pos) = raw.find('#') {
                task = raw[..pos].trim().to_string();
                tag = format!("#{}", &raw[pos + 1..]);
            }

            let sc = score(&raw);
            let ts = now();

            let entry = format!("{} | {} | {} | score:{}", ts, task, tag, sc);

            let mut file = OpenOptions::new()
.create(true)
                .append(true)
                .open("brain.db")
                .unwrap();

            writeln!(file, "{}", entry).unwrap();

            assistant::proactive_feedback(&raw, sc);

            let content = read_to_string("brain.db").unwrap_or(String::new());

            // PHASE 8–11 STACK
            assistant::cognitive_state_report(&content);
            assistant::predict_next_action(&content);
            assistant::auto_suggest(&content);

            // PHASE 12
            run_phase_12(&content);

            // PHASE 14 (ADDED HERE)
            run_phase_14(&content);
        }

        /* =========================
           VIEW
           ========================= */
        else if input == "hub view" {
            let content = read_to_string("brain.db").unwrap_or(String::new());

            println!("\n=========== MEMORY ===========");

            let mut lines: Vec<&str> = content.lines().collect();

            lines.sort_by(|a, b| {
                let a_score = if a.contains("score:3") { 1 } else { 0 };
                let b_score = if b.contains("score:3") { 1 } else { 0 };
                b_score.cmp(&a_score)
            });

            for line in lines {
                if line.contains("score:3") {
                    println!("🔥 {}", line);
                } else if line.contains("score:2") {
                    println!("💡 {}", line);
                } else {
                    println!("🧠 {}", line);
                }
            }

            let content = read_to_string("brain.db").unwrap_or(String::new());

            let urgent = content.matches("score:3").count();
            if urgent > 0 {
                println!("\n🤖 {} urgent memories detected.", urgent);
            }

            assistant::attention_layer(&content);

            println!("==============================\n");
        }

        /* =========================
           SEARCH
           ========================= */
        else if input.starts_with("hub search ") {
            let query = input.replace("hub search ", "").to_lowercase();

            let content = read_to_string("brain.db").unwrap_or(String::new());

            println!("\n========= SEARCH =========");

            let mut found = false;

            for line in content.lines() {
                if line.to_lowercase().contains(&query) {
                    println!("• {}", line);
                    found = true;
                }
            }

            if !found {
                println!("No matching memory found.");
            }

            println!("==========================\n");
        }

        /* =========================
           SUMMARY
           ========================= */
        else if input == "hub summary" {
            let content = read_to_string("brain.db").unwrap_or(String::new());
            summary(&content);
        }

        /* =========================
           REFLECT
           ========================= */
        else if input == "hub reflect" {
            let content = read_to_string("brain.db").unwrap_or(String::new());
            assistant::reflect(&content);
        }

        /* =========================
           STATE
           ========================= */
        else if input == "hub state" {
            let content = read_to_string("brain.db").unwrap_or(String::new());
            assistant::cognitive_state_report(&content);
        }

        /* =========================
           PHASE 12 MANUAL
           ========================= */
        else if input == "hub compress" {
            let content = read_to_string("brain.db").unwrap_or(String::new());
            run_phase_12(&content);
        }

        /* =========================
           IDLE MODE
           ========================= */
        else if input == "" {
            let content = read_to_string("brain.db").unwrap_or(String::new());

            println!("🧠 System idle... analyzing memory state...");
            assistant::auto_suggest(&content);
        }
/* =========================
           EXIT
           ========================= */
        else if input == "hub exit" {
            println!("Saving system state...");
            println!("Shutting down Cognitive OS...");
            break;
        }

        /* =========================
           NATURAL LANGUAGE
           ========================= */
        else if input.contains("status") {
            println!("🤖 Cognitive OS is running normally.");
        } else if input.contains("motivate") {
            println!("🤖 You're building your own local intelligence system.");
        }

        else {
            println!("Unknown command.");
            println!("Type: hub help");
        }
    }
}