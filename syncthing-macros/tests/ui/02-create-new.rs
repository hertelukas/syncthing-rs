use syncthing_macros::New;

#[derive(New)]
pub struct Device {
    pub device_id: String,
    pub foo: u32,
    pub opt: Option<()>,
}

fn main() {
    let _ = NewDevice::new();
}
