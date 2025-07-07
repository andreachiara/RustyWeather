use core::hash;
use serde::{Deserialize, Serialize, de};
use std::default;
use std::time::Duration;
use ureq::Agent;

const lat: f64 = 0;
const lon: f64 = 0;
const api_key: &str = "xxxxxxxxxxxxxxxxxxxxx";

#[derive(Default, Serialize, Deserialize, Debug)]
enum WheatherCode {
    #[default]
    UnknownWeatherCode,
    ThunderstormWithLightRain = 200,
    ThunderstormWithRain,
    ThunderstormWithHeavyRain,
    LightThunderstorm = 210,
    Thunderstorm,
    HeavyThunderstorm,
    RaggedThunderstorm = 221,
    ThunderstormWithLightDrizzle = 230,
    ThunderstormWithDrizzle,
    ThunderstormWithHeavyDrizzle,

    LightIntensityDrizzle = 300,
    Drizzle,
    HeavyIntensityDrizzle,
    LightIntensityDrizzleRain = 310,
    DrizzleRain,
    HeavyIntensityDrizzleRain,
    ShowerRainAndDrizzle,
    HeavyShowerRainAndDrizzle,
    ShowerDrizzle = 321,

    LightRain = 500,
    ModerateRain,
    HeavyIntensityRain,
    VeryHeavyRain,
    ExtremeRain,
    FreezingRain = 511,
    LightIntensityShowerRain = 520,
    ShowerRain,
    HeavyIntensityShowerRain,
    RaggedShowerRain = 531,

    LightSnow = 600,
    Snow,
    HeavySnow,
    Sleet = 611,
    LightShowerSleet,
    ShowerSleet,
    LightRainAndSnow = 615,
    RainAndSnow = 616,
    LightShowerSnow = 620,
    ShowerSnow,
    HeavyShowerSnow,

    Mist = 701,
    Smoke = 711,
    Haze = 721,
    SandDustWhirls = 731,
    Fog = 741,
    Sand = 751,
    Dust = 761,
    Ash = 762,
    Squall = 771,
    Tornado = 781,

    Clear = 800,

    FewClouds,
    ScatteredClouds,
    BrokenClouds,
    OvercastClouds,
}

impl From<u16> for WheatherCode {
    fn from(item: u16) -> Self {
        match item {
            200 => Self::ThunderstormWithLightRain,
            201 => Self::ThunderstormWithRain,
            202 => Self::ThunderstormWithHeavyRain,
            210 => Self::LightThunderstorm,
            211 => Self::Thunderstorm,
            212 => Self::HeavyThunderstorm,
            221 => Self::RaggedThunderstorm,
            230 => Self::ThunderstormWithLightDrizzle,
            231 => Self::ThunderstormWithDrizzle,
            232 => Self::ThunderstormWithHeavyDrizzle,

            300 => Self::LightIntensityDrizzle,
            301 => Self::Drizzle,
            302 => Self::HeavyIntensityDrizzle,
            310 => Self::LightIntensityDrizzleRain,
            311 => Self::DrizzleRain,
            312 => Self::HeavyIntensityDrizzleRain,
            313 => Self::ShowerRainAndDrizzle,
            314 => Self::HeavyShowerRainAndDrizzle,
            321 => Self::ShowerDrizzle,

            500 => Self::LightRain,
            501 => Self::ModerateRain,
            502 => Self::HeavyIntensityRain,
            503 => Self::VeryHeavyRain,
            504 => Self::ExtremeRain,
            511 => Self::FreezingRain,
            520 => Self::LightIntensityShowerRain,
            521 => Self::ShowerRain,
            522 => Self::HeavyIntensityShowerRain,
            531 => Self::RaggedShowerRain,

            600 => Self::LightSnow,
            601 => Self::Snow,
            602 => Self::HeavySnow,
            611 => Self::Sleet,
            612 => Self::LightShowerSleet,
            615 => Self::LightRainAndSnow,
            616 => Self::RainAndSnow,
            620 => Self::LightShowerSnow,
            621 => Self::ShowerSnow,
            622 => Self::HeavyShowerSnow,

            701 => Self::Mist,
            711 => Self::Smoke,
            721 => Self::Haze,
            731 => Self::SandDustWhirls,
            741 => Self::Fog,
            751 => Self::Sand,
            761 => Self::Dust,
            762 => Self::Ash,
            771 => Self::Squall,
            781 => Self::Tornado,

            800 => Self::Clear,

            801 => Self::FewClouds,
            802 => Self::ScatteredClouds,
            803 => Self::BrokenClouds,
            804 => Self::OvercastClouds,

            _ => Self::UnknownWeatherCode,
        }
    }
}

impl std::fmt::Display for WheatherCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThunderstormWithLightRain
            | Self::ThunderstormWithRain
            | Self::ThunderstormWithHeavyRain
            | Self::LightThunderstorm
            | Self::Thunderstorm
            | Self::HeavyThunderstorm
            | Self::RaggedThunderstorm => {
                write!(f, "Thunderstorm")
            }
            Self::ThunderstormWithLightDrizzle
            | Self::ThunderstormWithDrizzle
            | Self::ThunderstormWithHeavyDrizzle => {
                write!(f, "Thunderstorm With Drizzle")
            }
            Self::LightIntensityDrizzle
            | Self::Drizzle
            | Self::HeavyIntensityDrizzle
            | Self::LightIntensityDrizzleRain
            | Self::DrizzleRain
            | Self::HeavyIntensityDrizzleRain
            | Self::ShowerRainAndDrizzle
            | Self::HeavyShowerRainAndDrizzle
            | Self::ShowerDrizzle => {
                write!(f, "Drizzle")
            }
            Self::LightRain
            | Self::ModerateRain
            | Self::HeavyIntensityRain
            | Self::VeryHeavyRain
            | Self::ExtremeRain
            | Self::FreezingRain
            | Self::LightIntensityShowerRain
            | Self::ShowerRain
            | Self::HeavyIntensityShowerRain
            | Self::RaggedShowerRain => {
                write!(f, "Rain")
            }
            Self::LightSnow | Self::Snow | Self::HeavySnow | Self::Sleet => {
                write!(f, "Snow")
            }
            Self::LightShowerSleet
            | Self::ShowerSleet
            | Self::LightRainAndSnow
            | Self::RainAndSnow
            | Self::LightShowerSnow
            | Self::ShowerSnow
            | Self::HeavyShowerSnow => {
                write!(f, "Rain And Snow")
            }
            Self::Mist => {
                write!(f, "mist")
            }
            Self::Smoke => {
                write!(f, "Smoke")
            }
            Self::Haze => {
                write!(f, "Haze")
            }
            Self::SandDustWhirls => {
                write!(f, "Sand or Dust Whirls")
            }
            Self::Fog => {
                write!(f, "Fog")
            }
            Self::Sand => {
                write!(f, "Sand")
            }
            Self::Dust => {
                write!(f, "Dust")
            }
            Self::Ash => {
                write!(f, "Ash (volcanic)")
            }
            Self::Squall => {
                write!(f, "Squall")
            }
            Self::Tornado => {
                write!(f, "Tornado")
            }
            Self::Clear => {
                write!(f, "Clear")
            }
            Self::FewClouds => {
                write!(f, "Few Cloud")
            }
            Self::ScatteredClouds => {
                write!(f, "Scattered Clouds")
            }
            Self::BrokenClouds => {
                write!(f, "Broken Clouds")
            }
            Self::OvercastClouds => {
                write!(f, "Overcast Clouds")
            }
            _ => {
                write!(f, "UNKNOWN CODE")
            }
        }
    }
}

fn to_weather_code(int: u16) {
    let wc: WheatherCode = int.into();
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: u16,
    #[serde(skip)]
    code: WheatherCode,
    main: String,
    description: String,
    icon: String,
}

impl std::fmt::Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MeteoDataCurrent {
    dt: u64,
    sunrise: u64,
    sunset: u64,
    temp: f64,
    feels_like: f64,
    pressure: u16,
    humidity: u8,
    dew_point: f64,
    uvi: f64,
    clouds: u8,
    visibility: u16,
    wind_speed: f64,
    wind_deg: u16,
    #[serde(default)]
    wind_gust: f64,
    weather: Vec<Weather>,
}

impl std::fmt::Display for MeteoDataCurrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dt: {}\nsunrise: {} | sunset: {}\ntemp: {} | feels like: {}\npressure: {} | humidity: {}\ndew_point: {}\nuvi: {} | clouds cover: {}\nvisibility: {}\n wind speed: {} | deg: {} | gust: {}\nWeather: {}",
            self.dt,
            self.sunrise,
            self.sunset,
            self.temp,
            self.feels_like,
            self.pressure,
            self.humidity,
            self.dew_point,
            self.uvi,
            self.clouds,
            self.visibility,
            self.wind_speed,
            self.wind_deg,
            self.wind_gust,
            self.weather[0]
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    lat: f64,
    lon: f64,
    timezone: String,
    timezone_offset: i32,
    current: MeteoDataCurrent,
}

impl std::fmt::Display for APIResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lat: {}, lon: {}, tz: {}\nWEATHER DATA\n{}",
            self.lat, self.lon, self.timezone, self.current
        )
    }
}

fn main() -> Result<(), ureq::Error> {
    let mut config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build();

    let agent: Agent = config.into();
    let exclude_str = "hourly,daily,minutely,alerts";

    let req_url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={lat}&lon={lon}&exclude={exclude_str}&appid={api_key}"
    );
    println!("{}", req_url);

    println!(
        "{}",
        agent.get(&req_url).call()?.body_mut().read_to_string()?
    );

    let mut body = agent
        .get(req_url)
        .call()?
        .body_mut()
        .read_json::<APIResponse>()?;
    body.current.weather[0].code = body.current.weather[0].id.into();
    println!("{}", body);
    Ok(())
}
