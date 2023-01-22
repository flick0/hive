clear:
	rm -rf ./target

plugin:
	cargo build --release --all

build:
	cargo build --release

install: all
	sudo cp target/release/hive /usr/bin/hive
	sudo mkdir -p /usr/lib/hive/plugins

	sudo cp target/release/bee-hyprland /usr/lib/hive/plugins

uninstall:
	sudo rm -rf /usr/bin/hive
	sudo rm -rf /usr/lib/hive

all: clear plugin
 
help:
	@echo "usage: make [build|install|all|help]"
