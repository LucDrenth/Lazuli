run: 
	./scripts/run.sh

test:
	RUST_BACKTRACE=1 cargo test

new-project: 
	./scripts/create-project.sh $(filter-out $@,$(MAKECMDGOALS))
