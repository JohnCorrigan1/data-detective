MAINNET ?= eth.substreams.pinax.network:443
# MAINNET ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 18000000

OTHER_BLOCK ?= 14159918
PUDGY ?= 12876179
MILADY ?= 13090020
PIXELMON ?= 14154677
STOP_BLOCK ?= +100000

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(MAINNET) substreams.yaml map_deployments -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: runrpc
runrpc: build
	substreams run -e $(MAINNET) substreams.yaml map_deployments_rpc -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(MAINNET) substreams.yaml map_deployments -s $(START_BLOCK) -t $(STOP_BLOCK) --production-mode

.PHONY: guirpc
guirpc: build
	substreams gui -e $(MAINNET) substreams.yaml map_deployments_rpc -s $(START_BLOCK) -t $(STOP_BLOCK) --production-mode

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/rpc,sf/substreams/v1,sf/substreams/sink"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
