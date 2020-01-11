use crate::intcode::{Error, Machine};

pub fn score_setting<I>(machine: &Machine, setting: I) -> Result<Option<i32>, Error>
where
    I: IntoIterator<Item = i32>,
{
    setting.into_iter().try_fold(Some(0), |p, s| {
        p.map_or(Ok(None), |input| {
            machine
                .clone()
                .run([s, input].iter().copied())
                .map(|v| v.get(0).copied())
        })
    })
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
}
