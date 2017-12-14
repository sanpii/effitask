CARGO=cargo
CARGO_FLAGS=

ifneq ($(MODE),debug)
	TARGET=target/release/effitask
	CARGO_FLAGS+=--release
else
	TARGET=target/debug/effitask
endif

all: build

build:
	$(CARGO) build $(CARGO_FLAGS)

install:
	install --directory $(PREFIX)/usr/bin
	install $(TARGET) $(PREFIX)/usr/bin/
	install --directory $(PREFIX)/usr/share/effitask
	install --mode 644 resources/*.png $(PREFIX)/usr/share/effitask/
	install --mode 644 resources/*.css $(PREFIX)/usr/share/effitask/

.PHONY: all build install
