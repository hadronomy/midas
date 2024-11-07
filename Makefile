# Makefile

# Variables
APP_NAME := midas
PKG := ./...
BIN := ./bin/$(APP_NAME)

# Run the app
run:
	go run ./cmd/cli/main.go

watch:
	air

# Build the binary
build:
	go build -o $(BIN) ./cmd/cli/main.go

# Clean up binaries and temporary files
clean:
	rm -rf $(BIN)

# Run tests
test:
	go test $(PKG) -v

# Run tests with race detection
test-race:
	go test -race $(PKG)

# Lint the code using Golangci-lint
lint:
	golangci-lint run

# Format the code
fmt:
	gofumpt -l -w $(PKG)

# Install dependencies
deps:
	go mod tidy

# Generate documentation (optional if using godoc)
docs:
	@echo "Generating Go documentation..."
	@mkdir -p ./docs
	@echo "Open http://localhost:6060/pkg to view documentation"
	@godoc -http=:6060

# Help
help:
	@echo "Usage:"
	@echo "  make run               Run the application"
	@echo "  make build             Build the application binary"
	@echo "  make clean             Remove build artifacts"
	@echo "  make test              Run tests"
	@echo "  make test-race         Run tests with race detection"
	@echo "  make lint              Run Golangci-lint on the code"
	@echo "  make fmt               Format the code"
	@echo "  make deps              Tidy up dependencies"
	@echo "  make docs              Start godoc server for documentation"
	@echo "  make help              Show this help message"
