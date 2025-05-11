use serde_json::to_value;
use serde::Serialize;
use syncthing_macros::New;

#[derive(New, Serialize)]
pub struct Device {
    #[required]
    pub device_id: String,
    #[serde(rename = "bar")]
    pub foo: u32,
    pub opt: Option<()>,
}

fn main() {
    let device = NewDevice::new("foo".to_string())
        .device_id("bar".to_string())
        .foo(12)
        .opt(Some(()));

    let value = to_value(&device).unwrap();
    let obj = value.as_object().unwrap();

    assert_eq!(obj.get("device_id").unwrap(), "bar");
    assert!(
        obj.get("bar").is_some(),
        "Field 'foo' should be renamed to 'bar'"
    );
    assert_eq!(obj.get("bar").unwrap(), 12);
}
