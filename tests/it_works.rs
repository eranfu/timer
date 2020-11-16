use std::time::{Duration, Instant};

use async_std::task;

use timer::TimerFuture;

#[test]
fn it_works() {
    let duration = Duration::from_secs(4);
    let begin = Instant::now();
    task::block_on(TimerFuture::new(duration));
    let end = Instant::now();
    assert!(end - begin >= duration);
}
