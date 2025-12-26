import { createRouter as createTanStackRouter } from "@tanstack/react-router";
import { routeTree } from "./route-tree.gen";
import { NotFound } from "@/components/not-found";

export function getRouter() {
  return createTanStackRouter({
    routeTree,
    defaultPreload: "intent",
    scrollRestoration: true,
    defaultNotFoundComponent: NotFound,
  });
}
