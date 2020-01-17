# CargoMake by NeoSmart Technologies
# Written and maintained by Mahmoud Al-Qudsi <mqudsi@neosmart.net>
# Released under the MIT public license
# Obtain updates from https://github.com/neosmart/CargoMake

USERNAME = alngo
COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)

.PHONY: all bench build check clean doc install publish run test update

all: build

bench:
	@$(CARGO) bench

build:
	@$(CARGO) build

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean
	rm -rf pkg

doc:
	@$(CARGO) doc

install: build
	@$(CARGO) install

run: build
	@$(CARGO) run

test: build
	@$(CARGO) test -- --nocapture

update:
	@$(CARGO) update

pkg:
	wasm-pack build --scope $(USERNAME)

pack:
	wasm-pack pack

publish:
	wasm-pack publish
