PROTO_IN_DIR=./protos/

GO_PROTO_OUT_TYPES_DIR=./go/types_grpc/
GO_PROTO_OUT_COMPILER_DIR=./go/compiler_grpc/
GO_PROTO_OUT_DS_DIR=./go/ds_grpc/
GO_PROTO_OUT_METADATA_DIR=./go/metadata_grpc/
GO_PROTO_OUT_VM_DIR=./go/vm_grpc/
GO_PROTO_OUT_DATA_DIR=./go/data_grpc/

PROTOBUF_TYPES_FILES=./protos/common-types.proto
PROTOBUF_COMPILER_FILES=./protos/compiler.proto
PROTOBUF_DS_FILES=./protos/data-source.proto
PROTOBUF_METADATA_FILES=./protos/metadata.proto
PROTOBUF_VM_FILES=./protos/vm.proto
PROTOBUF_DATA_FILES=./protos/data-service.proto

all: deps gen-go

gen-go:
	mkdir -p ${GO_PROTO_OUT_TYPES_DIR}
	mkdir -p ${GO_PROTO_OUT_COMPILER_DIR}
	mkdir -p ${GO_PROTO_OUT_DS_DIR}
	mkdir -p ${GO_PROTO_OUT_METADATA_DIR}
	mkdir -p ${GO_PROTO_OUT_VM_DIR}

	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_COMPILER_DIR) $(PROTOBUF_COMPILER_FILES)
	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_DS_DIR) $(PROTOBUF_DS_FILES)
	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_METADATA_DIR) $(PROTOBUF_METADATA_FILES)
	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_VM_DIR) $(PROTOBUF_VM_FILES)
	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_TYPES_DIR) --go_opt=paths=source_relative  $(PROTOBUF_TYPES_FILES)
	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_DATA_DIR) $(PROTOBUF_DATA_FILES)

deps:
	@echo "  >  Checking if there is any missing dependencies..."
	go get -u github.com/golang/protobuf/protoc-gen-go
	(cd ./go/ && go mod verify)
