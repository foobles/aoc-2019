use crate::intcode::{Error, Machine};
use std::iter;

pub fn score_setting<I>(machine: &Machine, setting: I) -> Result<Option<i32>, Error>
where
    I: IntoIterator<Item = i32>,
{
    setting.into_iter().try_fold(Some(0), |p, s| {
        p.map_or(Ok(None), |input| {
            machine
                .clone()
                .run_to_end([s, input].iter().copied())
                .map(|v| v.get(0).copied())
        })
    })
}

pub fn score_setting_feedback(machine: &Machine, setting: &[i32]) -> Result<Option<i32>, Error> {
    let mut machines = vec![machine.clone(); setting.len()];
    let mut feedback_input = vec![0];
    for (&s, m) in setting.iter().zip(machines.iter_mut()) {
        m.run_with(iter::once(s), &mut feedback_input)?;
    }
    let mut done = false;
    while !done {
        for m in &mut machines {
            let mut out = Vec::new();
            m.run_with(feedback_input, &mut out)?;
            if out.len() == 0 {
                return Ok(None);
            }
            done = m.done();
            feedback_input = out;
        }
    }
    Ok(Some(feedback_input[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_score_setting(machine: &[i32], setting: &[i32], expected: i32) {
        assert_eq!(
            score_setting(&Machine::new(machine.to_vec()), setting.iter().copied())
                .unwrap()
                .unwrap(),
            expected
        );
    }

    fn test_score_feedback_setting(machine: &[i32], setting: &[i32], expected: i32) {
        assert_eq!(
            score_setting_feedback(&Machine::new(machine.to_vec()), setting)
                .unwrap()
                .unwrap(),
            expected
        );
    }

    #[test]
    fn test_score_setting_add() {
        test_score_setting(&[3, 0, 3, 1, 1, 0, 1, 0, 4, 0, 99], &[1, 2, 3, 4], 10);
    }

    #[test]
    fn test_score_setting_gt_true() {
        test_score_setting(
            &[3, 0, 3, 1, 7, 1, 0, 2, 1005, 2, 14, 104, 0, 99, 4, 0, 99],
            &[1, 2, 5, 8],
            8,
        );
    }

    #[test]
    fn test_score_setting_gt_false() {
        test_score_setting(
            &[3, 0, 3, 1, 7, 1, 0, 2, 1005, 2, 14, 104, 0, 99, 4, 0, 99],
            &[1, 2, 5, 5],
            0,
        );
    }

    #[test]
    fn test_score_setting_feedback() {
        test_score_feedback_setting(
            &[3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
            &[9,8,7,6,5],
            139629729
        );
        test_score_feedback_setting(
            &[3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],
            &[9,7,8,5,6],
            18216
        );
    }
}
