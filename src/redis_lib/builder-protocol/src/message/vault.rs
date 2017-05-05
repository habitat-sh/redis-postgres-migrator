// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Origin {
    // message fields
    id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    private_key_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Origin {}

impl Origin {
    pub fn new() -> Origin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Origin {
        static mut instance: ::protobuf::lazy::Lazy<Origin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Origin,
        };
        unsafe {
            instance.get(Origin::new)
        }
    }

    // required uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // required string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required uint64 owner_id = 3;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional string private_key_name = 4;

    pub fn clear_private_key_name(&mut self) {
        self.private_key_name.clear();
    }

    pub fn has_private_key_name(&self) -> bool {
        self.private_key_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_key_name(&mut self, v: ::std::string::String) {
        self.private_key_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_key_name(&mut self) -> &mut ::std::string::String {
        if self.private_key_name.is_none() {
            self.private_key_name.set_default();
        };
        self.private_key_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_key_name(&mut self) -> ::std::string::String {
        self.private_key_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_key_name(&self) -> &str {
        match self.private_key_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_key_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_key_name
    }

    fn mut_private_key_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_key_name
    }
}

impl ::protobuf::Message for Origin {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_key_name)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.private_key_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.private_key_name.as_ref() {
            os.write_string(4, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Origin {
    fn new() -> Origin {
        Origin::new()
    }

    fn descriptor_static(_: ::std::option::Option<Origin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Origin::get_id_for_reflect,
                    Origin::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Origin::get_name_for_reflect,
                    Origin::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    Origin::get_owner_id_for_reflect,
                    Origin::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_key_name",
                    Origin::get_private_key_name_for_reflect,
                    Origin::mut_private_key_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Origin>(
                    "Origin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Origin {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_owner_id();
        self.clear_private_key_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Origin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Origin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginCreate {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    owner_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginCreate {}

impl OriginCreate {
    pub fn new() -> OriginCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginCreate,
        };
        unsafe {
            instance.get(OriginCreate::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required uint64 owner_id = 2;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // required string owner_name = 3;

    pub fn clear_owner_name(&mut self) {
        self.owner_name.clear();
    }

    pub fn has_owner_name(&self) -> bool {
        self.owner_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_name(&mut self, v: ::std::string::String) {
        self.owner_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_name(&mut self) -> &mut ::std::string::String {
        if self.owner_name.is_none() {
            self.owner_name.set_default();
        };
        self.owner_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_name(&mut self) -> ::std::string::String {
        self.owner_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner_name(&self) -> &str {
        match self.owner_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_owner_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.owner_name
    }

    fn mut_owner_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.owner_name
    }
}

impl ::protobuf::Message for OriginCreate {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        if self.owner_name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner_name)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.owner_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.owner_name.as_ref() {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginCreate {
    fn new() -> OriginCreate {
        OriginCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginCreate::get_name_for_reflect,
                    OriginCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginCreate::get_owner_id_for_reflect,
                    OriginCreate::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner_name",
                    OriginCreate::get_owner_name_for_reflect,
                    OriginCreate::mut_owner_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginCreate>(
                    "OriginCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginCreate {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_owner_id();
        self.clear_owner_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginDelete {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginDelete {}

impl OriginDelete {
    pub fn new() -> OriginDelete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginDelete {
        static mut instance: ::protobuf::lazy::Lazy<OriginDelete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginDelete,
        };
        unsafe {
            instance.get(OriginDelete::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for OriginDelete {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginDelete {
    fn new() -> OriginDelete {
        OriginDelete::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginDelete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginDelete::get_name_for_reflect,
                    OriginDelete::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginDelete>(
                    "OriginDelete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginDelete {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginDelete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginDelete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginGet {}

impl OriginGet {
    pub fn new() -> OriginGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginGet,
        };
        unsafe {
            instance.get(OriginGet::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for OriginGet {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginGet {
    fn new() -> OriginGet {
        OriginGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginGet::get_name_for_reflect,
                    OriginGet::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginGet>(
                    "OriginGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginGet {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginMemberRemove {
    // message fields
    origin_id: ::std::option::Option<u64>,
    user_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginMemberRemove {}

impl OriginMemberRemove {
    pub fn new() -> OriginMemberRemove {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginMemberRemove {
        static mut instance: ::protobuf::lazy::Lazy<OriginMemberRemove> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginMemberRemove,
        };
        unsafe {
            instance.get(OriginMemberRemove::new)
        }
    }

    // required uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // required uint64 user_id = 2;

    pub fn clear_user_id(&mut self) {
        self.user_id = ::std::option::Option::None;
    }

    pub fn has_user_id(&self) -> bool {
        self.user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: u64) {
        self.user_id = ::std::option::Option::Some(v);
    }

    pub fn get_user_id(&self) -> u64 {
        self.user_id.unwrap_or(0)
    }

    fn get_user_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.user_id
    }

    fn mut_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.user_id
    }
}

impl ::protobuf::Message for OriginMemberRemove {
    fn is_initialized(&self) -> bool {
        if self.origin_id.is_none() {
            return false;
        };
        if self.user_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.user_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.user_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.user_id {
            os.write_uint64(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginMemberRemove {
    fn new() -> OriginMemberRemove {
        OriginMemberRemove::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginMemberRemove>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginMemberRemove::get_origin_id_for_reflect,
                    OriginMemberRemove::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "user_id",
                    OriginMemberRemove::get_user_id_for_reflect,
                    OriginMemberRemove::mut_user_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginMemberRemove>(
                    "OriginMemberRemove",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginMemberRemove {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_user_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginMemberRemove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginMemberRemove {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginMemberListRequest {
    // message fields
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginMemberListRequest {}

impl OriginMemberListRequest {
    pub fn new() -> OriginMemberListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginMemberListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginMemberListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginMemberListRequest,
        };
        unsafe {
            instance.get(OriginMemberListRequest::new)
        }
    }

    // required uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for OriginMemberListRequest {
    fn is_initialized(&self) -> bool {
        if self.origin_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginMemberListRequest {
    fn new() -> OriginMemberListRequest {
        OriginMemberListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginMemberListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginMemberListRequest::get_origin_id_for_reflect,
                    OriginMemberListRequest::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginMemberListRequest>(
                    "OriginMemberListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginMemberListRequest {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginMemberListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginMemberListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginMemberListResponse {
    // message fields
    origin_id: ::std::option::Option<u64>,
    members: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginMemberListResponse {}

impl OriginMemberListResponse {
    pub fn new() -> OriginMemberListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginMemberListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginMemberListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginMemberListResponse,
        };
        unsafe {
            instance.get(OriginMemberListResponse::new)
        }
    }

    // required uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // repeated string members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[::std::string::String] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }
}

impl ::protobuf::Message for OriginMemberListResponse {
    fn is_initialized(&self) -> bool {
        if self.origin_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.members)?;
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.members {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.members {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginMemberListResponse {
    fn new() -> OriginMemberListResponse {
        OriginMemberListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginMemberListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginMemberListResponse::get_origin_id_for_reflect,
                    OriginMemberListResponse::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "members",
                    OriginMemberListResponse::get_members_for_reflect,
                    OriginMemberListResponse::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginMemberListResponse>(
                    "OriginMemberListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginMemberListResponse {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginMemberListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginMemberListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginListRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginListRequest {}

impl AccountOriginListRequest {
    pub fn new() -> AccountOriginListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginListRequest,
        };
        unsafe {
            instance.get(AccountOriginListRequest::new)
        }
    }

    // required uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountOriginListRequest {
    fn is_initialized(&self) -> bool {
        if self.account_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginListRequest {
    fn new() -> AccountOriginListRequest {
        AccountOriginListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginListRequest::get_account_id_for_reflect,
                    AccountOriginListRequest::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginListRequest>(
                    "AccountOriginListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginListRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountOriginListResponse {
    // message fields
    account_id: ::std::option::Option<u64>,
    origins: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountOriginListResponse {}

impl AccountOriginListResponse {
    pub fn new() -> AccountOriginListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountOriginListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountOriginListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountOriginListResponse,
        };
        unsafe {
            instance.get(AccountOriginListResponse::new)
        }
    }

    // required uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // repeated string origins = 2;

    pub fn clear_origins(&mut self) {
        self.origins.clear();
    }

    // Param is passed by value, moved
    pub fn set_origins(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.origins = v;
    }

    // Mutable pointer to the field.
    pub fn mut_origins(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.origins
    }

    // Take field
    pub fn take_origins(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.origins, ::protobuf::RepeatedField::new())
    }

    pub fn get_origins(&self) -> &[::std::string::String] {
        &self.origins
    }

    fn get_origins_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.origins
    }

    fn mut_origins_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.origins
    }
}

impl ::protobuf::Message for AccountOriginListResponse {
    fn is_initialized(&self) -> bool {
        if self.account_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.origins)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.origins {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.origins {
            os.write_string(2, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountOriginListResponse {
    fn new() -> AccountOriginListResponse {
        AccountOriginListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountOriginListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountOriginListResponse::get_account_id_for_reflect,
                    AccountOriginListResponse::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origins",
                    AccountOriginListResponse::get_origins_for_reflect,
                    AccountOriginListResponse::mut_origins_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountOriginListResponse>(
                    "AccountOriginListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountOriginListResponse {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_origins();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountOriginListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountOriginListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckOriginAccessRequest {
    // message oneof groups
    account_info: ::std::option::Option<CheckOriginAccessRequest_oneof_account_info>,
    origin_info: ::std::option::Option<CheckOriginAccessRequest_oneof_origin_info>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckOriginAccessRequest {}

#[derive(Clone,PartialEq)]
pub enum CheckOriginAccessRequest_oneof_account_info {
    account_id(u64),
    account_name(::std::string::String),
}

#[derive(Clone,PartialEq)]
pub enum CheckOriginAccessRequest_oneof_origin_info {
    origin_id(u64),
    origin_name(::std::string::String),
}

impl CheckOriginAccessRequest {
    pub fn new() -> CheckOriginAccessRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckOriginAccessRequest {
        static mut instance: ::protobuf::lazy::Lazy<CheckOriginAccessRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckOriginAccessRequest,
        };
        unsafe {
            instance.get(CheckOriginAccessRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_info = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(v))
    }

    pub fn get_account_id(&self) -> u64 {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(v)) => v,
            _ => 0,
        }
    }

    // optional string account_name = 2;

    pub fn clear_account_name(&mut self) {
        self.account_info = ::std::option::Option::None;
    }

    pub fn has_account_name(&self) -> bool {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(_)) = self.account_info {
        } else {
            self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(::std::string::String::new()));
        }
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        if self.has_account_name() {
            match self.account_info.take() {
                ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(ref v)) => v,
            _ => "",
        }
    }

    // optional uint64 origin_id = 3;

    pub fn clear_origin_id(&mut self) {
        self.origin_info = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(v))
    }

    pub fn get_origin_id(&self) -> u64 {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(v)) => v,
            _ => 0,
        }
    }

    // optional string origin_name = 4;

    pub fn clear_origin_name(&mut self) {
        self.origin_info = ::std::option::Option::None;
    }

    pub fn has_origin_name(&self) -> bool {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(_)) = self.origin_info {
        } else {
            self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(::std::string::String::new()));
        }
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        if self.has_origin_name() {
            match self.origin_info.take() {
                ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for CheckOriginAccessRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(is.read_uint64()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(is.read_uint64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.account_info {
            match v {
                &CheckOriginAccessRequest_oneof_account_info::account_id(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &CheckOriginAccessRequest_oneof_account_info::account_name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        };
        if let ::std::option::Option::Some(ref v) = self.origin_info {
            match v {
                &CheckOriginAccessRequest_oneof_origin_info::origin_id(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &CheckOriginAccessRequest_oneof_origin_info::origin_name(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.account_info {
            match v {
                &CheckOriginAccessRequest_oneof_account_info::account_id(v) => {
                    os.write_uint64(1, v)?;
                },
                &CheckOriginAccessRequest_oneof_account_info::account_name(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        };
        if let ::std::option::Option::Some(ref v) = self.origin_info {
            match v {
                &CheckOriginAccessRequest_oneof_origin_info::origin_id(v) => {
                    os.write_uint64(3, v)?;
                },
                &CheckOriginAccessRequest_oneof_origin_info::origin_name(ref v) => {
                    os.write_string(4, v)?;
                },
            };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckOriginAccessRequest {
    fn new() -> CheckOriginAccessRequest {
        CheckOriginAccessRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckOriginAccessRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "account_id",
                    CheckOriginAccessRequest::has_account_id,
                    CheckOriginAccessRequest::get_account_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "account_name",
                    CheckOriginAccessRequest::has_account_name,
                    CheckOriginAccessRequest::get_account_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "origin_id",
                    CheckOriginAccessRequest::has_origin_id,
                    CheckOriginAccessRequest::get_origin_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "origin_name",
                    CheckOriginAccessRequest::has_origin_name,
                    CheckOriginAccessRequest::get_origin_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckOriginAccessRequest>(
                    "CheckOriginAccessRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckOriginAccessRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckOriginAccessRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckOriginAccessRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckOriginAccessResponse {
    // message fields
    has_access: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckOriginAccessResponse {}

impl CheckOriginAccessResponse {
    pub fn new() -> CheckOriginAccessResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckOriginAccessResponse {
        static mut instance: ::protobuf::lazy::Lazy<CheckOriginAccessResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckOriginAccessResponse,
        };
        unsafe {
            instance.get(CheckOriginAccessResponse::new)
        }
    }

    // required bool has_access = 1;

    pub fn clear_has_access(&mut self) {
        self.has_access = ::std::option::Option::None;
    }

    pub fn has_has_access(&self) -> bool {
        self.has_access.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_access(&mut self, v: bool) {
        self.has_access = ::std::option::Option::Some(v);
    }

    pub fn get_has_access(&self) -> bool {
        self.has_access.unwrap_or(false)
    }

    fn get_has_access_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_access
    }

    fn mut_has_access_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_access
    }
}

impl ::protobuf::Message for CheckOriginAccessResponse {
    fn is_initialized(&self) -> bool {
        if self.has_access.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.has_access = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.has_access {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.has_access {
            os.write_bool(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckOriginAccessResponse {
    fn new() -> CheckOriginAccessResponse {
        CheckOriginAccessResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckOriginAccessResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_access",
                    CheckOriginAccessResponse::get_has_access_for_reflect,
                    CheckOriginAccessResponse::mut_has_access_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckOriginAccessResponse>(
                    "CheckOriginAccessResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckOriginAccessResponse {
    fn clear(&mut self) {
        self.clear_has_access();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckOriginAccessResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckOriginAccessResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInvitationListRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListRequest {}

impl AccountInvitationListRequest {
    pub fn new() -> AccountInvitationListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListRequest,
        };
        unsafe {
            instance.get(AccountInvitationListRequest::new)
        }
    }

    // required uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountInvitationListRequest {
    fn is_initialized(&self) -> bool {
        if self.account_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountInvitationListRequest {
    fn new() -> AccountInvitationListRequest {
        AccountInvitationListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListRequest::get_account_id_for_reflect,
                    AccountInvitationListRequest::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListRequest>(
                    "AccountInvitationListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInvitationListResponse {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitations: ::protobuf::RepeatedField<OriginInvitation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListResponse {}

impl AccountInvitationListResponse {
    pub fn new() -> AccountInvitationListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListResponse,
        };
        unsafe {
            instance.get(AccountInvitationListResponse::new)
        }
    }

    // required uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // repeated .vault.OriginInvitation invitations = 2;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<OriginInvitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<OriginInvitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[OriginInvitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<OriginInvitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }
}

impl ::protobuf::Message for AccountInvitationListResponse {
    fn is_initialized(&self) -> bool {
        if self.account_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.invitations {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountInvitationListResponse {
    fn new() -> AccountInvitationListResponse {
        AccountInvitationListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListResponse::get_account_id_for_reflect,
                    AccountInvitationListResponse::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginInvitation>>(
                    "invitations",
                    AccountInvitationListResponse::get_invitations_for_reflect,
                    AccountInvitationListResponse::mut_invitations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListResponse>(
                    "AccountInvitationListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListResponse {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationListRequest {
    // message fields
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationListRequest {}

impl OriginInvitationListRequest {
    pub fn new() -> OriginInvitationListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationListRequest,
        };
        unsafe {
            instance.get(OriginInvitationListRequest::new)
        }
    }

    // required uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for OriginInvitationListRequest {
    fn is_initialized(&self) -> bool {
        if self.origin_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginInvitationListRequest {
    fn new() -> OriginInvitationListRequest {
        OriginInvitationListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitationListRequest::get_origin_id_for_reflect,
                    OriginInvitationListRequest::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationListRequest>(
                    "OriginInvitationListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationListRequest {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationListResponse {
    // message fields
    origin_id: ::std::option::Option<u64>,
    invitations: ::protobuf::RepeatedField<OriginInvitation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationListResponse {}

impl OriginInvitationListResponse {
    pub fn new() -> OriginInvitationListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationListResponse,
        };
        unsafe {
            instance.get(OriginInvitationListResponse::new)
        }
    }

    // required uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // repeated .vault.OriginInvitation invitations = 2;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<OriginInvitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<OriginInvitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[OriginInvitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<OriginInvitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }
}

impl ::protobuf::Message for OriginInvitationListResponse {
    fn is_initialized(&self) -> bool {
        if self.origin_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.invitations {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginInvitationListResponse {
    fn new() -> OriginInvitationListResponse {
        OriginInvitationListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitationListResponse::get_origin_id_for_reflect,
                    OriginInvitationListResponse::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginInvitation>>(
                    "invitations",
                    OriginInvitationListResponse::get_invitations_for_reflect,
                    OriginInvitationListResponse::mut_invitations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationListResponse>(
                    "OriginInvitationListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationListResponse {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_invitations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitation {
    // message fields
    id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitation {}

impl OriginInvitation {
    pub fn new() -> OriginInvitation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitation {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitation,
        };
        unsafe {
            instance.get(OriginInvitation::new)
        }
    }

    // required uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // required uint64 account_id = 2;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // required string account_name = 3;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        };
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // required uint64 origin_id = 4;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // required string origin_name = 5;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        };
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // required uint64 owner_id = 6;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginInvitation {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.account_id.is_none() {
            return false;
        };
        if self.account_name.is_none() {
            return false;
        };
        if self.origin_id.is_none() {
            return false;
        };
        if self.origin_name.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.account_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.account_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.origin_id {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.origin_name.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(6, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginInvitation {
    fn new() -> OriginInvitation {
        OriginInvitation::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginInvitation::get_id_for_reflect,
                    OriginInvitation::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    OriginInvitation::get_account_id_for_reflect,
                    OriginInvitation::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    OriginInvitation::get_account_name_for_reflect,
                    OriginInvitation::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitation::get_origin_id_for_reflect,
                    OriginInvitation::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginInvitation::get_origin_name_for_reflect,
                    OriginInvitation::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginInvitation::get_owner_id_for_reflect,
                    OriginInvitation::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitation>(
                    "OriginInvitation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationCreate {
    // message fields
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationCreate {}

impl OriginInvitationCreate {
    pub fn new() -> OriginInvitationCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationCreate,
        };
        unsafe {
            instance.get(OriginInvitationCreate::new)
        }
    }

    // required uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // required string account_name = 2;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        };
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // required uint64 origin_id = 3;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // required string origin_name = 4;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        };
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // required uint64 owner_id = 5;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginInvitationCreate {
    fn is_initialized(&self) -> bool {
        if self.account_id.is_none() {
            return false;
        };
        if self.account_name.is_none() {
            return false;
        };
        if self.origin_id.is_none() {
            return false;
        };
        if self.origin_name.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.account_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.origin_id {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.origin_name.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(5, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginInvitationCreate {
    fn new() -> OriginInvitationCreate {
        OriginInvitationCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    OriginInvitationCreate::get_account_id_for_reflect,
                    OriginInvitationCreate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    OriginInvitationCreate::get_account_name_for_reflect,
                    OriginInvitationCreate::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitationCreate::get_origin_id_for_reflect,
                    OriginInvitationCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginInvitationCreate::get_origin_name_for_reflect,
                    OriginInvitationCreate::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginInvitationCreate::get_owner_id_for_reflect,
                    OriginInvitationCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationCreate>(
                    "OriginInvitationCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationCreate {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationAcceptRequest {
    // message fields
    account_accepting_request: ::std::option::Option<u64>,
    invite_id: ::std::option::Option<u64>,
    ignore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationAcceptRequest {}

impl OriginInvitationAcceptRequest {
    pub fn new() -> OriginInvitationAcceptRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationAcceptRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationAcceptRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationAcceptRequest,
        };
        unsafe {
            instance.get(OriginInvitationAcceptRequest::new)
        }
    }

    // required uint64 account_accepting_request = 1;

    pub fn clear_account_accepting_request(&mut self) {
        self.account_accepting_request = ::std::option::Option::None;
    }

    pub fn has_account_accepting_request(&self) -> bool {
        self.account_accepting_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_accepting_request(&mut self, v: u64) {
        self.account_accepting_request = ::std::option::Option::Some(v);
    }

    pub fn get_account_accepting_request(&self) -> u64 {
        self.account_accepting_request.unwrap_or(0)
    }

    fn get_account_accepting_request_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_accepting_request
    }

    fn mut_account_accepting_request_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_accepting_request
    }

    // required uint64 invite_id = 2;

    pub fn clear_invite_id(&mut self) {
        self.invite_id = ::std::option::Option::None;
    }

    pub fn has_invite_id(&self) -> bool {
        self.invite_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invite_id(&mut self, v: u64) {
        self.invite_id = ::std::option::Option::Some(v);
    }

    pub fn get_invite_id(&self) -> u64 {
        self.invite_id.unwrap_or(0)
    }

    fn get_invite_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invite_id
    }

    fn mut_invite_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invite_id
    }

    // required bool ignore = 3;

    pub fn clear_ignore(&mut self) {
        self.ignore = ::std::option::Option::None;
    }

    pub fn has_ignore(&self) -> bool {
        self.ignore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignore(&mut self, v: bool) {
        self.ignore = ::std::option::Option::Some(v);
    }

    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap_or(false)
    }

    fn get_ignore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ignore
    }

    fn mut_ignore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ignore
    }
}

impl ::protobuf::Message for OriginInvitationAcceptRequest {
    fn is_initialized(&self) -> bool {
        if self.account_accepting_request.is_none() {
            return false;
        };
        if self.invite_id.is_none() {
            return false;
        };
        if self.ignore.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.account_accepting_request = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.invite_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.ignore = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_accepting_request {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.invite_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ignore {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_accepting_request {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.invite_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.ignore {
            os.write_bool(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginInvitationAcceptRequest {
    fn new() -> OriginInvitationAcceptRequest {
        OriginInvitationAcceptRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationAcceptRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_accepting_request",
                    OriginInvitationAcceptRequest::get_account_accepting_request_for_reflect,
                    OriginInvitationAcceptRequest::mut_account_accepting_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invite_id",
                    OriginInvitationAcceptRequest::get_invite_id_for_reflect,
                    OriginInvitationAcceptRequest::mut_invite_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore",
                    OriginInvitationAcceptRequest::get_ignore_for_reflect,
                    OriginInvitationAcceptRequest::mut_ignore_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationAcceptRequest>(
                    "OriginInvitationAcceptRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationAcceptRequest {
    fn clear(&mut self) {
        self.clear_account_accepting_request();
        self.clear_invite_id();
        self.clear_ignore();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationAcceptRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationAcceptRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationAcceptResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationAcceptResponse {}

impl OriginInvitationAcceptResponse {
    pub fn new() -> OriginInvitationAcceptResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationAcceptResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationAcceptResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationAcceptResponse,
        };
        unsafe {
            instance.get(OriginInvitationAcceptResponse::new)
        }
    }
}

impl ::protobuf::Message for OriginInvitationAcceptResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginInvitationAcceptResponse {
    fn new() -> OriginInvitationAcceptResponse {
        OriginInvitationAcceptResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationAcceptResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationAcceptResponse>(
                    "OriginInvitationAcceptResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationAcceptResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationAcceptResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationAcceptResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginSecretKey {
    // message fields
    id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginSecretKey {}

impl OriginSecretKey {
    pub fn new() -> OriginSecretKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginSecretKey {
        static mut instance: ::protobuf::lazy::Lazy<OriginSecretKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginSecretKey,
        };
        unsafe {
            instance.get(OriginSecretKey::new)
        }
    }

    // required uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // required uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // required string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string revision = 4;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        };
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }

    // required bytes body = 5;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        };
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }

    // required uint64 owner_id = 6;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginSecretKey {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.origin_id.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        if self.revision.is_none() {
            return false;
        };
        if self.body.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.revision.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.body.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(6, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginSecretKey {
    fn new() -> OriginSecretKey {
        OriginSecretKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginSecretKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginSecretKey::get_id_for_reflect,
                    OriginSecretKey::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginSecretKey::get_origin_id_for_reflect,
                    OriginSecretKey::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginSecretKey::get_name_for_reflect,
                    OriginSecretKey::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginSecretKey::get_revision_for_reflect,
                    OriginSecretKey::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    OriginSecretKey::get_body_for_reflect,
                    OriginSecretKey::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginSecretKey::get_owner_id_for_reflect,
                    OriginSecretKey::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginSecretKey>(
                    "OriginSecretKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginSecretKey {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_id();
        self.clear_name();
        self.clear_revision();
        self.clear_body();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginSecretKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginSecretKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginSecretKeyCreate {
    // message fields
    origin_id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginSecretKeyCreate {}

impl OriginSecretKeyCreate {
    pub fn new() -> OriginSecretKeyCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginSecretKeyCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginSecretKeyCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginSecretKeyCreate,
        };
        unsafe {
            instance.get(OriginSecretKeyCreate::new)
        }
    }

    // required uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // required string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string revision = 3;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        };
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }

    // required bytes body = 4;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        };
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }

    // required uint64 owner_id = 5;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginSecretKeyCreate {
    fn is_initialized(&self) -> bool {
        if self.origin_id.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        if self.revision.is_none() {
            return false;
        };
        if self.body.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.revision.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.body.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(5, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OriginSecretKeyCreate {
    fn new() -> OriginSecretKeyCreate {
        OriginSecretKeyCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginSecretKeyCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginSecretKeyCreate::get_origin_id_for_reflect,
                    OriginSecretKeyCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginSecretKeyCreate::get_name_for_reflect,
                    OriginSecretKeyCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginSecretKeyCreate::get_revision_for_reflect,
                    OriginSecretKeyCreate::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    OriginSecretKeyCreate::get_body_for_reflect,
                    OriginSecretKeyCreate::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginSecretKeyCreate::get_owner_id_for_reflect,
                    OriginSecretKeyCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginSecretKeyCreate>(
                    "OriginSecretKeyCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginSecretKeyCreate {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_name();
        self.clear_revision();
        self.clear_body();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginSecretKeyCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginSecretKeyCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Project {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    plan_path: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    vcs: ::std::option::Option<Project_oneof_vcs>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Project {}

#[derive(Clone,PartialEq)]
pub enum Project_oneof_vcs {
    git(VCSGit),
}

impl Project {
    pub fn new() -> Project {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Project {
        static mut instance: ::protobuf::lazy::Lazy<Project> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Project,
        };
        unsafe {
            instance.get(Project::new)
        }
    }

    // required string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.id
    }

    // required uint64 owner_id = 2;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // required string plan_path = 3;

    pub fn clear_plan_path(&mut self) {
        self.plan_path.clear();
    }

    pub fn has_plan_path(&self) -> bool {
        self.plan_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plan_path(&mut self, v: ::std::string::String) {
        self.plan_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plan_path(&mut self) -> &mut ::std::string::String {
        if self.plan_path.is_none() {
            self.plan_path.set_default();
        };
        self.plan_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_plan_path(&mut self) -> ::std::string::String {
        self.plan_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_plan_path(&self) -> &str {
        match self.plan_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_plan_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.plan_path
    }

    fn mut_plan_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.plan_path
    }

    // optional .vault.VCSGit git = 4;

    pub fn clear_git(&mut self) {
        self.vcs = ::std::option::Option::None;
    }

    pub fn has_git(&self) -> bool {
        match self.vcs {
            ::std::option::Option::Some(Project_oneof_vcs::git(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_git(&mut self, v: VCSGit) {
        self.vcs = ::std::option::Option::Some(Project_oneof_vcs::git(v))
    }

    // Mutable pointer to the field.
    pub fn mut_git(&mut self) -> &mut VCSGit {
        if let ::std::option::Option::Some(Project_oneof_vcs::git(_)) = self.vcs {
        } else {
            self.vcs = ::std::option::Option::Some(Project_oneof_vcs::git(VCSGit::new()));
        }
        match self.vcs {
            ::std::option::Option::Some(Project_oneof_vcs::git(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_git(&mut self) -> VCSGit {
        if self.has_git() {
            match self.vcs.take() {
                ::std::option::Option::Some(Project_oneof_vcs::git(v)) => v,
                _ => panic!(),
            }
        } else {
            VCSGit::new()
        }
    }

    pub fn get_git(&self) -> &VCSGit {
        match self.vcs {
            ::std::option::Option::Some(Project_oneof_vcs::git(ref v)) => v,
            _ => VCSGit::default_instance(),
        }
    }
}

impl ::protobuf::Message for Project {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        if self.plan_path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.plan_path)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.vcs = ::std::option::Option::Some(Project_oneof_vcs::git(is.read_message()?));
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
        if let Some(v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.plan_path.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let ::std::option::Option::Some(ref v) = self.vcs {
            match v {
                &Project_oneof_vcs::git(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.plan_path.as_ref() {
            os.write_string(3, &v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.vcs {
            match v {
                &Project_oneof_vcs::git(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Project {
    fn new() -> Project {
        Project::new()
    }

    fn descriptor_static(_: ::std::option::Option<Project>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Project::get_id_for_reflect,
                    Project::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    Project::get_owner_id_for_reflect,
                    Project::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "plan_path",
                    Project::get_plan_path_for_reflect,
                    Project::mut_plan_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, VCSGit>(
                    "git",
                    Project::has_git,
                    Project::get_git,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Project>(
                    "Project",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Project {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_owner_id();
        self.clear_plan_path();
        self.clear_git();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Project {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Project {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectUpdate {
    // message fields
    requestor_id: ::std::option::Option<u64>,
    project: ::protobuf::SingularPtrField<Project>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectUpdate {}

impl ProjectUpdate {
    pub fn new() -> ProjectUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ProjectUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectUpdate,
        };
        unsafe {
            instance.get(ProjectUpdate::new)
        }
    }

    // required uint64 requestor_id = 1;

    pub fn clear_requestor_id(&mut self) {
        self.requestor_id = ::std::option::Option::None;
    }

    pub fn has_requestor_id(&self) -> bool {
        self.requestor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestor_id(&mut self, v: u64) {
        self.requestor_id = ::std::option::Option::Some(v);
    }

    pub fn get_requestor_id(&self) -> u64 {
        self.requestor_id.unwrap_or(0)
    }

    fn get_requestor_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.requestor_id
    }

    fn mut_requestor_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.requestor_id
    }

    // required .vault.Project project = 2;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: Project) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut Project {
        if self.project.is_none() {
            self.project.set_default();
        };
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> Project {
        self.project.take().unwrap_or_else(|| Project::new())
    }

    pub fn get_project(&self) -> &Project {
        self.project.as_ref().unwrap_or_else(|| Project::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<Project> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Project> {
        &mut self.project
    }
}

impl ::protobuf::Message for ProjectUpdate {
    fn is_initialized(&self) -> bool {
        if self.requestor_id.is_none() {
            return false;
        };
        if self.project.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.requestor_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.project)?;
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
        if let Some(v) = self.requestor_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.requestor_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.project.as_ref() {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ProjectUpdate {
    fn new() -> ProjectUpdate {
        ProjectUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "requestor_id",
                    ProjectUpdate::get_requestor_id_for_reflect,
                    ProjectUpdate::mut_requestor_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Project>>(
                    "project",
                    ProjectUpdate::get_project_for_reflect,
                    ProjectUpdate::mut_project_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectUpdate>(
                    "ProjectUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectUpdate {
    fn clear(&mut self) {
        self.clear_requestor_id();
        self.clear_project();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectCreate {
    // message fields
    project: ::protobuf::SingularPtrField<Project>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectCreate {}

impl ProjectCreate {
    pub fn new() -> ProjectCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectCreate {
        static mut instance: ::protobuf::lazy::Lazy<ProjectCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectCreate,
        };
        unsafe {
            instance.get(ProjectCreate::new)
        }
    }

    // required .vault.Project project = 1;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: Project) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut Project {
        if self.project.is_none() {
            self.project.set_default();
        };
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> Project {
        self.project.take().unwrap_or_else(|| Project::new())
    }

    pub fn get_project(&self) -> &Project {
        self.project.as_ref().unwrap_or_else(|| Project::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<Project> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Project> {
        &mut self.project
    }
}

impl ::protobuf::Message for ProjectCreate {
    fn is_initialized(&self) -> bool {
        if self.project.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.project)?;
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
        if let Some(v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.project.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ProjectCreate {
    fn new() -> ProjectCreate {
        ProjectCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Project>>(
                    "project",
                    ProjectCreate::get_project_for_reflect,
                    ProjectCreate::mut_project_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectCreate>(
                    "ProjectCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectCreate {
    fn clear(&mut self) {
        self.clear_project();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectDelete {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    requestor_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectDelete {}

impl ProjectDelete {
    pub fn new() -> ProjectDelete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectDelete {
        static mut instance: ::protobuf::lazy::Lazy<ProjectDelete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectDelete,
        };
        unsafe {
            instance.get(ProjectDelete::new)
        }
    }

    // required string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.id
    }

    // required uint64 requestor_id = 2;

    pub fn clear_requestor_id(&mut self) {
        self.requestor_id = ::std::option::Option::None;
    }

    pub fn has_requestor_id(&self) -> bool {
        self.requestor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestor_id(&mut self, v: u64) {
        self.requestor_id = ::std::option::Option::Some(v);
    }

    pub fn get_requestor_id(&self) -> u64 {
        self.requestor_id.unwrap_or(0)
    }

    fn get_requestor_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.requestor_id
    }

    fn mut_requestor_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.requestor_id
    }
}

impl ::protobuf::Message for ProjectDelete {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.requestor_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.requestor_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.requestor_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.requestor_id {
            os.write_uint64(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ProjectDelete {
    fn new() -> ProjectDelete {
        ProjectDelete::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectDelete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    ProjectDelete::get_id_for_reflect,
                    ProjectDelete::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "requestor_id",
                    ProjectDelete::get_requestor_id_for_reflect,
                    ProjectDelete::mut_requestor_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectDelete>(
                    "ProjectDelete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectDelete {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_requestor_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectDelete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectDelete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectGet {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectGet {}

impl ProjectGet {
    pub fn new() -> ProjectGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectGet {
        static mut instance: ::protobuf::lazy::Lazy<ProjectGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectGet,
        };
        unsafe {
            instance.get(ProjectGet::new)
        }
    }

    // required string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.id
    }
}

impl ::protobuf::Message for ProjectGet {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
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
        if let Some(v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            os.write_string(1, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ProjectGet {
    fn new() -> ProjectGet {
        ProjectGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    ProjectGet::get_id_for_reflect,
                    ProjectGet::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectGet>(
                    "ProjectGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectGet {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VCSGit {
    // message fields
    url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VCSGit {}

impl VCSGit {
    pub fn new() -> VCSGit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VCSGit {
        static mut instance: ::protobuf::lazy::Lazy<VCSGit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VCSGit,
        };
        unsafe {
            instance.get(VCSGit::new)
        }
    }

    // required string url = 1;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }
}

impl ::protobuf::Message for VCSGit {
    fn is_initialized(&self) -> bool {
        if self.url.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
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
        if let Some(v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.url.as_ref() {
            os.write_string(1, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VCSGit {
    fn new() -> VCSGit {
        VCSGit::new()
    }

    fn descriptor_static(_: ::std::option::Option<VCSGit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    VCSGit::get_url_for_reflect,
                    VCSGit::mut_url_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VCSGit>(
                    "VCSGit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VCSGit {
    fn clear(&mut self) {
        self.clear_url();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VCSGit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VCSGit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x76, 0x61, 0x75, 0x6c,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x22, 0x4e,
    0x0a, 0x06, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x5f,
    0x6b, 0x65, 0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x42,
    0x0a, 0x0c, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08,
    0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x12,
    0x0a, 0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x09, 0x22, 0x1c, 0x0a, 0x0c, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x22, 0x19, 0x0a, 0x09, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x47, 0x65, 0x74, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x38, 0x0a, 0x12, 0x4f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x65, 0x6d, 0x6f, 0x76,
    0x65, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x0f, 0x0a, 0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x2c, 0x0a, 0x17, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x4d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x04, 0x22, 0x3e, 0x0a, 0x18, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x4d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x04, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x09, 0x22, 0x2e, 0x0a, 0x18, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x04, 0x22, 0x40, 0x0a, 0x19, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x4f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x12, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x0f, 0x0a, 0x07, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x09, 0x22, 0x93, 0x01, 0x0a, 0x18, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x4f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x14, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x12, 0x16, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x12, 0x13, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x04, 0x48, 0x01, 0x12, 0x15, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x01, 0x42, 0x0e, 0x0a, 0x0c,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x42, 0x0d, 0x0a, 0x0b,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x22, 0x2f, 0x0a, 0x19, 0x43,
    0x68, 0x65, 0x63, 0x6b, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x68, 0x61, 0x73, 0x5f,
    0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x22, 0x32, 0x0a, 0x1c,
    0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x0a,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04,
    0x22, 0x61, 0x0a, 0x1d, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x76, 0x69, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x12, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x2c, 0x0a, 0x0b, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x76, 0x61, 0x75,
    0x6c, 0x74, 0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x22, 0x30, 0x0a, 0x1b, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76,
    0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x5f, 0x0a, 0x1c, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49,
    0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x2c, 0x0a, 0x0b, 0x69, 0x6e, 0x76, 0x69,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x76, 0x61, 0x75, 0x6c, 0x74, 0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x82, 0x01, 0x0a, 0x10, 0x4f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0a, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x02, 0x28, 0x04, 0x22, 0x7c, 0x0a, 0x16, 0x4f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72,
    0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x04, 0x22, 0x65, 0x0a, 0x1d, 0x4f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x63, 0x63,
    0x65, 0x70, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x19, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x69, 0x6e, 0x67, 0x5f,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x11, 0x0a,
    0x09, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04,
    0x12, 0x0e, 0x0a, 0x06, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08,
    0x22, 0x20, 0x0a, 0x1e, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x41, 0x63, 0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x70, 0x0a, 0x0f, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x53, 0x65, 0x63, 0x72,
    0x65, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x04, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x05, 0x20, 0x02,
    0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x06,
    0x20, 0x02, 0x28, 0x04, 0x22, 0x6a, 0x0a, 0x15, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x53, 0x65,
    0x63, 0x72, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x11, 0x0a,
    0x09, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04,
    0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10,
    0x0a, 0x08, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x0c, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x10,
    0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x04,
    0x22, 0x5f, 0x0a, 0x07, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x0a, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x6c, 0x61,
    0x6e, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x1c, 0x0a, 0x03,
    0x67, 0x69, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x76, 0x61, 0x75, 0x6c,
    0x74, 0x2e, 0x56, 0x43, 0x53, 0x47, 0x69, 0x74, 0x48, 0x00, 0x42, 0x05, 0x0a, 0x03, 0x76, 0x63,
    0x73, 0x22, 0x46, 0x0a, 0x0d, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x12, 0x14, 0x0a, 0x0c, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x6f, 0x72, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1f, 0x0a, 0x07, 0x70, 0x72, 0x6f, 0x6a,
    0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x76, 0x61, 0x75, 0x6c,
    0x74, 0x2e, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x22, 0x30, 0x0a, 0x0d, 0x50, 0x72, 0x6f,
    0x6a, 0x65, 0x63, 0x74, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x1f, 0x0a, 0x07, 0x70, 0x72,
    0x6f, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x76, 0x61,
    0x75, 0x6c, 0x74, 0x2e, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x22, 0x31, 0x0a, 0x0d, 0x50,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x12, 0x0a, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x18,
    0x0a, 0x0a, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x47, 0x65, 0x74, 0x12, 0x0a, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x15, 0x0a, 0x06, 0x56, 0x43, 0x53, 0x47,
    0x69, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x4a,
    0x98, 0x2e, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xb1, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x0d, 0x0a, 0x1b, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x08,
    0x01, 0x1a, 0x0f, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x04, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x05, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x05, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x06, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x06, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x06, 0x12, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x06, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x07, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x07, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x07,
    0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x0e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x10, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x11, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x14, 0x00,
    0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x14, 0x08, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x15, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x15, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x18, 0x00, 0x1b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x18, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1a, 0x1c, 0x1d, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1e, 0x00, 0x20,
    0x01, 0x1a, 0x1f, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1f, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1f, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x22, 0x00, 0x25, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x22, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x23, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x23, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x24, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x24, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x24, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x27, 0x00, 0x29,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x27, 0x08, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x28, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28,
    0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x00, 0x12, 0x03, 0x2c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x12,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x2d, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x2d, 0x1c, 0x1d, 0x0a, 0x6d, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x35, 0x00, 0x3e, 0x01,
    0x1a, 0x61, 0x20, 0x21, 0x21, 0x21, 0x4e, 0x4f, 0x54, 0x45, 0x21, 0x21, 0x21, 0x0a, 0x20, 0x21,
    0x21, 0x21, 0x4e, 0x4f, 0x54, 0x45, 0x21, 0x21, 0x21, 0x0a, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x69,
    0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x21, 0x21, 0x21, 0x4e,
    0x4f, 0x54, 0x45, 0x21, 0x21, 0x21, 0x0a, 0x20, 0x21, 0x21, 0x21, 0x4e, 0x4f, 0x54, 0x45, 0x21,
    0x21, 0x21, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x35, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x08, 0x00, 0x12, 0x04, 0x36, 0x02, 0x39, 0x03, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x08, 0x00, 0x01, 0x12, 0x03, 0x36, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x00, 0x12, 0x03, 0x37, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x37, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x37, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x38, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x38, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x38, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x08,
    0x01, 0x12, 0x04, 0x3a, 0x02, 0x3d, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x08, 0x01, 0x01,
    0x12, 0x03, 0x3a, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x3b,
    0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3b, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x0b, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x3c, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x3c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x3c, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x3c, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x40, 0x00, 0x42, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x40, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x00, 0x12, 0x03, 0x41, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x41, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41,
    0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x1d, 0x1e,
    0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x45, 0x00, 0x47, 0x01, 0x1a, 0x32, 0x20, 0x6c,
    0x69, 0x73, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20,
    0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x45, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x46, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x46, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x46, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x46, 0x1f,
    0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x49, 0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x49, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x00, 0x12, 0x03, 0x4a, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x4a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4a,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4a, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x4b, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x4b, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x4b, 0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x4b, 0x2a, 0x2b, 0x0a, 0x3d, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x4f, 0x00, 0x51, 0x01, 0x1a,
    0x31, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x70, 0x65, 0x6e, 0x64, 0x69,
    0x6e, 0x67, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x50, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x50, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x50, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x50, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x53, 0x00, 0x56, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x53, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x00, 0x12, 0x03, 0x54, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x54, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x54,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x54, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03, 0x55, 0x02, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x06, 0x12, 0x03, 0x55, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x55, 0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x55, 0x2a, 0x2b, 0x0a, 0x1b, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x59, 0x00, 0x66,
    0x01, 0x1a, 0x0f, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x59, 0x08, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x5a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x5a, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x5a, 0x17, 0x18, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x02, 0x21,
    0x1a, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x2f, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65,
    0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x02, 0x12, 0x03, 0x5e, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x5e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x12,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x21, 0x22, 0x0a,
    0x40, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x03, 0x61, 0x02, 0x20, 0x1a, 0x33, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x20, 0x69, 0x64, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x20, 0x69, 0x73, 0x20,
    0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x04, 0x12, 0x03, 0x61, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x05, 0x12, 0x03, 0x61, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x03, 0x61, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x03, 0x03, 0x12, 0x03, 0x61, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x04, 0x12, 0x03, 0x62, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x62, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x05, 0x12, 0x03, 0x62,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12, 0x03, 0x62, 0x12, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x03, 0x62, 0x20, 0x21, 0x0a, 0x33,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x05, 0x12, 0x03, 0x65, 0x02, 0x1f, 0x1a, 0x26, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x04, 0x12, 0x03, 0x65, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x05, 0x12, 0x03, 0x65, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x01, 0x12, 0x03, 0x65, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x05, 0x03, 0x12, 0x03, 0x65, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x10, 0x12, 0x04, 0x68, 0x00, 0x73, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03,
    0x68, 0x08, 0x1e, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x02, 0x21,
    0x1a, 0x18, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x62, 0x65, 0x69, 0x6e,
    0x67, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x6a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x6a, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6a,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x02, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x6b, 0x21, 0x22, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12,
    0x03, 0x6e, 0x02, 0x20, 0x1a, 0x33, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x20, 0x69, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x20, 0x69, 0x73, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e,
    0x76, 0x69, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x6e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x6e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x6e, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6e, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x03, 0x6f, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x03, 0x6f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x6f, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x6f, 0x20, 0x21, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x03,
    0x72, 0x02, 0x1f, 0x1a, 0x26, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x04, 0x04, 0x12, 0x03, 0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x72, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x72, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x72, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x04, 0x75, 0x00, 0x7a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x03, 0x75, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x00, 0x12, 0x03, 0x76, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x76, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x76, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76,
    0x12, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76, 0x2e, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x03, 0x77, 0x02, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x03, 0x77, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x05, 0x12, 0x03, 0x77, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x77, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x77, 0x1e, 0x1f, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x03, 0x79,
    0x02, 0x1b, 0x1a, 0x38, 0x20, 0x69, 0x66, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x20, 0x3d,
    0x3d, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x79, 0x27, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6a, 0x6f, 0x69, 0x6e, 0x69, 0x6e, 0x67,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x79, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x79, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x79, 0x19, 0x1a, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x03, 0x7c, 0x00, 0x2a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x26, 0x0a, 0x1c, 0x0a, 0x02, 0x04,
    0x13, 0x12, 0x05, 0x7f, 0x00, 0x8a, 0x01, 0x01, 0x1a, 0x0f, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x64, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x13, 0x01,
    0x12, 0x03, 0x7f, 0x08, 0x17, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0x81,
    0x01, 0x02, 0x19, 0x1a, 0x17, 0x20, 0x70, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x17, 0x18, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01,
    0x12, 0x04, 0x83, 0x01, 0x02, 0x20, 0x1a, 0x13, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0x83, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x83, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x83, 0x01, 0x1e, 0x1f, 0x0a, 0x18, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12,
    0x04, 0x85, 0x01, 0x02, 0x1b, 0x1a, 0x0a, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0x85, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05, 0x12, 0x04, 0x85, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0x85, 0x01, 0x12, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0x85, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0x86, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x03, 0x04, 0x12, 0x04, 0x86, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x03, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x86, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x86, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x04, 0x12,
    0x04, 0x87, 0x01, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x87, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x05, 0x12, 0x04, 0x87,
    0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0x87, 0x01,
    0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x03, 0x12, 0x04, 0x87, 0x01, 0x18,
    0x19, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x05, 0x12, 0x04, 0x89, 0x01, 0x02, 0x1f, 0x1a,
    0x20, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x04, 0x12, 0x04, 0x89, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x05, 0x12, 0x04, 0x89, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x01, 0x12, 0x04, 0x89, 0x01, 0x12, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x03, 0x12, 0x04, 0x89, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x14, 0x12, 0x06, 0x8c, 0x01, 0x00, 0x92, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x14, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00,
    0x12, 0x04, 0x8d, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x8d, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x8d, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8d,
    0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8d, 0x01,
    0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x02, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8e, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x02, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x8f, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04,
    0x90, 0x01, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x04, 0x90,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x05, 0x12, 0x04, 0x90, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0x90, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0x90, 0x01, 0x18, 0x19,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x04, 0x12, 0x04, 0x91, 0x01, 0x02, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x04, 0x91, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x04, 0x05, 0x12, 0x04, 0x91, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x04, 0x01, 0x12, 0x04, 0x91, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x04, 0x03, 0x12, 0x04, 0x91, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15,
    0x12, 0x06, 0x94, 0x01, 0x00, 0x9b, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12,
    0x04, 0x94, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x95,
    0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0x95, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0x95, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x12, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x95, 0x01, 0x17, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0x96, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0x96, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x96, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x02, 0x12, 0x04, 0x97, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x97, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12,
    0x04, 0x97, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x97, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0x97,
    0x01, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x15, 0x08, 0x00, 0x12, 0x06, 0x98, 0x01, 0x02,
    0x9a, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x08, 0x00, 0x01, 0x12, 0x04, 0x98, 0x01,
    0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0x99, 0x01, 0x04, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06, 0x12, 0x04, 0x99, 0x01, 0x04, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0x99, 0x01, 0x0b, 0x0e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0x99, 0x01, 0x11, 0x12, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x16, 0x12, 0x06, 0x9d, 0x01, 0x00, 0xa0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x16, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00,
    0x12, 0x04, 0x9e, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x9e, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x9e, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9e,
    0x01, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x01,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9f, 0x01, 0x0b, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x17, 0x12, 0x06, 0xa2, 0x01, 0x00, 0xa4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17,
    0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12,
    0x04, 0xa3, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xa3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa3,
    0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x01,
    0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x1d,
    0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xa6, 0x01, 0x00, 0xa9, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xa7, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa7, 0x01, 0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa7, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04,
    0xa8, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa8,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa8, 0x01,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x12,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x21, 0x22,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xab, 0x01, 0x00, 0xad, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x19, 0x02, 0x00, 0x12, 0x04, 0xac, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xac, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xac, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xac, 0x01, 0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xac, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xaf, 0x01, 0x00,
    0xb1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x01, 0x02, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x18, 0x19,
];

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
