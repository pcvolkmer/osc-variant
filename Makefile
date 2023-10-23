ifndef VERBOSE
.SILENT:
endif

TAG = `git describe --tag 2>/dev/null`

REV = git`git rev-parse HEAD | cut -c1-7`

package-all: win-package linux-package

.PHONY: win-package
win-package: win-binary-x86_64
	mkdir osc-variant || true
	cp target/x86_64-pc-windows-gnu/release/osc-variant.exe osc-variant/
	cp -r examples osc-variant/
	cp README.md osc-variant/
	cp LICENSE.txt osc-variant/
	zip osc-variant-$(TAG)_win64.zip osc-variant/* osc-variant/examples/*
	rm -rf osc-variant || true

.PHONY: linux-package
linux-package: linux-binary-x86_64
	mkdir osc-variant || true
	cp target/x86_64-unknown-linux-gnu/release/osc-variant osc-variant/
	cp -r examples osc-variant/
	cp README.md osc-variant/
	cp LICENSE.txt osc-variant/
	tar -czvf osc-variant-$(TAG)_linux.tar.gz osc-variant/
	rm -rf osc-variant || true

binary-all: win-binary-x86_64 linux-binary-x86_64

.PHONY: win-binary-x86_64
win-binary-x86_64:
	cargo build --release --target=x86_64-pc-windows-gnu --features unzip-osb

.PHONY: linux-binary-x86_64
linux-binary-x86_64:
	cargo build --release --target=x86_64-unknown-linux-gnu --features unzip-osb

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean
	rm -rf osc-variant 2>/dev/null || true
	rm *_win64.zip 2>/dev/null || true
	rm *_linux.tar.gz 2>/dev/null || true
