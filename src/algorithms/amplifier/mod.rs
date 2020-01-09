use crate::intcode::{Error, Machine};

pub fn score_setting<I>(machine: &Machine, setting: I) -> Result<Option<i32>, Error>
where
    I: IntoIterator<Item = i32>
{
    setting.into_iter().fold(Ok(Some(0)), |p, s| {
        p?.map_or(Ok(None), |input| {
            machine.clone()
                .run([s, input].iter().copied())
                .map(|v| v.get(0).copied())

        })
    })
}