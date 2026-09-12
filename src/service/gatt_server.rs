use crate::constant::peripheral_define::PeripheralDefine;
use blew::peripheral::AdvertisingConfig;
use blew::{BlewError, Peripheral};

pub struct GattServer {
    peripheral: Option<Peripheral>,
}

impl GattServer {
    /// 一次性完成：初始化蓝牙 → 注册服务
    /// 失败（如 NotPowered/未授权）时不留下半初始化对象，调用方可直接重试。
    pub async fn new() -> Result<Self, BlewError> {
        let peripheral: Peripheral = Peripheral::new().await?;
        for item in PeripheralDefine::get_services() {
            peripheral.add_service(&item).await?;
        }
        Ok(Self {
            peripheral: Some(peripheral),
        })
    }

    pub async fn start(&mut self) -> Result<(), BlewError> {
        if let Some(p) = self.peripheral.as_ref() {
            p.start_advertising(&AdvertisingConfig {
                local_name: "sketch".to_string(),
                service_uuids:vec![PeripheralDefine::SERVICE_UUID],
            })
            .await?;
        }
        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(p) = self.peripheral.take() {
            let _ = p.stop_advertising().await;
        }
    }
}
