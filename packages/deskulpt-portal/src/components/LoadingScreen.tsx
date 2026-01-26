import { Flex, Spinner, Text } from "@radix-ui/themes";

interface LoadingScreenProps {
  text?: string;
}

const LoadingScreen = ({ text }: LoadingScreenProps) => {
  return (
    <Flex
      height="100%"
      direction="column"
      align="center"
      justify="center"
      gap="3"
    >
      <Spinner size="3" />
      <Text size="2" color="gray">
        {text ?? "Loading..."}
      </Text>
    </Flex>
  );
};

export default LoadingScreen;
