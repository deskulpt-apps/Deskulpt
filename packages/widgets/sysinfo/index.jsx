import { useState, useEffect } from "@deskulpt-test/react";
import { Progress, Flex, Box, Text } from "@deskulpt-test/ui";
import apis from "@deskulpt-test/apis";

export default () => {
  const [cpuUsage, setCpuUsage] = useState(0);
  const [ramUsage, setRamUsage] = useState(0);
  const [swapUsage, setSwapUsage] = useState(0);

  const setUsage = async () => {
    const result = await apis.sys.getSystemInfo();
    // Take average of all CPU usage
    const totalCpuUsage = result.cpuInfo.reduce(
      (acc, cpu) => acc + cpu.totalCpuUsage,
      0,
    );
    const cpuUsage = totalCpuUsage / result.cpuInfo.length;
    const ramUsage = (result.usedMemory / result.totalMemory) * 100;
    const swapUsage = (result.usedSwap / result.totalSwap) * 100;
    setCpuUsage(cpuUsage);
    setRamUsage(ramUsage);
    setSwapUsage(swapUsage);
  };

  useEffect(() => {
    setUsage();
    const interval = setInterval(setUsage, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <Flex
      align="center"
      justify="center"
      width="100%"
      height="100%"
      direction="column"
    >
      <Flex align="center" gapX="3">
        <Box width="40px">
          <Text size="2">CPU</Text>
        </Box>
        <Box width="100px">
          <Progress value={cpuUsage} size="2" />
        </Box>
        <Box width="40px">
          <Text size="2">{cpuUsage.toFixed(0)}%</Text>
        </Box>
      </Flex>
      <Flex align="center" gapX="3">
        <Box width="40px">
          <Text size="2">RAM</Text>
        </Box>
        <Box width="100px">
          <Progress value={ramUsage} size="2" />
        </Box>
        <Box width="40px">
          <Text size="2">{ramUsage.toFixed(0)}%</Text>
        </Box>
      </Flex>
      <Flex align="center" gapX="3">
        <Box width="40px">
          <Text size="2">Swap</Text>
        </Box>
        <Box width="100px">
          <Progress value={swapUsage} size="2" />
        </Box>
        <Box width="40px">
          <Text size="2">{swapUsage.toFixed(0)}%</Text>
        </Box>
      </Flex>
    </Flex>
  );
};
