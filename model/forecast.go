package model

import (
	"fmt"
	"strings"
)

type Forecast struct {
	Main ForecastMain `json:"main"`
}

type ForecastMain struct {
	Temp      float64 `json:"temp"`
	FeelsLike float64 `json:"feels_like"`
	Min       float64 `json:"temp_min"`
	Max       float64 `json:"temp_max"`
	Humidity  float64 `json:"humidity"`
}

func (f *Forecast) String() string {
	var s strings.Builder
	properties := []string{"Temp", "Feels Like", "Min", "Max", "Humidity"}
	maxLength := 0
	for _, prop := range properties {
		if len(prop) > maxLength {
			maxLength = len(prop)
		}
	}

	fmt.Fprintf(&s, "\n===== Weather =====\n")
	fmt.Fprintf(&s, "%s :\t\t %f°C\n", "Temp"+strings.Repeat(" ", maxLength-len(strings.Split("Temp", ""))), f.Main.Temp)
	fmt.Fprintf(&s, "%s :\t\t %f°C\n", "Feels Like"+strings.Repeat(" ", maxLength-len(strings.Split("Feels Like", ""))), f.Main.FeelsLike)
	fmt.Fprintf(&s, "%s :\t\t %f°C\n", "Min temp"+strings.Repeat(" ", maxLength-len(strings.Split("Min temp", ""))), f.Main.Min)
	fmt.Fprintf(&s, "%s :\t\t %f°C\n", "Max temp"+strings.Repeat(" ", maxLength-len(strings.Split("Max temp", ""))), f.Main.Max)
	fmt.Fprintf(&s, "%s :\t\t %f%%\n", "Humidity"+strings.Repeat(" ", maxLength-len(strings.Split("Humidity", ""))), f.Main.Humidity)
	fmt.Fprintf(&s, "===== Weather =====\n")
	return s.String()
}
