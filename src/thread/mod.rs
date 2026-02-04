use crate::std::time::Duration;
use crate::sys::coreinit;

pub fn sleep(dur: Duration) {
    // the max duration is over 500 years, so this should be fine
    let nsec = (dur.as_nanos().min(u64::MAX as u128)) as u64;
    let timer_clock = unsafe { (*coreinit::system::system_info()).bus_clock_speed } / 4;

    let ticks = (nsec * (timer_clock as u64) / 31250) / 32000;

    unsafe {
        coreinit::thread::sleep(ticks);
    }
}
