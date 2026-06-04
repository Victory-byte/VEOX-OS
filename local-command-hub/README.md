<p align="center">
<pre>

██╗   ██╗███████╗ ██████╗ ██╗  ██╗      ██████╗ ███████╗
██║   ██║██╔════╝██╔═══██╗╚██╗██╔╝     ██╔═══██╗██╔════╝
██║   ██║█████╗  ██║   ██║ ╚███╔╝      ██║   ██║███████╗
╚██╗ ██╔╝██╔══╝  ██║   ██║ ██╔██╗      ██║   ██║╚════██║
 ╚████╔╝ ███████╗╚██████╔╝██╔╝ ██╗     ╚██████╔╝███████║
  ╚═══╝  ╚══════╝ ╚═════╝ ╚═╝  ╚═╝      ╚═════╝ ╚══════╝


Autonomous cognition in Rust.


[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-active-brightgreen?style=flat-square)]()

</pre >

---


## What is VEOX-OS?

Most "AI" tools are just wrappers. VEOX-OS is something different.

It's a terminal-based cognitive system written entirely in Rust built to simulate how an intelligent system might actually think:storing memory,reflecting on past behavior,predicting future actions and evolving its own internal state over time.

No LLM calls. No API keys.No cloud dependency.  
Just a cognitive engine,running locally,getting smarter the more you use it.

---

## Demo

<p align="center">
  <img src="veox-demo.gif" width="100%" />
</p>


---

## Architecture

VEOX-OS is built as a layered cognitive pipeline.Each layer has one job.

<pre>

┌─────────────────────────────────────────────────────────┐
│  INPUT LAYER        hub commands → structured intent    │
├─────────────────────────────────────────────────────────┤
│  MEMORY SYSTEM      brain.db · tasks · ideas · history  │
├──────────────────────┬──────────────────────────────────┤
│  COGNITIVE PIPELINE  │                                  │
│                      │  Reflection  → reads the past   │
│                      │  Prediction  → anticipates next │
│                      │  Attention   → surfaces signals  │
│                      │  Suggestion   → proposes actions │
│                      │  Compression  → strips the noise │
│                      │  Evolution    → adapts over time │
├──────────────────────┴──────────────────────────────────┤
│  AUTONOMOUS LOOP    background scan every 10s           │
│                     pattern detection · state mutation  │
├─────────────────────────────────────────────────────────┤
│  OUTPUT LAYER       cognitive reports · state changes   │
└─────────────────────────────────────────────────────────┘

</pre>


The autonomous loop is what makes VEOX-OS feel **alive** it continuously scans memory,detects behavioral patterns and evolves system state without you asking it to.



## Quick Start 

# Clone
git clone https://github.com/Victory-byte/VEOX-OS.git
cd local-command-hub

# Build & run
cargo build
cargo run

Requires Rust stable.Install via:https://rustup.rs (https://rustup.rs/)


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



## Tech Stack

Rust (stable systems programming language)  
CLI-native architecture  
Persistent file-based memory system (brain.db)  
Event-driven processing engine  
Modular cognitive engine design  
Layered autonomous reasoning system (memory, reflection, prediction, evolution)



## Why I Built This

I wanted to understand how intelligent behavior can emerge from layered internal processes without relying on pre-trained models or external APIs.
VEOX-OS is the result:a system that builds a model of your behavior from the ground up, evolves it continuously, and surfaces insights you didn’t explicitly ask for. It’s less a productivity tool and more a study in how cognition itself might be engineered.
The constraint of building it in Rust low-level, no garbage collector, brutal about memory safety forced every design decision to be intentional. That tension is part of the point.



## Roadmap

 • Reflection Engine
 • Prediction Engine
 • Attention Layer
 • Suggestion System
 • Compression Engine
 • Evolution Engine
 • Autonomous background loop
 • Persistent cross-session behavioral modeling
 • Exportable cognitive reports (JSON / Markdown)
 • Plugin interface for custom cognitive modules



Built in Rust. Runs in your terminal. Thinks on its own.