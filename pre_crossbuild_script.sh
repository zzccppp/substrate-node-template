# dpkg --add-architecture arm64 && apt-get update && apt-get install --assume-yes zlib1g-dev:arm64 unzip git clang-5.0 curl libssl-dev llvm-5.0 libudev-dev make
dpkg --add-architecture arm64 && apt-get update && apt-get install --assume-yes zlib1g-dev:arm64 unzip git clang curl libssl-dev llvm libudev-dev make llvm-dev libclang-dev
PROTOC_VERSION=$(curl -s "https://api.github.com/repos/protocolbuffers/protobuf/releases/latest" | grep -Po '"tag_name": "v\K[0-9.]+')
curl -Lo protoc.zip "https://github.com/protocolbuffers/protobuf/releases/latest/download/protoc-${PROTOC_VERSION}-linux-x86_64.zip"
unzip -q protoc.zip bin/protoc -d /usr/local
chmod a+x /usr/local/bin/protoc
