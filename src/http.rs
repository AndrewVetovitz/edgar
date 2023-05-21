use crate::errors::EdgarAPIError;
use crate::EdgarAPI;

use reqwest::{Response, StatusCode};

use std::{thread, time::Duration};

pub async fn get(wrapper: &mut EdgarAPI, endpoint: &str) -> Result<Response, EdgarAPIError> {
    loop {
        loop {
            match wrapper.rate_limiter.stall_for() {
                0 => {
                    wrapper.rate_limiter.claim();
                    break;
                }
                timeout => thread::sleep(Duration::from_millis(timeout)),
            }
        }

        let response = wrapper.http_client.get(endpoint).send().await?;

        if !did_hit_limit(&response) {
            return Ok(response);
        }
    }
}

fn did_hit_limit(response: &Response) -> bool {
    response.status() == StatusCode::TOO_MANY_REQUESTS
}
