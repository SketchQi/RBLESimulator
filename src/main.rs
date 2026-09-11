mod constant;
mod service;
pub mod utils;

use crate::service::gatt_server::GattServer;
use crate::utils::log::log;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log("程序启动");

    // 首次运行系统会弹蓝牙授权框。new() 内部只等 15s，
    // 若没来得及点"允许"会 NotPowered，因此这里循环重试。
    let mut server = loop {
        let p = GattServer::new().await;
        match p {
            Ok(mut s) => {
                s.start().await.expect("TODO: panic message");
                log("授权成功，已开始广播");
                break s;
            }
            Err(e) => {
                log(
                    (&format!("尚未就绪（{e}）。若屏幕上有蓝牙授权框请点“允许”，3 秒后重试…"))
                        .as_ref(),
                );
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    };

    log("广播运行中，按 Ctrl-C 退出…");
    tokio::signal::ctrl_c().await?;
    let _ = server.stop();
    log("正常退出");
    Ok(())
}
