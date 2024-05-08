package cavalry

type CommandHandler func(flags Flags)

type Command struct {
	Command     string
	Description string
	Handle      CommandHandler
}

func (c *Command) Run(flags Flags) {
	c.Handle(flags)
}
