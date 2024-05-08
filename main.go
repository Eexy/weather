package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"net/http"
	"os"
	"weather/dotenv"
)

func main() {
	dotenv.Parse()

	cityFlag := flag.String("city", "paris", "city flag")
	flag.Parse()
	fmt.Printf("City: %s\n", *cityFlag)

	apiKey, ok := os.LookupEnv("API_KEY")
	if !ok {
		fmt.Println("API key not found in environment")
		os.Exit(1)
	}

	geocodingRes, err := http.Get(fmt.Sprintf("http://api.openweathermap.org/geo/1.0/direct?q=%s&limit=1&appid=%s", *cityFlag, apiKey))
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer geocodingRes.Body.Close()
	var geocodings []Geocoding

	if err := json.NewDecoder(geocodingRes.Body).Decode(&geocodings); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	if len(geocodings) == 0 {
		fmt.Printf("No results found for city : %s\n", *cityFlag)
		os.Exit(1)
	}

	forecastRes, err := http.Get(fmt.Sprintf("https://api.openweathermap.org/data/2.5/weather?lat=%f&lon=%f&units=metric&appid=%s", geocodings[0].Lat, geocodings[0].Lon, apiKey))
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer forecastRes.Body.Close()

	var forecast Forecast
	if err := json.NewDecoder(forecastRes.Body).Decode(&forecast); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fmt.Printf("Temp at %s: %f°C", *cityFlag, forecast.Main.Temp)
}

type Geocoding struct {
	Name    string  `json:"name"`
	Lat     float64 `json:"lat"`
	Lon     float64 `json:"lon"`
	Country string  `json:"country"`
}

type Forecast struct {
	Main ForecastMain `json:"main"`
}

type ForecastMain struct {
	Temp float64 `json:"temp"`
}
