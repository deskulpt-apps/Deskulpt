import { useEffect, useState } from "@deskulpt-test/react";
import { Box, Flex, Heading, Progress, Text } from "@deskulpt-test/ui";

const TEST_URL = "https://speed.cloudflare.com/__down";
const TEST_SIZE_BYTES = 2 * 1024 * 1024;
const REFRESH_INTERVAL = 60 * 1000;

const formatTime = (date) => {
  if (!date) {
    return "";
  }

  try {
    return new Intl.DateTimeFormat(undefined, {
      hour: "numeric",
      minute: "2-digit",
      second: "2-digit",
    }).format(date);
  } catch (_) {
    return date.toLocaleTimeString();
  }
};

const measureDownloadSpeed = async (onProgress) => {
  const url = `${TEST_URL}?bytes=${TEST_SIZE_BYTES}&cache=${Date.now()}`;
  const startedAt = performance.now();

  const response = await fetch(url, {
    cache: "no-store",
    mode: "cors",
  });

  if (!response.ok) {
    throw new Error(`Request failed with ${response.status}`);
  }

  let bytesReceived = 0;

  if (response.body && response.body.getReader) {
    const reader = response.body.getReader();
    while (true) {
      const { done, value } = await reader.read();
      if (done) {
        break;
      }
      bytesReceived += value.length;
      onProgress?.(bytesReceived / TEST_SIZE_BYTES, bytesReceived);
    }
  } else {
    const buffer = await response.arrayBuffer();
    bytesReceived = buffer.byteLength;
    onProgress?.(bytesReceived / TEST_SIZE_BYTES, bytesReceived);
  }

  const durationSeconds = (performance.now() - startedAt) / 1000;
  const bitsPerSecond = (bytesReceived * 8) / durationSeconds;
  const megabitsPerSecond = bitsPerSecond / 1_000_000;

  return {
    bytesReceived,
    durationSeconds,
    megabitsPerSecond,
  };
};

export default () => {
  const [result, setResult] = useState(null);
  const [isTesting, setIsTesting] = useState(false);
  const [error, setError] = useState(null);
  const [lastCheckedAt, setLastCheckedAt] = useState(null);
  const [progress, setProgress] = useState(0);
  const [bytesDownloaded, setBytesDownloaded] = useState(0);

  useEffect(() => {
    let cancelled = false;
    let running = false;
    let timeoutId = null;

    const runTest = async () => {
      if (running) {
        return;
      }
      running = true;
      setIsTesting(true);
      setError(null);
      setProgress(0);
      setBytesDownloaded(0);

      try {
        const measurement = await measureDownloadSpeed((ratio, bytes) => {
          if (!cancelled) {
            setProgress(Math.min(Math.max(ratio, 0), 1));
            setBytesDownloaded(bytes);
          }
        });
        if (cancelled) {
          return;
        }
        setResult(measurement);
        setLastCheckedAt(new Date());
        setProgress(1);
        setBytesDownloaded(measurement.bytesReceived);
      } catch (err) {
        if (cancelled) {
          return;
        }
        setError(err instanceof Error ? err.message : "Speed test failed");
        setResult(null);
      } finally {
        if (!cancelled) {
          setIsTesting(false);
        }
        running = false;
        if (!cancelled) {
          timeoutId = setTimeout(runTest, REFRESH_INTERVAL);
        }
      }
    };

    runTest();

    return () => {
      cancelled = true;
      if (timeoutId !== null) {
        clearTimeout(timeoutId);
      }
    };
  }, []);

  const connectionInfo =
    typeof navigator !== "undefined" && navigator.connection
      ? navigator.connection.effectiveType
      : null;

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
        Internet Speed
      </Text>
      <Heading size="8">
        {result ? `${result.megabitsPerSecond.toFixed(1)} Mbps` : "-- Mbps"}
      </Heading>
      <Flex align="center" justify="center" width="100%" gapX="3">
        <Box width="35px">
          <Text size="1" color="gray">
            0
          </Text>
        </Box>
        <Flex flex="1">
          <Progress value={Math.round(progress * 100)} size="2" />
        </Flex>
        <Box width="35px" textAlign="right">
          <Text size="1" color="gray">
            100
          </Text>
        </Box>
      </Flex>
      {isTesting && (
        <Text size="2" color="gray">
          Testing...
        </Text>
      )}
      {!isTesting && error && (
        <Text size="1" color="gray">
          {error}
        </Text>
      )}
      {!isTesting && result && (
        <>
          <Text size="2" color="gray">
            {(bytesDownloaded / 1_000_000).toFixed(2)} MB in{" "}
            {result.durationSeconds.toFixed(2)}s
          </Text>
          <Text size="1" color="gray">
            Last check {formatTime(lastCheckedAt)}
            {connectionInfo ? ` - ${connectionInfo}` : ""}
          </Text>
        </>
      )}
    </Flex>
  );
};
