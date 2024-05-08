package cavalry

type CommandHandler func()

type Command struct {
	Command     string
	Description string
	Handle      CommandHandler
}

func (c *Command) Run() {
	c.Handle()
}
