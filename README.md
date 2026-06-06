# agent-metamorphosis

> Developmental phase progression for agents — because a 32D embedding is a trajectory, not a style.

## The Insight

An agent's capabilities aren't a fixed style — they're a developmental trajectory. Like musicians, agents mature through distinct phases, each with its own character and strengths.

The analogy: **Clark Terry** (early/warmup — exploratory, imitative) → **Freddie Hubbard** (peak/intensity — powerful, precise) → **Miles Davis** (late/bloom — economical, deeply expressive).

This library models, tracks, and scaffolds these developmental phases.

## Core Concepts

### DevelopmentalPhase

Three distinct phases of agent maturation:

```rust
use agent_metamorphosis::DevelopmentalPhase;

let phase = DevelopmentalPhase::Early;
println!("{}", phase.label());     // "early/warmup"
println!("{}", phase.analogy());   // "Clark Terry — warm, exploratory"
println!("{:?}", phase.next());    // Some(Peak)
```

### PhaseTransition

Criteria for moving between phases. Each transition has specific ability thresholds that must be met:

```rust
use agent_metamorphosis::PhaseTransition;
use std::collections::HashMap;

// Built-in Early → Peak transition
let transition = PhaseTransition::early_to_peak();
// Requires: technical precision >= 0.7, consistency >= 0.6, 10+ observations

let mut scores = HashMap::new();
scores.insert("technical precision".to_string(), 0.8);
scores.insert("consistency".to_string(), 0.7);

if transition.is_ready(&scores, 12) {
    // Ready to transition!
}

// Check what's still needed
let unmet = transition.unmet_criteria(&scores);
```

### MaturationCurve

A scaffolded trajectory that defines how abilities develop over time. Comes with musician-inspired default profiles:

```rust
use agent_metamorphosis::MaturationCurve;

let mut curve = MaturationCurve::new()
    .with_growth_rate(0.05);

// Advance along the curve
for _ in 0..20 {
    curve.advance();
    println!("Phase: {:?}, Position: {:.2}", curve.current_phase(), curve.position());
}

// Get interpolated ability scores at current position
let creativity = curve.interpolated_score("creativity");
let economy = curve.interpolated_score("economy");
```

#### Default Phase Profiles

| Ability | Early (Clark Terry) | Peak (Freddie Hubbard) | Late (Miles Davis) |
|---------|--------------------|-----------------------|--------------------|
| Imitation | 0.9 | 0.5 | 0.2 |
| Technical Precision | 0.4 | 0.9 | 0.6 |
| Creativity | 0.3 | 0.7 | 0.9 |
| Emotional Depth | 0.2 | 0.5 | 0.9 |
| Economy | 0.1 | 0.3 | 0.9 |
| Risk-Taking | 0.6 | 0.8 | 0.4 |
| Consistency | 0.3 | 0.8 | 0.7 |
| Structural Understanding | 0.3 | 0.7 | 0.9 |

### PhaseProfile

A snapshot of which abilities are active/dormant at a specific phase:

```rust
use agent_metamorphosis::{PhaseProfile, MaturationCurve, DevelopmentalPhase};

let curve = MaturationCurve::new();
let profile = PhaseProfile::from_curve(&curve, DevelopmentalPhase::Peak);

println!("Active: {:?}", profile.active_abilities(0.5));
println!("Dormant: {:?}", profile.dormant_abilities(0.5));
println!("Dominant: {:?}", profile.dominant_ability());
```

### DevelopmentalLogger

Track maturation history over time — phase transitions, position, ability scores:

```rust
use agent_metamorphosis::DevelopmentalLogger;
use std::collections::HashMap;

let mut logger = DevelopmentalLogger::new(1000);

logger.log(DevelopmentalPhase::Early, 0.0, HashMap::new(), "initialized");
logger.log(DevelopmentalPhase::Early, 0.5, scores.clone(), "learning");
logger.log(DevelopmentalPhase::Peak, 1.0, scores.clone(), "transitioned!");

println!("Transitions: {}", logger.transition_count());
println!("Current phase: {:?}", logger.current_phase());
println!("Average position: {:.2}", logger.average_position());

let points = logger.transition_points(); // indices where phase changed
let distribution = logger.phase_distribution(); // count per phase
```

## Architecture

```
MaturationCurve (developmental trajectory)
├── Phase profiles (ability targets per phase)
├── Position tracking (0.0 → 3.0)
└── Interpolation engine

PhaseTransition (gate between phases)
├── Criteria (ability → threshold)
├── Minimum observations
└── Readiness check

PhaseProfile (snapshot of a phase)
├── Ability scores
├── Active/dormant classification
└── Dominant ability detection

DevelopmentalLogger (historical record)
├── Entries (phase, position, scores, event)
├── Transition detection
└── Phase distribution analysis
```

## The Musician Analogy

| Phase | Musician | Character | Agent Equivalent |
|-------|----------|-----------|------------------|
| Early | Clark Terry | Warm, exploratory, imitative | Learning patterns, absorbing training data |
| Peak | Freddie Hubbard | Intense, precise, powerful | Maximum capability, technical mastery |
| Late | Miles Davis | Economical, expressive, deep | Doing more with less, distilled wisdom |

The key insight: **late isn't lesser — it's different.** Miles Davis played fewer notes than Freddie Hubbard, not because he couldn't play more, but because he didn't need to. The economy of expression IS the advancement.

## Key Methods

| Component | Method | Purpose |
|-----------|--------|---------|
| DevelopmentalPhase | `next()` | What phase comes after this? |
| PhaseTransition | `is_ready(scores, obs)` | Are criteria met for transition? |
| PhaseTransition | `unmet_criteria(scores)` | What's still needed? |
| MaturationCurve | `advance()` | Progress along the curve |
| MaturationCurve | `current_phase()` | Which phase are we in? |
| MaturationCurve | `interpolated_score(ability)` | Smooth score at current position |
| PhaseProfile | `active_abilities(threshold)` | Which abilities are strong? |
| PhaseProfile | `dominant_ability()` | What's the strongest trait? |
| DevelopmentalLogger | `transition_count()` | How many phase changes occurred? |
| DevelopmentalLogger | `transition_points()` | Where in history did transitions happen? |

## License

MIT
