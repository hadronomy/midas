package program

import "github.com/charmbracelet/bubbles/key"

type keyMap struct {
	ToggleFullscreen key.Binding
	Quit             key.Binding
}

func (k keyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.ToggleFullscreen, k.Quit}
}

func (k keyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{k.ToggleFullscreen, k.Quit},
	}
}
