use candid:: {CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager:: {MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures:: {BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable}; 
use std:: {borrow::Cow, cell::RefCell};
type Memory = VirtualMemory<DefaultMemoryImpl>;
extern crate request;
use std::io::Read;

const MAX_VALUE_SIZE: u32 = 100;

#[derive (CandidType, Deserialize)]
struct Exam {
    out_of: u8,
    course: String,
    curve: u8,
}

impl Storable for Exam {
    fn to_bytes(&self)-> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>)-> Self { 
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Exam {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; 
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init (DefaultMemoryImpl::default())); 
    static EXAM_MAP: RefCell<StableBTreeMap<u64, Exam, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m|m.borrow().get(MemoryId::new(0))),
    ));
    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m|m.borrow().get(MemoryId::new(1))),
    ));
}


#[ic_cdk_macros::query]
fn get_exam(key: u64) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow().get(&key))
}
#[ic_cdk_macros::query]
fn get_participation(key: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk_macros::update]
fn insert_exam(key: u64, value: Exam) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow_mut().insert(key, value))
}
#[ic_cdk_macros::update]
fn insert_participation(key: u64, value: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow_mut().insert(key, value))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define your API endpoint
    let api_endpoint = "http://api.openweathermap.org/data/2.5/weather?q=";

    // Define your city and country code
    let city = "Ankara";
    let country_code = "TR";

    // Define your OpenWeatherMap API key
    let open_weather_map_api_key = "467121ddb23343c53c075bc46e6c7b99";

    // Construct the full URL
    let full_url = format!("{}{},{}&APPID={}", api_endpoint, city, country_code, open_weather_map_api_key);

    // Use the request::get function to make the request
    let mut response = request::get(&full_url)?.text()?;

    // The response will be a JSON string containing weather data
    let weather_data: serde_json::Value = serde_json::from_str(&response)?;

    // You can now use the weather_data object to access the weather information
    println!("Temperature: {}", weather_data["main"]["temp"]);
    println!("Pressure: {}", weather_data["main"]["pressure"]);
    println!("Humidity: {}", weather_data["main"]["humidity"]);
    println!("Wind Speed: {}", weather_data["wind"]["speed"]);

    Ok(())
}

// dfx start --background
// dfx deploy
