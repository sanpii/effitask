CARGO=cargo
CARGO_FLAGS=

ifneq ($(MODE),debug)
	CARGO_FLAGS+=--release
endif

all: build

build:
	$(CARGO) build $(CARGO_FLAGS)

install:
	install --directory $(PREFIX)/usr/share/effitask
	install --mode 644 resources/*.png $(PREFIX)/usr/share/effitask/
	install --mode 644 resources/*.css $(PREFIX)/usr/share/effitask/

.PHONY: all build install
