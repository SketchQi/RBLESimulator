use crate::constant::peripheral_define::PeripheralDefine;
use blew::peripheral::AdvertisingConfig;
use blew::{BlewError, Peripheral};
use uuid::Uuid;

pub struct GattServer {
    peripheral: Option<Peripheral>,
}

impl GattServer {
    /// 先造一个空壳：此时还没有蓝牙外设
    pub fn new() -> Self {
        Self { peripheral: None }
    }

    /// 之后再初始化蓝牙、注册服务、开始广播
    pub async fn create(&mut self) -> Result<(), BlewError> {
        let p: Peripheral = Peripheral::new().await?;
        for item in PeripheralDefine::get_services() {
            p.add_service(&item).await?;
        }
        p.start_advertising(&AdvertisingConfig {
            local_name: "sketch".to_string(),
            service_uuids: vec![Uuid::new_v4()],
        })
        .await?;
        self.peripheral = Some(p);
        Ok(())
    }
}

impl Default for GattServer {
    fn default() -> Self {
        Self::new()
    }
}
