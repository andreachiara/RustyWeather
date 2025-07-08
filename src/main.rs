use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Utc};
use chrono_tz;
use core::hash;
use serde::{Deserialize, Serialize, de};
use std::default;
use std::env;
use std::fs;
use std::time::Duration;
use std::time::SystemTime;
use ureq::Agent;

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
            | Self::ThunderstormWithHeavyRain => {
                write!(f, "Thunderstorm With Rain")
            }
            Self::LightThunderstorm
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
fn get_icon_day(code: &WheatherCode) -> String {
    match code {
        WheatherCode::ThunderstormWithLightRain
        | WheatherCode::ThunderstormWithRain
        | WheatherCode::ThunderstormWithHeavyRain => String::from(""),
        WheatherCode::LightThunderstorm
        | WheatherCode::Thunderstorm
        | WheatherCode::HeavyThunderstorm
        | WheatherCode::RaggedThunderstorm => String::from(""),
        WheatherCode::ThunderstormWithLightDrizzle
        | WheatherCode::ThunderstormWithDrizzle
        | WheatherCode::ThunderstormWithHeavyDrizzle => String::from(""),
        WheatherCode::LightIntensityDrizzle
        | WheatherCode::Drizzle
        | WheatherCode::HeavyIntensityDrizzle
        | WheatherCode::LightIntensityDrizzleRain
        | WheatherCode::DrizzleRain
        | WheatherCode::HeavyIntensityDrizzleRain
        | WheatherCode::ShowerRainAndDrizzle
        | WheatherCode::HeavyShowerRainAndDrizzle
        | WheatherCode::ShowerDrizzle => String::from(""),
        WheatherCode::LightRain
        | WheatherCode::ModerateRain
        | WheatherCode::HeavyIntensityRain
        | WheatherCode::VeryHeavyRain
        | WheatherCode::ExtremeRain
        | WheatherCode::FreezingRain
        | WheatherCode::LightIntensityShowerRain
        | WheatherCode::ShowerRain
        | WheatherCode::HeavyIntensityShowerRain
        | WheatherCode::RaggedShowerRain => String::from(""),
        WheatherCode::LightSnow
        | WheatherCode::Snow
        | WheatherCode::HeavySnow
        | WheatherCode::Sleet => String::from(""),
        WheatherCode::LightShowerSleet | WheatherCode::ShowerSleet => String::from(""),
        WheatherCode::LightRainAndSnow
        | WheatherCode::RainAndSnow
        | WheatherCode::LightShowerSnow
        | WheatherCode::ShowerSnow
        | WheatherCode::HeavyShowerSnow => String::from(""),
        WheatherCode::Mist => String::from(""),
        WheatherCode::Smoke => String::from(""),
        WheatherCode::Haze => String::from(""),
        WheatherCode::SandDustWhirls => String::from(""),
        WheatherCode::Fog => String::from(""),
        WheatherCode::Sand => String::from(""),
        WheatherCode::Dust => String::from(""),
        WheatherCode::Ash => String::from(""),
        WheatherCode::Squall => String::from(""),
        WheatherCode::Tornado => String::from("󰼸"),
        WheatherCode::Clear => String::from("󰖙"),
        WheatherCode::FewClouds => String::from("󰖕"),
        WheatherCode::ScatteredClouds => String::from(""),
        WheatherCode::BrokenClouds => String::from(""),
        WheatherCode::OvercastClouds => String::from(""),
        _ => String::from(""),
    }
}

fn get_icon_night(code: &WheatherCode) -> String {
    match code {
        WheatherCode::ThunderstormWithLightRain
        | WheatherCode::ThunderstormWithRain
        | WheatherCode::ThunderstormWithHeavyRain => String::from(""),
        WheatherCode::LightThunderstorm
        | WheatherCode::Thunderstorm
        | WheatherCode::HeavyThunderstorm
        | WheatherCode::RaggedThunderstorm => String::from(""),
        WheatherCode::ThunderstormWithLightDrizzle
        | WheatherCode::ThunderstormWithDrizzle
        | WheatherCode::ThunderstormWithHeavyDrizzle => String::from(""),
        WheatherCode::LightIntensityDrizzle
        | WheatherCode::Drizzle
        | WheatherCode::HeavyIntensityDrizzle
        | WheatherCode::LightIntensityDrizzleRain
        | WheatherCode::DrizzleRain
        | WheatherCode::HeavyIntensityDrizzleRain
        | WheatherCode::ShowerRainAndDrizzle
        | WheatherCode::HeavyShowerRainAndDrizzle
        | WheatherCode::ShowerDrizzle => String::from(""),
        WheatherCode::LightRain
        | WheatherCode::ModerateRain
        | WheatherCode::HeavyIntensityRain
        | WheatherCode::VeryHeavyRain
        | WheatherCode::ExtremeRain
        | WheatherCode::FreezingRain
        | WheatherCode::LightIntensityShowerRain
        | WheatherCode::ShowerRain
        | WheatherCode::HeavyIntensityShowerRain
        | WheatherCode::RaggedShowerRain => String::from(""),
        WheatherCode::LightSnow | WheatherCode::Snow | WheatherCode::HeavySnow => String::from(""),
        WheatherCode::Sleet | WheatherCode::LightShowerSleet | WheatherCode::ShowerSleet => {
            String::from("")
        }
        WheatherCode::LightRainAndSnow
        | WheatherCode::RainAndSnow
        | WheatherCode::LightShowerSnow
        | WheatherCode::ShowerSnow
        | WheatherCode::HeavyShowerSnow => String::from(""),
        WheatherCode::Mist => String::from(""),
        WheatherCode::Smoke => String::from(""),
        WheatherCode::Haze => String::from(""),
        WheatherCode::SandDustWhirls => String::from(""),
        WheatherCode::Fog => String::from(""),
        WheatherCode::Sand => String::from(""),
        WheatherCode::Dust => String::from(""),
        WheatherCode::Ash => String::from(""),
        WheatherCode::Squall => String::from(""),
        WheatherCode::Tornado => String::from("󰼸"),
        WheatherCode::Clear => String::from("󰖔"),
        WheatherCode::FewClouds => String::from("󰼱"),
        WheatherCode::ScatteredClouds => String::from(""),
        WheatherCode::BrokenClouds => String::from(""),
        WheatherCode::OvercastClouds => String::from(""),
        _ => String::from(""),
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
    dt: i64,
    #[serde(default)]
    sunrise: i64,
    #[serde(default)]
    sunset: i64,
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

fn get_icon(api_data: &APIResponse) -> String {
    let current_time = Local.timestamp_opt(api_data.current.dt, 0).unwrap();
    let sunrise = Local.timestamp_opt(api_data.current.sunrise, 0).unwrap();
    let sunset = Local.timestamp_opt(api_data.current.sunset, 0).unwrap();
    let is_day: bool = (current_time - sunrise >= TimeDelta::seconds(0))
        && (current_time - sunset < TimeDelta::seconds(0));
    let icon: String;
    if is_day {
        icon = get_icon_day(&api_data.current.weather[0].code);
    } else {
        icon = get_icon_night(&api_data.current.weather[0].code);
    }

    icon
}

fn get_temp_range_color(temp: f64) -> String {
    if temp < 0.0 {
        String::from("\x1b[46;30m")
    } else if temp < 28.0 {
        String::from("\x1b[42;30m")
    } else if temp < 35.0 {
        String::from("\x1b[43;30m")
    } else {
        String::from("\x1b[41;30m")
    }
}

fn print_short_weather(api_data: APIResponse) {
    println!(
        "\x1b[47;30m {} {} \x1b[0m {} {}󰔄 \x1b[0m \x1b[30;43m {} \x1b[0m \x1b[30;44m {} \x1b[0m",
        get_icon(&api_data),
        api_data.current.weather[0].description,
        get_temp_range_color(api_data.current.temp),
        api_data.current.temp,
        Local
            .timestamp_opt(api_data.current.sunrise, 0)
            .unwrap()
            .format("%H:%M"),
        Local
            .timestamp_opt(api_data.current.sunset, 0)
            .unwrap()
            .format("%H:%M")
    )
}

fn main() -> Result<(), ureq::Error> {
    let exe_path = env::current_exe().unwrap();
    let parent_dir = exe_path.parent().unwrap();

    let lat = env::args().nth(1).unwrap_or(String::from("0.0"));
    let lon = env::args().nth(2).unwrap_or(String::from("0.0"));
    //    let api_key = env::args().nth(3).unwrap_or(String::from("0.0"));
    //
    let api_key_path_abs = format!("{}/api_key.txt", parent_dir.display());
    let api_key_path_rel = format!("./api_key.txt");

    let api_key = fs::read_to_string(api_key_path_abs)
        .unwrap_or(fs::read_to_string(api_key_path_rel).unwrap_or_default());
    let api_key_sanitized = api_key.replace("\n", "");
    let mut config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build();

    let agent: Agent = config.into();
    let exclude_str = "hourly,daily,minutely,alerts";

    let req_url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={lat}&lon={lon}&exclude={exclude_str}&units=metric&appid={api_key_sanitized}"
    );

    let mut body = agent
        .get(req_url)
        .call()?
        .body_mut()
        .read_json::<APIResponse>()?;
    body.current.weather[0].code = body.current.weather[0].id.into();

    print_short_weather(body);

    Ok(())
}
