import { DataList, Flex, IconButton, Link, Text } from "@radix-ui/themes";
import { css } from "@emotion/react";
import { deskulptWidgets } from "@deskulpt/bindings";
import { LuMail } from "react-icons/lu";
import { Fragment } from "react";

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
      nodes.push({
        node: <Text mr="1">,</Text>,
        key: `comma-${index}`,
      });
    }

    if (typeof author === "string") {
      nodes.push({
        node: <Text>{author}</Text>,
        key: `author-${index}`,
      });
      return nodes;
    }

    if (author.email !== undefined) {
      nodes.push({
        node: (
          <IconButton size="1" variant="ghost" css={styles.emailIcon} asChild>
            <Link href={`mailto:${author.email}`}>
              <LuMail size={14} />
            </Link>
          </IconButton>
        ),
        key: `email-${index}`,
      });
    }

    if (author.homepage === undefined) {
      nodes.push({
        node: <Text>{author.name}</Text>,
        key: `author-${index}`,
      });
    } else {
      nodes.push({
        node: <Link href={author.homepage}>{author.name}</Link>,
        key: `author-${index}`,
      });
    }

    return nodes;
  });
}

function displayUrl(url: string) {
  try {
    return new URL(url).hostname;
  } catch {
    return url;
  }
}

interface WidgetManifestProps {
  manifest: deskulptWidgets.WidgetManifest;
}

const WidgetManifest = ({ manifest }: WidgetManifestProps) => {
  const authorNodes = displayAuthors(manifest.authors ?? []);

  return (
    <DataList.Root size="2" mt="1" css={styles.root}>
      <DataList.Item>
        <DataList.Label minWidth="88px">Name</DataList.Label>
        <DataList.Value>{manifest.name}</DataList.Value>
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
              {authorNodes.map(({ node, key }) => (
                <Fragment key={key}>{node}</Fragment>
              ))}
            </Flex>
          </DataList.Value>
        </DataList.Item>
      )}
      {manifest.homepage !== undefined && (
        <DataList.Item>
          <DataList.Label minWidth="88px">Homepage</DataList.Label>
          <DataList.Value>
            <Link href={manifest.homepage}>
              {displayUrl(manifest.homepage)}
            </Link>
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
