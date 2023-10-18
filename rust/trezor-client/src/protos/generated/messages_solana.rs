// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 3.19.6
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `messages-solana.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:hw.trezor.messages.solana.SolanaGetPublicKey)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SolanaGetPublicKey {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.solana.SolanaGetPublicKey.address_n)
    pub address_n: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:hw.trezor.messages.solana.SolanaGetPublicKey.show_display)
    pub show_display: ::std::option::Option<bool>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.solana.SolanaGetPublicKey.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SolanaGetPublicKey {
    fn default() -> &'a SolanaGetPublicKey {
        <SolanaGetPublicKey as ::protobuf::Message>::default_instance()
    }
}

impl SolanaGetPublicKey {
    pub fn new() -> SolanaGetPublicKey {
        ::std::default::Default::default()
    }

    // optional bool show_display = 2;

    pub fn show_display(&self) -> bool {
        self.show_display.unwrap_or(false)
    }

    pub fn clear_show_display(&mut self) {
        self.show_display = ::std::option::Option::None;
    }

    pub fn has_show_display(&self) -> bool {
        self.show_display.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show_display(&mut self, v: bool) {
        self.show_display = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "address_n",
            |m: &SolanaGetPublicKey| { &m.address_n },
            |m: &mut SolanaGetPublicKey| { &mut m.address_n },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "show_display",
            |m: &SolanaGetPublicKey| { &m.show_display },
            |m: &mut SolanaGetPublicKey| { &mut m.show_display },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SolanaGetPublicKey>(
            "SolanaGetPublicKey",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SolanaGetPublicKey {
    const NAME: &'static str = "SolanaGetPublicKey";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.address_n)?;
                },
                8 => {
                    self.address_n.push(is.read_uint32()?);
                },
                16 => {
                    self.show_display = ::std::option::Option::Some(is.read_bool()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.address_n {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if let Some(v) = self.show_display {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.address_n {
            os.write_uint32(1, *v)?;
        };
        if let Some(v) = self.show_display {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SolanaGetPublicKey {
        SolanaGetPublicKey::new()
    }

    fn clear(&mut self) {
        self.address_n.clear();
        self.show_display = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SolanaGetPublicKey {
        static instance: SolanaGetPublicKey = SolanaGetPublicKey {
            address_n: ::std::vec::Vec::new(),
            show_display: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SolanaGetPublicKey {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SolanaGetPublicKey").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SolanaGetPublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SolanaGetPublicKey {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:hw.trezor.messages.solana.SolanaPublicKey)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SolanaPublicKey {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.solana.SolanaPublicKey.public_key)
    pub public_key: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.solana.SolanaPublicKey.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SolanaPublicKey {
    fn default() -> &'a SolanaPublicKey {
        <SolanaPublicKey as ::protobuf::Message>::default_instance()
    }
}

impl SolanaPublicKey {
    pub fn new() -> SolanaPublicKey {
        ::std::default::Default::default()
    }

    // required bytes public_key = 1;

    pub fn public_key(&self) -> &[u8] {
        match self.public_key.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_public_key(&mut self) {
        self.public_key = ::std::option::Option::None;
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "public_key",
            |m: &SolanaPublicKey| { &m.public_key },
            |m: &mut SolanaPublicKey| { &mut m.public_key },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SolanaPublicKey>(
            "SolanaPublicKey",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SolanaPublicKey {
    const NAME: &'static str = "SolanaPublicKey";

    fn is_initialized(&self) -> bool {
        if self.public_key.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.public_key = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.public_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.public_key.as_ref() {
            os.write_bytes(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SolanaPublicKey {
        SolanaPublicKey::new()
    }

    fn clear(&mut self) {
        self.public_key = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SolanaPublicKey {
        static instance: SolanaPublicKey = SolanaPublicKey {
            public_key: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SolanaPublicKey {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SolanaPublicKey").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SolanaPublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SolanaPublicKey {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:hw.trezor.messages.solana.SolanaGetAddress)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SolanaGetAddress {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.solana.SolanaGetAddress.address_n)
    pub address_n: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:hw.trezor.messages.solana.SolanaGetAddress.show_display)
    pub show_display: ::std::option::Option<bool>,
    // @@protoc_insertion_point(field:hw.trezor.messages.solana.SolanaGetAddress.chunkify)
    pub chunkify: ::std::option::Option<bool>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.solana.SolanaGetAddress.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SolanaGetAddress {
    fn default() -> &'a SolanaGetAddress {
        <SolanaGetAddress as ::protobuf::Message>::default_instance()
    }
}

impl SolanaGetAddress {
    pub fn new() -> SolanaGetAddress {
        ::std::default::Default::default()
    }

    // optional bool show_display = 2;

    pub fn show_display(&self) -> bool {
        self.show_display.unwrap_or(false)
    }

    pub fn clear_show_display(&mut self) {
        self.show_display = ::std::option::Option::None;
    }

    pub fn has_show_display(&self) -> bool {
        self.show_display.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show_display(&mut self, v: bool) {
        self.show_display = ::std::option::Option::Some(v);
    }

    // optional bool chunkify = 3;

    pub fn chunkify(&self) -> bool {
        self.chunkify.unwrap_or(false)
    }

    pub fn clear_chunkify(&mut self) {
        self.chunkify = ::std::option::Option::None;
    }

    pub fn has_chunkify(&self) -> bool {
        self.chunkify.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunkify(&mut self, v: bool) {
        self.chunkify = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "address_n",
            |m: &SolanaGetAddress| { &m.address_n },
            |m: &mut SolanaGetAddress| { &mut m.address_n },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "show_display",
            |m: &SolanaGetAddress| { &m.show_display },
            |m: &mut SolanaGetAddress| { &mut m.show_display },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "chunkify",
            |m: &SolanaGetAddress| { &m.chunkify },
            |m: &mut SolanaGetAddress| { &mut m.chunkify },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SolanaGetAddress>(
            "SolanaGetAddress",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SolanaGetAddress {
    const NAME: &'static str = "SolanaGetAddress";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.address_n)?;
                },
                8 => {
                    self.address_n.push(is.read_uint32()?);
                },
                16 => {
                    self.show_display = ::std::option::Option::Some(is.read_bool()?);
                },
                24 => {
                    self.chunkify = ::std::option::Option::Some(is.read_bool()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.address_n {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if let Some(v) = self.show_display {
            my_size += 1 + 1;
        }
        if let Some(v) = self.chunkify {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.address_n {
            os.write_uint32(1, *v)?;
        };
        if let Some(v) = self.show_display {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.chunkify {
            os.write_bool(3, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SolanaGetAddress {
        SolanaGetAddress::new()
    }

    fn clear(&mut self) {
        self.address_n.clear();
        self.show_display = ::std::option::Option::None;
        self.chunkify = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SolanaGetAddress {
        static instance: SolanaGetAddress = SolanaGetAddress {
            address_n: ::std::vec::Vec::new(),
            show_display: ::std::option::Option::None,
            chunkify: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SolanaGetAddress {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SolanaGetAddress").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SolanaGetAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SolanaGetAddress {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:hw.trezor.messages.solana.SolanaAddress)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SolanaAddress {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.solana.SolanaAddress.address)
    pub address: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.solana.SolanaAddress.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SolanaAddress {
    fn default() -> &'a SolanaAddress {
        <SolanaAddress as ::protobuf::Message>::default_instance()
    }
}

impl SolanaAddress {
    pub fn new() -> SolanaAddress {
        ::std::default::Default::default()
    }

    // required string address = 1;

    pub fn address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_address(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        if self.address.is_none() {
            self.address = ::std::option::Option::Some(::std::string::String::new());
        }
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "address",
            |m: &SolanaAddress| { &m.address },
            |m: &mut SolanaAddress| { &mut m.address },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SolanaAddress>(
            "SolanaAddress",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SolanaAddress {
    const NAME: &'static str = "SolanaAddress";

    fn is_initialized(&self) -> bool {
        if self.address.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.address = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.address.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.address.as_ref() {
            os.write_string(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SolanaAddress {
        SolanaAddress::new()
    }

    fn clear(&mut self) {
        self.address = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SolanaAddress {
        static instance: SolanaAddress = SolanaAddress {
            address: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SolanaAddress {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SolanaAddress").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SolanaAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SolanaAddress {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15messages-solana.proto\x12\x19hw.trezor.messages.solana\x1a\x15mess\
    ages-common.proto\"T\n\x12SolanaGetPublicKey\x12\x1b\n\taddress_n\x18\
    \x01\x20\x03(\rR\x08addressN\x12!\n\x0cshow_display\x18\x02\x20\x01(\x08\
    R\x0bshowDisplay\"0\n\x0fSolanaPublicKey\x12\x1d\n\npublic_key\x18\x01\
    \x20\x02(\x0cR\tpublicKey\"n\n\x10SolanaGetAddress\x12\x1b\n\taddress_n\
    \x18\x01\x20\x03(\rR\x08addressN\x12!\n\x0cshow_display\x18\x02\x20\x01(\
    \x08R\x0bshowDisplay\x12\x1a\n\x08chunkify\x18\x03\x20\x01(\x08R\x08chun\
    kify\")\n\rSolanaAddress\x12\x18\n\x07address\x18\x01\x20\x02(\tR\x07add\
    ress\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::messages_common::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(SolanaGetPublicKey::generated_message_descriptor_data());
            messages.push(SolanaPublicKey::generated_message_descriptor_data());
            messages.push(SolanaGetAddress::generated_message_descriptor_data());
            messages.push(SolanaAddress::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
