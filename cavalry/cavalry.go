package cavalry

import (
	"errors"
	"flag"
)

type Cavalry struct {
	Commands map[string]*Command
	flags    Flags
}

func NewCavalry() *Cavalry {

	return &Cavalry{
		Commands: make(map[string]*Command),
		flags: Flags{
			Flags: make(map[string]*string),
		},
	}
}

func (c *Cavalry) AddCommand(cmd *Command) {
	c.Commands[cmd.Command] = cmd
}

func (c *Cavalry) Flags() Flags {
	return c.flags
}

func (c *Cavalry) Run(args []string) error {
	flag.Parsed()

	if len(args) < 1 {
		return errors.New("no command specified")
	}

	command := c.Commands[args[1]]
	if command == nil {
		return errors.New("unknown command: " + args[0])
	}

	command.Run(c.flags)
	return nil
}
