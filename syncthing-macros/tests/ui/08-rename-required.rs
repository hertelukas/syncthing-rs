use serde_json::to_value;
use serde::Serialize;
use syncthing_macros::New;

#[derive(New, Serialize)]
pub struct Device {
    #[required]
    #[serde(rename = "deviceID")]
    pub device_id: String,
}

fn main() {
    let device = NewDevice::new("foo".to_string())
        .device_id("bar".to_string());

    let value = to_value(&device).unwrap();
    let obj = value.as_object().unwrap();

    assert!(
        obj.get("deviceID").is_some(),
        "Field 'device_id' should be renamed 'deviceID'"
    );
    assert_eq!(obj.get("deviceID").unwrap(), "bar");
}
