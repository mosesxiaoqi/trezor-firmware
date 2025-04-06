# Coin and FIDO Definitions

This directory hosts JSON definitions of recognized coins, tokens, and FIDO/U2F apps.

## FIDO

The [`fido/`](fido) subdirectory contains definitons of apps whose logos and
names are shown on Trezor T screen for FIDO/U2F authentication.
[`fido/`](fido) 子目录包含在 Trezor T 屏幕上显示的 FIDO/U2F 认证应用的标志和名称定义。

Each app must have a single JSON file in the `fido/` subdirectory. Every app must have
its `label` set to the user-recognizable application name. The `u2f` field is a list of
U2F origin hashes, and the `webauthn` field is a list of FIDO2/WebAuthn hostnames for
the app. At least one must be present.
每个应用必须在 `fido/` 子目录中有一个单独的 JSON 文件。每个应用必须设置 `label` 字段为用户可识别的应用名称。`u2f` 字段是一个 U2F 来源哈希列表，`webauthn` 字段是应用的 FIDO2/WebAuthn 主机名列表。至少需要存在一个字段。

Each app can have an icon. If present, it must be a 128x128 pixels RGBA PNG of the same
name as the corresponding JSON name. If the app does not have an icon, it must instead
have a field `no_icon` set to `true` in the JSON.
每个应用可以有一个图标。如果存在，图标必须是一个 128x128 像素的 RGBA PNG 文件，文件名与相应的 JSON 文件名相同。如果应用没有图标，则必须在 JSON 中设置 `no_icon` 字段为 `true`。

## Coins

We currently recognize five categories of coins.
我们目前识别五类币种。

#### `bitcoin`

The [`bitcoin/`](bitcoin) subdirectory contains definitions for Bitcoin and altcoins
based on Bitcoin code.
[`bitcoin/`](bitcoin) 子目录包含比特币及基于比特币代码的山寨币的定义。

Each Bitcoin-like coin must have a single JSON file in the `bitcoin/` subdirectory,
and a corresponding PNG image with the same name. The PNG must be 96x96 pixels and
the picture must be a circle suitable for displaying on Trezor T.
每个比特币类币种必须在 `bitcoin/` 子目录中有一个单独的 JSON 文件，以及一个相应的 PNG 图像，文件名相同。PNG 必须是 96x96 像素，图像必须是适合在 Trezor T 上显示的圆形。

Testnet is considered a separate coin, so it must have its own JSON and icon.
测试网被视为一个独立的币种，因此必须有自己的 JSON 和图标。

We will not support coins that have `address_type` 0, i.e., same as Bitcoin.
我们不支持 `address_type` 为 0 的币种，即与比特币相同的地址类型。

#### `eth` and `erc20`

Definitions for Ethereum chains (networks) and tokens (erc20) are split in two parts:
以太坊链（网络）和代币（erc20）的定义分为两部分：

1. built-in definitions - some of the chain and token definitions are built into the firmware
   image. List of built-in chains is stored in [`ethereum/networks.json`](ethereum/networks.json)
   and tokens in [`ethereum/tokens.json`](ethereum/tokens.json).
1. 内置定义 - 一些链和代币定义内置在固件镜像中。内置链的列表存储在 [`ethereum/networks.json`](ethereum/networks.json) 中，代币存储在 [`ethereum/tokens.json`](ethereum/tokens.json) 中。

2. external definitions - dynamically generated from multiple sources. Whole process is
   described in separate
   [document](https://docs.trezor.io/trezor-firmware/common/ethereum-definitions.html).
2. 外部定义 - 从多个来源动态生成。整个过程在单独的[文档](https://docs.trezor.io/trezor-firmware/common/ethereum-definitions.html)中描述。

We generally do not accept updates to the built-in definitions. Instead, make sure your
network or token is included in the external definitions. A good place to start is the
[`ethereum-lists` GitHub organization](https://gitub.com/ethereum-lists): add your token
to the [tokens](https://github.com/ethereum-lists/tokens) repository, or your EVM chain to the
[chains](https://github.com/ethereum-lists/chains) repository.
我们通常不接受对内置定义的更新。相反，请确保您的网络或代币包含在外部定义中。一个好的起点是 [`ethereum-lists` GitHub 组织](https://github.com/ethereum-lists)：将您的代币添加到 [tokens](https://github.com/ethereum-lists/tokens) 仓库，或将您的 EVM 链添加到 [chains](https://github.com/ethereum-lists/chains) 仓库。

#### `nem`

The file [`nem/nem_mosaics.json`](nem/nem_mosaics.json) describes NEM mosaics.
文件 [`nem/nem_mosaics.json`](nem/nem_mosaics.json) 描述了 NEM 马赛克。

#### `misc`

Supported coins that are not derived from Bitcoin, Ethereum or NEM are currently grouped
and listed in separate file [`misc/misc.json`](misc/misc.json). Each coin must also have
an icon in `misc/<short>.png`, where `short` is lowercased `shortcut` field from the JSON.
不派生自比特币、以太坊或 NEM 的支持币种目前分组并列在单独的文件 [`misc/misc.json`](misc/misc.json) 中。每个币种还必须在 `misc/<short>.png` 中有一个图标，其中 `short` 是 JSON 中 `shortcut` 字段的小写形式。

### Keys

Throughout the system, coins are identified by a _key_ - a colon-separated string
generated from the coin's type and shortcut:

* for Bitcoin-likes, key is `bitcoin:<shortcut>`
* for Ethereum networks, key is `eth:<shortcut>:<chain_id>`
* for ERC20 tokens, key is `erc20:<chain_symbol>:<token_shortcut>`
* for NEM mosaic, key is `nem:<shortcut>`
* for others, key is `misc:<shortcut>`

If a token shortcut has a suffix, such as `CAT (BlockCat)`, the whole thing is part
of the key (so the key is `erc20:eth:CAT (BlockCat)`).
在整个系统中，币种通过一个 _键_ 进行标识 - 一个由币种类型和快捷方式生成的冒号分隔字符串：

* 对于比特币类，键是 `bitcoin:<shortcut>`
* 对于以太坊网络，键是 `eth:<shortcut>:<chain_id>`
* 对于 ERC20 代币，键是 `erc20:<chain_symbol>:<token_shortcut>`
* 对于 NEM 马赛克，键是 `nem:<shortcut>`
* 对于其他币种，键是 `misc:<shortcut>`

如果代币快捷方式有后缀，例如 `CAT (BlockCat)`，整个字符串都是键的一部分（因此键是 `erc20:eth:CAT (BlockCat)`）。

Duplicate keys are not allowed and coins that would result in duplicate keys cannot be
added to the dataset.
不允许重复键，导致重复键的币种不能添加到数据集中。


# Support Information

We keep track of support status of each built-in coin over our devices. That is `T1B1`
for Trezor One, `T2T1` for Trezor T, `T2B1` and `T3B1` for Trezor Safe 3 (both models
should have identical entries, except for minimum versions which are higher on `T3B1`),
`T3T1` for Trezor Safe 5.
我们跟踪每个内置币种在我们设备上的支持状态。`T1B1` 代表 Trezor One，`T2T1` 代表 Trezor T，`T2B1` 和 `T3B1` 代表 Trezor Safe 3（两个型号应有相同的条目，除了 `T3B1` 的最低版本更高），`T3T1` 代表 Trezor Safe 5。

This information is stored in [`support.json`](support.json).
此信息存储在 [`support.json`](support.json) 中。

External contributors should not touch this file unless asked to.
外部贡献者不应修改此文件，除非被要求。

Each coin on each device can be in one of four support states:
每个设备上的每个币种可以处于四种支持状态之一：

* **supported** explicitly: coin's key is listed in the device's `supported`
  dictionary. If it's a Trezor device, it contains the firmware version from which
  it is supported. For connect and suite, the value is simply `true`.
* **明确支持**：币种的键列在设备的 `supported` 字典中。如果是 Trezor 设备，包含支持的固件版本。对于 connect 和 suite，值只是 `true`。

* **unsupported** explicitly: coin's key is listed in the device's `unsupported`
  dictionary. The value is a string with reason for not supporting.
  For connect and suite, if the key is not listed at all, it is also considered unsupported.
  ERC20 tokens detected as duplicates are also considered unsupported.
* **明确不支持**：币种的键列在设备的 `unsupported` 字典中。值是一个字符串，说明不支持的原因。对于 connect 和 suite，如果键根本没有列出，也被视为不支持。检测到重复的 ERC20 代币也被视为不支持。

* **unknown**: coin's key is not listed at all.
* **未知**：币种的键根本没有列出。

_Supported_ coins are used in code generation (i.e., included in built firmware).
_支持_ 的币种用于代码生成（即包含在构建的固件中）。

_Unsupported_ and _unknown_ coins are excluded from code generation.
_不支持_ 和 _未知_ 的币种被排除在代码生成之外。

You can edit `support.json` manually, but it is usually better to use the `support.py` tool.
您可以手动编辑 `support.json`，但通常最好使用 `support.py` 工具。

See [tools docs](../tools) for details.
有关详细信息，请参阅 [tools 文档](../tools)。
