import { css } from "@emotion/react";
import { Button, Flex, Select } from "@radix-ui/themes";
import { memo } from "react";
import { LuRepeat } from "react-icons/lu";

const styles = {
  select: css({ width: "100px" }),
};

interface HeaderProps {
  refresh: () => void;
}

const Header = memo(({ refresh }: HeaderProps) => {
  return (
    <Flex align="center" gap="2" justify="between">
      <Flex align="center" gap="2">
        <Select.Root size="1" defaultValue="widgets">
          <Select.Trigger css={styles.select} />
          <Select.Content position="popper">
            <Select.Item value="widgets">Widgets</Select.Item>
          </Select.Content>
        </Select.Root>
        <Select.Root size="1" defaultValue="all">
          <Select.Trigger css={styles.select} />
          <Select.Content position="popper">
            <Select.Item value="all">All</Select.Item>
            <Select.Item value="new">New</Select.Item>
            <Select.Item value="installed">Installed</Select.Item>
          </Select.Content>
        </Select.Root>
      </Flex>

      <Flex align="center" justify="end" gap="2">
        <Button size="1" variant="surface" onClick={refresh}>
          <LuRepeat /> Refresh
        </Button>
      </Flex>
    </Flex>
  );
});

export default Header;
