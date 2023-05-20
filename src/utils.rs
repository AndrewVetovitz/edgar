use std::error::Error;
use std::fs::File;
use std::io::copy;

use cik::CIK;
use reqwest::Response;

pub(crate) async fn download_response_file(response: Response) -> Result<(), Box<dyn Error>> {
    let mut dest = {
        let filename = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        #[cfg(test)]
        println!("file to download: '{}'", filename);

        File::create(filename)?
    };

    let content = response.text().await?;

    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}

pub(crate) fn left_pad_zeros(cik: CIK, final_length: usize) -> String {
    format!("{:0>width$}", cik.to_string(), width = final_length)
}

#[cfg(test)]
mod left_pad_tests {
    use super::left_pad_zeros;
    use cik::parse;
    use std::error::Error;

    #[test]
    fn general_test() -> Result<(), Box<dyn Error>> {
        let cik = parse("12345")?;
        let result = left_pad_zeros(cik, 10);

        assert_eq!("0000012345", result);

        Ok(())
    }
}
