use syncthing_macros::New;

#[derive(New)]
pub struct Device {
    #[required]
    pub device_id: String,
    pub foo: u32,
    pub opt: Option<()>,
}

fn main() {
    let mut device = NewDevice::new("foo".to_string());
    device.device_id("bar".to_string());
    device.foo(12);
    device.opt(Some(()));
}
