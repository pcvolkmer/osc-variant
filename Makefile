ifndef VERBOSE
.SILENT:
endif

GITTAG = $(shell git describe --tag --abbrev=0 2>/dev/null | sed -En 's/v(.*)$$/\1/p')
ifeq ($(findstring -, $(GITTAG)), -)
    GITDEV = $(shell git describe --tag 2>/dev/null | sed -En 's/v(.*)-([0-9]+)-g([0-9a-f]+)$$/.dev.\2+\3/p')
else
    GITDEV = $(shell git describe --tag 2>/dev/null | sed -En 's/v(.*)-([0-9]+)-g([0-9a-f]+)$$/-dev.\2+\3/p')
endif
VERSION := "$(GITTAG)$(GITDEV)"

package-all: win-package linux-package

.PHONY: win-package
win-package: win-binary-x86_64
	mkdir osc-variant || true
	cp target/x86_64-pc-windows-gnu/release/osc-variant.exe osc-variant/
	cp -r examples osc-variant/
	cp README.md osc-variant/
	cp LICENSE.txt osc-variant/
	# first try (linux) zip command, then powershell sub command to create ZIP file
	zip osc-variant-$(VERSION)_win64.zip osc-variant/* osc-variant/examples/* || powershell Compress-ARCHIVE osc-variant osc-variant-$(VERSION)_win64.zip
	rm -rf osc-variant || true

.PHONY: linux-package
linux-package: linux-binary-x86_64
	mkdir osc-variant || true
	cp target/x86_64-unknown-linux-gnu/release/osc-variant osc-variant/
	cp -r examples osc-variant/
	cp README.md osc-variant/
	cp LICENSE.txt osc-variant/
	tar -czvf osc-variant-$(VERSION)_linux.tar.gz osc-variant/
	rm -rf osc-variant || true

.PHONY: linux-deb
linux-deb: linux-binary-x86_64
	cargo deb --no-build --strip --target=x86_64-unknown-linux-gnu --output=.

.PHONY: linux-rpm
linux-rpm: linux-binary-x86_64
	cargo generate-rpm --target=x86_64-unknown-linux-gnu --output=.

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
