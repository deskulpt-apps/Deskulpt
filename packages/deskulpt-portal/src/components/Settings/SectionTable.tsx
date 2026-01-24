import { css } from "@emotion/react";
import { Box, Heading, Table } from "@radix-ui/themes";
import { PropsWithChildren } from "react";

const styles = {
  root: css({
    border: "1px solid var(--gray-a6)",
    borderRadius: "var(--radius-2)",
  }),
  title: css({ backgroundColor: "var(--gray-1)" }),
  table: css({
    "--table-cell-padding": "var(--space-1) var(--space-2)",
    "--table-cell-min-height": 0,
    "& tr": { "--table-row-box-shadow": "none" },
    "& th": { width: "240px", paddingLeft: "var(--space-3)" },
  }),
};

interface SectionTableProps {
  title: string;
}

const SectionTable = ({
  title,
  children,
}: PropsWithChildren<SectionTableProps>) => {
  return (
    <Box position="relative" mt="2" pt="4" pb="2" px="1" css={styles.root}>
      <Box
        position="absolute"
        top="calc(-0.5 * var(--heading-line-height-2))"
        left="2"
        px="2"
        css={styles.title}
        asChild
      >
        <Heading size="2" color="gray" weight="medium">
          {title}
        </Heading>
      </Box>
      <Table.Root size="1" css={styles.table}>
        <Table.Body>{children}</Table.Body>
      </Table.Root>
    </Box>
  );
};

export default SectionTable;
