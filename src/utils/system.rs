#[allow(dead_code)]
pub const COUNT: u32 = 1000000;

#[allow(dead_code)]
pub fn os() -> String {
    if cfg!(target_os = "windows") {
        return "windows".to_string();
    } else if cfg!(target_os = "linux") {
        return "linux".to_string();
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        return "macos".to_string();
    } else {
        return "other".to_string();
    }
}