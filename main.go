package main

import (
	"io"
	"log"
	"os"
	"weather/cavalry"
	"weather/command"
	"weather/dotenv"
)

func main() {
	dotenv.Parse()
	cli := cavalry.NewCavalry()

	// set logger with console and file output
	f, err := os.OpenFile("log.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	writers := io.MultiWriter(f, os.Stdout)
	logger := log.New(writers, "", log.LstdFlags)
	cli.Logger = logger

	cli.AddCommand(command.NewGetWeatherCommand(cli))
	cli.Run(os.Args)
}
