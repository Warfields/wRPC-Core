// This file is generated by rust-protobuf 2.8.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `request.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct FnRequest {
    // message fields
    pub functionName: ::std::string::String,
    pub params: ::protobuf::RepeatedField<super::RPC_Module::Parameter>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FnRequest {
    fn default() -> &'a FnRequest {
        <FnRequest as ::protobuf::Message>::default_instance()
    }
}

impl FnRequest {
    pub fn new() -> FnRequest {
        ::std::default::Default::default()
    }

    // string functionName = 1;


    pub fn get_functionName(&self) -> &str {
        &self.functionName
    }
    pub fn clear_functionName(&mut self) {
        self.functionName.clear();
    }

    // Param is passed by value, moved
    pub fn set_functionName(&mut self, v: ::std::string::String) {
        self.functionName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_functionName(&mut self) -> &mut ::std::string::String {
        &mut self.functionName
    }

    // Take field
    pub fn take_functionName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.functionName, ::std::string::String::new())
    }

    // repeated .Parameter params = 2;


    pub fn get_params(&self) -> &[super::RPC_Module::Parameter] {
        &self.params
    }
    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    // Param is passed by value, moved
    pub fn set_params(&mut self, v: ::protobuf::RepeatedField<super::RPC_Module::Parameter>) {
        self.params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_params(&mut self) -> &mut ::protobuf::RepeatedField<super::RPC_Module::Parameter> {
        &mut self.params
    }

    // Take field
    pub fn take_params(&mut self) -> ::protobuf::RepeatedField<super::RPC_Module::Parameter> {
        ::std::mem::replace(&mut self.params, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for FnRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.params {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.functionName)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.params)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.functionName.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.functionName);
        }
        for value in &self.params {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.functionName.is_empty() {
            os.write_string(1, &self.functionName)?;
        }
        for v in &self.params {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FnRequest {
        FnRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "functionName",
                    |m: &FnRequest| { &m.functionName },
                    |m: &mut FnRequest| { &mut m.functionName },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::RPC_Module::Parameter>>(
                    "params",
                    |m: &FnRequest| { &m.params },
                    |m: &mut FnRequest| { &mut m.params },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FnRequest>(
                    "FnRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static FnRequest {
        static mut instance: ::protobuf::lazy::Lazy<FnRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FnRequest,
        };
        unsafe {
            instance.get(FnRequest::new)
        }
    }
}

impl ::protobuf::Clear for FnRequest {
    fn clear(&mut self) {
        self.functionName.clear();
        self.params.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FnRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FnRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ModuleRequest {
    // message fields
    pub moduleName: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ModuleRequest {
    fn default() -> &'a ModuleRequest {
        <ModuleRequest as ::protobuf::Message>::default_instance()
    }
}

impl ModuleRequest {
    pub fn new() -> ModuleRequest {
        ::std::default::Default::default()
    }

    // string moduleName = 1;


    pub fn get_moduleName(&self) -> &str {
        &self.moduleName
    }
    pub fn clear_moduleName(&mut self) {
        self.moduleName.clear();
    }

    // Param is passed by value, moved
    pub fn set_moduleName(&mut self, v: ::std::string::String) {
        self.moduleName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_moduleName(&mut self) -> &mut ::std::string::String {
        &mut self.moduleName
    }

    // Take field
    pub fn take_moduleName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.moduleName, ::std::string::String::new())
    }
}

impl ::protobuf::Message for ModuleRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.moduleName)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.moduleName.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.moduleName);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.moduleName.is_empty() {
            os.write_string(1, &self.moduleName)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ModuleRequest {
        ModuleRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "moduleName",
                    |m: &ModuleRequest| { &m.moduleName },
                    |m: &mut ModuleRequest| { &mut m.moduleName },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ModuleRequest>(
                    "ModuleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ModuleRequest {
        static mut instance: ::protobuf::lazy::Lazy<ModuleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModuleRequest,
        };
        unsafe {
            instance.get(ModuleRequest::new)
        }
    }
}

impl ::protobuf::Clear for ModuleRequest {
    fn clear(&mut self) {
        self.moduleName.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ModuleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModuleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rrequest.proto\x1a\x10RPC_Module.proto\"S\n\tFnRequest\x12\"\n\x0cfun\
    ctionName\x18\x01\x20\x01(\tR\x0cfunctionName\x12\"\n\x06params\x18\x02\
    \x20\x03(\x0b2\n.ParameterR\x06params\"/\n\rModuleRequest\x12\x1e\n\nmod\
    uleName\x18\x01\x20\x01(\tR\nmoduleNameb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}