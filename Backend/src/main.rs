use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::completion::{self, CompletionRequest};
use actix_cors::Cors;
use std::io;
use std::env;

#[derive(Debug, Deserialize)]
struct CsvItem {
    #[serde(rename = "0")]
    amount: f64,
    #[serde(rename = "1")]
    description: String,
}

#[derive(Serialize)]
struct CategorySummary {
    categories: HashMap<String, f64>,
}

// Health Check Endpoint
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Credit Card Analyzer is Running!")
}

async fn process_csv(csv_content: String) -> HttpResponse {


    // Find the start of the CSV content
    let start_index = match csv_content.find("\r\n\r\n") {
        Some(index) => index + 4, 
        None => {
            eprintln!("CSV content not found in form data start");
            return HttpResponse::BadRequest().body("CSV content not found");
        }
    };
    let end_index = match csv_content.find("\r\n-") {
        Some(index) => index,
        None => {
            eprintln!("CSV content not found in form data end");
            csv_content.len()
        }
    };

    // Extract CSV content
    let csv_content = &csv_content[start_index..end_index];

    // Parse CSV content into a vector of CsvItem
    let mut reader = csv::ReaderBuilder::new().has_headers(false).delimiter(b',').from_reader(csv_content.as_bytes());
    let mut category_totals: HashMap<String, f64> = HashMap::new();

    for result in reader.deserialize::<CsvItem>() {

        match result {
            Ok(item) => {
                // Communicate with OpenAI API to categorize the description
                let category = categorize_with_openai(&item.description).await;
                let string_result = category.unwrap_or_else(|err| format!("Error: {}", err));

                // Update the category_totals hashmap
                let total = category_totals.entry(string_result).or_insert(0.0);
                *total += item.amount;
            }
            Err(e) => {
                eprintln!("Error parsing CSV: {}", e);
                return HttpResponse::BadRequest().body("Error parsing CSV");
            }
        }
    }

    // Return the category_totals hashmap
    HttpResponse::Ok().json(CategorySummary {
        categories: category_totals,
    })
}


async fn categorize_with_openai(description: &str) -> Result<String, Box<dyn std::error::Error>> {

    let value = env::var("OPENAI_API_KEY").unwrap_or("OPENAI_API_KEY".to_string());
    let client: Client = Client::new(value.to_string());


    let req = CompletionRequest::new(
        completion::GPT3_TEXT_DAVINCI_003.to_string(),
        String::from(format!("Coinsider the following categories: [Online Shopping, Groceries, Restaurants, Utilities, Rent, Transportation, Entertainment, Clothes, Healthcare, Insurance, Auto, Education, Technology, Subscriptions, Miscellaneous, Travel]. Now categorize the following purchase on my credit card: {}\nCategory:", description)),
    )
    .max_tokens(3000)
    .temperature(0.5)
    .top_p(1.0)
    .stop(vec![String::from(" Human:"), String::from(" AI:")]);

    let result = client.completion(req)?;
    println!("{:}", result.choices[0].text);

    Ok(format!("{}", result.choices[0].text))
    

}


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_headers(vec!["Content-Type"])
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .max_age(3600),
            )
            .service(web::resource("/").to(index))
            .service(web::resource("/process_csv").to(process_csv))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

