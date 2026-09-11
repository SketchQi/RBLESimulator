mod constant;
mod service;

use crate::service::gatt_server::GattServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 先得到空壳（不需要 await，也不需要 mut 来 new）
    let mut server = GattServer::new();

    // 2. 异步初始化蓝牙 + 注册服务 + 开始广播
    server.create().await?;
    println!("正在广播 \"sketch\"，按 Ctrl-C 退出…");

    // 3. 别让 main 立刻结束，否则广播随之停止
    tokio::signal::ctrl_c().await?;
    println!("收到退出信号，结束。");
    Ok(())
}
