import { useEffect, useState } from "@deskulpt-test/react";
import { Flex, Heading, Text } from "@deskulpt-test/ui";

const CAMBRIDGE_COORDS = {
  latitude: 42.3736,
  longitude: -71.1097,
};

const WEATHER_REFRESH_INTERVAL = 15 * 60 * 1000;

const formatUpdatedTime = (isoTime) => {
  try {
    const time = new Date(isoTime);
    return new Intl.DateTimeFormat(undefined, {
      hour: "numeric",
      minute: "2-digit",
    }).format(time);
  } catch (_) {
    return isoTime;
  }
};

export default () => {
  const [weather, setWeather] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    let cancelled = false;

    const toFahrenheit = (tempC) => (tempC * 9) / 5 + 32;
    const toCelsius = (tempF) => ((tempF - 32) * 5) / 9;
    const getEmojiForForecast = (forecast) => {
      if (!forecast) {
        return "🌡️";
      }

      const lower = forecast.toLowerCase();

      if (lower.includes("thunder")) return "⛈️";
      if (lower.includes("snow")) return "❄️";
      if (lower.includes("sleet") || lower.includes("ice"))
        return "🌨️";
      if (lower.includes("rain") || lower.includes("showers"))
        return "🌧️";
      if (lower.includes("storm")) return "🌩️";
      if (lower.includes("fog")) return "🌫️";
      if (lower.includes("cloud")) return "☁️";
      if (lower.includes("sunny") || lower.includes("clear"))
        return "☀️";
      if (lower.includes("wind")) return "💨";

      return "🌡️";
    };

    const fetchWeather = async () => {
      try {
        const response = await fetch(
          `https://api.weather.gov/points/${CAMBRIDGE_COORDS.latitude},${CAMBRIDGE_COORDS.longitude}`,
        );

        if (!response.ok) {
          throw new Error(`Request failed with ${response.status}`);
        }

        const data = await response.json();
        if (cancelled) {
          return;
        }

        const locationProps = data.properties?.relativeLocation?.properties;
        const locationLabel =
          locationProps?.city && locationProps?.state
            ? `${locationProps.city}, ${locationProps.state}`
            : "Cambridge, MA";

        const forecastUrl =
          data.properties?.forecastHourly ?? data.properties?.forecast;

        if (!forecastUrl) {
          throw new Error("Forecast link unavailable");
        }

        const forecastResponse = await fetch(forecastUrl);
        if (!forecastResponse.ok) {
          throw new Error(`Forecast request failed with ${forecastResponse.status}`);
        }

        const forecastData = await forecastResponse.json();
        const periods = forecastData.properties?.periods ?? [];
        const current = periods[0];

        if (!current) {
          throw new Error("No forecast periods returned");
        }

        const temperatureUnit = current.temperatureUnit ?? "F";
        const temperatureF =
          temperatureUnit === "F"
            ? current.temperature
            : toFahrenheit(current.temperature);
        const temperatureC =
          temperatureUnit === "C"
            ? current.temperature
            : toCelsius(current.temperature);

        setWeather({
          temperatureF,
          temperatureC,
          shortForecast: current.shortForecast,
          detailedForecast: current.detailedForecast,
          windSpeed: current.windSpeed,
          windDirection: current.windDirection,
          locationLabel,
          updatedAt: current.startTime,
          emoji: getEmojiForForecast(current.shortForecast),
        });
        setError(null);
      } catch (err) {
        if (!cancelled) {
          setError(
            err instanceof Error ? err.message : "Failed to fetch weather",
          );
          setWeather(null);
        }
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    };

    const requestWeather = () => {
      setIsLoading(true);
      fetchWeather();
    };

    requestWeather();
    const intervalId = setInterval(requestWeather, WEATHER_REFRESH_INTERVAL);

    return () => {
      cancelled = true;
      clearInterval(intervalId);
    };
  }, []);

  return (
    <Flex
      align="center"
      justify="center"
      width="100%"
      height="100%"
      direction="column"
      gapY="2"
    >
      <Text size="2" color="gray">
        Weather
      </Text>
      {isLoading && (
        <Text size="2" color="gray">
          Loading&hellip;
        </Text>
      )}
      {!isLoading && error && (
        <>
          <Text size="3">Unable to load weather</Text>
          <Text size="1" color="gray">
            {error}
          </Text>
        </>
      )}
      {!isLoading && !error && weather && (
        <>
          <Heading size="8">
            {weather.emoji} {Math.round(weather.temperatureF)}
            °F
          </Heading>
          <Text size="2" color="gray">
            {Math.round(weather.temperatureC)}°C &middot;{" "}
            {weather.shortForecast}
          </Text>
          <Text size="2" color="gray">
            {weather.locationLabel}
          </Text>
          <Text size="1" color="gray">
            Updated {formatUpdatedTime(weather.updatedAt)}
          </Text>
          <Text size="1" color="gray">
            Wind {weather.windDirection} {weather.windSpeed}
          </Text>
        </>
      )}
    </Flex>
  );
};
