extern crate serde;
extern crate serde_json;
extern crate reqwest;

use serde::{Deserialize, Serialize};

const url: &str = "https://coronavirus-19-api.herokuapp.com/countries/";

#[derive(Deserialize, Debug)]
struct Country {
    country: String,
    cases: i32,
    todayCases: i32,
    deaths: i32,
    todayDeaths: i32,
    recovered: i32,
    active: i32,
    critical: i32,
    casesPerOneMillion: i32,
    deathsPerOneMillion: i32,
    totalTests: i32,
    testsPerOneMillion: i32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    println!("Check quickly todays COVID-19 statistics per country.\n");

    while (true) {

    //Detail per Country
    let mut line = String::new();
    println!("Enter Country: press 'q' or 'quit' to exit");

    std::io::stdin().read_line(&mut line).unwrap();

    if (line.trim() == "q" || line.trim() == "quit") {
        break;
    }

    let c_url: String = url.to_string() + &line;

    let client = reqwest::Client::new();
    let res = client.get(&c_url).send().await?;

    //println!("DEBUG: Status: {}", res.status());
    //println!("DEBUG: Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    //println!("Body: \n{}", body);

    let data_obj: Country = serde_json::from_str(&body).unwrap();
    
    println!();
    println!("Country: {:#?}", data_obj.country);
    println!("Cases: {:#?}", data_obj.cases);
    println!("Today Cases: {:#?}", data_obj.todayCases);
    println!("Deaths: {:#?}", data_obj.deaths);
    println!("Today Deaths: {:#?}", data_obj.todayDeaths);
    println!("Recovered: {:#?}", data_obj.recovered);
    println!("Active: {:#?}", data_obj.active);
    println!("Critical: {:#?}", data_obj.critical);
    println!("Cases Per One Million People: {:#?}", data_obj.casesPerOneMillion);
    println!("Deaths Per One Million People: {:#?}", data_obj.deathsPerOneMillion);
    println!("Total Tests: {:#?}", data_obj.totalTests);
    println!("Tests Per One Million People: {:#?}", data_obj.testsPerOneMillion);
    println!();
    
    }

    Ok(())
}
