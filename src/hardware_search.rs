use serde_json::Serializer;

enum Error {
    GenerationFailure,
}

// Most of these values are self explanatory, priority however is measured with lower integers
// representing higher priority. The user will manually set these in correnspondance with compute
// power for each system.
struct Device {
    has_gpu: bool,
    nvidia: bool,
    amd: bool,
    in_use: bool,
    port: String,
    priority: i32,
}

pub fn generate_priority_listing() -> std::vec::Vec<Device> {
    todo!()
}

// We want to check if there is a hardware.json file
// if not we will generate one

fn generate_hardware_json() -> Result<(), Error> {
    todo!()
}
