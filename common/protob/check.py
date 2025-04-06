#!/usr/bin/env python3
import os
import re # 正则表达式库
import sys
from glob import glob # 用于查找匹配特定模式的文件路径名

error = False

# 这行代码用于获取当前 Python 脚本文件所在的目录路径，并将其赋值给变量 `MYDIR`。
MYDIR = os.path.dirname(__file__)

# `re.compile()` 函数编译一个正则表达式模式，创建一个可重用的正则表达式对象。
# 这个正则表达式可能用于从文件名中提取消息类别
# 提取(\w+)的内容，(\w+)是一个捕获组
EXPECTED_PREFIX_RE = re.compile(r"messages-(\w+)(?:-.*)?\.proto")

# Checking all protobuf files for their `enum` and `message` declarations
# and making sure their names start with expected prefix
# `sorted(...)` - 对找到的文件路径进行字母顺序排序，确保处理顺序的一致性
for fn in sorted(glob(os.path.join(MYDIR, "messages-*.proto"))):
    '''
        - `"r"`: 读取模式（read）- 文件必须已存在，只允许读取内容，不能写入
        - `"t"`: 文本模式（text）- 将文件内容作为文本处理（而非二进制数据）

        `"rt"` 实际上等同于只写 `"r"`，因为 `"t"` 是默认模式。
        但有时开发者会显式写出 `"rt"` 以增强代码可读性，
        明确表示是以文本模式读取文件。
    '''
    with open(fn, "rt") as f:
        # `.group(1)` - 获取正则表达式中第一个捕获组的内容。
        # 在这个正则表达式中，第一个捕获组是 `(\w+)`
        # `.capitalize()` - 将提取出的字符串首字母大写。例如：
        # - "bitcoin" 变成 "Bitcoin"
        # - "ethereum" 变成 "Ethereum"
        prefix = EXPECTED_PREFIX_RE.search(fn).group(1).capitalize()
        if prefix in ("Bitcoin", "Bootloader", "Common", "Crypto", "Management"):
            continue
        if prefix == "Nem":
            prefix = "NEM"
        elif prefix == "Webauthn":
            prefix = "WebAuthn"
        for line in f:
            line = line.strip().split(" ") # 去除行首尾空白并按空格分割
            if line[0] not in ("enum", "message"):
                continue
            expected_prefixes = (prefix, f"Debug{prefix}")
            if not line[1].startswith(expected_prefixes):
                print("ERROR: protobuf message does not start with expected prefix")
                print(f"    file: {fn}")
                print(f"    message: {line[1]}")
                print(f"    expected prefixes: {expected_prefixes}")
                error = True

if error:
    sys.exit(1)
