package command

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
	"weather/cavalry"
	"weather/model"
)

func NewGetWeatherCommand(cmd *cavalry.Cavalry) *cavalry.Command {
	city := cmd.Flags().String("city", "", "city name")
	units := cmd.Flags().String("units", "metric", "temperature")

	return &cavalry.Command{
		Command:     "get",
		Description: "Get the current weather for the specified location",
		Handle: func() {
			apiKey := os.Getenv("API_KEY")

			if city == nil || *city == "" {
				cmd.Logger.Println("Unable to get city. Invalid parameter")
				os.Exit(1)
			}

			location, err := getLocation(*city, apiKey)
			if err != nil {
				cmd.Logger.Printf("Unable to get location. Error: %s\n", err)
				os.Exit(1)
			}

			err = parseUnitFlag(units)
			if err != nil {
				fmt.Printf(err.Error())
				os.Exit(1)
			}

			forecast, err := getForecast(location, *units, apiKey)
			if err != nil {
				cmd.Logger.Printf("Unable to get forecast. Error: %s\n", err)
				os.Exit(1)
			}

			cmd.Logger.Println(forecast.String())
		},
	}
}

func parseUnitFlag(unit *string) error {
	if unit == nil {
		return errors.New("invalid unit parameter. Parameter should be 'metric' or 'imperial'")
	}

	if *unit != "metric" && *unit != "imperial" {
		return errors.New("invalid unit parameter. Parameter should be 'metric' or 'imperial'")
	}

	return nil
}

func getLocation(city string, apiKey string) (*model.Geocoding, error) {
	geocodingRes, err := http.Get(fmt.Sprintf("http://api.openweathermap.org/geo/1.0/direct?q=%s&limit=1&appid=%s", city, apiKey))
	if err != nil {
		return nil, err
	}
	defer geocodingRes.Body.Close()
	var geocodings []model.Geocoding

	if err := json.NewDecoder(geocodingRes.Body).Decode(&geocodings); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	if len(geocodings) == 0 {
		return nil, errors.New("no location found")
	}

	return &geocodings[0], nil
}

func getForecast(location *model.Geocoding, units string, apiKey string) (*model.Forecast, error) {
	forecastRes, err := http.Get(fmt.Sprintf("https://api.openweathermap.org/data/2.5/weather?lat=%f&lon=%f&units=%s&appid=%s", location.Lat, location.Lon, units, apiKey))
	if err != nil {
		return nil, err
	}
	defer forecastRes.Body.Close()

	var forecast model.Forecast
	if err := json.NewDecoder(forecastRes.Body).Decode(&forecast); err != nil {
		return nil, err
	}

	return &forecast, nil
}
