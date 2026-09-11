use blew::gatt::{AttributePermissions, CharacteristicProperties, GattCharacteristic, GattService};

pub struct PeripheralDefine{

}

impl PeripheralDefine {

    pub fn get_services() -> Vec<GattService> {
        let mut vector: Vec<GattService> = Vec::new();
       vector.push(GattService {
           uuid: Default::default(),
           primary: true,
           characteristics: vec![
               GattCharacteristic{
                   uuid: Default::default(),
                   properties: CharacteristicProperties::READ | CharacteristicProperties::WRITE,
                   permissions: AttributePermissions::WRITE | AttributePermissions::READ,
                   value: vec![],
                   descriptors: vec![],
               }
           ],
       });
        vector
    }
}