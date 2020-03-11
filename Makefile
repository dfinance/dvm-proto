PROTO_IN_DIR=./protos/
GO_PROTO_OUT_VM_DIR=./go/vm_grpc/
GO_PROTO_OUT_DS_DIR=./go/ds_grpc/
PROTOBUF_VM_FILES=./protos/vm.proto
PROTOBUF_DS_FILES=./protos/data-source.proto

all: deps gen-go

gen-go:
	mkdir -p ${GO_PROTO_OUT_VM_DIR}
	mkdir -p ${GO_PROTO_OUT_DS_DIR}
	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_VM_DIR) $(PROTOBUF_VM_FILES)
	protoc -I ${PROTO_IN_DIR} --go_out=plugins=grpc:$(GO_PROTO_OUT_DS_DIR) $(PROTOBUF_DS_FILES)

deps:
	@echo "  >  Checking if there is any missing dependencies..."
	go get -u github.com/golang/protobuf/protoc-gen-go
	(cd ./go/ && go mod verify)
