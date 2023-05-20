use std::collections::LinkedList;
use std::time::{SystemTime, UNIX_EPOCH};

const ONE_SECOND_AS_MILLISECONDS: u128 = 1000;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    limit: u16,
    buffer: LinkedList<u128>,
}

impl RateLimiter {
    pub fn new(limit: u16) -> RateLimiter {
        RateLimiter {
            limit,
            buffer: LinkedList::new(),
        }
    }

    pub fn stall_for(&mut self) -> u64 {
        let current_time = unix_timestamp();
        remove_old_entries(&mut self.buffer, current_time);

        if self.buffer.len() < self.limit as usize {
            return 0;
        }

        let next_request_time = self.buffer.front().unwrap() + ONE_SECOND_AS_MILLISECONDS;

        (next_request_time - current_time) as u64
    }

    pub fn claim(&mut self) {
        self.buffer.push_back(unix_timestamp());

        #[cfg(debug_assertions)]
        print_requests(self.buffer.len());
    }
}

#[cfg(debug_assertions)]
fn print_requests(len: usize) {
    println!("Total number of requests: {:?}", len);
}

fn remove_old_entries(list: &mut LinkedList<u128>, current_time: u128) {
    while !list.is_empty() && *(list.front().unwrap()) < current_time - ONE_SECOND_AS_MILLISECONDS {
        list.pop_front();
    }

    #[cfg(debug_assertions)]
    print_requests(list.len());
}

fn unix_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
