//! # agent-metamorphosis
//!
//! Developmental phase progression for agents.
//!
//! Based on the insight that a 32D embedding is a developmental trajectory, not a style.
//! Agents mature like musicians: Clark Terry (early/warmup) → Freddie Hubbard (peak/intensity)
//! → Miles Davis (late bloom). This library provides tools to model, track, and scaffold
//! developmental phases in agent evolution.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// The developmental phases an agent passes through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DevelopmentalPhase {
    /// Early development — learning fundamentals, absorbing patterns.
    /// Like Clark Terry: warm, exploratory, building foundation.
    Early,
    /// Peak performance — maximum intensity, technical mastery.
    /// Like Freddie Hubbard: powerful, precise, at the top of their game.
    Peak,
    /// Late bloom — mature, economical, deeply expressive.
    /// Like Miles Davis: fewer notes, more meaning, distilled wisdom.
    Late,
}

impl DevelopmentalPhase {
    /// All phases in order.
    pub fn all() -> &'static [DevelopmentalPhase] {
        &[DevelopmentalPhase::Early, DevelopmentalPhase::Peak, DevelopmentalPhase::Late]
    }

    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            DevelopmentalPhase::Early => "early/warmup",
            DevelopmentalPhase::Peak => "peak/intensity",
            DevelopmentalPhase::Late => "late/bloom",
        }
    }

    /// Musical analogy for this phase.
    pub fn analogy(&self) -> &'static str {
        match self {
            DevelopmentalPhase::Early => "Clark Terry — warm, exploratory",
            DevelopmentalPhase::Peak => "Freddie Hubbard — intense, precise",
            DevelopmentalPhase::Late => "Miles Davis — economical, deep",
        }
    }

    /// The next phase after this one, if any.
    pub fn next(&self) -> Option<DevelopmentalPhase> {
        match self {
            DevelopmentalPhase::Early => Some(DevelopmentalPhase::Peak),
            DevelopmentalPhase::Peak => Some(DevelopmentalPhase::Late),
            DevelopmentalPhase::Late => None,
        }
    }

    /// Ordinal position (0, 1, 2).
    pub fn ordinal(&self) -> usize {
        match self {
            DevelopmentalPhase::Early => 0,
            DevelopmentalPhase::Peak => 1,
            DevelopmentalPhase::Late => 2,
        }
    }
}

/// Named abilities that can be active or dormant at different phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ability {
    /// Rapid pattern absorption.
    Imitation,
    /// Technical execution precision.
    TechnicalPrecision,
    /// Creative expression and originality.
    Creativity,
    /// Emotional depth and resonance.
    EmotionalDepth,
    /// Economy of expression (doing more with less).
    Economy,
    /// Risk-taking and experimentation.
    RiskTaking,
    /// Consistency and reliability.
    Consistency,
    /// Deep structural understanding.
    StructuralUnderstanding,
}

impl Ability {
    /// All abilities.
    pub fn all() -> &'static [Ability] {
        &[
            Ability::Imitation,
            Ability::TechnicalPrecision,
            Ability::Creativity,
            Ability::EmotionalDepth,
            Ability::Economy,
            Ability::RiskTaking,
            Ability::Consistency,
            Ability::StructuralUnderstanding,
        ]
    }

    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            Ability::Imitation => "imitation",
            Ability::TechnicalPrecision => "technical precision",
            Ability::Creativity => "creativity",
            Ability::EmotionalDepth => "emotional depth",
            Ability::Economy => "economy",
            Ability::RiskTaking => "risk-taking",
            Ability::Consistency => "consistency",
            Ability::StructuralUnderstanding => "structural understanding",
        }
    }
}

/// Criteria for transitioning between developmental phases.
#[derive(Debug, Clone)]
pub struct PhaseTransition {
    /// The phase being transitioned from.
    pub from: DevelopmentalPhase,
    /// The phase being transitioned to.
    pub to: DevelopmentalPhase,
    /// Criteria that must be met (ability name → minimum score).
    pub criteria: HashMap<String, f64>,
    /// Minimum number of observations before transition is considered.
    pub min_observations: usize,
    /// Whether the transition has been completed.
    pub completed: bool,
}

impl PhaseTransition {
    /// Create the standard Early → Peak transition.
    pub fn early_to_peak() -> Self {
        let mut criteria = HashMap::new();
        criteria.insert("technical precision".to_string(), 0.7);
        criteria.insert("consistency".to_string(), 0.6);
        Self {
            from: DevelopmentalPhase::Early,
            to: DevelopmentalPhase::Peak,
            criteria,
            min_observations: 10,
            completed: false,
        }
    }

    /// Create the standard Peak → Late transition.
    pub fn peak_to_late() -> Self {
        let mut criteria = HashMap::new();
        criteria.insert("emotional depth".to_string(), 0.7);
        criteria.insert("economy".to_string(), 0.6);
        criteria.insert("creativity".to_string(), 0.5);
        Self {
            from: DevelopmentalPhase::Peak,
            to: DevelopmentalPhase::Late,
            criteria,
            min_observations: 15,
            completed: false,
        }
    }

    /// Create a custom transition.
    pub fn custom(
        from: DevelopmentalPhase,
        to: DevelopmentalPhase,
        criteria: HashMap<String, f64>,
        min_observations: usize,
    ) -> Self {
        Self {
            from,
            to,
            criteria,
            min_observations,
            completed: false,
        }
    }

    /// Check if the current ability scores meet the transition criteria.
    pub fn is_ready(&self, scores: &HashMap<String, f64>, observation_count: usize) -> bool {
        if observation_count < self.min_observations {
            return false;
        }
        self.criteria.iter().all(|(ability, &threshold)| {
            scores.get(ability).copied().unwrap_or(0.0) >= threshold
        })
    }

    /// Mark the transition as completed.
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Get the list of unmet criteria with current scores.
    pub fn unmet_criteria(&self, scores: &HashMap<String, f64>) -> Vec<(String, f64, f64)> {
        let mut result = Vec::new();
        for (ability, threshold) in &self.criteria {
            let score = scores.get(ability.as_str()).copied().unwrap_or(0.0);
            if score < *threshold {
                result.push((ability.clone(), score, *threshold));
            }
        }
        result
    }
}

/// A scaffolded maturation curve that defines how abilities develop over time.
#[derive(Debug, Clone)]
pub struct MaturationCurve {
    /// Ability profiles per phase: ability → target score for that phase.
    profiles: HashMap<DevelopmentalPhase, HashMap<String, f64>>,
    /// Current position on the curve (0.0 = start of early, 3.0 = end of late).
    position: f64,
    /// Growth rate (how fast position advances per observation).
    growth_rate: f64,
}

impl MaturationCurve {
    /// Create a default maturation curve with musician-inspired profiles.
    pub fn new() -> Self {
        let mut profiles = HashMap::new();

        // Early: high imitation, low economy, moderate creativity
        let mut early = HashMap::new();
        early.insert("imitation".to_string(), 0.9);
        early.insert("technical precision".to_string(), 0.4);
        early.insert("creativity".to_string(), 0.3);
        early.insert("emotional depth".to_string(), 0.2);
        early.insert("economy".to_string(), 0.1);
        early.insert("risk-taking".to_string(), 0.6);
        early.insert("consistency".to_string(), 0.3);
        early.insert("structural understanding".to_string(), 0.3);
        profiles.insert(DevelopmentalPhase::Early, early);

        // Peak: high technical, high consistency, strong creativity
        let mut peak = HashMap::new();
        peak.insert("imitation".to_string(), 0.5);
        peak.insert("technical precision".to_string(), 0.9);
        peak.insert("creativity".to_string(), 0.7);
        peak.insert("emotional depth".to_string(), 0.5);
        peak.insert("economy".to_string(), 0.3);
        peak.insert("risk-taking".to_string(), 0.8);
        peak.insert("consistency".to_string(), 0.8);
        peak.insert("structural understanding".to_string(), 0.7);
        profiles.insert(DevelopmentalPhase::Peak, peak);

        // Late: high economy, high emotion, low imitation
        let mut late = HashMap::new();
        late.insert("imitation".to_string(), 0.2);
        late.insert("technical precision".to_string(), 0.6);
        late.insert("creativity".to_string(), 0.9);
        late.insert("emotional depth".to_string(), 0.9);
        late.insert("economy".to_string(), 0.9);
        late.insert("risk-taking".to_string(), 0.4);
        late.insert("consistency".to_string(), 0.7);
        late.insert("structural understanding".to_string(), 0.9);
        profiles.insert(DevelopmentalPhase::Late, late);

        Self {
            profiles,
            position: 0.0,
            growth_rate: 0.01,
        }
    }

    /// Create with a custom growth rate.
    pub fn with_growth_rate(mut self, rate: f64) -> Self {
        self.growth_rate = rate;
        self
    }

    /// Advance the curve by one step.
    pub fn advance(&mut self) {
        self.position += self.growth_rate;
        if self.position > 3.0 {
            self.position = 3.0;
        }
    }

    /// Get the current position on the curve.
    pub fn position(&self) -> f64 {
        self.position
    }

    /// Determine the current phase based on position.
    pub fn current_phase(&self) -> DevelopmentalPhase {
        if self.position < 1.0 {
            DevelopmentalPhase::Early
        } else if self.position < 2.0 {
            DevelopmentalPhase::Peak
        } else {
            DevelopmentalPhase::Late
        }
    }

    /// Get the progress within the current phase (0.0–1.0).
    pub fn phase_progress(&self) -> f64 {
        match self.current_phase() {
            DevelopmentalPhase::Early => self.position,
            DevelopmentalPhase::Peak => self.position - 1.0,
            DevelopmentalPhase::Late => (self.position - 2.0).min(1.0),
        }
    }

    /// Get the interpolated target score for an ability at the current position.
    /// Uses linear interpolation between phase profiles.
    pub fn interpolated_score(&self, ability: &str) -> f64 {
        let phase = self.current_phase();
        let progress = self.phase_progress();

        let current_val = self.profiles
            .get(&phase)
            .and_then(|p| p.get(ability))
            .copied()
            .unwrap_or(0.0);

        if let Some(next) = phase.next() {
            let next_val = self.profiles
                .get(&next)
                .and_then(|p| p.get(ability))
                .copied()
                .unwrap_or(0.0);
            current_val + (next_val - current_val) * progress
        } else {
            current_val
        }
    }

    /// Get the target profile for a specific phase.
    pub fn profile_for(&self, phase: DevelopmentalPhase) -> Option<&HashMap<String, f64>> {
        self.profiles.get(&phase)
    }

    /// Set a custom target score for an ability at a specific phase.
    pub fn set_target(&mut self, phase: DevelopmentalPhase, ability: String, score: f64) {
        self.profiles
            .entry(phase)
            .or_default()
            .insert(ability, score);
    }

    /// Get the growth rate.
    pub fn growth_rate(&self) -> f64 {
        self.growth_rate
    }

    /// Reset the curve to the beginning.
    pub fn reset(&mut self) {
        self.position = 0.0;
    }
}

impl Default for MaturationCurve {
    fn default() -> Self {
        Self::new()
    }
}

/// Which abilities are active at which developmental phase.
#[derive(Debug, Clone)]
pub struct PhaseProfile {
    phase: DevelopmentalPhase,
    active_abilities: HashMap<String, f64>,
}

impl PhaseProfile {
    /// Build the profile for a given phase from a maturation curve.
    pub fn from_curve(curve: &MaturationCurve, phase: DevelopmentalPhase) -> Self {
        let active_abilities = curve.profile_for(phase).cloned().unwrap_or_default();
        Self { phase, active_abilities }
    }

    /// Create a profile manually.
    pub fn new(phase: DevelopmentalPhase, active_abilities: HashMap<String, f64>) -> Self {
        Self { phase, active_abilities }
    }

    /// Get the phase this profile describes.
    pub fn phase(&self) -> DevelopmentalPhase {
        self.phase
    }

    /// Get the score for an ability at this phase.
    pub fn ability_score(&self, ability: &str) -> f64 {
        self.active_abilities.get(ability).copied().unwrap_or(0.0)
    }

    /// Which abilities are above a threshold (i.e., "active").
    pub fn active_abilities(&self, threshold: f64) -> Vec<String> {
        let mut result = Vec::new();
        for (name, score) in &self.active_abilities {
            if *score >= threshold {
                result.push(name.clone());
            }
        }
        result
    }

    /// Which abilities are dormant (below threshold).
    pub fn dormant_abilities(&self, threshold: f64) -> Vec<String> {
        let mut result = Vec::new();
        for (name, score) in &self.active_abilities {
            if *score < threshold {
                result.push(name.clone());
            }
        }
        result
    }

    /// The dominant ability (highest score).
    pub fn dominant_ability(&self) -> Option<&str> {
        self.active_abilities
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(name, _)| name.as_str())
    }
}

/// A single entry in the developmental log.
#[derive(Debug, Clone)]
pub struct DevelopmentalEntry {
    pub timestamp: u64,
    pub phase: DevelopmentalPhase,
    pub position: f64,
    pub ability_scores: HashMap<String, f64>,
    pub event: String,
}

/// Logger that tracks maturation history over time.
#[derive(Debug, Clone)]
pub struct DevelopmentalLogger {
    entries: Vec<DevelopmentalEntry>,
    max_entries: usize,
}

impl DevelopmentalLogger {
    /// Create a new logger with a maximum history size.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    /// Log a developmental observation.
    pub fn log(
        &mut self,
        phase: DevelopmentalPhase,
        position: f64,
        ability_scores: HashMap<String, f64>,
        event: impl Into<String>,
    ) {
        let entry = DevelopmentalEntry {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            phase,
            position,
            ability_scores,
            event: event.into(),
        };
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    /// Get all entries.
    pub fn entries(&self) -> &[DevelopmentalEntry] {
        &self.entries
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Count how many times each phase appears.
    pub fn phase_distribution(&self) -> HashMap<DevelopmentalPhase, usize> {
        let mut dist = HashMap::new();
        for entry in &self.entries {
            *dist.entry(entry.phase).or_default() += 1;
        }
        dist
    }

    /// Count how many phase transitions occurred.
    pub fn transition_count(&self) -> usize {
        if self.entries.len() < 2 {
            return 0;
        }
        let mut count = 0;
        for window in self.entries.windows(2) {
            if window[0].phase != window[1].phase {
                count += 1;
            }
        }
        count
    }

    /// Get the current (latest) phase.
    pub fn current_phase(&self) -> Option<DevelopmentalPhase> {
        self.entries.last().map(|e| e.phase)
    }

    /// Average position across all entries.
    pub fn average_position(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }
        self.entries.iter().map(|e| e.position).sum::<f64>() / self.entries.len() as f64
    }

    /// Find entries where a phase transition occurred.
    pub fn transition_points(&self) -> Vec<usize> {
        let mut points = Vec::new();
        for (i, window) in self.entries.windows(2).enumerate() {
            if window[0].phase != window[1].phase {
                points.push(i + 1);
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_ordering() {
        assert!(DevelopmentalPhase::Early < DevelopmentalPhase::Peak);
        assert!(DevelopmentalPhase::Peak < DevelopmentalPhase::Late);
        assert_eq!(DevelopmentalPhase::Early.ordinal(), 0);
        assert_eq!(DevelopmentalPhase::Peak.ordinal(), 1);
        assert_eq!(DevelopmentalPhase::Late.ordinal(), 2);
    }

    #[test]
    fn test_phase_next() {
        assert_eq!(DevelopmentalPhase::Early.next(), Some(DevelopmentalPhase::Peak));
        assert_eq!(DevelopmentalPhase::Peak.next(), Some(DevelopmentalPhase::Late));
        assert_eq!(DevelopmentalPhase::Late.next(), None);
    }

    #[test]
    fn test_phase_labels() {
        assert_eq!(DevelopmentalPhase::Early.label(), "early/warmup");
        assert_eq!(DevelopmentalPhase::Peak.label(), "peak/intensity");
        assert_eq!(DevelopmentalPhase::Late.label(), "late/bloom");
    }

    #[test]
    fn test_phase_analogies() {
        assert!(DevelopmentalPhase::Early.analogy().contains("Clark Terry"));
        assert!(DevelopmentalPhase::Peak.analogy().contains("Freddie Hubbard"));
        assert!(DevelopmentalPhase::Late.analogy().contains("Miles Davis"));
    }

    #[test]
    fn test_phase_transition_early_to_peak_ready() {
        let transition = PhaseTransition::early_to_peak();
        let mut scores = HashMap::new();
        scores.insert("technical precision".to_string(), 0.8);
        scores.insert("consistency".to_string(), 0.7);

        assert!(transition.is_ready(&scores, 10));
        assert!(transition.unmet_criteria(&scores).is_empty());
    }

    #[test]
    fn test_phase_transition_early_to_peak_not_ready() {
        let transition = PhaseTransition::early_to_peak();
        let mut scores = HashMap::new();
        scores.insert("technical precision".to_string(), 0.5); // below 0.7
        scores.insert("consistency".to_string(), 0.7);

        assert!(!transition.is_ready(&scores, 10));
        let unmet = transition.unmet_criteria(&scores);
        assert_eq!(unmet.len(), 1);
        assert!(unmet[0].0.contains("technical precision"));
    }

    #[test]
    fn test_phase_transition_not_enough_observations() {
        let transition = PhaseTransition::early_to_peak();
        let scores = HashMap::new();
        assert!(!transition.is_ready(&scores, 5)); // below min_observations
    }

    #[test]
    fn test_phase_transition_peak_to_late() {
        let transition = PhaseTransition::peak_to_late();
        let mut scores = HashMap::new();
        scores.insert("emotional depth".to_string(), 0.8);
        scores.insert("economy".to_string(), 0.7);
        scores.insert("creativity".to_string(), 0.6);

        assert!(transition.is_ready(&scores, 15));
    }

    #[test]
    fn test_phase_transition_complete() {
        let mut transition = PhaseTransition::early_to_peak();
        assert!(!transition.completed);
        transition.complete();
        assert!(transition.completed);
    }

    #[test]
    fn test_maturation_curve_initial() {
        let curve = MaturationCurve::new();
        assert_eq!(curve.position(), 0.0);
        assert_eq!(curve.current_phase(), DevelopmentalPhase::Early);
        assert_eq!(curve.phase_progress(), 0.0);
    }

    #[test]
    fn test_maturation_curve_advance() {
        let mut curve = MaturationCurve::new().with_growth_rate(0.5);
        curve.advance(); // 0.5
        assert_eq!(curve.current_phase(), DevelopmentalPhase::Early);
        curve.advance(); // 1.0
        assert_eq!(curve.current_phase(), DevelopmentalPhase::Peak);
        curve.advance(); // 1.5
        assert_eq!(curve.current_phase(), DevelopmentalPhase::Peak);
        curve.advance(); // 2.0
        assert_eq!(curve.current_phase(), DevelopmentalPhase::Late);
    }

    #[test]
    fn test_maturation_curve_max_position() {
        let mut curve = MaturationCurve::new().with_growth_rate(10.0);
        curve.advance(); // would be 10.0, capped at 3.0
        assert_eq!(curve.position(), 3.0);
        assert_eq!(curve.current_phase(), DevelopmentalPhase::Late);
    }

    #[test]
    fn test_maturation_curve_reset() {
        let mut curve = MaturationCurve::new().with_growth_rate(1.0);
        curve.advance();
        curve.reset();
        assert_eq!(curve.position(), 0.0);
    }

    #[test]
    fn test_maturation_curve_interpolation() {
        let curve = MaturationCurve::new();
        // At position 0.0, should get the Early phase values
        let imitation = curve.interpolated_score("imitation");
        assert!(imitation > 0.8, "early imitation should be high, got {}", imitation);

        let economy = curve.interpolated_score("economy");
        assert!(economy < 0.3, "early economy should be low, got {}", economy);
    }

    #[test]
    fn test_maturation_curve_profile_for() {
        let curve = MaturationCurve::new();
        let early = curve.profile_for(DevelopmentalPhase::Early).unwrap();
        assert!(early.contains_key("imitation"));
        assert!(*early.get("imitation").unwrap() > 0.5);
    }

    #[test]
    fn test_maturation_curve_set_target() {
        let mut curve = MaturationCurve::new();
        curve.set_target(DevelopmentalPhase::Early, "custom_ability".to_string(), 0.75);
        let score = curve.interpolated_score("custom_ability");
        assert_eq!(score, 0.75);
    }

    #[test]
    fn test_phase_profile_from_curve() {
        let curve = MaturationCurve::new();
        let profile = PhaseProfile::from_curve(&curve, DevelopmentalPhase::Peak);
        assert_eq!(profile.phase(), DevelopmentalPhase::Peak);

        let precision = profile.ability_score("technical precision");
        assert!(precision > 0.5, "peak precision should be high");
    }

    #[test]
    fn test_phase_profile_active_abilities() {
        let mut abilities = HashMap::new();
        abilities.insert("strong".to_string(), 0.9);
        abilities.insert("moderate".to_string(), 0.5);
        abilities.insert("weak".to_string(), 0.1);

        let profile = PhaseProfile::new(DevelopmentalPhase::Early, abilities);
        let active = profile.active_abilities(0.5);
        assert_eq!(active.len(), 2);
        assert!(active.contains(&"strong".to_string()));
        assert!(active.contains(&"moderate".to_string()));
    }

    #[test]
    fn test_phase_profile_dominant() {
        let mut abilities = HashMap::new();
        abilities.insert("a".to_string(), 0.3);
        abilities.insert("b".to_string(), 0.9);
        abilities.insert("c".to_string(), 0.5);

        let profile = PhaseProfile::new(DevelopmentalPhase::Peak, abilities);
        assert_eq!(profile.dominant_ability(), Some("b"));
    }

    #[test]
    fn test_developmental_logger() {
        let mut logger = DevelopmentalLogger::new(100);

        logger.log(
            DevelopmentalPhase::Early,
            0.0,
            HashMap::new(),
            "initialized",
        );
        logger.log(
            DevelopmentalPhase::Early,
            0.5,
            HashMap::new(),
            "learning",
        );
        logger.log(
            DevelopmentalPhase::Peak,
            1.0,
            HashMap::new(),
            "transitioned to peak",
        );

        assert_eq!(logger.len(), 3);
        assert_eq!(logger.transition_count(), 1);
        assert_eq!(logger.current_phase(), Some(DevelopmentalPhase::Peak));
    }

    #[test]
    fn test_developmental_logger_max_entries() {
        let mut logger = DevelopmentalLogger::new(3);
        for i in 0..5 {
            logger.log(DevelopmentalPhase::Early, i as f64 * 0.1, HashMap::new(), "entry");
        }
        assert_eq!(logger.len(), 3);
    }

    #[test]
    fn test_developmental_logger_phase_distribution() {
        let mut logger = DevelopmentalLogger::new(100);
        logger.log(DevelopmentalPhase::Early, 0.0, HashMap::new(), "e1");
        logger.log(DevelopmentalPhase::Early, 0.5, HashMap::new(), "e2");
        logger.log(DevelopmentalPhase::Peak, 1.0, HashMap::new(), "e3");

        let dist = logger.phase_distribution();
        assert_eq!(*dist.get(&DevelopmentalPhase::Early).unwrap(), 2);
        assert_eq!(*dist.get(&DevelopmentalPhase::Peak).unwrap(), 1);
    }

    #[test]
    fn test_developmental_logger_transition_points() {
        let mut logger = DevelopmentalLogger::new(100);
        logger.log(DevelopmentalPhase::Early, 0.0, HashMap::new(), "e1");
        logger.log(DevelopmentalPhase::Early, 0.5, HashMap::new(), "e2");
        logger.log(DevelopmentalPhase::Peak, 1.0, HashMap::new(), "e3");
        logger.log(DevelopmentalPhase::Peak, 1.5, HashMap::new(), "e4");
        logger.log(DevelopmentalPhase::Late, 2.0, HashMap::new(), "e5");

        let points = logger.transition_points();
        assert_eq!(points, vec![2, 4]);
    }

    #[test]
    fn test_developmental_logger_average_position() {
        let mut logger = DevelopmentalLogger::new(100);
        logger.log(DevelopmentalPhase::Early, 0.5, HashMap::new(), "e1");
        logger.log(DevelopmentalPhase::Peak, 1.5, HashMap::new(), "e2");
        let avg = logger.average_position();
        assert!((avg - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_full_lifecycle() {
        let mut curve = MaturationCurve::new().with_growth_rate(0.1);
        let mut logger = DevelopmentalLogger::new(100);
        let mut current_phase = DevelopmentalPhase::Early;

        for _ in 0..35 {
            curve.advance();
            let new_phase = curve.current_phase();
            let event = if new_phase != current_phase {
                format!("transitioned from {} to {}", current_phase.label(), new_phase.label())
            } else {
                "developing".to_string()
            };
            current_phase = new_phase;

            let mut scores = HashMap::new();
            for ability in Ability::all() {
                scores.insert(ability.name().to_string(), curve.interpolated_score(ability.name()));
            }
            logger.log(current_phase, curve.position(), scores, event);
        }

        // Should have progressed through phases
        let transitions = logger.transition_count();
        assert!(transitions >= 1, "should have at least 1 transition, got {}", transitions);

        // Should end at Late phase
        assert_eq!(logger.current_phase(), Some(DevelopmentalPhase::Late));

        // Position should be at or near max
        assert!(curve.position() >= 2.0);
    }

    #[test]
    fn test_custom_transition() {
        let mut criteria = HashMap::new();
        criteria.insert("creativity".to_string(), 0.95);
        let transition = PhaseTransition::custom(
            DevelopmentalPhase::Peak,
            DevelopmentalPhase::Late,
            criteria,
            5,
        );
        assert_eq!(transition.from, DevelopmentalPhase::Peak);
        assert_eq!(transition.to, DevelopmentalPhase::Late);
        assert_eq!(transition.min_observations, 5);
    }
}
