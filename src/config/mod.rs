use self::constants::RG_DEFAULT_PORT;
use self::constants::RG_DEFAULT_LISTEN_ADDR;

mod constants;

pub struct RgConfigProvider {}

fn from_env_or<T: std::str::FromStr>(env_name: &str, default_value: T) -> T {
    if let Ok(it) = std::env::var(env_name) {
        if let Ok(it) = it.parse::<T>() {
            return it
        }
    }
    default_value
}

fn from_env_str_or(env_name: &str, default_value: &'static str) -> String {
    if let Ok(it) = std::env::var(env_name) {
        return it;
    }
    default_value.to_string()
}


impl RgConfigProvider {
    pub fn new() -> RgConfigProvider {
        RgConfigProvider{}
    }

    pub fn get_port(&self) -> u16 {
        from_env_or("RG_SERVER_PORT", RG_DEFAULT_PORT)
    }

    pub fn get_listen_addr(&self) -> String {
        from_env_str_or("RG_SERVER_ADDR", RG_DEFAULT_LISTEN_ADDR)
    }
    
}