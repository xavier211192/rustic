use std::{thread, time::Duration};
fn main() {
    let bar = indicatif::ProgressBar::new(1000);
    for _i in 0..1001{
        thread::sleep(Duration::from_millis(10));
        bar.inc(1);
    }
    bar.finish()
}