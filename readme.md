<div align="center">

# SQLx PostgreSQL UINT128 扩展支持
**为 SQLx 添加 PostgreSQL `pg-uint128` 扩展的无符号整数类型支持**

</div>

## 功能说明

本扩展为 [SQLx](https://github.com/launchbadge/sqlx) 添加了对 PostgreSQL [`pg-uint128`](https://github.com/pg-uint/pg-uint128) 扩展的完整支持：

- ✅ 支持所有无符号整数类型读取转换：
  ```rust
  u8, u16, u32, u64, u128, usize
  ```
- ⚡ 自动处理数据库的 `uint128` 类型与 Rust 原生类型转换
- 🛡️ 包含边界检查防止数据溢出

## 快速使用

### 添加依赖
```toml
[dependencies]
sqlx = { version = "*", features = ["postgres"] }
sqlx-pg-ext-uint = "0.1"
```

### 代码示例
TODO

## 运行示例
1. 创建数据库并启用 `pg_uint128` 扩展
2. 配置数据库连接字符串
3. 执行 `cargo run`

## 技术说明
- 支持直接读取为 Rust 原生整数类型
- 自动验证数值范围（如 `u8` 值超过 255 会报错）
- `usize` 类型会根据目标平台自动处理

## 许可证
<!--MIT 许可证 - 详见 [LICENSE](LICENSE)-->
