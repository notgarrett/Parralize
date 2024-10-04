use serde_json::Serializer;

enum Error {
    GenerationFailure,
}

struct Device {
    has_gpu: bool,
    has_cuda: bool,
    in_use: bool,
    port: String,
    strength: i32,
}

pub fn generate_priority_listing() -> std::vec::Vec<Device> {
    todo!()
}

// We want to check if there is a hardware.json file
// if not we will generate one
//

fn generate_hardware_json() -> Result<(), Error> {
    todo!()
}
