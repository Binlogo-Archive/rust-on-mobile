use serde::{Deserialize, Serialize};

const SWAPI_BASE_URL: &str = "https://swapi.dev/api";

lazy_static! {
    // Runtime
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();
    // HTTP cliet share instance
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsePeople {
    pub count: i64,
    pub next: String,
    pub results: Vec<People>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct People {
    pub name: String,
    pub height: String,
    pub mass: String,
    #[serde(rename = "hair_color")]
    pub hair_color: String,
    #[serde(rename = "skin_color")]
    pub skin_color: String,
    #[serde(rename = "eye_color")]
    pub eye_color: String,
    #[serde(rename = "birth_year")]
    pub birth_year: String,
    pub gender: String,
    pub homeworld: String,
    pub films: Vec<String>,
    pub species: Vec<String>,
    pub vehicles: Vec<String>,
    pub starships: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}

// Callback
#[allow(non_snake_case)]
pub trait SwapiCallback {
    fn onLoad(&mut self, res: Vec<People>);
    fn onError(&mut self, err: &str);
}

pub struct SwapiClient();

#[allow(non_snake_case)]
impl SwapiClient {
    pub fn new() -> SwapiClient {
        SwapiClient()
    }

    pub fn loadAllPeople(&self, mut callback: Box<dyn SwapiCallback + Send>) {
        (*RUNTIME).spawn(async move {
            let res = load_all_people().await;
            match res {
                Ok(r) => callback.onLoad(r.results),
                Err(e) => callback.onError(format!("Failed: {}", e).as_str()),
            }
        });
    }
}

pub async fn load_all_people() -> Result<ResponsePeople, Box<dyn std::error::Error>> {
    let end_point = "/people";
    let url = format!("{}{}", SWAPI_BASE_URL, end_point);
    let res: ResponsePeople = HTTP_CLIENT.get(url.as_str()).send().await?.json().await?;
    Ok(res)
}
