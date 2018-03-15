CARGO=cargo
CARGO_FLAGS=

ifneq ($(MODE),debug)
	TARGET=target/release/effitask
	CARGO_FLAGS+=--release
else
	TARGET=target/debug/effitask
endif

all: build

build: gtk+-3.0
	$(CARGO) build $(CARGO_FLAGS)

gtk+-3.0:
	@if ! pkg-config $@; then \
		printf '%s not installed\n' "$@" >&2; \
		exit 1; \
	fi

install:
	install --directory $(PREFIX)/usr/bin
	install $(TARGET) $(PREFIX)/usr/bin/
	install --directory $(PREFIX)/usr/share/effitask
	install --mode 644 resources/*.png $(PREFIX)/usr/share/effitask/
	install --mode 644 resources/*.css $(PREFIX)/usr/share/effitask/

test:
	$(CARGO) test $(CARGO_FLAGS)

.PHONY: all build install test
