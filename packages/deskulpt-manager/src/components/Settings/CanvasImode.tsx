import { Select } from "@radix-ui/themes";
import { deskulptSettings } from "@deskulpt/bindings";
import { useSettingsStore } from "../../hooks";
import { useCallback } from "react";

const options: { value: deskulptSettings.CanvasImode; label: string }[] = [
  { value: "auto", label: "Auto" },
  { value: "float", label: "Float" },
  { value: "sink", label: "Sink" },
];

const CanvasImode = () => {
  const canvasImode = useSettingsStore((state) => state.canvasImode);

  const onValueChange = useCallback((value: deskulptSettings.CanvasImode) => {
    deskulptSettings.commands
      .update({ canvasImode: value })
      .catch(console.error);
  }, []);

  return (
    <Select.Root size="1" value={canvasImode} onValueChange={onValueChange}>
      <Select.Trigger />
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

export default CanvasImode;
