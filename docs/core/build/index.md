# Build
# 构建

_Building for Trezor Model One? See the [legacy](../../legacy/index.md) documentation._
_构建 Trezor Model One？请参阅 [legacy](../../legacy/index.md) 文档。_

## New Project
## 新项目

Run the following to checkout the project:
运行以下命令检出项目：

```sh
git clone --recurse-submodules https://github.com/trezor/trezor-firmware.git
cd trezor-firmware
poetry install
cd core
```

After this you will need to install some software dependencies based on what flavor
of Core you want to build. You can either build the Emulator or the actual firmware
running on ARM devices. Emulator (also called _unix_ port) is a unix version that can
run on your computer. See [Emulator](../emulator/index.md) for more information.
之后，您需要根据想要构建的 Core 类型安装一些软件依赖项。您可以构建模拟器或在 ARM 设备上运行的实际固件。模拟器（也称为 _unix_ 端口）是可以在您计算机上运行的 unix 版本。有关更多信息，请参阅 [模拟器](../emulator/index.md)。

## Existing Project
## 现有项目

If you are building from an existing checkout, do not forget to refresh the submodules
 and the poetry environment:
如果您从现有的检出构建，请不要忘记刷新子模块和 poetry 环境：

```sh
git submodule update --init --recursive --force
poetry install --sync
```

## Poetry


We use [Poetry](https://python-poetry.org/) to install and track Python dependencies. You need to install it, sync the packages and then use `poetry run` for every command or enter `poetry shell` before typing any commands. **The commands in this section suppose you are in a `poetry shell` environment!**
我们使用 [Poetry](https://python-poetry.org/) 来安装和跟踪 Python 依赖项。您需要安装它，同步包，然后对每个命令使用 `poetry run`，或者在输入任何命令之前进入 `poetry shell`。**本节中的命令假设您处于 `poetry shell` 环境中！**

```sh
sudo pip3 install poetry
poetry install
poetry shell
```
