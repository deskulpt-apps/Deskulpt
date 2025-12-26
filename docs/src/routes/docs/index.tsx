import { createFileRoute, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/docs/")({
  loader: () => {
    throw redirect({
      to: "/docs/$",
      params: { _splat: "guide" },
      replace: true,
    });
  },
});
