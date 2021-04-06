PROTO_IN_DIR=./proto/dfinance/dvm

GO_PROTO_OUT_DIR=./go

all: deps gen-go

gen-go:
	mkdir -p ${GO_PROTO_OUT_DIR}

	protoc --proto_path=./proto --go_out=plugins=grpc:$(GO_PROTO_OUT_DIR) $(PROTO_IN_DIR)/*.proto

deps:
	go get -u github.com/golang/protobuf/protoc-gen-go
