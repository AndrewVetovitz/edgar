use std::fs::File;
use std::io::copy;
use std::error::Error;

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