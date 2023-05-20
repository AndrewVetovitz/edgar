use std::collections::LinkedList;
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

static linked_list: LinkedList<u128> = LinkedList::new();
static TEN_MINUTES_AS_MILLISECONDS: u128 = 600000;

#[derive(Debug)]
pub struct RateLimiter {
    limit: u128,
}

impl RateLimiter {
    pub fn new(limit: u128) -> RateLimiter {
        RateLimiter { limit }
    }

    pub fn acquire(&self) {
        // remove_old_entries(linked_list, unix_timestamp());
    }
}

fn remove_old_entries(mut list: LinkedList<u128>, current_time: u128) {
    while !list.is_empty() && *(list.front().unwrap()) < current_time - TEN_MINUTES_AS_MILLISECONDS  {
        list.pop_front();
    }
}

fn unix_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[cfg(test)]
fn test() {
    fn test_static() {

    }
}