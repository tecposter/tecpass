use std::{
  ops::Add,
  time::{Duration, SystemTime, UNIX_EPOCH},
};

use time::OffsetDateTime;

pub fn time2millis(t: SystemTime) -> u128 {
  let since_the_epoch = t
    .duration_since(UNIX_EPOCH)
    .expect("Clock may have gone blockwards");
  since_the_epoch.as_millis()
}

pub fn millis2time(m: u64) -> SystemTime {
  let dur = Duration::from_millis(m);
  UNIX_EPOCH.add(dur)
}

pub fn time2string(st: SystemTime) -> String {
  let t: OffsetDateTime = st.into();
  format!("{t}")
}

pub fn millis2string(m: u64) -> String {
  let dur = Duration::from_millis(m);
  let t: OffsetDateTime = UNIX_EPOCH.add(dur).into();
  format!("{t}")
}

pub fn current_millis() -> u128 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Clock may have gone backwards")
    .as_millis()
}
