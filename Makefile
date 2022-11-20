help: ## print available targets
	@cat $(MAKEFILE_LIST) | \
	grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

docker: export DOCKER_BUILDKIT=1
docker: ## Builds the docker image that can be deployed or ran independantly
	@docker build .

run: ## Runs the binary in debug mode
	@cargo run

deploy: ## Deploys the app to fly.io
	@fly deploy
