# agent-metamorphosis

**Developmental phase progression for agents — because a 32D embedding is a trajectory, not a style.**

## Why This Exists

When you look at an agent's embedding space, it's tempting to see it as a snapshot: here's where the agent "is" in capability space. But that's the wrong frame. A 32-dimensional embedding isn't a position — it's a *trajectory*. The agent isn't at a point; it's on a path.

Musicians understand this intuitively. Clark Terry's early career was warm, exploratory, imitative — absorbing everything. Freddie Hubbard's peak was intense, precise, technically overwhelming. Miles Davis's late period was economical, deeply expressive — fewer notes, more meaning. These aren't three different musicians. They're one musician at three phases of development, each with distinct strengths.

The key insight: **late isn't lesser — it's different.** Miles Davis played fewer notes than Freddie Hubbard not because he couldn't play more, but because he didn't need to. The economy of expression IS the advancement.

This library models, tracks, and scaffolds these developmental phases for autonomous agents.

## The Key Insight

Agents mature through three distinct phases, each with its own character:

| Phase | Musician | Character | Agent Equivalent |
|-------|----------|-----------|------------------|
| Early | Clark Terry | Warm, exploratory, imitative | Learning patterns, absorbing training data |
| Peak | Freddie Hubbard | Intense, precise, powerful | Maximum capability, technical mastery |
| Late | Miles Davis | Economical, expressive, deep | Doing more with less, distilled wisdom |

Phase transitions aren't just "getting better." They're qualitative reorganizations of the capability profile. An early-phase agent has high imitation but low economy. A late-phase agent has low imitation but high economy. The shift isn't improvement in one dimension — it's a redistribution across all dimensions.

## Quick Start

```rust
use agent_metamorphosis::*;

// Track an agent's maturation curve
let mut curve = MaturationCurve::new().with_growth_rate(0.05);

for _ in 0..20 {
    curve.advance();
    let phase = curve.current_phase();
    let creativity = curve.interpolated_score("creativity");
    let economy = curve.interpolated_score("economy");
    println!("{:?} — creativity: {:.2}, economy: {:.2}", phase, creativity, economy);
}

// Check transition readiness
let transition = PhaseTransition::early_to_peak();
let mut scores = std::collections::HashMap::new();
scores.insert("technical precision".to_string(), 0.8);
scores.insert("consistency".to_string(), 0.7);

if transition.is_ready(&scores, 12) {
    println!("Ready for Early → Peak transition!");
}
```

## Architecture

```
DevelopmentalPhase (Early / Peak / Late)
├── label() → "early/warmup", "peak/intensity", "late/bloom"
├── analogy() → "Clark Terry", "Freddie Hubbard", "Miles Davis"
├── next() → Option<DevelopmentalPhase>
└── ordinal() → 0, 1, 2

Ability (8 dimensions of capability)
├── Imitation, TechnicalPrecision, Creativity
├── EmotionalDepth, Economy, RiskTaking
├── Consistency, StructuralUnderstanding
└── name() → human-readable

MaturationCurve (scaffolded trajectory)
├── profiles: HashMap<Phase, HashMap<Ability, target_score>>
├── position: 0.0 → 3.0 (Early=0–1, Peak=1–2, Late=2–3)
├── advance() → move forward by growth_rate
├── current_phase() → which phase are we in?
├── interpolated_score(ability) → smooth value at current position
└── set_target(phase, ability, score) → customize

PhaseTransition (gate between phases)
├── criteria: HashMap<String, f64> → ability thresholds
├── min_observations → experience requirement
├── is_ready(scores, obs_count) → can we transition?
├── unmet_criteria(scores) → what's still needed?
└── early_to_peak() / peak_to_late() → built-in transitions

PhaseProfile (snapshot of a phase)
├── active_abilities(threshold) → which are strong?
├── dormant_abilities(threshold) → which are weak?
└── dominant_ability() → strongest trait

DevelopmentalLogger (historical record)
├── log(phase, position, scores, event)
├── transition_count() → how many phase changes?
├── transition_points() → where in history?
├── phase_distribution() → count per phase
└── average_position() → mean developmental progress
```

## Default Phase Profiles

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

Notice the tradeoffs. Imitation drops from 0.9 to 0.2 — the late-phase agent doesn't copy anymore. Economy rises from 0.1 to 0.9 — the late-phase agent does more with less. Technical precision peaks in the middle — necessary for mastery, but not the endgame.

## API Reference

### MaturationCurve

```rust
let mut curve = MaturationCurve::new().with_growth_rate(0.1);

curve.advance();           // position: 0.0 → 0.1
println!("{}", curve.position());           // 0.1
println!("{:?}", curve.current_phase());    // Early
println!("{:.2}", curve.phase_progress());  // 0.1 (10% through Early)

// Interpolated scores: smooth transition between phase profiles
let imitation = curve.interpolated_score("imitation"); // ~0.9 (still early)
let economy = curve.interpolated_score("economy");     // ~0.1 (early stage)
```

| Method | Returns | Purpose |
|--------|---------|---------|
| `new()` | `MaturationCurve` | Default curve with musician profiles |
| `with_growth_rate(r)` | `Self` | Custom growth per advance |
| `advance()` | `()` | Progress one step |
| `current_phase()` | `DevelopmentalPhase` | Early / Peak / Late |
| `position()` | `f64` | 0.0–3.0 |
| `phase_progress()` | `f64` | 0.0–1.0 within current phase |
| `interpolated_score(ability)` | `f64` | Smooth score at current position |
| `set_target(phase, ability, score)` | `()` | Customize a profile |
| `reset()` | `()` | Back to start |

### PhaseTransition

```rust
// Built-in transitions
let t1 = PhaseTransition::early_to_peak();
// Requires: technical precision ≥ 0.7, consistency ≥ 0.6, 10+ observations

let t2 = PhaseTransition::peak_to_late();
// Requires: emotional depth ≥ 0.7, economy ≥ 0.6, creativity ≥ 0.5, 15+ observations

// Custom transition
let custom = PhaseTransition::custom(
    DevelopmentalPhase::Peak,
    DevelopmentalPhase::Late,
    my_criteria,
    20,
);

// Check readiness
let unmet = t1.unmet_criteria(&current_scores);
for (ability, current, required) in &unmet {
    println!("{}: {:.2} / {:.2}", ability, current, required);
}
```

| Method | Returns | Purpose |
|--------|---------|---------|
| `early_to_peak()` | `PhaseTransition` | Standard Early→Peak gate |
| `peak_to_late()` | `PhaseTransition` | Standard Peak→Late gate |
| `custom(from, to, criteria, min_obs)` | `PhaseTransition` | Custom transition |
| `is_ready(scores, obs)` | `bool` | All criteria met? |
| `unmet_criteria(scores)` | `Vec<(String, f64, f64)>` | What's still needed |
| `complete()` | `()` | Mark as done |

### DevelopmentalLogger

```rust
let mut logger = DevelopmentalLogger::new(1000);
logger.log(DevelopmentalPhase::Early, 0.0, scores.clone(), "initialized");
logger.log(DevelopmentalPhase::Early, 0.5, scores.clone(), "learning");
logger.log(DevelopmentalPhase::Peak, 1.0, scores.clone(), "transitioned!");

println!("Transitions: {}", logger.transition_count()); // 1
println!("Current: {:?}", logger.current_phase());       // Peak
println!("Avg position: {:.2}", logger.average_position());
println!("Points: {:?}", logger.transition_points());    // [2]
```

## Real-World Example: Full Lifecycle

```rust
use agent_metamorphosis::*;
use std::collections::HashMap;

let mut curve = MaturationCurve::new().with_growth_rate(0.1);
let mut logger = DevelopmentalLogger::new(200);
let mut phase = DevelopmentalPhase::Early;

for step in 0..35 {
    curve.advance();
    let new_phase = curve.current_phase();

    if new_phase != phase {
        println!("=== {} → {} at position {:.2} ===",
            phase.label(), new_phase.label(), curve.position());
        phase = new_phase;
    }

    let mut scores = HashMap::new();
    for ability in Ability::all() {
        scores.insert(ability.name().to_string(), curve.interpolated_score(ability.name()));
    }
    logger.log(phase, curve.position(), scores, "step");
}

// Final state
println!("Final phase: {:?}", logger.current_phase()); // Late
println!("Transitions: {}", logger.transition_count()); // 2
println!("Distribution: {:?}", logger.phase_distribution());
```

## Performance

- **O(1) per advance** — position increment + phase lookup
- **O(k) per interpolated score** — k=2 phases for interpolation
- **O(n) per transition scan** — n = log entries
- **O(1) per phase check** — HashMap lookup
- **Bounded log** — max_entries prevents unbounded growth

## The Deeper Idea

The interpolation engine uses linear interpolation between phase profiles. At position 0.5 (midway through Early), the score for "imitation" is: `early_imitation + (peak_imitation - early_imitation) × 0.5` = `0.9 + (0.5 − 0.9) × 0.5` = 0.7. This creates smooth transitions rather than abrupt jumps.

The phase transition gates enforce that developmental progression is earned, not just timed. An agent can't move to Peak until it has 10+ observations AND technical precision ≥ 0.7 AND consistency ≥ 0.6. This prevents premature transitions — an agent that hasn't built the foundation can't skip ahead.

The musician analogy is deliberately specific. Clark Terry, Freddie Hubbard, and Miles Davis represent real developmental arcs observed in jazz trumpet players. The profiles capture genuine tradeoffs: early imitation gives way to technical mastery, which gives way to economy and depth. These aren't arbitrary numbers — they're modeled after observed human developmental patterns.

## Open Questions

- **More than three phases**: Is Early/Peak/Late sufficient, or do agents pass through sub-phases?
- **Regression**: Can agents regress to earlier phases? Under what conditions?
- **Asymmetric profiles**: What if an agent has a non-standard profile — high creativity but low technical precision?
- **Phase velocity**: Should growth rate vary by phase? Early might need faster growth than Late.
- **Inter-phase plateaus**: Do agents sometimes stall at phase boundaries? Is this failure or consolidation?

## Ecosystem Connections

- **`agent-phase-change`** — Detects unexpected phase transitions; this crate models expected ones
- **`agent-self-rivalry`** — Self-rivalry drives the developmental progression this crate models
- **`agent-dream-cycle`** — Dream cycles consolidate the experiences that enable phase transitions
- **`agent-orchestration`** — An agent's phase affects its orchestral role and dynamic range

## License

MIT
