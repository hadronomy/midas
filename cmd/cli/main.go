package main

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"

	"github.com/hadronomy/midas/internal/ui/model"
)

func main() {
	_, err := tea.NewProgram(model.NewModel(), tea.WithAltScreen()).Run()
	if err != nil {
		fmt.Println("Oh no:", err)
		os.Exit(1)
	}
}
