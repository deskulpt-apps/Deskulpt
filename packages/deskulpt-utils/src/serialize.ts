import { DeskulptCore } from "@deskulpt/bindings";
import * as superjson from "superjson";

superjson.allowErrorProps("message", "cause");

export function serialize(instance: unknown): DeskulptCore.JsonValue {
  try {
    return superjson.serialize(instance).json as DeskulptCore.JsonValue;
  } catch {
    return { __unserializable: true, preview: String(instance) };
  }
}

export function stringify(instance: unknown): string {
  if (typeof instance === "string") {
    return instance;
  }

  const serialized = serialize(instance);
  if (typeof serialized === "string") {
    return serialized;
  }

  try {
    return JSON.stringify(serialized);
  } catch {
    return "[unstringifiable]";
  }
}
