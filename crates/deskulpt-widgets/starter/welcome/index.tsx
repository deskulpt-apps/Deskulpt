import React from "@deskulpt-test/react";
import { Flex, Text } from "@deskulpt-test/ui";

export default function Widget() {
  return (
    <Flex
      style={{
        inset: 0,
        position: "fixed",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        pointerEvents: "none",
      }}
    >
      <Flex
        direction="column"
        gap={8}
        style={{
          padding: 16,
          borderRadius: 12,
          background: "rgba(24,27,31,0.9)",
          color: "#fff",
          pointerEvents: "auto",
        }}
      >
        <Text size={18}>Hello world</Text>
      </Flex>
    </Flex>
  );
}
