use std::collections::LinkedList;
use std::time::{SystemTime, UNIX_EPOCH};

const LINKED_LIST: LinkedList<u128> = LinkedList::new();
const ONE_MINUTE_AS_MILLISECONDS: u128 = 60000;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    limit: u16,
}

impl RateLimiter {
    pub fn new(limit: u16) -> RateLimiter {
        RateLimiter { limit }
    }

    pub fn acquire(&self) {
        remove_old_entries(LINKED_LIST, unix_timestamp());

        if LINKED_LIST.len() >= self.limit as usize {
            return;
        }
    }
}

fn remove_old_entries(mut list: LinkedList<u128>, current_time: u128) {
    while !list.is_empty() && *(list.front().unwrap()) < current_time - ONE_MINUTE_AS_MILLISECONDS  {
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