use std::io;
use std::fs::{File, create_dir_all};
use url::{Url};
use serde::{Deserialize};
use reqwest;

pub struct NewspaperDate {
    pub year: String,
    pub month: String,
    pub day: String,
    pub short: Option<String>
}

impl NewspaperDate {
    pub fn to_string(&self) -> String {
        return format!("{}/{}/{}", self.year, self.month, self.day);
    }
}

#[derive(Deserialize)]
struct OliveResponse {
    pdf: String,
}

pub struct OliveApi {
    olive_url: String
}

impl OliveApi {
    pub fn new(newspaper: &str) -> OliveApi {
        OliveApi {
            olive_url: format!("https://digital.olivesoftware.com/olive/apa/{}", newspaper)
        }
    }

    pub fn get_pdf_id_from_date(&self, date: &NewspaperDate) -> Result<String, Box<dyn std::error::Error>> {
        let olive_api_route = format!("/get/prxml.ashx?kind=doc&href=dok/{}/{}/{}", date.year, date.month, date.day);
        let request_url = &Url::parse(&format!("{}{}", self.olive_url, olive_api_route))?.into_string();
        let response: OliveResponse = reqwest::blocking::get(request_url)?.json()?;
        Ok(response.pdf)
    }

    pub fn download_pdf(&self, pdf_id: String, download_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let olive_api_route = format!("/get/pdf.ashx?pdf={}", pdf_id);
        let request_url = &Url::parse(&format!("{}{}", self.olive_url, olive_api_route))?.into_string();
        let mut response = reqwest::blocking::get(request_url)?;

        if std::path::Path::new(&download_path).exists() {
            Err(format!("{} already exists", download_path))?
        } else {
            let newspaper_directory = &download_path.split("/").collect::<Vec<&str>>().drain(..4).collect::<Vec<&str>>().join("/");
            create_dir_all(&newspaper_directory)?;
            let mut download_file = File::create(download_path)?;

            io::copy(&mut response, &mut download_file)?;

            Ok(())
        }
    }
}


