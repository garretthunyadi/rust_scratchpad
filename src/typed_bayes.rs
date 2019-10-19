use std::cmp::Ordering;

pub fn main() -> std::io::Result<()> {
    println!("bayes");
    println!("{:?}", flu_stats_given_counts(2, 1, 2, 2));
    Ok(())
}

fn approx_eq(a: f32, b: f32) -> bool {
    match a.partial_cmp(&b) {
        None | Some(Ordering::Equal) => (a - b).abs() < 0.000_000_1,
        _ => false,
    }
}

#[allow(clippy::excessive_precision)]
#[test]
fn test_approx_eq() {
    assert!(approx_eq(0.5, 0.5));
    assert!(!approx_eq(0.51, 0.5));
    assert!(approx_eq(0.5, 0.500_000_01));
    assert!(!approx_eq(0.5, 0.500_000_2));
}

macro_rules! approx_eq {
    ($left:expr, $right:expr) => {
        // ($left as f32 - $right as f32).abs() < 0.000_001
        approx_eq($left, $right)
    };
}
#[test]
fn test_approx_eq_macro() {
    assert!(approx_eq!(0.5, 0.5));
    assert!(!approx_eq!(0.51, 0.5));
}

#[derive(Debug, PartialEq, Clone)]
struct CauseCount(usize);
#[derive(Debug, PartialEq, Clone)]
struct EffectCount(usize);
#[derive(Debug, PartialEq, Clone)]
struct BothCount(usize);
#[derive(Debug, PartialEq, Clone)]
struct TotalCount(usize);

#[derive(Debug, PartialEq, Clone)]
struct PCause(f32);
#[derive(Debug, PartialEq, Clone)]
struct PEffect(f32);
#[derive(Debug, PartialEq, Clone)]
struct PBoth(f32);
#[derive(Debug, PartialEq, Clone)]
struct PCauseGEffect(f32);
#[derive(Debug, PartialEq, Clone)]
struct PEffectGCause(f32);

#[derive(Debug, PartialEq)]
struct Stats {
    counts: (CauseCount, EffectCount, BothCount, TotalCount),
    p_cause: PCause,
    p_effect: PEffect,
    p_both: PBoth,
    p_cause_given_effect: PCauseGEffect,
    p_effect_given_cause: PEffectGCause,
}

fn flu_stats_given_counts(flu: usize, fever: usize, both: usize, total: usize) -> Stats {
    stats_given_counts(
        &CauseCount(flu),
        &EffectCount(fever),
        &BothCount(both),
        &TotalCount(total),
    )
}

fn stats_given_counts(
    cause: &CauseCount,
    effect: &EffectCount,
    both: &BothCount,
    total: &TotalCount,
) -> Stats {
    let p_cause = p_cause(&cause, &total);
    let p_effect = p_effect(&effect, &total);
    let p_both = p_both(&both, &total);

    let p_effect_given_cause = p_effect_given_cause(&p_cause, &p_both);
    let p_cause_given_effect = p_cause_given_effect(&p_cause, &p_effect, &p_effect_given_cause);

    Stats {
        counts: (cause.clone(), effect.clone(), both.clone(), total.clone()),
        p_cause,
        p_effect,
        p_both,
        p_cause_given_effect,
        p_effect_given_cause,
    }
}

#[test]
fn test_stats_given_counts() {
    assert_eq!(
        stats_given_counts(
            &CauseCount(2),
            &EffectCount(1),
            &BothCount(0),
            &TotalCount(2)
        ),
        Stats {
            counts: (CauseCount(2), EffectCount(1), BothCount(0), TotalCount(2)),
            p_cause: PCause(1.0),
            p_effect: PEffect(0.5),
            p_both: PBoth(0.0),
            p_cause_given_effect: PCauseGEffect(0.0),
            p_effect_given_cause: PEffectGCause(0.0),
        }
    );

    assert_eq!(
        stats_given_counts(
            &CauseCount(14),
            &EffectCount(20),
            &BothCount(11),
            &TotalCount(100)
        ),
        Stats {
            counts: (
                CauseCount(14),
                EffectCount(20),
                BothCount(11),
                TotalCount(100)
            ),
            p_cause: PCause(0.14),
            p_effect: PEffect(0.20),
            p_both: PBoth(0.11),
            p_cause_given_effect: PCauseGEffect(0.55),
            p_effect_given_cause: PEffectGCause(0.785_714_27),
        }
    );
}

fn ratio(some_cnt: usize, total_cnt: usize) -> f32 {
    some_cnt as f32 / total_cnt as f32
}

#[test]
fn test_ratio() {
    assert!(approx_eq!(ratio(50, 100), 0.5));
    assert!(approx_eq!(ratio(0, 100), 0.0));
}

fn p_cause(cause_cnt: &CauseCount, total_cnt: &TotalCount) -> PCause {
    PCause(ratio(cause_cnt.0, total_cnt.0))
}

#[test]
fn test_p_cause() {
    assert!(approx_eq!(
        p_cause(&CauseCount(50), &TotalCount(100)).0,
        0.5
    ));
    assert!(approx_eq!(p_cause(&CauseCount(0), &TotalCount(100)).0, 0.));
}

fn p_effect(effect_cnt: &EffectCount, total_cnt: &TotalCount) -> PEffect {
    PEffect(ratio(effect_cnt.0, total_cnt.0))
}
fn p_both(both_cnt: &BothCount, total_cnt: &TotalCount) -> PBoth {
    PBoth(ratio(both_cnt.0, total_cnt.0))
}

fn p_effect_given_cause(p_cause: &PCause, p_both: &PBoth) -> PEffectGCause {
    PEffectGCause(p_both.0 / p_cause.0)
}
fn p_cause_given_effect(
    p_cause: &PCause,
    p_effect: &PEffect,
    p_effect_g_cause: &PEffectGCause,
) -> PCauseGEffect {
    PCauseGEffect(p_cause.0 * p_effect_g_cause.0 / p_effect.0)
}
