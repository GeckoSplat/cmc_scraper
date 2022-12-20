
extern crate serde;
extern crate serde_json;

//fn main (){                              //mess as I thought i would come back to it later and try to get api key
                                            // to load .

// #[derive(Deserialize)]
// #[derive(Debug)]
//#[derive(Clone)]
//#[derive(Eq, Hash, PartialEq)]

// struct Apidata{
//     apikey: String
// }


// let apikey = std::fs::read_to_string("apidata.json").unwrap();
// let accepts = "Accepts:application/json" ;
fn main (){

let cm_url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest".to_string();
let client = reqwest::blocking::Client::new();


let res = client.get(cm_url)
.header("X-CMC_PRO_API_KEY","need API key hardcoded here")// Hardcoded here as 
// I couldn't get the api key to load just its value from file, was always the key value pair. 
.query(&[("start", "1"),("limit","20"),("convert","GBP")])//parameters
.send()
.unwrap()
.text()
.unwrap();

let json: serde_json::Value = serde_json::from_str(&res).unwrap();
let items = json.get("data").unwrap();//have tried index :"quote" here but does not work
let cryptos = items.as_array().unwrap();

 for crypto in cryptos{
    let name=crypto["name"].as_str().unwrap();
    let symbol = crypto["symbol"].as_str().unwrap();
    let price = crypto["quote"].as_str();// fails here as None value in quote if .unwrap(); is added .    
    //let last = format!("${:.20}", crypto["quote"].as_f64().unwrap()); // was something i tried and failed.
    println!("{} {} {:?}",name,symbol,price);

}    
}

// TODO : I cannot get the array to convert to a list i can index into .
// "quote" seems to be a nested array/dict , need to access this, key "quote" is not doing it.