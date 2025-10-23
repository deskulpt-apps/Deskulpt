import { Select, Text } from "@radix-ui/themes";
import { useSettingsStore } from "../../hooks";
import { CanvasImode, commands } from "@deskulpt/bindings";
import { useCallback } from "react";

const CanvasImodeSwitch = () => {
  const canvasImode = useSettingsStore((state) => state.canvasImode);

  const items: { value: CanvasImode; label: string }[] = [
    { value: "sink", label: "Sink" },
    { value: "float", label: "Float" },
  ];

  const onValueChange = useCallback((value: CanvasImode) => {
    commands.core.updateSettings({ canvasImode: value }).catch(console.error);
  }, []);

  return (
    <Select.Root size="1" value={canvasImode} onValueChange={onValueChange}>
      <Select.Trigger />
      <Select.Content>
        {items.map((item) => (
          <Select.Item key={item.value} value={item.value}>
            <Text weight="medium">{item.label}</Text>
          </Select.Item>
        ))}
      </Select.Content>
    </Select.Root>
  );
};

export default CanvasImodeSwitch;
