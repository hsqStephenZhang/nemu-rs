use std::collections::BinaryHeap;

use tracing::info;

type Time = u64; // 纳秒为单位的虚拟时间

pub trait Callback: FnMut(Time, Time) + 'static {}

impl<T> Callback for T where T: FnMut(Time, Time) + 'static {}

pub struct VirtualTimer {
    when: Time,
    period: Option<Time>,
    callback: Box<dyn Callback>,
}

impl std::fmt::Debug for VirtualTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VirtualTimer(when: {}, period: {:?})",
            self.when, self.period
        )
    }
}

impl PartialEq for VirtualTimer {
    fn eq(&self, other: &Self) -> bool {
        self.when == other.when
    }
}

impl Eq for VirtualTimer {}

impl PartialOrd for VirtualTimer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.when.cmp(&self.when)) // reverse order for min-heap
    }
}
impl Ord for VirtualTimer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.when.cmp(&self.when)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElapsePolicy {
    Once,
    Compensation,
}

#[derive(Debug)]
pub struct VirtualClock {
    now: Time,
    policy: ElapsePolicy,
    timers: BinaryHeap<VirtualTimer>,
}

impl VirtualClock {
    pub fn new() -> Self {
        Self {
            now: 0,
            policy: ElapsePolicy::Once,
            timers: BinaryHeap::new(),
        }
    }

    pub fn with_policy(self, policy: ElapsePolicy) -> Self {
        Self { policy, ..self }
    }

    pub fn total(&self) -> usize {
        self.timers.len()
    }

    pub fn advance(&mut self, delta: Time) {
        self.now += delta;
        self.check_timers();
    }

    fn check_timers(&mut self) {
        while let Some(timer) = self.timers.peek() {
            if timer.when <= self.now {
                let mut timer = self.timers.pop().unwrap();
                (timer.callback)(self.now, timer.when);
                if let Some(period) = timer.period {
                    // 如果有周期性定时器，重新注册
                    if self.policy == ElapsePolicy::Compensation {
                        timer.when += period;
                    } else {
                        timer.when = self.now + period;
                    }
                    self.timers.push(timer);
                }
            } else {
                break;
            }
        }
    }

    pub fn register_timer<F: Callback>(&mut self, delay: Time, callback: F, period: Option<Time>) {
        let timer = VirtualTimer {
            when: self.now + delay,
            period,
            callback: Box::new(callback),
        };
        info!("Registering timer: {:?}", timer);
        self.timers.push(timer);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic::AtomicUsize};

    use super::*;

    #[test]
    fn test_virtual_clock() {
        let mut clock = VirtualClock::new();

        clock.register_timer(
            100,
            |now, when| {
                println!("Timer called at {} (was scheduled for {})", now, when);
            },
            None,
        );

        clock.advance(101);

        assert_eq!(clock.total(), 0, "All timers should have been executed");
    }

    #[test]
    fn test_periodic_timer() {
        let mut clock = VirtualClock::new();
        let count = Arc::new(AtomicUsize::new(0));

        let c = count.clone();
        clock.register_timer(
            50,
            move |now, when| {
                c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                println!(
                    "Periodic timer called at {} (was scheduled for {})",
                    now, when
                );
            },
            Some(50),
        );

        // step
        clock.advance(200);
        // step
        clock.advance(51);

        assert_eq!(
            count.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "Periodic timer should have been called 4 times"
        );

        let mut clock = VirtualClock::new().with_policy(ElapsePolicy::Compensation);
        let count = Arc::new(AtomicUsize::new(0));

        let c = count.clone();
        clock.register_timer(
            50,
            move |now, when| {
                c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                println!(
                    "Compensation Periodic timer called at {} (was scheduled for {})",
                    now, when
                );
            },
            Some(50),
        );

        // once
        clock.advance(200);

        assert_eq!(
            count.load(std::sync::atomic::Ordering::SeqCst),
            4,
            "Periodic timer should have been called 4 times"
        );
    }

    #[test]
    fn test_multi_timers() {
        let mut clock = VirtualClock::new();
        let count = Arc::new(AtomicUsize::new(0));

        let c = count.clone();
        clock.register_timer(
            100,
            move |_, _| {
                let res = c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                println!("Timer called, count: {}", res);
            },
            Some(100),
        );
        clock.register_timer(
            100,
            |_, _| {
                println!("Another timer called");
            },
            Some(100),
        );
        clock.register_timer(
            100,
            |_, _| {
                println!("Yet another timer called");
            },
            Some(100),
        );

        for _ in 0..10 {
            clock.advance(100);
        }
    }
}
