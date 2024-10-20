use core::arch::x86_64::_rdtsc;

const CYCLES_PER_SECOND: u64 = 3333333333;

/**
 * Don't use this for anything that requires accuracy. But it's pretty solid for approximate timing.
 * 
 * Parameter ms is number of milliseconds to sleep for.
 */
pub fn sleep(ms: u64) {
    unsafe {
        // I timed 30 seconds for 10 sets of each 10,000,000,000 (10 billion) clock cycles (this may be machine dependent I do not know.)
        // this is just for testing purposes (for now), but it's very useful.
        // that's 3 seconds per 10bil cycles. Or ~3.33 bil cycles per second.
        
        // retrieve cycles
        let cycle: u64 = _rdtsc();
        
        // loop until clock cycle amount has caught up to + ~ 1 second * ms/1000
        while cycle + (CYCLES_PER_SECOND * ms / 1000) > _rdtsc() {
        }
    }
}
