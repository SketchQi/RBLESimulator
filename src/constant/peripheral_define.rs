use blew::gatt::{AttributePermissions, CharacteristicProperties, GattCharacteristic, GattService};
use uuid::Uuid;

pub struct PeripheralDefine;

impl PeripheralDefine {
    /// 固定的服务/特征 UUID（真机才能稳定识别）
    pub const SERVICE_UUID: Uuid = Uuid::from_u128(0x626c_6577_0000_0000_0000_0000_0000_0000);
    pub const PRIMARY_SERVICE_UUID: Uuid =
        Uuid::from_u128(0x626c_6577_0000_0000_0000_0000_0000_0001);
    pub const ECHO_CHAR_UUID: Uuid = Uuid::from_u128(0x626c_6577_0000_0000_0000_0000_0000_0003);

    pub const ADVERTISING_NAME: &str= "";

    pub fn get_services() -> Vec<GattService> {
        vec![GattService {
            uuid: Self::PRIMARY_SERVICE_UUID,
            primary: true,
            characteristics: vec![GattCharacteristic {
                uuid: Self::ECHO_CHAR_UUID,
                properties: CharacteristicProperties::READ
                    | CharacteristicProperties::WRITE
                    | CharacteristicProperties::NOTIFY,
                permissions: AttributePermissions::READ | AttributePermissions::WRITE,
                value: vec![],
                descriptors: vec![],
            }],
        }]
    }
}
