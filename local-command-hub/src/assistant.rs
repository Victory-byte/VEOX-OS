pub fn boot_screen() {
    println!("====================================");
    println!("        COGNITIVE OS v2.0");
    println!("====================================");
    println!("Booting memory engine...");
    println!("Loading assistant layer...");
    println!("Initializing cognition...");
    println!("System online.\n");
}

pub fn assistant_reply(input: &str) {
    if input.contains("status") {
        println!("🤖 Cognitive OS running normally.");
    } else if input.contains("motivate") {
        println!("🤖 You're building your own intelligence system.");
    } else {
        println!("Unknown command.");
        println!("Type: hub help");
    }
}

/* ================================
   REFLECTION ENGINE (PHASE 7)
   ================================ */

pub fn reflect(content: &str) {
    let mut total = 0;
    let mut urgent = 0;
    let mut ideas = 0;
    let mut study = 0;

    for line in content.lines() {
        total += 1;
        if line.contains("score:3") { urgent += 1; }
        if line.contains("#idea") { ideas += 1; }
        if line.contains("#study") { study += 1; }
    }

    println!("\n🤖 REFLECTION ENGINE");
    println!("Total memories: {}", total);
    println!("Urgent tasks: {}", urgent);
    println!("Ideas: {}", ideas);
    println!("Study tasks: {}", study);

    if urgent > 0 {
        println!("⚠ Focus required: you have urgent tasks pending.");
    }

    if study > ideas {
        println!("📚 You are in learning mode.");
    } else if ideas > study {
        println!("💡 You are in creation mode.");
    } else {
        println!("⚖ Balanced cognitive state.");
    }

    println!();
}

/* ================================
   PROACTIVE FEEDBACK (PHASE 8)
   ================================ */

pub fn proactive_feedback(task: &str, score: u8) {
    println!("🧠 Memory integrated into cognition layer.");

    if score == 3 {
        println!("⚠ High-priority pattern detected.");
    }
    if task.contains("#study") {
        println!("📚 Learning trajectory detected.");
    }
    if task.contains("#idea") {
        println!("💡 Creative workflow detected.");
    }

    println!();
}

/* ================================
   COGNITIVE STATE REPORT (PHASE 8)
   ================================ */

pub fn cognitive_state_report(content: &str) {
    let mut total = 0;
    let mut urgent = 0;
    let mut study = 0;
    let mut ideas = 0;

    for line in content.lines() {
        total += 1;
        if line.contains("score:3") { urgent += 1; }
        if line.contains("#study") { study += 1; }
        if line.contains("#idea") { ideas += 1; }
    }

    println!("\n🧠 COGNITIVE STATE REPORT");
    println!("Total memory items: {}", total);
    println!("Urgent tasks: {}", urgent);
    println!("Study items: {}", study);
    println!("Idea items: {}", ideas);

    let urgency_ratio = if total > 0 { urgent as f32 / total as f32 } else { 0.0 };

    println!("\n🧠 Cognitive Pulse:");

    if urgency_ratio > 0.5 {
        println!("🔥 HIGH PRESSURE STATE");
    } else if study > ideas {
        println!("📚 LEARNING MODE ACTIVE");
    } else if ideas > study {
        println!("💡 CREATIVE MODE ACTIVE");
    } else {
        println!("⚖ BALANCED STATE");
    }

    println!();
}

/* ================================
   PHASE 9 — PREDICTION ENGINE
   ================================ */

pub fn predict_next_action(content: &str) {
    let mut urgent = 0;
    let mut study = 0;
    let mut ideas = 0;

    for line in content.lines() {
        if line.contains("score:3") { urgent += 1; }
        if line.contains("#study") { study += 1; }
        if line.contains("#idea") { ideas += 1; }
    }

    println!("🧠 PREDICTION ENGINE");

    if urgent >= 3 {
        println!("⚠ You are overloaded. Consider clearing tasks.");
    }
    if study >= 3 {
        println!("📚 You are in heavy learning mode. Stay consistent.");
    }
    if ideas >= 3 {
        println!("💡 Many ideas detected. Consider execution mode.");
    }
    if urgent == 0 && study == 0 && ideas == 0 {
println!("🟢 System is idle. No strong patterns detected.");
    }

    println!();
}

/* ================================
   PHASE 10 — ATTENTION LAYER
   ================================ */

pub fn attention_layer(content: &str) {
    let mut urgent = 0;
    let mut total = 0;

    for line in content.lines() {
        total += 1;
        if line.contains("score:3") { urgent += 1; }
    }

    println!("\n🧠 ATTENTION LAYER ACTIVE");

    if total == 0 {
        println!("🟢 No memory stored yet.");
        return;
    }

    let ratio = urgent as f32 / total as f32;

    if ratio > 0.5 {
        println!("🔥 CRITICAL: High urgency load detected");
    } else if ratio > 0.2 {
        println!("⚠ Moderate urgency detected");
    } else {
        println!("🟢 System stable");
    }

    println!();
}

/* ================================
   PHASE 11 — AUTONOMOUS SUGGESTION
   ================================ */

pub fn auto_suggest(content: &str) {
    let mut urgent = 0;
    let mut ideas = 0;
    let mut study = 0;

    for line in content.lines() {
        if line.contains("score:3") { urgent += 1; }
        if line.contains("#idea") { ideas += 1; }
        if line.contains("#study") { study += 1; }
    }

    println!("\n🧠 AUTONOMOUS SUGGESTION LAYER");

    if urgent >= 3 {
        println!("⚠ Run: hub view (high urgency load)");
    }
    if study > ideas {
        println!("📚 Suggestion: continue learning");
    }
    if ideas > study {
        println!("💡 Suggestion: execute ideas");
    }
    if urgent == 0 && ideas == 0 && study == 0 {
        println!("🟢 System idle — add memory");
    }

    println!();
}

/* ================================
   PHASE 12 — MEMORY COMPRESSION
   ================================ */

pub fn memory_compression(content: &str) {
    let mut urgent = 0;
    let mut ideas = 0;
    let mut study = 0;
    let mut total = 0;

    for line in content.lines() {
        total += 1;
        if line.contains("score:3") { urgent += 1; }
        if line.contains("#idea") { ideas += 1; }
        if line.contains("#study") { study += 1; }
    }

    println!("\n🧠 MEMORY COMPRESSION ENGINE");

    if total == 0 {
        println!("No memory to compress.");
        return;
    }

    println!("• Total: {}", total);
    println!("• Urgent: {}", urgent);
    println!("• Ideas: {}", ideas);
    println!("• Study: {}", study);

    if urgent > ideas && urgent > study {
        println!("→ URGENCY DOMINANT");
    } else if ideas > study {
        println!("→ CREATIVE DOMINANT");
    } else {
        println!("→ LEARNING DOMINANT");
    }

    println!();
}

/* ================================
   PHASE 12 — SELF WRITING LOGS
   ================================ */

pub fn self_writing_logs(content: &str) {
    let urgent = content.matches("score:3").count();
    let ideas = content.matches("#idea").count();
    let study = content.matches("#study").count();

    println!("\n🧠 SYSTEM LOG");

    if urgent > 0 {
        println!("• Urgency detected");
    }
    if ideas > study {
        println!("• Creative bias");
    }
    if study > ideas {
        println!("• Learning bias");
    }
    if urgent > 3 {
        println!("• WARNING: High pressure");
    }

    println!();
}

/* ================================
   PHASE 12 — HABIT DETECTION
   ================================ */

pub fn habit_detection(content: &str) {
    let study = content.matches("#study").count();
    let ideas = content.matches("#idea").count();
    let urgent = content.matches("score:3").count();

    println!("\n🧠 HABIT DETECTION");

    if study >= 5 {
        println!("📚 Habit: CONSISTENT LEARNER");
    }
    if ideas >= 5 {
        println!("💡 Habit: IDEA GENERATOR");
    }
    if urgent >= 5 {
        println!("⚠ Habit: HIGH STRESS PATTERN");
    }
    if study < 2 && ideas < 2 && urgent < 2 {
        println!("🟢 No strong habits");
    }

    println!();
}

/* ================================
   PHASE 12 — DAILY REPORT
   ================================ */

pub fn daily_brain_report(content: &str) {
let total = content.lines().count();
    let urgent = content.matches("score:3").count();
    let ideas = content.matches("#idea").count();
    let study = content.matches("#study").count();

    println!("\n📅 DAILY REPORT");

    if urgent > 5 {
        println!("🔥 High urgency day");
    } else if ideas > study {
        println!("💡 Idea-heavy day");
    } else if study > ideas {
        println!("📚 Learning-heavy day");
    } else {
        println!("⚖ Balanced day");
    }

    println!("Total: {}", total);
    println!();
}

/* ================================
   PHASE 12 — AUTONOMOUS GOALS
   ================================ */

pub fn autonomous_goal_generation(content: &str) {
    let urgent = content.matches("score:3").count();
    let ideas = content.matches("#idea").count();
    let study = content.matches("#study").count();

    println!("\n🎯 AUTONOMOUS GOALS");

    if urgent >= 3 {
        println!("• Clear urgent tasks");
    }
    if ideas >= 3 {
        println!("• Execute ideas");
    }
    if study >= 3 {
        println!("• 30min study session");
    }
    if urgent == 0 && ideas == 0 && study == 0 {
        println!("• Add memory");
    }

    println!();
}

/* ================================
   PHASE 14 — MEMORY EVOLUTION ENGINE
   ================================ */

pub fn memory_evolution(content: &str) {
    let mut urgent = 0;
    let mut idea = 0;
    let mut study = 0;
    let mut noise = 0;

    for line in content.lines() {
        if line.contains("score:3") {
            urgent += 1;
        } else if line.contains("#idea") {
            idea += 1;
        } else if line.contains("#study") {
            study += 1;
        } else {
            noise += 1;
        }
    }

    println!("\n🧠 MEMORY EVOLUTION ENGINE");

    let total = urgent + idea + study + noise;

    if total == 0 {
        println!("No memory data.");
        return;
    }

    let u = urgent as f32 / total as f32;
    let i = idea as f32 / total as f32;
    let s = study as f32 / total as f32;
    let n = noise as f32 / total as f32;

    println!("• Urgency: {:.2}", u);
    println!("• Creativity: {:.2}", i);
    println!("• Learning: {:.2}", s);
    println!("• Noise: {:.2}", n);

    if n > 0.6 {
        println!("⚠ SYSTEM DRIFT");
    }
    if u > 0.5 {
        println!("🔥 HIGH PRESSURE");
    }
    if i > s {
        println!("💡 CREATIVE DOMINANCE");
    } else {
        println!("📚 LEARNING DOMINANCE");
    }

    println!();
}