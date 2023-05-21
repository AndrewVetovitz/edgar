// TODO testing
// make rate limiter thread safe
// TODO documentation
// TODO release

pub mod enums;
pub mod errors;
mod http;
mod limiter;
mod utils;

pub use cik;
use cik::CIK;
use reqwest::{
    header::{HeaderMap, ACCEPT_ENCODING, CONNECTION, USER_AGENT},
    Client, ClientBuilder, Response,
};

use enums::Quarter;
use errors::EdgarAPIError;
use limiter::RateLimiter;
use utils::{download_response_file, left_pad_zeros};

pub(crate) const SEC_BASE_URL: &str = "https://www.sec.gov";
pub(crate) const SEC_DATA_BASE_URL: &str = "https://data.sec.gov";
const SEC_RATE_LIMIT: u16 = 10;

#[derive(Debug, Clone)]
pub struct EdgarAPI {
    http_client: Client,
    rate_limiter: RateLimiter,
}

impl EdgarAPI {
    pub fn new(user_agent: &str) -> Result<EdgarAPI, EdgarAPIError> {
        let mut header_map = HeaderMap::new();

        header_map.insert(USER_AGENT, user_agent.parse().unwrap());
        header_map.insert(ACCEPT_ENCODING, "txt".parse().unwrap());
        header_map.insert(CONNECTION, "keep-alive".parse().unwrap());

        let http_client = ClientBuilder::new().default_headers(header_map).build()?;

        let wrapper = EdgarAPI {
            http_client,
            rate_limiter: RateLimiter::new(SEC_RATE_LIMIT),
        };

        Ok(wrapper)
    }

    pub async fn download_cik_lookup_data(&mut self) -> Result<(), EdgarAPIError> {
        let endpoint: &str = &format!("{}/Archives/edgar/cik-lookup-data.txt", SEC_BASE_URL);

        let response = http::get(self, endpoint).await?;

        download_response_file(response).await?;

        Ok(())
    }

    pub async fn get_cik_data(&mut self, cik_code: CIK) -> Result<Response, EdgarAPIError> {
        let endpoint: &str = &format!(
            "{}/submissions/CIK{}.json",
            SEC_DATA_BASE_URL,
            left_pad_zeros(cik_code, 10),
        );

        http::get(self, endpoint).await
    }

    pub async fn get_xbrl_company_concept_data(
        &mut self,
        cik_code: CIK,
    ) -> Result<Response, EdgarAPIError> {
        let endpoint: &str = &format!(
            "{}/api/xbrl/companyconcept/CIK{}/us-gaap/AccountsPayableCurrent.json",
            SEC_DATA_BASE_URL,
            left_pad_zeros(cik_code, 10)
        );

        http::get(self, endpoint).await
    }

    pub async fn get_xbrl_company_facts_data(
        &mut self,
        cik_code: CIK,
    ) -> Result<Response, EdgarAPIError> {
        let endpoint: &str = &format!(
            "{}/api/xbrl/companyfacts/CIK{}.json",
            SEC_DATA_BASE_URL,
            left_pad_zeros(cik_code, 10)
        );

        http::get(self, endpoint).await
    }

    pub async fn get_xbrl_frames_data(
        &mut self,
        year: u8,
        quarter: Option<Quarter>,
        instantaneous: Option<bool>,
    ) -> Result<Response, EdgarAPIError> {
        let mut query_data = format!("CY{}", year);

        if let Some(some_quarter) = quarter {
            query_data = format!("{}Q{}", query_data, some_quarter.to_string());

            if let Some(some_instantaneous) = instantaneous {
                if some_instantaneous {
                    query_data = format!("{}I", query_data);
                }
            }
        }

        let endpoint: &str = &format!(
            "{}/api/xbrl/frames/us-gaap/AccountsPayableCurrent/USD/{}.json",
            SEC_DATA_BASE_URL, query_data,
        );

        http::get(self, endpoint).await
    }

    pub async fn download_bulk_company_facts(&mut self) -> Result<(), EdgarAPIError> {
        let endpoint: &str = &format!(
            "{}/Archives/edgar/daily-index/xbrl/companyfacts.zip",
            SEC_BASE_URL
        );

        let response = http::get(self, endpoint).await?;

        download_response_file(response).await?;

        Ok(())
    }

    pub async fn download_bulk_submissions(&mut self) -> Result<(), EdgarAPIError> {
        let endpoint: &str = &format!(
            "{}/Archives/edgar/daily-index/bulkdata/submissions.zip",
            SEC_BASE_URL
        );

        let response = http::get(self, endpoint).await?;

        download_response_file(response).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::EdgarAPI;

    fn normal_test<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn normal_types() {
        normal_test::<EdgarAPI>();
    }
}
