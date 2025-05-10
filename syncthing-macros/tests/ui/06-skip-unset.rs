use syncthing_macros::New;
use serde_json::to_value;

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
        .opt(Some(()));

    let value = to_value(&device).unwrap();
    let obj = value.as_object().unwrap();

    assert_eq!(obj.get("device_id").unwrap(), "bar");
    assert!(obj.get("foo").is_none(), "Field 'foo' should not be serialized");
    assert!(obj.get("opt").is_some(), "Field 'opt' should be serialized");
}
