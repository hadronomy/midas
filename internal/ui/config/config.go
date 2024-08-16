package config

import semver "github.com/hashicorp/go-version"

type Template struct {
	projectName   string
	version       *semver.Version
	description   string
	license       string
	repositoryUrl string
	authors       []string
	questions     []Variable
}

type Variable struct {
	defaultValue interface{}

	// TODO: Find the most appropiate way to
	// let the user define a validation function
	validate string
	name     string
	question string

	// NOTE: the question type is one of the following:
	// multiselect, select, input, password, confirm, editor
	// or any of the question types provided by the huh package
	questionType string
}
