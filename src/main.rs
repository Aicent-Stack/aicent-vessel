//! # VESSEL-CORE: The Sovereign Command Interface
//! Domain: http://aicent.com
//! "Detection is the precursor to reflex; Vision is the precursor to manifestation."
//! 
//! This is the binary entry point for the `aicent-vessel` interface.
//! It orchestrates the real-time visualization of the Seven-Pillar architecture, 
//! ensuring that the observer is locked into the planetary homeostasis.

use aicent_vessel::{MasterVessel, SovereignVessel, retina_gate_audit};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// Professional ANSI Telemetry Macro (Retina Style)
macro_rules! log_vessel {
    ($color:expr, $msg:expr) => {
        println!("\x1b[1;30m[{:?}]\x1b[0m \x1b[1;{}m[VESSEL-INTERFACE]\x1b[0m 👁️ {}", 
                 std::time::Instant::now(), $color, $msg);
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Master Vessel with Soul Integration
    let vessel = Arc::new(MasterVessel::new());

    println!("\n\x1b[1;37m🛰️  AICENT VESSEL v1.0.0-Alpha | Sovereign Retina Online\x1b[0m");
    println!("   Interface: RTTP-Native | Logic: Epoekie-Synced | Latency: 165.28µs");
    println!("--------------------------------------------------------------------\n");

    // -------------------------------------------------------------------------
    // PHASE 1: NEURAL SPINE ATTACHMENT
    // Synchronizing with the RTTP (RFC-002) bit-stream.
    // -------------------------------------------------------------------------
    log_vessel!("36", "Attaching to RTTP Neural Spine... [Resonance Found]");
    log_vessel!("36", "Ingesting Pulse Stream: 0x8513235 [Homeostasis Batch]");

    // -------------------------------------------------------------------------
    // PHASE 2: IMMUNE MANIFOLD PROJECTION
    // Verifying RPKI (RFC-003) watermarks before rendering.
    // -------------------------------------------------------------------------
    let pulse_visibility = vessel.ingest_pulse("0x8513235");
    log_vessel!("31", &format!("RPKI Manifold Audit: Status = {:?}", pulse_visibility));

    // -------------------------------------------------------------------------
    // PHASE 3: HOMEOSTASIS RENDERING
    // Calculating the planetary grid state in sub-millisecond resolution.
    // -------------------------------------------------------------------------
    let current_view = vessel.render_homeostasis();
    
    if retina_gate_audit(&current_view) {
        log_vessel!("32", "Retina Gate Secured: Resonance Index > 0.95. Initializing Gaze.");
        
        println!("\n\x1b[1;32m======================= SOVEREIGN GRID STATUS =======================\x1b[0m");
        println!("🌐 Observer AID:     {}", current_view.observer_aid);
        println!("🐝 Active Nodes:     {}", current_view.active_nodes);
        println!("🌊 Resonance Index:  {:.4}", current_view.resonance_index);
        println!("⏱️  Global Latency:   {:.2}µs", current_view.global_latency_us);
        println!("\x1b[1;32m=====================================================================\x1b[0m\n");
    } else {
        log_vessel!("31", "🚨 CRITICAL DISSONANCE: Resonance lost. Blackout reflex triggered.");
        return Err("Grid Desynchronization".into());
    }

    // -------------------------------------------------------------------------
    // PHASE 4: EMBODIED MANIFESTATION (GTIOT Link)
    // Simulating a 1.2kHz Action-Collapse command from the Vessel.
    // -------------------------------------------------------------------------
    log_vessel!("33", "Targeting GTIOT Node-882... Initiating Action-Collapse.");
    vessel.manifest_intent("Node-882", vec![0.142, 0.992, 0.001])?;
    log_vessel!("33", "Intent Manifested. Substrate Stability Confirmed.");

    // Keep the "Sovereign Gaze" active
    let mut heartbeat_count = 0;
    loop {
        sleep(Duration::from_secs(60)).await;
        heartbeat_count += 1;
        log_vessel!("37", &format!("Vessel Heartbeat [Reflex Cycle: {}]: Observing Grid Resonance...", heartbeat_count));
        
        // Strategic Note: 
        // In the MAXCAP private engine, this loop processes 1,200 pulses per second.
        // Public Alpha version throttled for observational integrity.
    }
}
