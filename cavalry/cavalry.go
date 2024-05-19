package cavalry

import (
	"errors"
	"flag"
	"fmt"
)

type Cavalry struct {
	Commands map[string]*Command
	flags    *flag.FlagSet
	Version  string
}

func NewCavalry() *Cavalry {

	cmd := &Cavalry{
		Commands: make(map[string]*Command),
		flags:    flag.NewFlagSet("cli", flag.ContinueOnError),
		Version:  "1.0.0",
	}

	cmd.AddCommand(newVersionCommand(cmd))
	return cmd
}

func (c *Cavalry) AddCommand(cmd *Command) {
	c.Commands[cmd.Command] = cmd
}

func (c *Cavalry) Flags() *flag.FlagSet {
	return c.flags
}

func (c *Cavalry) Run(args []string) error {
	c.flags.Parse(args[2:])

	if len(args) < 1 {
		return errors.New("no command specified")
	}

	command := c.Commands[args[1]]
	if command == nil {
		return errors.New("unknown command: " + args[0])
	}

	command.Run()
	return nil
}

func newVersionCommand(cmd *Cavalry) *Command {
	return &Command{
		Command:     "version",
		Description: "Get version",
		Handle: func() {
			fmt.Printf("version: %s\n", cmd.Version)
		},
	}
}
