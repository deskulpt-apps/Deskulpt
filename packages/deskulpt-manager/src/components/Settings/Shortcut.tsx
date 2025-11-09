import {
  Button,
  Flex,
  IconButton,
  Kbd,
  Popover,
  Text,
  TextField,
} from "@radix-ui/themes";
import {
  FocusEvent,
  KeyboardEvent as ReactKeyboardEvent,
  useCallback,
  useRef,
  useState,
} from "react";
import { FaEdit } from "react-icons/fa";
import { MdClear } from "react-icons/md";
import { deskulptSettings } from "@deskulpt/bindings";
import { useSettingsStore } from "../../hooks";
import { toast } from "sonner";
import { INVALID_KEYCODES, KEYCODES, MODIFIERS } from "./keyboard";
import { css } from "@emotion/react";

const INITIAL_PLACEHOLDER = "Shortcut disabled";

const styles = {
  input: css({
    width: "240px",
    fontSize: "var(--font-size-2)",
    paddingLeft: "var(--space-1)",
    "> input": { cursor: "text" },
    "--text-field-focus-color": "var(--accent-8)",
  }),
  inputInvalid: css({ "--text-field-focus-color": "var(--red-8)" }),
};

interface Props {
  shortcutKey: deskulptSettings.ShortcutKey;
}

const ShortcutAction = ({ shortcutKey }: Props) => {
  const shortcut = useSettingsStore((state) => state.shortcuts[shortcutKey]);

  const inputRef = useRef<HTMLInputElement>(null);
  const clearButtonRef = useRef<HTMLButtonElement>(null);
  const confirmButtonRef = useRef<HTMLButtonElement>(null);

  const [value, setValue] = useState(shortcut ?? "");
  const [placeholder, setPlaceholder] = useState(INITIAL_PLACEHOLDER);
  const [isValid, setIsValid] = useState(true);

  const handleFocus = useCallback(() => {
    setPlaceholder("Press key combination...");
  }, []);

  const handleBlur = useCallback(
    (event: FocusEvent) => {
      // Prevent resetting when clicking on the clear or confirm button,
      // otherwise they cannot access the correct value
      if (
        event.relatedTarget === confirmButtonRef.current ||
        event.relatedTarget === clearButtonRef.current
      ) {
        return;
      }
      setValue(shortcut ?? "");
      setPlaceholder(INITIAL_PLACEHOLDER);
    },
    [shortcut],
  );

  const handleOpenChange = useCallback(
    (open: boolean) => {
      if (open) {
        // Reset states on popover open
        setValue(shortcut ?? "");
        setPlaceholder(INITIAL_PLACEHOLDER);
        setIsValid(true);
      }
    },
    [shortcut],
  );

  const handleKeyDown = useCallback((event: ReactKeyboardEvent) => {
    if (event.key === "Tab" || event.repeat) {
      // Ignore key repeats for performance; ignore tab key for keyboard
      // accessibility, i.e., on should be able to use it to navigate to the
      // close and confirm buttons instead of being trapped in the input field
      return;
    }
    event.preventDefault();

    const keys = [];
    let localHasKey = false;
    let localHasModifier = false;

    // Check for modifier keys
    if (event.metaKey) {
      keys.push(MODIFIERS.Meta);
      localHasModifier = true;
    }
    if (event.ctrlKey) {
      keys.push(MODIFIERS.Ctrl);
      localHasModifier = true;
    }
    if (event.shiftKey) {
      keys.push(MODIFIERS.Shift);
      localHasModifier = true;
    }
    if (event.altKey) {
      keys.push(MODIFIERS.Alt);
      localHasModifier = true;
    }

    // Only include non-modifier keys as the final main key of the shortcut
    if (event.code in KEYCODES) {
      keys.push(KEYCODES[event.code as keyof typeof KEYCODES]);
      localHasKey = true;
    } else if (event.code in INVALID_KEYCODES) {
      keys.push(INVALID_KEYCODES[event.code as keyof typeof INVALID_KEYCODES]);
    }

    setValue(keys.join(" + "));
    setIsValid(localHasKey && localHasModifier);
  }, []);

  const confirmAction = useCallback(() => {
    deskulptSettings.commands
      .update({ shortcuts: { [shortcutKey]: value === "" ? null : value } })
      .then(() => {
        setPlaceholder(INITIAL_PLACEHOLDER);
        setIsValid(true);
        toast.success("Shortcut updated.");
      })
      .catch(() => {
        toast.error("Failed to update shortcut.");
      });
  }, [shortcutKey, value]);

  const clearAction = useCallback(() => {
    if (inputRef.current === null) {
      setPlaceholder(INITIAL_PLACEHOLDER);
    } else {
      inputRef.current.focus();
    }
    setValue("");
    setIsValid(true);
  }, []);

  return (
    <Flex align="center" justify="end" gap="4">
      {shortcut === undefined ? (
        <Text color="gray">Disabled</Text>
      ) : (
        <Kbd size="3">{shortcut}</Kbd>
      )}
      <Popover.Root onOpenChange={handleOpenChange}>
        <Popover.Trigger>
          <Button size="1" variant="surface">
            <FaEdit /> Edit
          </Button>
        </Popover.Trigger>
        <Popover.Content size="1" width="400px">
          <Text size="2" as="div" mb="3">
            A keyboard shortcut must contain at least one modifier key (Ctrl,
            Shift, Alt) and one alphanumeric key. A red border means that the
            shortcut is invalid.
          </Text>
          <Flex gap="3" align="center">
            <TextField.Root
              ref={inputRef}
              size="1"
              variant="surface"
              readOnly
              value={value}
              placeholder={placeholder}
              onFocus={handleFocus}
              onBlur={handleBlur}
              onKeyDown={handleKeyDown}
              css={[styles.input, !isValid && styles.inputInvalid]}
            >
              <TextField.Slot side="right">
                <IconButton
                  ref={clearButtonRef}
                  size="1"
                  variant="ghost"
                  disabled={value === ""}
                  onClick={clearAction}
                >
                  <MdClear size="15" />
                </IconButton>
              </TextField.Slot>
            </TextField.Root>
            {isValid && (shortcut ?? "") !== value && (
              <Popover.Close>
                <Button
                  ref={confirmButtonRef}
                  size="1"
                  variant="surface"
                  onClick={confirmAction}
                >
                  {value === "" ? "Disable" : "Confirm"}
                </Button>
              </Popover.Close>
            )}
          </Flex>
        </Popover.Content>
      </Popover.Root>
    </Flex>
  );
};

export default ShortcutAction;
