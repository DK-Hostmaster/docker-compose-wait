#![feature(conservative_impl_trait)]

pub mod env_reader;
pub mod sleeper;
pub mod tcp;

pub struct Config {
    pub hosts: String,
    pub timeout : u64,
    pub wait_before : u64,
    pub wait_after : u64
}

pub fn wait(sleep: &sleeper::Sleeper, config: &Config, on_timeout : &mut FnMut() ) {

    if config.wait_before > 0 {
        println!("Waiting {} seconds before checking for hosts availability", config.wait_before);
        sleep.sleep(config.wait_before);
    }

    if !config.hosts.trim().is_empty() {
        let mut count = 0;
        //let start = Instant::now();
        for host in config.hosts.trim().split(',') {
            println!("Checking availability of {}", host);
            while !tcp::is_reachable(&host.trim().to_string()) {
                println!("Host {} not yet availabile", host);
                count = count + 1;
                if count > config.timeout {
                //if (start.elapsed().as_secs() > wait_timeout) {
                    println!("Timeout! After {} seconds some hosts are still not reachable", config.timeout);
                    on_timeout();
                    return;
                }
                sleep.sleep(1);
            }
            println!("Host {} is now availabile", host);
        }
    }

    if config.wait_after > 0 {
        println!("Waiting {} seconds after hosts availability", config.wait_after);
        sleep.sleep(config.wait_after);
    }
}

pub fn config_from_env() -> Config {
    Config {
        hosts: env_reader::env_var(&"WAIT_HOSTS".to_string(), "".to_string()),
        timeout: to_int(env_reader::env_var(&"WAIT_HOSTS_TIMEOUT".to_string(), "".to_string()), 30),
        wait_before: to_int(env_reader::env_var(&"WAIT_BEFORE_HOSTS".to_string(), "".to_string()), 0),
        wait_after: to_int(env_reader::env_var(&"WAIT_AFTER_HOSTS".to_string(), "".to_string()), 0),
    }
}

fn to_int(number: String, default : u64) -> u64 {
    match number.parse::<u64>() {
        Ok(value) => value,
        Err(_e) => default
    }
}

#[cfg(test)]
mod test {

    use std::env;
    use super::*;

    #[test]
    fn should_return_int_value() {
        let value = to_int("32".to_string(), 0);
        assert!(32 == value)
    }

    #[test]
    fn should_return_zero_when_negative_value() {
        let value = to_int("-32".to_string(), 10);
        assert!(10 == value)
    }

    #[test]
    fn should_return_zero_when_Invalid_value() {
        let value = to_int("hello".to_string(), 0);
        assert!(0 == value)
    }

    #[test]
    fn should_return_zero_when_empty_value() {
        let value = to_int("".to_string(), 11);
        assert!(11 == value)
    }

/*
    #[test]
    fn default_timeout_should_be_30() {
        set_env("", "", "10o", "10");
        let config = config_from_env();
        assert_eq!("".to_string(), config.hosts);
        assert_eq!(30, config.timeout);
        assert_eq!(0, config.wait_before);
        assert_eq!(10, config.wait_after);
    }
*/

    #[test]
    fn should_get_config_values_from_env() {
        set_env("localhost:1234", "", "2", "3");
        let config = config_from_env();
        assert_eq!("localhost:1234".to_string(), config.hosts);
        assert_eq!(30, config.timeout);
        assert_eq!(2, config.wait_before);
        assert_eq!(3, config.wait_after);
    }

    fn set_env(hosts: &str, timeout: &str, before: &str, after: &str) {
        env::set_var("WAIT_BEFORE_HOSTS", before.to_string());
        env::set_var("WAIT_AFTER_HOSTS", after.to_string());
        env::set_var("WAIT_HOSTS_TIMEOUT", timeout.to_string());
        env::set_var("WAIT_HOSTS", hosts.to_string());
    }
}
