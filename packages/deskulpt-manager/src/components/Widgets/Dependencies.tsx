import { css } from "@emotion/react";
import { Code, Inset, Link, Popover, Table, Text } from "@radix-ui/themes";
import { memo } from "react";

const styles = {
  table: css({
    "--table-cell-padding": "var(--space-1) var(--space-2)",
    "--table-cell-min-height": 0,
    "[data-radix-scroll-area-viewport]": { maxHeight: "150px" },
    "& tr:last-child": { "--table-row-box-shadow": "none" },
  }),
};

interface DependenciesProps {
  dependencies: Record<string, string>;
}

const Dependencies = memo(({ dependencies }: DependenciesProps) => {
  const dependenciesArray = Object.entries(dependencies);

  return dependenciesArray.length > 0 ? (
    <Popover.Root>
      <Popover.Trigger>
        <Link title="View the dependencies" asChild>
          <button>View ({dependenciesArray.length})</button>
        </Link>
      </Popover.Trigger>
      <Popover.Content size="1">
        <Inset side="all">
          <Table.Root size="1" css={styles.table}>
            <Table.Body>
              {dependenciesArray.map(([name, version]) => (
                <Table.Row key={name}>
                  <Table.RowHeaderCell>
                    <Link href={`https://www.npmjs.com/package/${name}`}>
                      {name}
                    </Link>
                  </Table.RowHeaderCell>
                  <Table.Cell>
                    <Code variant="ghost">{version}</Code>
                  </Table.Cell>
                </Table.Row>
              ))}
            </Table.Body>
          </Table.Root>
        </Inset>
      </Popover.Content>
    </Popover.Root>
  ) : (
    <Text color="gray">None</Text>
  );
});

export default Dependencies;
