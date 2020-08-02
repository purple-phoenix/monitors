install: check-env
	cargo install --path .

check-env:
ifndef MONITOR_CONFIG_DIR
	$(error MONITOR_CONFIG_DIR is undefined pass with make install MONITOR_CONFIG_DIR=xxxx)
endif
