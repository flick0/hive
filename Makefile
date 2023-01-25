clear:
	rm -rf ./target

plugin:
	cargo build --release --all

build:
	cargo build --release

install: plugin
	sudo cp target/release/hive /usr/bin/
	sudo cp target/release/hive-daemon /usr/bin/
	sudo mkdir -p /usr/lib/hive/plugins

	sudo cp target/release/libhyprland_hive.so /usr/lib/hive/plugins

uninstall:
	sudo rm -rf /usr/bin/hive
	sudo rm -rf /usr/lib/hive
	sudo rm -rf /usr/bin/hive-daemon

all: clear plugin
 
help:
	@echo "usage: make [build|install|all|help]"
