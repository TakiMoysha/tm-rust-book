
.PHONY: update_dependencies
update_dependencies:
	cargo update


.PHONY: audit
audit_dependencies:
	pre-commit run --all


.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build

.PHONY: docker_build
docker_build:
	docker build -t tm-dev/demo-medium.rs .
