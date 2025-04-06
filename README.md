# Trezor Firmware

![img](https://repository-images.githubusercontent.com/180590388/968e6880-6538-11e9-9da6-4aef78157e94)

## Repository Structure

* **[`ci`](ci/)**: [Gitlab CI](https://gitlab.com/satoshilabs/trezor/trezor-firmware) configuration files
* **[`common/defs`](common/defs/)**: JSON 币种定义和支持表
* **[`common/protob`](common/protob/)**: Trezor 协议的通用 protobuf 定义
* **[`common/tools`](common/tools/)**: 管理币种定义和相关数据的工具
* **[`core`](core/)**: Trezor Core，Trezor T 的固件实现
* **[`crypto`](crypto/)**: 独立的加密库，供 Trezor Core 和 Trezor One 固件使用
* **[`docs`](docs/)**: 各种文档
* **[`legacy`](legacy/)**: Trezor One 固件实现
* **[`python`](python/)**: Python [客户端库](https://pypi.org/project/trezor) 和 `trezorctl` 命令
* **[`storage`](storage/)**: NORCOW 存储实现，供 Trezor Core 和 Trezor One 固件使用
* **[`tests`](tests/)**: 固件单元测试套件
* **[`tools`](tools/)**: 各种构建和辅助脚本
* **[`vendor`](vendor/)**: 外部依赖的子模块


## Contribute

See [CONTRIBUTING.md](docs/misc/contributing.md).

强烈建议使用 [Conventional Commits](COMMITS.md)，未来可能会强制执行。

在贡献之前，请查看 `docs` 文件夹中的文档或访问 [docs.trezor.io](https://docs.trezor.io)。特别是 [misc](docs/misc/index.md) 章节，因为它包含一些有用的知识。

## Security vulnerability disclosure

请私下报告怀疑的安全漏洞至 [security@satoshilabs.com](mailto:security@satoshilabs.com)，也请参阅 [Trezor.io 网站上的披露部分](https://trezor.io/support/a/how-to-report-a-security-issue)。请不要为怀疑的安全漏洞创建公开可见的问题。

## Documentation

请参阅 `docs` 文件夹或访问 [docs.trezor.io](https://docs.trezor.io)。
