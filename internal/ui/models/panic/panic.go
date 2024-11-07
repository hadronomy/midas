package panichandler

import (
	"fmt"
	"os"
	"strings"

	"github.com/charmbracelet/bubbles/help"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/hadronomy/midas/internal/app"
	"github.com/hadronomy/midas/internal/ui"
	"github.com/hadronomy/midas/internal/ui/color"
)

// Model represents the panic UI model.
type Model struct {
	lg     *lipgloss.Renderer
	styles *ui.Styles
	err    interface{}
	help   help.Model
}

// NewModel initializes the model with error data and prepares key bindings and help menu.
func NewModel(err interface{}) *Model {
	return &Model{
		err:  err,
		help: help.New(),
	}
}

// Init initializes the model and logs the error.
func (m *Model) Init() tea.Cmd {
	m.lg = lipgloss.DefaultRenderer()
	m.styles = ui.NewStyles(m.lg)
	app.Logger.Errorw("Application panicked", "error", m.err)
	return nil
}

// Update handles incoming messages and key events.
func (m *Model) Update(_ tea.Msg) (tea.Model, tea.Cmd) {
	return m, tea.Quit
}

// View renders the UI.
func (m *Model) View() string {
	// Define box width and custom top border with title
	boxWidth := 60
	title := "Application Panic Detected"
	titleLine := renderTopBorderWithTitle(title, boxWidth)

	// Box Style without top border
	boxStyle := lipgloss.NewStyle().
		Width(boxWidth).
		Align(lipgloss.Left).
		Padding(1, 1).
		BorderBottom(true).
		BorderLeft(true).
		BorderRight(true)

	// Error message section within the box
	errorHeader := createErrorHeader("Error")
	errorMessage := createErrorMessage(m.err)
	errorComplete := lipgloss.JoinHorizontal(lipgloss.Left, errorHeader, errorMessage)

	// Description and main box content
	descStyle := lipgloss.NewStyle().Foreground(color.Indigo).Align(lipgloss.Left).Margin(1, 1).Width(40)
	description := descStyle.Render("The application encountered a critical error and needs to close.")
	logFileReport := createLogFileReport()
	box := lipgloss.JoinVertical(lipgloss.Left, description, errorComplete, logFileReport)

	// Final content inside the box
	content := lipgloss.JoinVertical(lipgloss.Left, titleLine, boxStyle.Render(box))

	// Ensure padding at the end
	return content + "\n\n"
}

// HandlePanic starts the Bubble Tea program to display the panic UI.
func HandlePanic(err interface{}) {
	p := tea.NewProgram(NewModel(err))
	if _, err := p.Run(); err != nil {
		app.Logger.Errorw("Failed to start panic UI", "error", err)
		fmt.Fprintf(os.Stderr, "Failed to start panic UI: %v\n", err)
	}
}

// renderTopBorderWithTitle creates a custom top border line with the title embedded.
func renderTopBorderWithTitle(title string, width int) string {
	titleStyle := lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Color("9"))
	titleText := titleStyle.Render(fmt.Sprintf("─ %s ─", title))

	// Calculate remaining width for border dashes on either side of the title
	borderDash := "─"
	titleLen := lipgloss.Width(titleText)
	sideWidth := (width - titleLen) / 2
	leftSide := lipgloss.NewStyle().
		Foreground(lipgloss.Color("9")).
		Render(strings.Repeat(borderDash, sideWidth))
	rightSide := lipgloss.NewStyle().
		Foreground(lipgloss.Color("9")).
		Render(strings.Repeat(borderDash, width-titleLen-sideWidth))

	// Combine left dashes, title, and right dashes
	return lipgloss.JoinHorizontal(lipgloss.Center, leftSide, titleText, rightSide)
}

// createErrorHeader creates the styled error header.
func createErrorHeader(header string) string {
	return lipgloss.NewStyle().
		Foreground(lipgloss.Color("0")).
		Background(lipgloss.Color("9")).
		Padding(0, 1).
		Margin(0, 1, 0, 0).
		Bold(true).
		Render(header)
}

// createErrorMessage renders the error message in the intended style.
func createErrorMessage(err interface{}) string {
	return lipgloss.NewStyle().Foreground(lipgloss.Color("F")).Bold(true).Render(fmt.Sprintf("%v", err))
}

func createLogFileReport() string {
	top := lipgloss.NewStyle().Foreground(color.Indigo).Render("A log file has been created at:")
	bottom := lipgloss.NewStyle().Foreground(lipgloss.Color("4")).Render(fmt.Sprintf("file://%s", app.LogFilePath))
	content := lipgloss.JoinVertical(lipgloss.Left, top, bottom)
	return lipgloss.NewStyle().Margin(1, 1).Render(content)
}
