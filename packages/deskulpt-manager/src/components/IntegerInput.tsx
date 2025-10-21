import { css } from "@emotion/react";
import { Box, BoxProps, Reset } from "@radix-ui/themes";
import { ChangeEvent, KeyboardEvent, useCallback } from "react";

const styles = {
  root: css({
    backgroundColor: "var(--gray-5)",
    borderRadius: "var(--radius-2)",
    lineHeight: "1.6",
  }),
};

type IntegerInputProps = BoxProps & {
  value?: number;
  onValueChange: (value: number) => void;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  readOnly?: boolean;
};

// Snap value to the nearest step within the min and max range
function snap(value: number, min: number, max: number, step: number) {
  return Math.min(
    max,
    Math.max(min, min + Math.round((value - min) / step) * step),
  );
}

const IntegerInput = ({
  value,
  onValueChange,
  min = -Infinity,
  max = Infinity,
  step = 1,
  disabled = false,
  readOnly = false,
  ...boxProps
}: IntegerInputProps) => {
  const handleChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      const targetValue = Number(event.target.value);
      if (!Number.isInteger(targetValue)) return;
      onValueChange(snap(targetValue, min, max, step));
    },
    [onValueChange, min, max, step],
  );

  const handleKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    let nextValue = value ?? snap(0, min, max, step);

    switch (event.key) {
      case "ArrowUp":
        event.preventDefault();
        if (event.shiftKey) nextValue += 10 * step;
        else nextValue += step;
        break;
      case "ArrowDown":
        event.preventDefault();
        if (event.shiftKey) nextValue -= 10 * step;
        else nextValue -= step;
        break;
      case "PageUp":
        event.preventDefault();
        nextValue += 10 * step;
        break;
      case "PageDown":
        event.preventDefault();
        nextValue -= 10 * step;
        break;
      case "Home":
        if (!Number.isFinite(min)) return;
        event.preventDefault();
        nextValue = min;
        break;
      case "End":
        if (!Number.isFinite(max)) return;
        event.preventDefault();
        nextValue = max;
        break;
      default:
        return;
    }

    onValueChange(snap(nextValue, min, max, step));
  };

  return (
    <Box pl="2" css={styles.root} {...boxProps} asChild>
      <Reset>
        <input
          type="number"
          value={value}
          min={min}
          max={max}
          step={step}
          disabled={disabled}
          readOnly={readOnly}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          aria-valuenow={value}
          aria-valuemin={min}
          aria-valuemax={max}
          aria-disabled={disabled}
          aria-readonly={readOnly}
        />
      </Reset>
    </Box>
  );
};

export default IntegerInput;
