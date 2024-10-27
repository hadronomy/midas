package parsers

type Parser interface {
	Parse() (interface{}, error)
	Validate() error
}
