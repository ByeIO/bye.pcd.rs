# bye_pcd_rs: 从 **PCD** 文件格式读取点云数据

`bye_pcd_rs` 允许你从文件或二进制缓冲区解析 PCD 点云数据。

支持的 PCD 格式: ascii, binary, binary_compressed

## 使用方法

要将此 crate 添加到你的项目中，请运行以下命令：

```bash
cargo add bye_pcd_rs
```
如果需要点云预览功能(不添加feature就不会多编译300+个依赖包😂):
```bash
cargo add bye_pcd_rs --feature viewer
```

请访问 [docs.rs](https://docs.rs/bye_pcd_rs/) 了解更多使用细节。

## 示例

示例代码可以在 `examples` 目录中找到。运行 `cargo run --example` 以列出所有可用的示例二进制文件。

## 许可证

MIT 许可证。请参阅 [LICENSE](LICENSE) 文件。
