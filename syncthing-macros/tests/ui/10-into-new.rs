use serde::Serialize;
use syncthing_macros::New;

#[derive(New, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[required]
    pub device_id: String,
    pub foo: u32,
}

fn main() {
    let device = Device {
        device_id: "id".to_string(),
        foo: 12,
    };

    let new_device: NewDevice = device.into();

    assert_eq!(&new_device.device_id, "id");
    assert_eq!(new_device.foo, Some(12));
}
