import { DataList, Flex, IconButton, Link, Text } from "@radix-ui/themes";
import { css } from "@emotion/react";
import { deskulptWidgets } from "@deskulpt/bindings";
import { LuHouse, LuMail } from "react-icons/lu";
import { useMemo } from "react";

const styles = {
  root: css({
    gap: "var(--space-2)",
  }),
  emailIcon: css({
    marginRight: "2px",
  }),
  homepageIcon: css({
    marginLeft: "2px",
  }),
};

function displayAuthors(authors: deskulptWidgets.WidgetManifestAuthor[]) {
  return authors.flatMap((author, index) => {
    const nodes = [];
    if (index > 0) {
      nodes.push(<Text mr="1">,</Text>);
    }

    if (typeof author === "string") {
      nodes.push(<Text>{author}</Text>);
      return nodes;
    }

    if (author.email !== undefined) {
      nodes.push(
        <IconButton size="1" variant="ghost" css={styles.emailIcon} asChild>
          <Link href={`mailto:${author.email}`}>
            <LuMail size={14} />
          </Link>
        </IconButton>,
      );
    }

    if (author.homepage === undefined) {
      nodes.push(<Text>{author.name}</Text>);
    } else {
      nodes.push(<Link href={author.homepage}>{author.name}</Link>);
    }

    return nodes;
  });
}

interface WidgetManifestProps {
  manifest: deskulptWidgets.WidgetManifest;
}

const WidgetManifest = ({ manifest }: WidgetManifestProps) => {
  const authorNodes = useMemo(
    () => displayAuthors(manifest?.authors ?? []),
    [manifest?.authors],
  );

  return (
    <DataList.Root size="2" mt="1" css={styles.root}>
      <DataList.Item>
        <DataList.Label minWidth="88px">Name</DataList.Label>
        <DataList.Value>
          <Flex display="inline-flex" align="center" wrap="wrap">
            {manifest.name}
            {manifest.homepage !== undefined && (
              <IconButton
                size="1"
                variant="ghost"
                css={styles.homepageIcon}
                asChild
              >
                <Link href={manifest.homepage}>
                  <LuHouse size={14} />
                </Link>
              </IconButton>
            )}
          </Flex>
        </DataList.Value>
      </DataList.Item>
      {manifest.version !== undefined && (
        <DataList.Item>
          <DataList.Label minWidth="88px">Version</DataList.Label>
          <DataList.Value>{manifest.version}</DataList.Value>
        </DataList.Item>
      )}
      {manifest.license !== undefined && (
        <DataList.Item>
          <DataList.Label minWidth="88px">License</DataList.Label>
          <DataList.Value>{manifest.license}</DataList.Value>
        </DataList.Item>
      )}
      {authorNodes.length > 0 && (
        <DataList.Item>
          <DataList.Label minWidth="88px">Authors</DataList.Label>
          <DataList.Value>
            <Flex display="inline-flex" align="center" wrap="wrap">
              {authorNodes}
            </Flex>
          </DataList.Value>
        </DataList.Item>
      )}
      {manifest.description !== undefined && (
        <DataList.Item>
          <DataList.Label minWidth="88px">Description</DataList.Label>
          <DataList.Value>{manifest.description}</DataList.Value>
        </DataList.Item>
      )}
    </DataList.Root>
  );
};

export default WidgetManifest;
