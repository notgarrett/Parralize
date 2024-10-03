use serde_json::Serializer;

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
