package cavalry

import (
	"errors"
	"flag"
	"fmt"
)

type Flags struct {
	Flags map[string]*string
}

func (f Flags) Add(name string, value string, desc string) {
	f.Flags[name] = flag.String(name, value, desc)
}

func (f Flags) Get(name string) (*string, error) {
	current := f.Flags[name]

	if current == nil {
		return nil, errors.New(fmt.Sprintf("flag %s not found", name))
	}

	return current, nil
}
