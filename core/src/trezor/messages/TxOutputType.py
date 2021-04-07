# Automatically generated by pb2py
# fmt: off
import protobuf as p

from .MultisigRedeemScriptType import MultisigRedeemScriptType

if __debug__:
    try:
        from typing import Dict, List, Optional  # noqa: F401
        from typing_extensions import Literal  # noqa: F401
        EnumTypeOutputScriptType = Literal[0, 1, 2, 3, 4, 5]
    except ImportError:
        pass


class TxOutputType(p.MessageType):

    def __init__(
        self,
        *,
        amount: int,
        address_n: Optional[List[int]] = None,
        address: Optional[str] = None,
        script_type: EnumTypeOutputScriptType = 0,
        multisig: Optional[MultisigRedeemScriptType] = None,
        op_return_data: Optional[bytes] = None,
        orig_hash: Optional[bytes] = None,
        orig_index: Optional[int] = None,
    ) -> None:
        self.address_n = address_n if address_n is not None else []
        self.amount = amount
        self.address = address
        self.script_type = script_type
        self.multisig = multisig
        self.op_return_data = op_return_data
        self.orig_hash = orig_hash
        self.orig_index = orig_index

    @classmethod
    def get_fields(cls) -> Dict:
        return {
            1: ('address', p.UnicodeType, None),
            2: ('address_n', p.UVarintType, p.FLAG_REPEATED),
            3: ('amount', p.UVarintType, p.FLAG_REQUIRED),
            4: ('script_type', p.EnumType("OutputScriptType", (0, 1, 2, 3, 4, 5,)), 0),  # default=PAYTOADDRESS
            5: ('multisig', MultisigRedeemScriptType, None),
            6: ('op_return_data', p.BytesType, None),
            10: ('orig_hash', p.BytesType, None),
            11: ('orig_index', p.UVarintType, None),
        }
