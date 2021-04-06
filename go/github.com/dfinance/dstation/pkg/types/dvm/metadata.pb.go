// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0-devel
// 	protoc        v3.13.0
// source: dfinance/dvm/metadata.proto

package dvm

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// Bytecode.
type Bytecode struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Code []byte `protobuf:"bytes,1,opt,name=code,proto3" json:"code,omitempty"` // bytecode of script
}

func (x *Bytecode) Reset() {
	*x = Bytecode{}
	if protoimpl.UnsafeEnabled {
		mi := &file_dfinance_dvm_metadata_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Bytecode) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Bytecode) ProtoMessage() {}

func (x *Bytecode) ProtoReflect() protoreflect.Message {
	mi := &file_dfinance_dvm_metadata_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Bytecode.ProtoReflect.Descriptor instead.
func (*Bytecode) Descriptor() ([]byte, []int) {
	return file_dfinance_dvm_metadata_proto_rawDescGZIP(), []int{0}
}

func (x *Bytecode) GetCode() []byte {
	if x != nil {
		return x.Code
	}
	return nil
}

// Struct field.
type Field struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	Type string `protobuf:"bytes,2,opt,name=type,proto3" json:"type,omitempty"`
}

func (x *Field) Reset() {
	*x = Field{}
	if protoimpl.UnsafeEnabled {
		mi := &file_dfinance_dvm_metadata_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Field) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Field) ProtoMessage() {}

func (x *Field) ProtoReflect() protoreflect.Message {
	mi := &file_dfinance_dvm_metadata_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Field.ProtoReflect.Descriptor instead.
func (*Field) Descriptor() ([]byte, []int) {
	return file_dfinance_dvm_metadata_proto_rawDescGZIP(), []int{1}
}

func (x *Field) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Field) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

/// Struct representation.
type Struct struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name           string   `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	IsResource     bool     `protobuf:"varint,2,opt,name=isResource,proto3" json:"isResource,omitempty"`
	TypeParameters []string `protobuf:"bytes,3,rep,name=type_parameters,json=typeParameters,proto3" json:"type_parameters,omitempty"`
	Field          []*Field `protobuf:"bytes,4,rep,name=field,proto3" json:"field,omitempty"`
}

func (x *Struct) Reset() {
	*x = Struct{}
	if protoimpl.UnsafeEnabled {
		mi := &file_dfinance_dvm_metadata_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Struct) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Struct) ProtoMessage() {}

func (x *Struct) ProtoReflect() protoreflect.Message {
	mi := &file_dfinance_dvm_metadata_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Struct.ProtoReflect.Descriptor instead.
func (*Struct) Descriptor() ([]byte, []int) {
	return file_dfinance_dvm_metadata_proto_rawDescGZIP(), []int{2}
}

func (x *Struct) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Struct) GetIsResource() bool {
	if x != nil {
		return x.IsResource
	}
	return false
}

func (x *Struct) GetTypeParameters() []string {
	if x != nil {
		return x.TypeParameters
	}
	return nil
}

func (x *Struct) GetField() []*Field {
	if x != nil {
		return x.Field
	}
	return nil
}

/// Function representation.
type Function struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name           string   `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	IsPublic       bool     `protobuf:"varint,2,opt,name=isPublic,proto3" json:"isPublic,omitempty"`
	IsNative       bool     `protobuf:"varint,3,opt,name=isNative,proto3" json:"isNative,omitempty"`
	TypeParameters []string `protobuf:"bytes,4,rep,name=type_parameters,json=typeParameters,proto3" json:"type_parameters,omitempty"`
	Arguments      []string `protobuf:"bytes,5,rep,name=arguments,proto3" json:"arguments,omitempty"`
	Returns        []string `protobuf:"bytes,6,rep,name=returns,proto3" json:"returns,omitempty"`
}

func (x *Function) Reset() {
	*x = Function{}
	if protoimpl.UnsafeEnabled {
		mi := &file_dfinance_dvm_metadata_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Function) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Function) ProtoMessage() {}

func (x *Function) ProtoReflect() protoreflect.Message {
	mi := &file_dfinance_dvm_metadata_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Function.ProtoReflect.Descriptor instead.
func (*Function) Descriptor() ([]byte, []int) {
	return file_dfinance_dvm_metadata_proto_rawDescGZIP(), []int{3}
}

func (x *Function) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Function) GetIsPublic() bool {
	if x != nil {
		return x.IsPublic
	}
	return false
}

func (x *Function) GetIsNative() bool {
	if x != nil {
		return x.IsNative
	}
	return false
}

func (x *Function) GetTypeParameters() []string {
	if x != nil {
		return x.TypeParameters
	}
	return nil
}

func (x *Function) GetArguments() []string {
	if x != nil {
		return x.Arguments
	}
	return nil
}

func (x *Function) GetReturns() []string {
	if x != nil {
		return x.Returns
	}
	return nil
}

// Script metadata.
type ScriptMeta struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	SignersCount   uint32      `protobuf:"varint,1,opt,name=signers_count,json=signersCount,proto3" json:"signers_count,omitempty"`
	TypeParameters []string    `protobuf:"bytes,2,rep,name=type_parameters,json=typeParameters,proto3" json:"type_parameters,omitempty"`
	Arguments      []VMTypeTag `protobuf:"varint,3,rep,packed,name=arguments,proto3,enum=dfinance.dvm.VMTypeTag" json:"arguments,omitempty"`
}

func (x *ScriptMeta) Reset() {
	*x = ScriptMeta{}
	if protoimpl.UnsafeEnabled {
		mi := &file_dfinance_dvm_metadata_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ScriptMeta) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ScriptMeta) ProtoMessage() {}

func (x *ScriptMeta) ProtoReflect() protoreflect.Message {
	mi := &file_dfinance_dvm_metadata_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ScriptMeta.ProtoReflect.Descriptor instead.
func (*ScriptMeta) Descriptor() ([]byte, []int) {
	return file_dfinance_dvm_metadata_proto_rawDescGZIP(), []int{4}
}

func (x *ScriptMeta) GetSignersCount() uint32 {
	if x != nil {
		return x.SignersCount
	}
	return 0
}

func (x *ScriptMeta) GetTypeParameters() []string {
	if x != nil {
		return x.TypeParameters
	}
	return nil
}

func (x *ScriptMeta) GetArguments() []VMTypeTag {
	if x != nil {
		return x.Arguments
	}
	return nil
}

// Module metadata.
type ModuleMeta struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name      string      `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`           // module name.
	Types     []*Struct   `protobuf:"bytes,2,rep,name=types,proto3" json:"types,omitempty"`         // Types defined in a module.
	Functions []*Function `protobuf:"bytes,3,rep,name=functions,proto3" json:"functions,omitempty"` // Functions defined in a module.
}

func (x *ModuleMeta) Reset() {
	*x = ModuleMeta{}
	if protoimpl.UnsafeEnabled {
		mi := &file_dfinance_dvm_metadata_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ModuleMeta) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ModuleMeta) ProtoMessage() {}

func (x *ModuleMeta) ProtoReflect() protoreflect.Message {
	mi := &file_dfinance_dvm_metadata_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ModuleMeta.ProtoReflect.Descriptor instead.
func (*ModuleMeta) Descriptor() ([]byte, []int) {
	return file_dfinance_dvm_metadata_proto_rawDescGZIP(), []int{5}
}

func (x *ModuleMeta) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *ModuleMeta) GetTypes() []*Struct {
	if x != nil {
		return x.Types
	}
	return nil
}

func (x *ModuleMeta) GetFunctions() []*Function {
	if x != nil {
		return x.Functions
	}
	return nil
}

// Bytecode metadata.
type Metadata struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Meta:
	//	*Metadata_Script
	//	*Metadata_Module
	Meta isMetadata_Meta `protobuf_oneof:"meta"`
}

func (x *Metadata) Reset() {
	*x = Metadata{}
	if protoimpl.UnsafeEnabled {
		mi := &file_dfinance_dvm_metadata_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Metadata) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Metadata) ProtoMessage() {}

func (x *Metadata) ProtoReflect() protoreflect.Message {
	mi := &file_dfinance_dvm_metadata_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Metadata.ProtoReflect.Descriptor instead.
func (*Metadata) Descriptor() ([]byte, []int) {
	return file_dfinance_dvm_metadata_proto_rawDescGZIP(), []int{6}
}

func (m *Metadata) GetMeta() isMetadata_Meta {
	if m != nil {
		return m.Meta
	}
	return nil
}

func (x *Metadata) GetScript() *ScriptMeta {
	if x, ok := x.GetMeta().(*Metadata_Script); ok {
		return x.Script
	}
	return nil
}

func (x *Metadata) GetModule() *ModuleMeta {
	if x, ok := x.GetMeta().(*Metadata_Module); ok {
		return x.Module
	}
	return nil
}

type isMetadata_Meta interface {
	isMetadata_Meta()
}

type Metadata_Script struct {
	// In case the provided bytecode is a script.
	Script *ScriptMeta `protobuf:"bytes,1,opt,name=script,proto3,oneof"`
}

type Metadata_Module struct {
	// In case the provided bytecode is a module.
	Module *ModuleMeta `protobuf:"bytes,2,opt,name=module,proto3,oneof"`
}

func (*Metadata_Script) isMetadata_Meta() {}

func (*Metadata_Module) isMetadata_Meta() {}

var File_dfinance_dvm_metadata_proto protoreflect.FileDescriptor

var file_dfinance_dvm_metadata_proto_rawDesc = []byte{
	0x0a, 0x1b, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2f, 0x64, 0x76, 0x6d, 0x2f, 0x6d,
	0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x64,
	0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x64, 0x76, 0x6d, 0x1a, 0x1f, 0x64, 0x66, 0x69,
	0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2f, 0x64, 0x76, 0x6d, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
	0x2d, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1e, 0x0a, 0x08,
	0x42, 0x79, 0x74, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x22, 0x2f, 0x0a, 0x05,
	0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70,
	0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x90, 0x01,
	0x0a, 0x06, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1e, 0x0a, 0x0a,
	0x69, 0x73, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08,
	0x52, 0x0a, 0x69, 0x73, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x27, 0x0a, 0x0f,
	0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18,
	0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0e, 0x74, 0x79, 0x70, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d,
	0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x29, 0x0a, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04,
	0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e,
	0x64, 0x76, 0x6d, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x52, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64,
	0x22, 0xb7, 0x01, 0x0a, 0x08, 0x46, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a,
	0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x12, 0x1a, 0x0a, 0x08, 0x69, 0x73, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x08, 0x52, 0x08, 0x69, 0x73, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x12, 0x1a, 0x0a,
	0x08, 0x69, 0x73, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52,
	0x08, 0x69, 0x73, 0x4e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x74, 0x79, 0x70,
	0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x04, 0x20, 0x03,
	0x28, 0x09, 0x52, 0x0e, 0x74, 0x79, 0x70, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65,
	0x72, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18,
	0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73,
	0x12, 0x18, 0x0a, 0x07, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28,
	0x09, 0x52, 0x07, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x22, 0x91, 0x01, 0x0a, 0x0a, 0x53,
	0x63, 0x72, 0x69, 0x70, 0x74, 0x4d, 0x65, 0x74, 0x61, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x69, 0x67,
	0x6e, 0x65, 0x72, 0x73, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
	0x52, 0x0c, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x72, 0x73, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x27,
	0x0a, 0x0f, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72,
	0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0e, 0x74, 0x79, 0x70, 0x65, 0x50, 0x61, 0x72,
	0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x35, 0x0a, 0x09, 0x61, 0x72, 0x67, 0x75, 0x6d,
	0x65, 0x6e, 0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x66, 0x69,
	0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x64, 0x76, 0x6d, 0x2e, 0x56, 0x4d, 0x54, 0x79, 0x70, 0x65,
	0x54, 0x61, 0x67, 0x52, 0x09, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x22, 0x82,
	0x01, 0x0a, 0x0a, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x12, 0x12, 0x0a,
	0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x12, 0x2a, 0x0a, 0x05, 0x74, 0x79, 0x70, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
	0x32, 0x14, 0x2e, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x64, 0x76, 0x6d, 0x2e,
	0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x74, 0x79, 0x70, 0x65, 0x73, 0x12, 0x34, 0x0a,
	0x09, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
	0x32, 0x16, 0x2e, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x64, 0x76, 0x6d, 0x2e,
	0x46, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x09, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69,
	0x6f, 0x6e, 0x73, 0x22, 0x7a, 0x0a, 0x08, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12,
	0x32, 0x0a, 0x06, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x18, 0x2e, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x64, 0x76, 0x6d, 0x2e, 0x53,
	0x63, 0x72, 0x69, 0x70, 0x74, 0x4d, 0x65, 0x74, 0x61, 0x48, 0x00, 0x52, 0x06, 0x73, 0x63, 0x72,
	0x69, 0x70, 0x74, 0x12, 0x32, 0x0a, 0x06, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x64,
	0x76, 0x6d, 0x2e, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x48, 0x00, 0x52,
	0x06, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x42, 0x06, 0x0a, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x32,
	0x56, 0x0a, 0x13, 0x44, 0x56, 0x4d, 0x42, 0x79, 0x74, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x4d, 0x65,
	0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x3f, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x4d, 0x65, 0x74,
	0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x16, 0x2e, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65,
	0x2e, 0x64, 0x76, 0x6d, 0x2e, 0x42, 0x79, 0x74, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x1a, 0x16, 0x2e,
	0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x64, 0x76, 0x6d, 0x2e, 0x4d, 0x65, 0x74,
	0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x00, 0x42, 0x2c, 0x5a, 0x2a, 0x67, 0x69, 0x74, 0x68, 0x75,
	0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x64, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x2f, 0x64,
	0x73, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x70, 0x6b, 0x67, 0x2f, 0x74, 0x79, 0x70, 0x65,
	0x73, 0x2f, 0x64, 0x76, 0x6d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_dfinance_dvm_metadata_proto_rawDescOnce sync.Once
	file_dfinance_dvm_metadata_proto_rawDescData = file_dfinance_dvm_metadata_proto_rawDesc
)

func file_dfinance_dvm_metadata_proto_rawDescGZIP() []byte {
	file_dfinance_dvm_metadata_proto_rawDescOnce.Do(func() {
		file_dfinance_dvm_metadata_proto_rawDescData = protoimpl.X.CompressGZIP(file_dfinance_dvm_metadata_proto_rawDescData)
	})
	return file_dfinance_dvm_metadata_proto_rawDescData
}

var file_dfinance_dvm_metadata_proto_msgTypes = make([]protoimpl.MessageInfo, 7)
var file_dfinance_dvm_metadata_proto_goTypes = []interface{}{
	(*Bytecode)(nil),   // 0: dfinance.dvm.Bytecode
	(*Field)(nil),      // 1: dfinance.dvm.Field
	(*Struct)(nil),     // 2: dfinance.dvm.Struct
	(*Function)(nil),   // 3: dfinance.dvm.Function
	(*ScriptMeta)(nil), // 4: dfinance.dvm.ScriptMeta
	(*ModuleMeta)(nil), // 5: dfinance.dvm.ModuleMeta
	(*Metadata)(nil),   // 6: dfinance.dvm.Metadata
	(VMTypeTag)(0),     // 7: dfinance.dvm.VMTypeTag
}
var file_dfinance_dvm_metadata_proto_depIdxs = []int32{
	1, // 0: dfinance.dvm.Struct.field:type_name -> dfinance.dvm.Field
	7, // 1: dfinance.dvm.ScriptMeta.arguments:type_name -> dfinance.dvm.VMTypeTag
	2, // 2: dfinance.dvm.ModuleMeta.types:type_name -> dfinance.dvm.Struct
	3, // 3: dfinance.dvm.ModuleMeta.functions:type_name -> dfinance.dvm.Function
	4, // 4: dfinance.dvm.Metadata.script:type_name -> dfinance.dvm.ScriptMeta
	5, // 5: dfinance.dvm.Metadata.module:type_name -> dfinance.dvm.ModuleMeta
	0, // 6: dfinance.dvm.DVMBytecodeMetadata.GetMetadata:input_type -> dfinance.dvm.Bytecode
	6, // 7: dfinance.dvm.DVMBytecodeMetadata.GetMetadata:output_type -> dfinance.dvm.Metadata
	7, // [7:8] is the sub-list for method output_type
	6, // [6:7] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_dfinance_dvm_metadata_proto_init() }
func file_dfinance_dvm_metadata_proto_init() {
	if File_dfinance_dvm_metadata_proto != nil {
		return
	}
	file_dfinance_dvm_common_types_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_dfinance_dvm_metadata_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Bytecode); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_dfinance_dvm_metadata_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Field); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_dfinance_dvm_metadata_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Struct); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_dfinance_dvm_metadata_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Function); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_dfinance_dvm_metadata_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ScriptMeta); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_dfinance_dvm_metadata_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ModuleMeta); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_dfinance_dvm_metadata_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Metadata); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_dfinance_dvm_metadata_proto_msgTypes[6].OneofWrappers = []interface{}{
		(*Metadata_Script)(nil),
		(*Metadata_Module)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_dfinance_dvm_metadata_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   7,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_dfinance_dvm_metadata_proto_goTypes,
		DependencyIndexes: file_dfinance_dvm_metadata_proto_depIdxs,
		MessageInfos:      file_dfinance_dvm_metadata_proto_msgTypes,
	}.Build()
	File_dfinance_dvm_metadata_proto = out.File
	file_dfinance_dvm_metadata_proto_rawDesc = nil
	file_dfinance_dvm_metadata_proto_goTypes = nil
	file_dfinance_dvm_metadata_proto_depIdxs = nil
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConnInterface

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion6

// DVMBytecodeMetadataClient is the client API for DVMBytecodeMetadata service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type DVMBytecodeMetadataClient interface {
	GetMetadata(ctx context.Context, in *Bytecode, opts ...grpc.CallOption) (*Metadata, error)
}

type dVMBytecodeMetadataClient struct {
	cc grpc.ClientConnInterface
}

func NewDVMBytecodeMetadataClient(cc grpc.ClientConnInterface) DVMBytecodeMetadataClient {
	return &dVMBytecodeMetadataClient{cc}
}

func (c *dVMBytecodeMetadataClient) GetMetadata(ctx context.Context, in *Bytecode, opts ...grpc.CallOption) (*Metadata, error) {
	out := new(Metadata)
	err := c.cc.Invoke(ctx, "/dfinance.dvm.DVMBytecodeMetadata/GetMetadata", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// DVMBytecodeMetadataServer is the server API for DVMBytecodeMetadata service.
type DVMBytecodeMetadataServer interface {
	GetMetadata(context.Context, *Bytecode) (*Metadata, error)
}

// UnimplementedDVMBytecodeMetadataServer can be embedded to have forward compatible implementations.
type UnimplementedDVMBytecodeMetadataServer struct {
}

func (*UnimplementedDVMBytecodeMetadataServer) GetMetadata(context.Context, *Bytecode) (*Metadata, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetMetadata not implemented")
}

func RegisterDVMBytecodeMetadataServer(s *grpc.Server, srv DVMBytecodeMetadataServer) {
	s.RegisterService(&_DVMBytecodeMetadata_serviceDesc, srv)
}

func _DVMBytecodeMetadata_GetMetadata_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(Bytecode)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(DVMBytecodeMetadataServer).GetMetadata(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/dfinance.dvm.DVMBytecodeMetadata/GetMetadata",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(DVMBytecodeMetadataServer).GetMetadata(ctx, req.(*Bytecode))
	}
	return interceptor(ctx, in, info, handler)
}

var _DVMBytecodeMetadata_serviceDesc = grpc.ServiceDesc{
	ServiceName: "dfinance.dvm.DVMBytecodeMetadata",
	HandlerType: (*DVMBytecodeMetadataServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetMetadata",
			Handler:    _DVMBytecodeMetadata_GetMetadata_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "dfinance/dvm/metadata.proto",
}