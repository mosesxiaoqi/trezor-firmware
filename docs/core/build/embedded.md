# Build instructions for Embedded (ARM port)
# 嵌入式（ARM 端口）构建说明


First, clone the repository and initialize the submodules as defined [here](index.md).
首先，克隆仓库并按照[这里](index.md)定义的方式初始化子模块。

Then, you need to install all necessary requirements.
然后，您需要安装所有必要的依赖项。

## Requirements
## 依赖项

The recommended way to control the requirements across all systems is to install **nix-shell**, which automatically installs all requirements in an isolated environment using the `shell.nix` configuration file located in the repository root.
跨所有系统控制依赖项的推荐方法是安装 **nix-shell**，它使用位于仓库根目录的 `shell.nix` 配置文件，在隔离环境中自动安装所有依赖项。

To install nix-shell, follow the instructions [here](https://nix.dev/manual/nix/2.18/installation/installing-binary).
要安装 nix-shell，请按照[这里](https://nix.dev/manual/nix/2.18/installation/installing-binary)的说明进行操作。

Once nix-shell is installed, go to the **repository root** and run:
一旦安装了 nix-shell，前往**仓库根目录**并运行：

```sh
nix-shell
```

### Working with Developer Tools
### 使用开发者工具

If you need to work with embedded development tools such as OpenOCD, gcc-arm-embedded, gdb, etc., you can run nix-shell with the following argument to enable additional development tools:
如果您需要使用嵌入式开发工具，如 OpenOCD、gcc-arm-embedded、gdb 等，您可以使用以下参数运行 nix-shell 来启用额外的开发工具：

```sh
nix-shell --arg devTools true
```

### Manual Requirements Installation
### 手动安装依赖项

If you prefer to install the requirements manually, look into the shell.nix file where you can find a list of requirements with versions.
如果您更喜欢手动安装依赖项，可以查看 shell.nix 文件，其中包含带版本号的依赖项列表。

## Python Dependencies
## Python 依赖项

All Python dependencies and packages are handled with Poetry. If you work in nix-shell, Poetry will be installed automatically. Then, you can install the dependencies and run the Poetry shell in the repository root.
所有 Python 依赖项和包都通过 Poetry 处理。如果您在 nix-shell 中工作，Poetry 将会自动安装。然后，您可以在仓库根目录安装依赖项并运行 Poetry shell。

```sh
poetry install
poetry shell
```

**Note: The recommended way of initializing your environment is to first run nix-shell and then initialize the Poetry shell within it.**
**注意：初始化环境的推荐方式是先运行 nix-shell，然后在其中初始化 Poetry shell。**

## Protobuf Compiler
## Protobuf 编译器

The protocol buffer compiler `protoc` is needed to (unsurprisingly) compile protocol buffer files. [Follow the installation instructions for your system](https://grpc.io/docs/protoc-installation/).
协议缓冲区编译器 `protoc` 是编译协议缓冲区文件所必需的。[请按照您系统的安装说明进行操作](https://grpc.io/docs/protoc-installation/)。

## Rust

Install the appropriate target with [`rustup`](https://rustup.rs/):
使用 [`rustup`](https://rustup.rs/) 安装适当的目标平台：

```sh
rustup target add thumbv7em-none-eabihf  # for TT
rustup target add thumbv7em-none-eabihf  # 用于 TT（Trezor T）
rustup target add thumbv7m-none-eabi     # for T1
rustup target add thumbv7m-none-eabi     # 用于 T1（Trezor One）
```

## Building
## 构建

```sh
make vendor build_boardloader build_bootloader build_firmware
```

## Uploading
## 上传

Use `make upload` to upload the firmware to a production device.
使用 `make upload` 将固件上传到正式设备。

* For TT: Do not forget to [enter bootloader](https://www.youtube.com/watch?v=3hes1H4qRbw) on the device beforehand.
* 对于 TT（Trezor T）：事先不要忘记让设备[进入引导加载程序模式]
(https://www.youtube.com/watch?v=3hes1H4qRbw)。


* For TS3: You will have to [unlock bootloader](https://trezor.io/learn/a/unlocking-the-bootloader-on-trezor-safe-3) first. Make sure to read the link in completeness for potentially unwanted effects.
* 对于 TS3（Trezor Safe 3）：您需要先[解锁引导加载程序](https://trezor.io/learn/a/unlocking-the-bootloader-on-trezor-safe-3)。请务必完整阅读链接内容，了解可能的不良影响。

## Flashing
## 刷写

For flashing firmware to blank device (without bootloader) use `make flash`.
You need to have OpenOCD installed.
要将固件刷写到空白设备（没有引导加载程序的设备）上，请使用 `make flash`。
您需要安装 OpenOCD。

## Building in debug mode
## 在调试模式下构建

You can also build firmware in debug mode to see log output or run tests.
您还可以在调试模式下构建固件，以查看日志输出或运行测试。

```sh
PYOPT=0 make build_firmware
```

To get a full debug build, use:
要获得完整的调试构建，请使用：

```sh
make build_firmware BITCOIN_ONLY=0 PYOPT=0
```

Use `screen` to enter the device's console. Do not forget to add your user to the `dialout` group or use `sudo`. Note that both the group and the tty name can differ, use `ls -l /dev/tty*` or `ls /dev/tty* | grep usb` to find out proper names on your machine.
使用 `screen` 进入设备控制台。不要忘记将您的用户添加到 `dialout` 组或使用 `sudo`。请注意，组名和 tty 名称可能有所不同，使用 `ls -l /dev/tty*` 或 `ls /dev/tty* | grep usb` 在您的机器上找出正确的名称。

```sh
screen /dev/ttyACM0
```
