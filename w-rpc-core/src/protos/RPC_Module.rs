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
//! Generated file from `RPC_Module.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct Module {
    // message fields
    pub module_name: ::std::string::String,
    pub init_script: ::std::string::String,
    pub additionalBoilerPlate: ::std::string::String,
    pub functions: ::protobuf::RepeatedField<Function>,
    pub meta_data: ::std::string::String,
    pub packager: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Module {
    fn default() -> &'a Module {
        <Module as ::protobuf::Message>::default_instance()
    }
}

impl Module {
    pub fn new() -> Module {
        ::std::default::Default::default()
    }

    // string module_name = 1;


    pub fn get_module_name(&self) -> &str {
        &self.module_name
    }
    pub fn clear_module_name(&mut self) {
        self.module_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_module_name(&mut self, v: ::std::string::String) {
        self.module_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_module_name(&mut self) -> &mut ::std::string::String {
        &mut self.module_name
    }

    // Take field
    pub fn take_module_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.module_name, ::std::string::String::new())
    }

    // string init_script = 2;


    pub fn get_init_script(&self) -> &str {
        &self.init_script
    }
    pub fn clear_init_script(&mut self) {
        self.init_script.clear();
    }

    // Param is passed by value, moved
    pub fn set_init_script(&mut self, v: ::std::string::String) {
        self.init_script = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_init_script(&mut self) -> &mut ::std::string::String {
        &mut self.init_script
    }

    // Take field
    pub fn take_init_script(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.init_script, ::std::string::String::new())
    }

    // string additionalBoilerPlate = 3;


    pub fn get_additionalBoilerPlate(&self) -> &str {
        &self.additionalBoilerPlate
    }
    pub fn clear_additionalBoilerPlate(&mut self) {
        self.additionalBoilerPlate.clear();
    }

    // Param is passed by value, moved
    pub fn set_additionalBoilerPlate(&mut self, v: ::std::string::String) {
        self.additionalBoilerPlate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_additionalBoilerPlate(&mut self) -> &mut ::std::string::String {
        &mut self.additionalBoilerPlate
    }

    // Take field
    pub fn take_additionalBoilerPlate(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.additionalBoilerPlate, ::std::string::String::new())
    }

    // repeated .Function functions = 4;


    pub fn get_functions(&self) -> &[Function] {
        &self.functions
    }
    pub fn clear_functions(&mut self) {
        self.functions.clear();
    }

    // Param is passed by value, moved
    pub fn set_functions(&mut self, v: ::protobuf::RepeatedField<Function>) {
        self.functions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_functions(&mut self) -> &mut ::protobuf::RepeatedField<Function> {
        &mut self.functions
    }

    // Take field
    pub fn take_functions(&mut self) -> ::protobuf::RepeatedField<Function> {
        ::std::mem::replace(&mut self.functions, ::protobuf::RepeatedField::new())
    }

    // string meta_data = 5;


    pub fn get_meta_data(&self) -> &str {
        &self.meta_data
    }
    pub fn clear_meta_data(&mut self) {
        self.meta_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_meta_data(&mut self, v: ::std::string::String) {
        self.meta_data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta_data(&mut self) -> &mut ::std::string::String {
        &mut self.meta_data
    }

    // Take field
    pub fn take_meta_data(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.meta_data, ::std::string::String::new())
    }

    // string packager = 6;


    pub fn get_packager(&self) -> &str {
        &self.packager
    }
    pub fn clear_packager(&mut self) {
        self.packager.clear();
    }

    // Param is passed by value, moved
    pub fn set_packager(&mut self, v: ::std::string::String) {
        self.packager = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packager(&mut self) -> &mut ::std::string::String {
        &mut self.packager
    }

    // Take field
    pub fn take_packager(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.packager, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Module {
    fn is_initialized(&self) -> bool {
        for v in &self.functions {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.module_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.init_script)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.additionalBoilerPlate)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.functions)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.meta_data)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.packager)?;
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
        if !self.module_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.module_name);
        }
        if !self.init_script.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.init_script);
        }
        if !self.additionalBoilerPlate.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.additionalBoilerPlate);
        }
        for value in &self.functions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.meta_data.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.meta_data);
        }
        if !self.packager.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.packager);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.module_name.is_empty() {
            os.write_string(1, &self.module_name)?;
        }
        if !self.init_script.is_empty() {
            os.write_string(2, &self.init_script)?;
        }
        if !self.additionalBoilerPlate.is_empty() {
            os.write_string(3, &self.additionalBoilerPlate)?;
        }
        for v in &self.functions {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.meta_data.is_empty() {
            os.write_string(5, &self.meta_data)?;
        }
        if !self.packager.is_empty() {
            os.write_string(6, &self.packager)?;
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

    fn new() -> Module {
        Module::new()
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
                    "module_name",
                    |m: &Module| { &m.module_name },
                    |m: &mut Module| { &mut m.module_name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "init_script",
                    |m: &Module| { &m.init_script },
                    |m: &mut Module| { &mut m.init_script },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "additionalBoilerPlate",
                    |m: &Module| { &m.additionalBoilerPlate },
                    |m: &mut Module| { &mut m.additionalBoilerPlate },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Function>>(
                    "functions",
                    |m: &Module| { &m.functions },
                    |m: &mut Module| { &mut m.functions },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "meta_data",
                    |m: &Module| { &m.meta_data },
                    |m: &mut Module| { &mut m.meta_data },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "packager",
                    |m: &Module| { &m.packager },
                    |m: &mut Module| { &mut m.packager },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Module>(
                    "Module",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Module {
        static mut instance: ::protobuf::lazy::Lazy<Module> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Module,
        };
        unsafe {
            instance.get(Module::new)
        }
    }
}

impl ::protobuf::Clear for Module {
    fn clear(&mut self) {
        self.module_name.clear();
        self.init_script.clear();
        self.additionalBoilerPlate.clear();
        self.functions.clear();
        self.meta_data.clear();
        self.packager.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Module {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Module {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Function {
    // message fields
    pub name: ::std::string::String,
    pub parameter: ::protobuf::RepeatedField<Parameter>,
    pub returnType: Type,
    pub returnedValue: ::std::vec::Vec<u8>,
    pub canRaiseException: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Function {
    fn default() -> &'a Function {
        <Function as ::protobuf::Message>::default_instance()
    }
}

impl Function {
    pub fn new() -> Function {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // repeated .Parameter parameter = 2;


    pub fn get_parameter(&self) -> &[Parameter] {
        &self.parameter
    }
    pub fn clear_parameter(&mut self) {
        self.parameter.clear();
    }

    // Param is passed by value, moved
    pub fn set_parameter(&mut self, v: ::protobuf::RepeatedField<Parameter>) {
        self.parameter = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parameter(&mut self) -> &mut ::protobuf::RepeatedField<Parameter> {
        &mut self.parameter
    }

    // Take field
    pub fn take_parameter(&mut self) -> ::protobuf::RepeatedField<Parameter> {
        ::std::mem::replace(&mut self.parameter, ::protobuf::RepeatedField::new())
    }

    // .Type returnType = 3;


    pub fn get_returnType(&self) -> Type {
        self.returnType
    }
    pub fn clear_returnType(&mut self) {
        self.returnType = Type::ERROR;
    }

    // Param is passed by value, moved
    pub fn set_returnType(&mut self, v: Type) {
        self.returnType = v;
    }

    // bytes returnedValue = 4;


    pub fn get_returnedValue(&self) -> &[u8] {
        &self.returnedValue
    }
    pub fn clear_returnedValue(&mut self) {
        self.returnedValue.clear();
    }

    // Param is passed by value, moved
    pub fn set_returnedValue(&mut self, v: ::std::vec::Vec<u8>) {
        self.returnedValue = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_returnedValue(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.returnedValue
    }

    // Take field
    pub fn take_returnedValue(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.returnedValue, ::std::vec::Vec::new())
    }

    // bool canRaiseException = 5;


    pub fn get_canRaiseException(&self) -> bool {
        self.canRaiseException
    }
    pub fn clear_canRaiseException(&mut self) {
        self.canRaiseException = false;
    }

    // Param is passed by value, moved
    pub fn set_canRaiseException(&mut self, v: bool) {
        self.canRaiseException = v;
    }
}

impl ::protobuf::Message for Function {
    fn is_initialized(&self) -> bool {
        for v in &self.parameter {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.parameter)?;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.returnType, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.returnedValue)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.canRaiseException = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.parameter {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.returnType != Type::ERROR {
            my_size += ::protobuf::rt::enum_size(3, self.returnType);
        }
        if !self.returnedValue.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.returnedValue);
        }
        if self.canRaiseException != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.parameter {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.returnType != Type::ERROR {
            os.write_enum(3, self.returnType.value())?;
        }
        if !self.returnedValue.is_empty() {
            os.write_bytes(4, &self.returnedValue)?;
        }
        if self.canRaiseException != false {
            os.write_bool(5, self.canRaiseException)?;
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

    fn new() -> Function {
        Function::new()
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
                    "name",
                    |m: &Function| { &m.name },
                    |m: &mut Function| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Parameter>>(
                    "parameter",
                    |m: &Function| { &m.parameter },
                    |m: &mut Function| { &mut m.parameter },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Type>>(
                    "returnType",
                    |m: &Function| { &m.returnType },
                    |m: &mut Function| { &mut m.returnType },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "returnedValue",
                    |m: &Function| { &m.returnedValue },
                    |m: &mut Function| { &mut m.returnedValue },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "canRaiseException",
                    |m: &Function| { &m.canRaiseException },
                    |m: &mut Function| { &mut m.canRaiseException },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Function>(
                    "Function",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Function {
        static mut instance: ::protobuf::lazy::Lazy<Function> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Function,
        };
        unsafe {
            instance.get(Function::new)
        }
    }
}

impl ::protobuf::Clear for Function {
    fn clear(&mut self) {
        self.name.clear();
        self.parameter.clear();
        self.returnType = Type::ERROR;
        self.returnedValue.clear();
        self.canRaiseException = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Function {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Function {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Parameter {
    // message fields
    pub name: ::std::string::String,
    pub field_type: Type,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Parameter {
    fn default() -> &'a Parameter {
        <Parameter as ::protobuf::Message>::default_instance()
    }
}

impl Parameter {
    pub fn new() -> Parameter {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // .Type type = 2;


    pub fn get_field_type(&self) -> Type {
        self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = Type::ERROR;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Type) {
        self.field_type = v;
    }

    // bytes data = 3;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Parameter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.field_type != Type::ERROR {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.field_type != Type::ERROR {
            os.write_enum(2, self.field_type.value())?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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

    fn new() -> Parameter {
        Parameter::new()
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
                    "name",
                    |m: &Parameter| { &m.name },
                    |m: &mut Parameter| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Type>>(
                    "type",
                    |m: &Parameter| { &m.field_type },
                    |m: &mut Parameter| { &mut m.field_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    |m: &Parameter| { &m.data },
                    |m: &mut Parameter| { &mut m.data },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Parameter>(
                    "Parameter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Parameter {
        static mut instance: ::protobuf::lazy::Lazy<Parameter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Parameter,
        };
        unsafe {
            instance.get(Parameter::new)
        }
    }
}

impl ::protobuf::Clear for Parameter {
    fn clear(&mut self) {
        self.name.clear();
        self.field_type = Type::ERROR;
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Parameter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Type {
    ERROR = 0,
    INT32 = 1,
    INT64 = 2,
    UINT32 = 3,
    UINT64 = 4,
    STRING = 5,
    OPTION = 6,
    OBJECT = 7,
    OTHER = 8,
    VOID = 9,
}

impl ::protobuf::ProtobufEnum for Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Type> {
        match value {
            0 => ::std::option::Option::Some(Type::ERROR),
            1 => ::std::option::Option::Some(Type::INT32),
            2 => ::std::option::Option::Some(Type::INT64),
            3 => ::std::option::Option::Some(Type::UINT32),
            4 => ::std::option::Option::Some(Type::UINT64),
            5 => ::std::option::Option::Some(Type::STRING),
            6 => ::std::option::Option::Some(Type::OPTION),
            7 => ::std::option::Option::Some(Type::OBJECT),
            8 => ::std::option::Option::Some(Type::OTHER),
            9 => ::std::option::Option::Some(Type::VOID),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Type] = &[
            Type::ERROR,
            Type::INT32,
            Type::INT64,
            Type::UINT32,
            Type::UINT64,
            Type::STRING,
            Type::OPTION,
            Type::OBJECT,
            Type::OTHER,
            Type::VOID,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Type {
}

impl ::std::default::Default for Type {
    fn default() -> Self {
        Type::ERROR
    }
}

impl ::protobuf::reflect::ProtobufValue for Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10RPC_Module.proto\"\xe2\x01\n\x06Module\x12\x1f\n\x0bmodule_name\
    \x18\x01\x20\x01(\tR\nmoduleName\x12\x1f\n\x0binit_script\x18\x02\x20\
    \x01(\tR\ninitScript\x124\n\x15additionalBoilerPlate\x18\x03\x20\x01(\tR\
    \x15additionalBoilerPlate\x12'\n\tfunctions\x18\x04\x20\x03(\x0b2\t.Func\
    tionR\tfunctions\x12\x1b\n\tmeta_data\x18\x05\x20\x01(\tR\x08metaData\
    \x12\x1a\n\x08packager\x18\x06\x20\x01(\tR\x08packager\"\xc3\x01\n\x08Fu\
    nction\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12(\n\tparameter\
    \x18\x02\x20\x03(\x0b2\n.ParameterR\tparameter\x12%\n\nreturnType\x18\
    \x03\x20\x01(\x0e2\x05.TypeR\nreturnType\x12$\n\rreturnedValue\x18\x04\
    \x20\x01(\x0cR\rreturnedValue\x12,\n\x11canRaiseException\x18\x05\x20\
    \x01(\x08R\x11canRaiseException\"N\n\tParameter\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x19\n\x04type\x18\x02\x20\x01(\x0e2\x05.Typ\
    eR\x04type\x12\x12\n\x04data\x18\x03\x20\x01(\x0cR\x04data*x\n\x04Type\
    \x12\t\n\x05ERROR\x10\0\x12\t\n\x05INT32\x10\x01\x12\t\n\x05INT64\x10\
    \x02\x12\n\n\x06UINT32\x10\x03\x12\n\n\x06UINT64\x10\x04\x12\n\n\x06STRI\
    NG\x10\x05\x12\n\n\x06OPTION\x10\x06\x12\n\n\x06OBJECT\x10\x07\x12\t\n\
    \x05OTHER\x10\x08\x12\x08\n\x04VOID\x10\tb\x06proto3\
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
