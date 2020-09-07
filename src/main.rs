mod apis;

fn sequential() {
    let olive_api = apis::olive::OliveApi::new("Oklahoman");

    for year in 1901..2020 {
        for month in 1..13 {
            for day in 1..32 {
                match olive_api.get_pdf_id_from_date(
                    year.to_string(),
                    format!("{:02}", month),
                    format!("{:02}", day)
                ) {
                    Ok(pdf_id) => {
                        match olive_api.download_pdf(
                            pdf_id,
                            format!(
                                "./newspapers/{}/{}/{}",
                                year.to_string(),
                                format!("{:02}", month),
                                format!("DOK_{}_{:02}_{:02}.pdf", year, month, day)
                            )
                        ) {
                            Ok(_) => {
                                println!(
                                    "Downloaded PDF for {}/{}/{}",
                                    year.to_string(),
                                    format!("{:02}", month),
                                    format!("{:02}", day)
                                );
                            },
                            Err(err) => {
                                println!(
                                    "Error downloading the PDF for {}/{}/{}",
                                    year.to_string(),
                                    format!("{:02}", month),
                                    format!("{:02}", day)
                                );
                                println!("\t{}", err);
                            }
                        };
                    },
                    Err(_) => {
                        println!(
                            "Error getting the PDF ID for {}/{}/{}",
                            year.to_string(),
                            format!("{:02}", month),
                            format!("{:02}", day)
                        );
                    }
                };
            }
        }
    }
}

fn main() {
    sequential();
}
