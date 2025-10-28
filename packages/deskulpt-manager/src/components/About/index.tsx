import { Avatar, Box, Flex, Heading, Table } from "@radix-ui/themes";
import CopyLink from "../CopyLink";
import { memo } from "react";
import { FaGithub } from "react-icons/fa";
import { css } from "@emotion/react";

const styles = {
  logo: css({
    ".dark &": {
      filter: "invert(90%) hue-rotate(170deg)",
      opacity: 0.9,
    },
  }),
  table: css({
    "--table-cell-padding": "var(--space-1) 0",
    "--table-cell-min-height": 0,
    "& tr": { "--table-row-box-shadow": "none" },
    "& th": { color: "var(--gray-11)", width: "100px" },
  }),
};

const AboutTab = memo(() => {
  return (
    <Flex height="100%" pb="8" justify="center" align="center">
      <Flex align="center" justify="center" flexGrow="1">
        <Avatar src="/deskulpt.svg" fallback="D" size="8" css={styles.logo} />
      </Flex>
      <Box flexGrow="1">
        <Heading size="6" mb="1">
          Deskulpt
        </Heading>
        <Heading size="3" mb="4" weight="regular" color="gray">
          A cross-platform desktop customization tool
        </Heading>
        <Table.Root size="1" css={styles.table}>
          <Table.Body>
            <Table.Row align="center">
              <Table.RowHeaderCell>Version</Table.RowHeaderCell>
              <Table.Cell>{__VERSION__}</Table.Cell>
            </Table.Row>
            <Table.Row align="center">
              <Table.RowHeaderCell>Authors</Table.RowHeaderCell>
              <Table.Cell>The Deskulpt Development Team</Table.Cell>
            </Table.Row>
            <Table.Row align="center">
              <Table.RowHeaderCell>Repository</Table.RowHeaderCell>
              <Table.Cell>
                <CopyLink href="https://github.com/deskulpt-apps/Deskulpt">
                  <Flex align="center" gap="1">
                    <FaGithub /> deskulpt-apps/Deskulpt
                  </Flex>
                </CopyLink>
              </Table.Cell>
            </Table.Row>
            <Table.Row align="center">
              <Table.RowHeaderCell>Homepage</Table.RowHeaderCell>
              <Table.Cell>
                <CopyLink href="https://deskulpt-apps.github.io/">
                  deskulpt-apps.github.io
                </CopyLink>
              </Table.Cell>
            </Table.Row>
          </Table.Body>
        </Table.Root>
      </Box>
    </Flex>
  );
});

export default AboutTab;
