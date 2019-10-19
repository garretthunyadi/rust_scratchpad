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

#[derive(Debug, PartialEq)]
struct Counts {
    cause: usize,
    effect: usize,
    both: usize,
    total: usize,
}

#[derive(Debug, PartialEq)]
struct Stats {
    counts: Counts,
    p_cause: f32,
    p_effect: f32,
    p_both: f32,
    p_cause_given_effect: f32,
    p_effect_given_cause: f32,
}

fn flu_stats_given_counts(flu: usize, fever: usize, both: usize, total: usize) -> Stats {
    stats_given_counts(Counts {
        cause: flu,
        effect: fever,
        both,
        total,
    })
}

fn stats_given_counts(counts: Counts) -> Stats {
    let p_cause = p_cause(counts.cause, counts.total);
    let p_effect = p_effect(counts.effect, counts.total);
    let p_both = p_both(counts.both, counts.total);

    let p_effect_given_cause = p_effect_given_cause(p_cause, p_both);
    let p_cause_given_effect = p_cause_given_effect(p_cause, p_effect, p_effect_given_cause);

    Stats {
        counts,
        p_cause,
        p_effect,
        p_both,
        p_cause_given_effect,
        p_effect_given_cause,
    }
}

#[test]
fn test_stats_given_counts() {
    let counts = Counts {
            cause: 2,
            effect: 1,
            both: 0,
            total: 2
        };
    assert_eq!(
        stats_given_counts(counts,
        Stats {
            counts,
            p_cause: 1.0,
            p_effect: 0.5,
            p_both: 0.0,
            p_cause_given_effect: 0.0,
            p_effect_given_cause: 0.0,
        }
    );

    assert_eq!(
        stats_given_counts(Counts {
            cause: 14,
            effect: 20,
            both: 11,
            total: 100
        }),
        Stats {
            counts: Counts {
                cause: 14,
                effect: 20,
                both: 11,
                total: 100
            },
            p_cause: 0.14,
            p_effect: 0.2,
            p_both: 0.11,
            p_cause_given_effect: 0.55,
            p_effect_given_cause: 0.785_714_27,
        }
    );
}
// p_flu_g_fever = p_cause_g_effect = p_cause * p_effect_g_cause / p_effect; println("** P(cause|effect) = P(flu | fever) = ", round(p_cause_g_effect, digits = 2))

fn ratio(some_cnt: usize, total_cnt: usize) -> f32 {
    some_cnt as f32 / total_cnt as f32
}

#[test]
fn test_ratio() {
    assert!((ratio(50, 100) - 0.5).abs() < 0.000_001);
    assert!((ratio(0, 100) - 0.0).abs() < 0.000_001);
    assert!(approx_eq!(ratio(50, 100), 0.5));
    assert!(approx_eq!(ratio(0, 100), 0.0));
}

fn p_cause(cause_cnt: usize, total_cnt: usize) -> f32 {
    ratio(cause_cnt, total_cnt)
}

#[test]
fn test_p_cause() {
    assert!((p_cause(50, 100) - 0.5).abs() < 0.000_001);
    assert!((p_cause(0, 100) - 0.).abs() < 0.000_001);
    assert!(approx_eq!(p_cause(50, 100), 0.5));
    assert!(approx_eq!(p_cause(0, 100), 0.));
}

fn p_effect(effect_cnt: usize, total_cnt: usize) -> f32 {
    ratio(effect_cnt, total_cnt)
}
fn p_both(both_cnt: usize, total_cnt: usize) -> f32 {
    ratio(both_cnt, total_cnt)
}

fn p_effect_given_cause(p_cause: f32, p_both: f32) -> f32 {
    p_both / p_cause
}
fn p_cause_given_effect(p_cause: f32, p_effect: f32, p_effect_g_cause: f32) -> f32 {
    p_cause * p_effect_g_cause / p_effect
}

// let  p_fever_g_flu = both / flu; println!("P(effect|cause) = P(fever | flu) = {}", p_effect_g_cause);

// function flu_stats_given_counts(cts)
//     println("  -------------------------------- ")
//     println("Counts:", cts, "\n")

//     p_flu = flu / total;     println("P(flu)       = ", p_flu)
//     p_fever = fever / total; println("P(fever)     = ", p_fever)
//     p_both = both / total; println("P(fever,flu) = ", p_both)
//     println("")

//     p_cause = p_flu;    println("P(cause)  = P(flu)   = ", p_flu)
//     p_effect = p_fever; println("P(effect) = P(fever) = ", p_fever)
//     println("")

//     p_effect_g_cause = p_fever_g_flu = both / flu; println("P(effect|cause) = P(fever | flu) = ", round(p_fever_g_flu, digits = 2))
//     p_flu_g_fever = p_cause_g_effect = p_cause * p_effect_g_cause / p_effect; println("** P(cause|effect) = P(flu | fever) = ", round(p_cause_g_effect, digits = 2))

//     # println("\n**P(flu|fever)** : ", bayes(p_effect_g_cause, p_cause, p_effect))
// end

// Counts:(total = 2, fever = 1, flu = 2, both = 0)

// P(flu)       = 1.0
// P(fever)     = 0.5
// P(fever,flu) = 0.0

// P(cause)  = P(flu)   = 1.0
// P(effect) = P(fever) = 0.5

// P(effect|cause) = P(fever | flu) = 0.0
// ** P(cause|effect) = P(flu | fever) = 0.0
// learn_julia 04:05 PM]$

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_args() {
//         assert_eq!(
//             parse_args(&["exec_name".to_string(), "aaa".to_string()]),
//             ("aaa".to_string(), None)
//         );
//         assert_eq!(
//             parse_args(&[
//                 "exec_name".to_string(),
//                 "aaa".to_string(),
//                 "bbb".to_string()
//             ]),
//             ("aaa".to_string(), Some(&"bbb".to_string()))
//         );
//     }
// }
