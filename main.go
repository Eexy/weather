package main

import (
	"os"
	"weather/cavalry"
	"weather/command"
	"weather/dotenv"
)

func main() {
	dotenv.Parse()
	cli := cavalry.NewCavalry()
	cli.AddCommand(command.NewGetWeatherCommand())
	cli.Flags().Add("city", "paris", "City")
	cli.Run(os.Args)
}
