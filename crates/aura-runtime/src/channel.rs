//! CSP Channels with fiber park/unpark integration.

use crate::fiber::Fiber;
use crate::scheduler::{current_fiber_ptr, park_current, unpark_fiber};
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct AuraChannel {
    capacity: usize,
    buffer: Mutex<ChannelInner>,
}

struct ChannelInner {
    items: VecDeque<*mut ()>,
    blocked_senders: VecDeque<(*mut Fiber, *mut ())>,
    blocked_receivers: VecDeque<(*mut Fiber, *mut *mut ())>,
    closed: bool,
}

impl AuraChannel {
    pub fn new(capacity: usize) -> Self {
        AuraChannel {
            capacity,
            buffer: Mutex::new(ChannelInner {
                items: VecDeque::new(),
                blocked_senders: VecDeque::new(),
                blocked_receivers: VecDeque::new(),
                closed: false,
            }),
        }
    }

    pub fn send(&self, val: *mut ()) {
        loop {
            let should_park = {
                let mut inner = match self.buffer.lock() {
                    Ok(g) => g,
                    Err(p) => p.into_inner(),
                };
                if inner.closed {
                    return;
                }

                // If a receiver is already waiting, hand off directly
                if let Some((rx_fiber, out_slot)) = inner.blocked_receivers.pop_front() {
                    unsafe {
                        *out_slot = val;
                    }
                    unpark_fiber(rx_fiber);
                    return;
                }

                // If buffered and space available, push to buffer
                if self.capacity > 0 && inner.items.len() < self.capacity {
                    inner.items.push_back(val);
                    return;
                }

                // Must park current fiber
                let cur = current_fiber_ptr();
                assert!(
                    !cur.is_null(),
                    "Cannot block channel outside of fiber context"
                );
                inner.blocked_senders.push_back((cur, val));
                true
            };

            if should_park {
                park_current();
                // When woken up, our value was delivered or channel was closed
                return;
            }
        }
    }

    pub fn recv(&self) -> *mut () {
        let mut out_val: *mut () = std::ptr::null_mut();

        loop {
            let should_park = {
                let mut inner = match self.buffer.lock() {
                    Ok(g) => g,
                    Err(p) => p.into_inner(),
                };

                // If there are buffered items, consume one
                if let Some(item) = inner.items.pop_front() {
                    // If a sender was blocked waiting for buffer space, unpark it
                    if let Some((tx_fiber, tx_val)) = inner.blocked_senders.pop_front() {
                        inner.items.push_back(tx_val);
                        unpark_fiber(tx_fiber);
                    }
                    return item;
                }

                // If an unbuffered sender is waiting, take value directly
                if let Some((tx_fiber, tx_val)) = inner.blocked_senders.pop_front() {
                    unpark_fiber(tx_fiber);
                    return tx_val;
                }

                if inner.closed {
                    return std::ptr::null_mut();
                }

                // Must park current fiber waiting for a sender
                let cur = current_fiber_ptr();
                assert!(
                    !cur.is_null(),
                    "Cannot block channel outside of fiber context"
                );
                inner.blocked_receivers.push_back((cur, &mut out_val));
                true
            };

            if should_park {
                park_current();
                return out_val;
            }
        }
    }

    pub fn close(&self) {
        let mut inner = match self.buffer.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        inner.closed = true;
        while let Some((rx_fiber, out_slot)) = inner.blocked_receivers.pop_front() {
            unsafe {
                *out_slot = std::ptr::null_mut();
            }
            unpark_fiber(rx_fiber);
        }
        while let Some((tx_fiber, _)) = inner.blocked_senders.pop_front() {
            unpark_fiber(tx_fiber);
        }
    }
}
