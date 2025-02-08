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

## 提示
```rst
As of version 0.7, the **.PCD** file format uses three different modes for storing data:

* in **ASCII** form, with each point on a new line::

    p_1
    p_2
    p_3
    p_4
    ...

    p_n

.. note::

  Starting with PCL version 1.0.1 the string representation for NaN is "nan".

* in **binary** form, where the data is a complete memory copy of the
  `pcl::PointCloud.points` array/vector. On Linux systems, we use `mmap`/`munmap`
  operations for the fastest possible read/write access to the data.

* in **binary_compressed** form. The body (everything after the header) starts with a 32 bit unsigned binary number which specifies the size in bytes of the data in *compressed* form. Next is another 32 bit unsigned binary number which specifies the size in bytes of the data in *uncompressed* form. Then follows the compressed data. The compression and decompression is done using Marc Lehmann's LZF algorithm. It is mediocre in terms of size reduction, but very fast. For typical point clouds, the compressed data has 30 to 60 percent of the original size. Before compressing, the data is reordered to improve compression, from the standard array-of-structures layout to a structure-of-arrays layout. So for example a cloud with three points and fields x, y, z would be reordered from xyzxyzxyz to xxxyyyzzz.
```