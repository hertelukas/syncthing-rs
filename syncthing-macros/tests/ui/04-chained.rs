use syncthing_macros::New;

#[derive(New)]
pub struct Device {
    pub device_id: String,
    pub foo: u32,
    pub opt: Option<()>,
}

fn main() {
    let _device = NewDevice::new()
        .device_id("foo".to_string())
        .foo(12)
        .opt(Some(()));
}
