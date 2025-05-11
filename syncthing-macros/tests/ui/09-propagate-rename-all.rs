use serde_json::to_value;
use serde::Serialize;
use syncthing_macros::New;

#[derive(New, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[required]
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub foo_bar: u32,
}

fn main() {
    let device = NewDevice::new("foo".to_string())
        .device_id("bar".to_string())
        .foo_bar(12);

    let value = to_value(&device).unwrap();
    let obj = value.as_object().unwrap();

    assert!(
        obj.get("deviceID").is_some(),
        "Field 'device_id' should be renamed 'deviceID'"
    );
    assert_eq!(obj.get("deviceID").unwrap(), "bar");
    assert!(
        obj.get("fooBar").is_some(),
        "Field 'foo_bar' should be renamed to camelCase: 'fooBar'"
    );
    assert_eq!(obj.get("fooBar").unwrap(), 12);
}
