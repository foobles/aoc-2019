use crate::intcode::{Error, Machine};

pub fn score_setting(machine: &Machine, setting: &[i32]) -> Result<Option<i32>, Error> {
    setting.iter().fold(Ok(Some(0)), |p, &s| {
        p?.map_or(Ok(None), |input| {
            machine.clone()
                .run([s, input].iter().copied())
                .map(|v| v.get(0).copied())

        })
    })
}