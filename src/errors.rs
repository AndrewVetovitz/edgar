#[derive(Hash, Clone, Debug, PartialEq)]
pub struct EdgarAPIError {
    code: String,
    message: String,
}

impl EdgarAPIError {
    pub fn from_raw(code: String, message: String) -> EdgarAPIError {
        EdgarAPIError { code, message }
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl From<reqwest::Error> for EdgarAPIError {
    fn from(reqwest_error: reqwest::Error) -> EdgarAPIError {
        EdgarAPIError::from_raw(
            "HttpClientError".to_string(),
            format!("Unable to parse successful response: {}", reqwest_error),
        )
    }
}

impl From<std::io::Error> for EdgarAPIError {
    fn from(io_error: std::io::Error) -> EdgarAPIError {
        EdgarAPIError::from_raw(
            io_error.kind().to_string(),
            format!(
                "Encountered IO Error while downloading file: {}",
                io_error
            ),
        )
    }
}
