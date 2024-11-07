package main

import (
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"go.uber.org/zap"

	"github.com/hadronomy/midas/internal/app"
	panicHandler "github.com/hadronomy/midas/internal/ui/models/panic"
	"github.com/hadronomy/midas/internal/ui/program"
)

const (
	ExitCodeOK             = 0
	ExitCodeError          = 1
	ExitCodeInvalidArgs    = 2
	ExitCodeConfigError    = 3
	ExitCodeNotFound       = 4
	ExitCodePermission     = 5
	ExitCodeUnavailable    = 6
	ExitCodeTimeout        = 7
	ExitCodeDependencyFail = 8
)

func main() {
	os.Exit(func() int {
		app.InitLogger(app.DefaultLoggerConfig())
		defer app.SyncLogger()

		defer func() {
			if r := recover(); r != nil {
				panicHandler.HandlePanic(r)
				os.Exit(1)
			}
		}()

		app.Logger.Info("Starting midas")
		_, err := tea.NewProgram(program.NewModel()).Run()
		if err != nil {
			app.Logger.Info("Error running program", zap.Error(err))
			return ExitCodeError
		}
		return ExitCodeOK
	}())
}
