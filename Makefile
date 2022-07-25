all: build

build: models
	cargo build

models: cleanmodel smithy/model/common.smithy smithy/model/main.smithy smithy/model/post.smithy
	cd smithy && ./gradlew build
	sed -i '' -e 's/version = "99.99.99"/git = "https:\/\/github.com\/eduardomourar\/smithy-rs"\nrev = "e05b302b058e21cc4d2f4d4acd63b3dd6b734972"/' smithy/build/output/bison-rates/rust-server-codegen/Cargo.toml
	rsync -a --delete smithy/build/output/bison-rates/rust-server-codegen/ crates/sdk-server-bison-rates
cleanmodel:
	rm -rf smithy/build
