package app

import (
	"fmt"
	"os"
	"path/filepath"
	"time"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

// LoggerConfig holds configuration options for initializing the logger.
type LoggerConfig struct {
	consoleOutput bool
}

func DefaultLoggerConfig() LoggerConfig {
	return LoggerConfig{
		consoleOutput: os.Getenv("DEBUG") == "true",
	}
}

var (
	Logger      *zap.SugaredLogger
	LogFilePath string
)

// InitLogger initializes a Zap logger that logs to both console (conditionally) and a temp file.
func InitLogger(config LoggerConfig) {
	// Encoder configuration for file and console
	encoderConfig := zap.NewProductionEncoderConfig()
	encoderConfig.EncodeTime = zapcore.ISO8601TimeEncoder

	// File encoder (JSON format for structured logging)
	fileEncoder := zapcore.NewJSONEncoder(encoderConfig)

	// Get the path for the temp log file with a timestamp
	tempDir := os.TempDir()
	timestamp := time.Now().Format("2006-01-02T15-04-05")
	LogFilePath = filepath.Join(tempDir, fmt.Sprintf("midas-%s.log", timestamp))

	// Open the file in append mode, creating it if necessary
	logFile, err := os.OpenFile(LogFilePath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0666)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to open log file: %v\n", err)
		return
	}

	// Create the core for file output
	cores := []zapcore.Core{
		zapcore.NewCore(fileEncoder, zapcore.AddSync(logFile), zap.InfoLevel),
	}

	// Optionally add console output if enabled in config
	if config.consoleOutput {
		consoleEncoderConfig := zap.NewDevelopmentEncoderConfig()
		consoleEncoderConfig.EncodeLevel = zapcore.CapitalColorLevelEncoder
		consoleEncoderConfig.EncodeTime = zapcore.ISO8601TimeEncoder
		consoleEncoder := zapcore.NewConsoleEncoder(consoleEncoderConfig)

		cores = append(cores, zapcore.NewCore(consoleEncoder, zapcore.AddSync(os.Stdout), zap.DebugLevel))
	}

	// Combine the cores
	core := zapcore.NewTee(cores...)

	// Build the logger with the combined core
	Logger = zap.New(core, zap.AddCaller()).Sugar()
	Logger.Infow("Logging initialized", "file", LogFilePath, "config", config)
}

// SyncLogger flushes any buffered log entries to the file.
func SyncLogger() {
	if Logger != nil {
		_ = Logger.Sync()
	}
}
