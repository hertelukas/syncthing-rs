use syncthing_macros::New;

#[derive(New)]
pub struct Device {
    pub device_id: String,
    pub foo: u32,
    pub opt: Option<()>,
}

fn main() {
    let mut device = NewDevice::new();
    device = device.device_id("foo".to_string());
    device = device.foo(12);
    device = device.opt(Some(()));

    let _ = device;
}
