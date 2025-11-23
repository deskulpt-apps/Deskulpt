import React from "@deskulpt-test/react";
import { Box, Button, Stack, Text, Title } from "@deskulpt-test/ui";

export default function Widget() {
  return (
    <Box
      style={{
        inset: 0,
        position: "fixed",
        display: "grid",
        placeItems: "center",
        pointerEvents: "none",
      }}
    >
      <Stack
        gap={16}
        style={{
          maxWidth: 420,
          padding: 24,
          borderRadius: 16,
          background: "rgba(24,27,31,0.92)",
          color: "#fff",
          boxShadow: "0 18px 48px rgba(0,0,0,0.35)",
          pointerEvents: "auto",
        }}
      >
        <Title level={2}>Welcome to Deskulpt</Title>
        <Text>
          Your widgets directory is empty. Browse the catalog or drop widgets
          into the folder to get started.
        </Text>
        <Stack direction="row" gap={12}>
          <Button onClick={() => open("https://deskulpt.app/widgets")}>
            Browse widgets
          </Button>
          <Button
            variant="secondary"
            onClick={() => open(__DESKULPT_BASE_URL__)}
          >
            Open widgets folder
          </Button>
        </Stack>
      </Stack>
    </Box>
  );
}
