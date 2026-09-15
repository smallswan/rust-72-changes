use s3::{Bucket, creds::Credentials, region::Region};
use std::collections::HashMap;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 配置 RustFS 凭证和端点
    let access_key = "rustfsadmin";
    let secret_key = "rustfsadmin";
    let endpoint = "http://localhost:9000"; // 替换为你的 RustFS 地址
    let bucket_name = "linduo";
    let region = "us-east-1"; // RustFS 通常使用 us-east-1

    // 2. 创建 Credentials 实例
    let credentials =
        Credentials::new(Some(access_key), Some(secret_key), None, None, None).unwrap();

    // 3. 创建 Bucket 实例，并配置为路径风格 (Path-Style)
    // RustFS 通常需要使用路径风格，而非虚拟主机风格
    let mut bucket = Bucket::new(
        bucket_name,
        Region::Custom {
            region: region.to_string(),
            endpoint: endpoint.to_string(),
        },
        credentials,
    )?
    .with_path_style();

    // --- 生成 GET 预签名 URL (用于下载/查看) ---
    let object_key = "elephant.png";
    let expires_in_seconds = 300; // 5分钟后过期

    // presign_get 方法用于生成 GET 请求的预签名 URL[reference:4]
    let presigned_get_url = bucket
        .presign_get(object_key, expires_in_seconds, None)
        .await?;
    println!("生成的GET预签名URL (用于下载): \n{}", presigned_get_url);

    // --- 生成 PUT 预签名 URL (用于上传) ---
    // presign_put 方法用于生成 PUT 请求的预签名 URL[reference:5]
    let presigned_put_url = bucket
        .presign_put(object_key, expires_in_seconds, None, None)
        .await?;
    println!("生成的PUT预签名URL (用于上传): \n{}", presigned_put_url);

    Ok(())
}
