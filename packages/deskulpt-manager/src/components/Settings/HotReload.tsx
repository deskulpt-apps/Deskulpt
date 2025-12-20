import { Select } from "@radix-ui/themes";
import { deskulptSettings } from "@deskulpt/bindings";
import { useSettingsStore } from "../../hooks";
import { useCallback } from "react";
import { logger } from "@deskulpt/utils";

const options = [
  { value: "on", label: "On" },
  { value: "off", label: "Off" },
] as const;

const HotReload = () => {
  const hotReloadEnabled = useSettingsStore((state) => state.hotReloadEnabled);
  const triggerStyle = { width: "100px" };

  const onValueChange = useCallback((value: "on" | "off") => {
    deskulptSettings.commands
      .update({ hotReloadEnabled: value === "on" })
      .catch(logger.error);
  }, []);

  return (
    <Select.Root
      size="1"
      value={hotReloadEnabled ? "on" : "off"}
      onValueChange={onValueChange}
    >
      <Select.Trigger style={triggerStyle} />
      <Select.Content>
        {options.map((option) => (
          <Select.Item key={option.value} value={option.value}>
            {option.label}
          </Select.Item>
        ))}
      </Select.Content>
    </Select.Root>
  );
};

export default HotReload;
