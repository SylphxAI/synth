//! Pure Halstead + maintainability arithmetic — mirrors
//! `packages/synth-metrics/src/analyzer.ts` formulas (no AST traversal).
//! FLEET-PRODUCTS-WAVE5 pure residual. NO authority_rust / ts_deleted.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HalsteadMetrics {
    pub n1: f64,
    pub n2: f64,
    pub n1_total: f64,
    pub n2_total: f64,
    pub vocabulary: f64,
    pub length: f64,
    pub calculated_length: f64,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
    pub time: f64,
    pub bugs: f64,
}

/// Compute Halstead metrics from unique/total operator/operand counts.
#[must_use]
pub fn compute_halstead(n1: f64, n2: f64, n1_total: f64, n2_total: f64) -> HalsteadMetrics {
    let vocabulary = n1 + n2;
    let length = n1_total + n2_total;
    let calculated_length = n1 * (n1.max(1.0)).log2() + n2 * (n2.max(1.0)).log2();
    let volume = length * (vocabulary.max(1.0)).log2();
    let difficulty = (n1 / 2.0) * (n2_total / n2.max(1.0));
    let effort = difficulty * volume;
    let time = effort / 18.0;
    let bugs = effort.powf(2.0 / 3.0) / 3000.0;
    HalsteadMetrics {
        n1,
        n2,
        n1_total,
        n2_total,
        vocabulary,
        length,
        calculated_length,
        volume,
        difficulty,
        effort,
        time,
        bugs,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintainabilityLevel {
    /// index >= 85 — low difficulty to maintain
    LowDifficulty,
    /// index >= 65
    Moderate,
    /// index >= 20
    High,
    /// index < 20
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaintainabilityMetrics {
    pub index: f64,
    pub level: MaintainabilityLevel,
    pub maintainable: bool,
}

/// Maintainability Index = 171 - 5.2*ln(V) - 0.23*G - 16.2*ln(LOC), clamped 0..100.
#[must_use]
pub fn calculate_maintainability(volume: f64, cyclomatic: f64, loc: f64) -> MaintainabilityMetrics {
    let v = if volume <= 0.0 { 1.0 } else { volume };
    let loc = if loc <= 0.0 { 1.0 } else { loc };
    let mut index = 171.0 - 5.2 * v.ln() - 0.23 * cyclomatic - 16.2 * loc.ln();
    index = index.clamp(0.0, 100.0);
    let level = if index >= 85.0 {
        MaintainabilityLevel::LowDifficulty
    } else if index >= 65.0 {
        MaintainabilityLevel::Moderate
    } else if index >= 20.0 {
        MaintainabilityLevel::High
    } else {
        MaintainabilityLevel::VeryHigh
    };
    MaintainabilityMetrics {
        index,
        level,
        maintainable: index >= 20.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halstead_basic() {
        let h = compute_halstead(4.0, 5.0, 10.0, 12.0);
        assert!((h.vocabulary - 9.0).abs() < 1e-12);
        assert!((h.length - 22.0).abs() < 1e-12);
        assert!(h.volume > 0.0);
        assert!(h.effort > 0.0);
        assert!((h.time - h.effort / 18.0).abs() < 1e-9);
    }

    #[test]
    fn maintainability_clamps_and_levels() {
        let m = calculate_maintainability(1.0, 1.0, 1.0);
        assert!(m.index >= 85.0);
        assert_eq!(m.level, MaintainabilityLevel::LowDifficulty);
        assert!(m.maintainable);

        let hard = calculate_maintainability(1_000_000.0, 200.0, 10_000.0);
        assert!(hard.index < 20.0 || !hard.maintainable || hard.level == MaintainabilityLevel::VeryHigh || hard.level == MaintainabilityLevel::High);
        assert!((0.0..=100.0).contains(&hard.index));
    }

    #[test]
    fn zero_volume_loc_defaults() {
        let m = calculate_maintainability(0.0, 1.0, 0.0);
        assert!((0.0..=100.0).contains(&m.index));
    }
}
