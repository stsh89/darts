# Web

## Start

```
puma -I ./lib -p 3000
```

## Compile proto definitions

```
grpc_tools_ruby_protoc \
    --proto_path ../protobuf \
    --ruby_out=lib \
    --grpc_out=lib \
    ../protobuf/proto/games.proto
```

## Start mock server

```
ruby -I ./lib ./grpc_server_mock/main.rb
```
