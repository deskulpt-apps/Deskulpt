import { Flex, FlexProps, IconButton, Link, LinkProps } from "@radix-ui/themes";
import { RxCopy } from "react-icons/rx";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { toast } from "sonner";
import { useCallback } from "react";

interface CopyLinkProps extends LinkProps {
  gap?: FlexProps["gap"];
}

const CopyLink = ({ gap = "2", children, ...linkProps }: CopyLinkProps) => {
  const handleCopy = useCallback(() => {
    if (linkProps.href !== undefined) {
      writeText(linkProps.href).then(() =>
        toast.success("Copied to clipboard."),
      );
    }
  }, [linkProps.href]);

  return (
    <Flex gap={gap} align="center">
      <Link {...linkProps}>{children}</Link>
      {linkProps.href !== undefined && (
        <IconButton
          size="1"
          variant="ghost"
          title="Copy link"
          onClick={handleCopy}
        >
          <RxCopy />
        </IconButton>
      )}
    </Flex>
  );
};

export default CopyLink;
