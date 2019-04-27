EXE=prompter

release: version
	cargo build --release

run: build
	RUST_LOG=debug RUST_BACKTRACE=1 target/debug/${EXE} 0

build: version
	cargo build

stat:
	@find . -name \*.rs -print0 | xargs -0 wc -l | sort -n

test:
	cargo test -- --nocapture

test-single:
	cargo test -- --nocapture $(filter-out $@,$(MAKECMDGOALS))

doctoc:
	doctoc .

.PHONY: version
version:
	@./generate-version.bash

clean:
	cargo clean
	@rm src/constants.rs

debug:
	docker build . -t ms140569/prompter
	docker run --cap-add=SYS_PTRACE --security-opt seccomp=unconfined -it ms140569/prompter bash

