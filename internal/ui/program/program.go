package program

import (
	"errors"
	"fmt"
	"strings"

	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/huh"
	"github.com/charmbracelet/lipgloss"
	"github.com/hadronomy/midas/internal/ui"
)

const (
	maxWidth = 90
)

var keys = keyMap{
	ToggleFullscreen: key.NewBinding(
		key.WithKeys("f"),
		key.WithHelp("f", "toggle fullscreen"),
	),
	Quit: key.NewBinding(
		key.WithKeys("q", "ctrl+c"),
		key.WithHelp("q", "quit"),
	),
}

type Model struct {
	help         help.Model
	lg           *lipgloss.Renderer
	styles       *ui.Styles
	form         *huh.Form
	keys         keyMap
	width        int
	height       int
	isFullscreen bool
	isQuitting   bool
}

func NewModel() Model {
	m := Model{
		width:        maxWidth,
		isFullscreen: false,
		keys:         keys,
		help:         help.New(),
	}
	m.lg = lipgloss.DefaultRenderer()
	m.styles = ui.NewStyles(m.lg)

	formWidth := 45

	m.form = huh.NewForm(
		huh.NewGroup(
			huh.NewSelect[string]().
				Key("class").
				Options(huh.NewOptions("Warrior", "Mage", "Rogue")...).
				Title("Choose your class").
				Description("This will determine your department"),

			huh.NewSelect[string]().
				Key("level").
				Options(huh.NewOptions("1", "20", "9999")...).
				Title("Choose your level").
				Description("This will determine your benefits package"),

			huh.NewMultiSelect[string]().
				Key("skills").
				Options(huh.NewOptions(
					"🌞 Light Magic",
					"🔮 Enchantment",
				)...).
				Title("Choose your skills").
				Description("This will determine your starting equipment"),

			huh.NewConfirm().
				Key("done").
				Title("All done?").
				Validate(func(v bool) error {
					if !v {
						return errors.New("welp, finish up then")
					}
					return nil
				}).
				Affirmative("Yep").
				Negative("Wait, no"),
		),
	).
		WithWidth(formWidth).
		WithShowHelp(false).
		WithShowErrors(false)
	return m
}

func (m Model) Init() tea.Cmd {
	var cmds []tea.Cmd
	if m.isFullscreen {
		cmds = append(cmds, tea.EnterAltScreen)
	} else {
		cmds = append(cmds, tea.ExitAltScreen)
	}
	cmds = append(cmds, m.form.Init())
	return tea.Batch(cmds...)
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = min(msg.Width, maxWidth) - m.styles.Base.GetHorizontalFrameSize()
		m.height = msg.Height
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, m.keys.Quit):
			m.isQuitting = true
			return m, tea.Quit
		case key.Matches(msg, m.keys.ToggleFullscreen):
			m.isFullscreen = !m.isFullscreen
			if m.isFullscreen {
				return m, tea.EnterAltScreen
			} else {
				return m, tea.ExitAltScreen
			}
		}
	}

	var cmds []tea.Cmd

	// Process the form
	form, cmd := m.form.Update(msg)
	if f, ok := form.(*huh.Form); ok {
		m.form = f
		cmds = append(cmds, cmd)
	}

	if m.form.State == huh.StateCompleted {
		if m.isFullscreen {
			m.isFullscreen = false
			cmds = append(cmds, tea.ExitAltScreen)
			return m, tea.Batch(cmds...)
		}
		// Quit when the form is done.
		cmds = append(cmds, tea.Quit)
	}

	return m, tea.Batch(cmds...)
}

func (m Model) View() string {
	s := m.styles

	if m.isQuitting {
		return ""
	}

	//exhaustive:ignore
	switch m.form.State {
	case huh.StateCompleted:
		title, role := m.getRole()
		title = s.Highlight.Render(title)
		var b strings.Builder
		fmt.Fprintf(&b, "Congratulations, you're Charm's newest\n%s!\n\n", title)
		fmt.Fprintf(&b, "Your job description is as follows:\n\n%s\n\nPlease proceed to HR immediately.", role)
		return s.Status.Margin(0, 1).Padding(1, 2).Width(48).Render(b.String()) + "\n\n"
	default:
		var class string
		if m.form.GetString("class") != "" {
			class = "Class: " + m.form.GetString("class")
		}

		// Form (left side)
		v := strings.TrimSuffix(m.form.WithHeight(m.height-15).View(), "\n\n")
		form := m.lg.NewStyle().Margin(1, 0).Render(v)

		// Status (right side)
		var status string
		{
			var (
				buildInfo      = "(None)"
				role           string
				jobDescription string
				level          string
			)

			if m.form.GetString("level") != "" {
				level = "Level: " + m.form.GetString("level")
				role, jobDescription = m.getRole()
				role = "\n\n" + s.StatusHeader.Render("Projected Role") + "\n" + role
				jobDescription = "\n\n" + s.StatusHeader.Render("Duties") + "\n" + jobDescription
			}
			if m.form.GetString("class") != "" {
				buildInfo = fmt.Sprintf("%s\n%s", class, level)
			}

			formWidth := lipgloss.Width(form)
			const maxStatusWidth = 30
			const minStatusWidth = 20
			statusWidth := m.width - formWidth
			if statusWidth > maxStatusWidth {
				statusWidth = maxStatusWidth
			}
			//mnd:ignore
			statusMarginLeft := m.width - statusWidth - formWidth - 2
			if statusWidth >= minStatusWidth && statusMarginLeft > 0 {
				status = s.Status.
					Height(lipgloss.Height(form)).
					Width(statusWidth).
					MarginLeft(statusMarginLeft).
					Render(s.StatusHeader.Render("Current Build") + "\n" +
						buildInfo +
						role +
						jobDescription)
			}
		}

		errors := m.form.Errors()
		header := m.appOkBoundaryView("Charm Employment Application")
		if len(errors) > 0 {
			header = m.appErrorBoundaryView(m.errorView())
		}
		body := lipgloss.JoinHorizontal(lipgloss.Top, form, status)

		footerStyle := m.styles.HeaderText
		if len(errors) > 0 {
			footerStyle = m.styles.ErrorHeaderText
		}

		footer := lipgloss.JoinVertical(
			lipgloss.Top,
			m.appBoundaryView(
				m.form.Help().ShortHelpView(
					m.form.KeyBinds(),
				),
				footerStyle,
			),
			m.appBoundaryView(
				m.help.View(m.keys),
				footerStyle,
			),
		)

		return s.Base.Render(
			lipgloss.JoinVertical(
				lipgloss.Top,
				header,
				body,
				"\n",
				footer,
			),
		)
	}
}

func (m Model) errorView() string {
	var s string
	for _, err := range m.form.Errors() {
		s += err.Error()
	}
	return s
}

func (m Model) appBoundaryView(text string, style lipgloss.Style) string {
	return lipgloss.PlaceHorizontal(
		m.width,
		lipgloss.Left,
		style.Render(text),
		lipgloss.WithWhitespaceChars("/"),
		lipgloss.WithWhitespaceForeground(style.GetForeground()),
	)
}

func (m Model) appOkBoundaryView(text string) string {
	return m.appBoundaryView(text, m.styles.HeaderText)
}

func (m Model) appErrorBoundaryView(text string) string {
	return m.appBoundaryView(text, m.styles.ErrorHeaderText)
}

func (m Model) getRole() (string, string) {
	level := m.form.GetString("level")
	switch m.form.GetString("class") {
	case "Warrior":
		switch level {
		case "1":
			return "Tank Intern", "Assists with tank-related activities. Paid position."
		case "9999":
			return "Tank Manager", "Manages tanks and tank-related activities."
		default:
			return "Tank", "General tank. Does damage, takes damage. Responsible for tanking."
		}
	case "Mage":
		switch level {
		case "1":
			return "DPS Associate", "Finds DPS deals and passes them on to DPS Manager."
		case "9999":
			return "DPS Operating Officer", "Oversees all DPS activities."
		default:
			return "DPS", "Does damage and ideally does not take damage. Logs hours in JIRA."
		}
	case "Rogue":
		switch level {
		case "1":
			return "Stealth Junior Designer", "Designs rougue-like activities. Reports to Stealth Lead."
		case "9999":
			return "Stealth Lead", "Lead designer for all things stealth. Some travel required."
		default:
			return "Sneaky Person", "Sneaks around and does sneaky things. Reports to Stealth Lead."
		}
	default:
		return "", ""
	}
}
