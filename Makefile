run: 
	./scripts/run.sh

new-project: 
	./scripts/create-project.sh $(filter-out $@,$(MAKECMDGOALS))
