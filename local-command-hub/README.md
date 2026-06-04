<div align="center">
██╗   ██╗███████╗ ██████╗ ██╗  ██╗      ██████╗ ███████╗
██║   ██║██╔════╝██╔═══██╗╚██╗██╔╝     ██╔═══██╗██╔════╝
██║   ██║█████╗  ██║   ██║ ╚███╔╝      ██║   ██║███████╗
╚██╗ ██╔╝██╔══╝  ██║   ██║ ██╔██╗      ██║   ██║╚════██║
 ╚████╔╝ ███████╗╚██████╔╝██╔╝ ██╗     ╚██████╔╝███████║
  ╚═══╝  ╚══════╝ ╚═════╝ ╚═╝  ╚═╝      ╚═════╝ ╚══════╝

Autonomous cognition in Rust.  
Memory. Reflection. Prediction. Running in your terminal.

[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-brightgreen?style=flat-square)]()

</div>

---

## What is VEOX-OS?

Most "AI" tools are just wrappers. VEOX-OS is something different.

It's a terminal-based cognitive system written entirely in Rust — built to simulate how an intelligent system might actually think: storing memory, reflecting on past behavior, predicting future actions, and evolving its own internal state over time.

No LLM calls. No API keys. No cloud dependency.  
Just a cognitive engine, running locally, getting smarter the more you use it.

---

## Demo

<p align="center">
  <img src="veox-demo.gif" width="100%" />
</p>
COGNITIVE STATE REPORT
─────────────────────────────────
  Total memory items : 12
  Urgent tasks       : 3
  Ideas in queue     : 5
  Study nodes        : 4
─────────────────────────────────
  Cognitive Pulse    : CREATIVE MODE ACTIVE
  Last evolved       : 2 cycles ago
─────────────────────────────────
  → Suggestion: You've been adding ideas but closing no tasks. Refocus?

---

## Architecture

VEOX-OS is built as a layered cognitive pipeline. Each layer has one job.
┌─────────────────────────────────────────────────────────┐
│  INPUT LAYER        hub commands → structured intent    │
├─────────────────────────────────────────────────────────┤
│  MEMORY SYSTEM      brain.db · tasks · ideas · history  │
├──────────────────────┬──────────────────────────────────┤
│  COGNITIVE PIPELINE  │                                  │
│                      │  Reflection  → reads the past   │
│                      │  Prediction  → anticipates next │
│                      │  Attention   → surfaces signals  │
│                      │  Suggestion  → proposes actions  │
│                      │  Compression → strips the noise  │
│                      │  Evolution   → adapts over time  │
├──────────────────────┴──────────────────────────────────┤
│  AUTONOMOUS LOOP    background scan every 10s           │
│                     pattern detection · state mutation  │
├─────────────────────────────────────────────────────────┤
│  OUTPUT LAYER       cognitive reports · state changes   │
└─────────────────────────────────────────────────────────┘

The autonomous loop is what makes VEOX-OS feel alive — it continuously scans memory, detects behavioral patterns, and evolves system state without you asking it to.

---

## Quick Start
# Clone
git clone https://github.com/Victory-byte/VEOX-OS.git
cd VEOX-OS

# Build & run
cargo build
cargo run

> Requires Rust stable. Install via [rustup.rs](https://rustup.rs).

---

## Usage

Add to memory
hub add fix login bug        #urgent
hub add build new feature    #idea
hub add study rust ownership #study

Query memory
hub view           # full memory dump
hub search rust    # keyword search

Run cognitive analysis
hub summary        # high-level state overview
hub reflect        # deep reflection on past behavior
hub state          # current cognitive pulse
hub compress       # prune noise, consolidate memory

Exit
hub exit

---

## Tech Stack

| Layer | Tech |
|---|---|
| Language | Rust (stable) |
| Interface | CLI — zero UI overhead |
| Storage | File-based · brain.db |
| Processing | Event-driven loops |
| Design | Modular cognitive engine |

---

## Why I Built This