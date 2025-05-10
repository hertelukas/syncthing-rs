use syncthing_macros::New;

#[derive(New)]
pub struct Device {
    #[required]
    pub device_id: String,
    pub foo: u32,
    pub opt: Option<()>,
}

fn main() {
    let device = NewDevice::new("foo".to_string())
        .device_id("bar".to_string())
        .foo(12)
        .opt(Some(()));

    assert_eq!(device.foo, Some(12));
    assert_eq!(&device.device_id, "bar");
}
