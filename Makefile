NAME = helm-lintr
BINDIR = target/release

HELM_HOME ?= $(shell helm home)
HELM_PLUGIN_DIR ?= $(HELM_HOME)/plugins/$(NAME)
VERSION := $(shell sed -n -e 's/version:[ "]*\([^"]*\).*/\1/p' plugin.yaml)

# architecture
HOST_ARCH := $(shell uname -m)
ifeq ($(HOST_ARCH),x86_64)
    HOST_ARCH := amd64
else ifneq ($(HOST_ARCH),i386)
    $(error ERROR - unsupported value $(HOST_ARCH) for HOST_ARCH!)
endif

# operating system
HOST_OS := $(shell uname -s 2>/dev/null | tr "[:upper:]" "[:lower:]")
ifeq (,$(filter $(HOST_OS),linux darwin))
    $(error ERROR - unsupported value $(HOST_OS) for HOST_OS!)
endif

DIST_DIR = dist
DIST_TARGET_DIR = $(DIST_DIR)/$(HOST_OS)-$(HOST_ARCH)

build:
	cargo build --release

clean:
	rm -rf target/
	rm -rf $(DIST_DIR)

install:
	install -D plugin.yaml $(HELM_PLUGIN_DIR)/plugin.yaml
	install -D -m0755 $(BINDIR)/$(NAME) $(HELM_PLUGIN_DIR)/bin/$(NAME)

uninstall:
	rm -rf $(HELM_PLUGIN_DIR)

dist:
	mkdir -p $(DIST_TARGET_DIR) $(DIST_TARGET_DIR)/bin
	cp LICENSE $(DIST_TARGET_DIR)
	cp README.md $(DIST_TARGET_DIR)
	cp plugin.yaml $(DIST_TARGET_DIR)
	cp $(BINDIR)/$(NAME) $(DIST_TARGET_DIR)/bin/$(NAME)
	cd $(DIST_DIR) && tar -zcf $(NAME)-$(VERSION)-$(HOST_OS)-$(HOST_ARCH).tar.gz $(HOST_OS)-$(HOST_ARCH)

.PHONY: build clean install uninstall dist
