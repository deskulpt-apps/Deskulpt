import { Flex, FlexProps, IconButton, Link, LinkProps } from "@radix-ui/themes";
import { LuCopy } from "react-icons/lu";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { toast } from "sonner";

interface CopyLinkProps extends LinkProps {
  gap?: FlexProps["gap"];
}

const CopyLink = ({ gap = "2", children, ...linkProps }: CopyLinkProps) => {
  return (
    <Flex gap={gap} align="center">
      <Link {...linkProps}>{children}</Link>
      {linkProps.href !== undefined && (
        <IconButton
          size="1"
          variant="ghost"
          title="Copy link"
          onClick={() => {
            if (linkProps.href !== undefined) {
              writeText(linkProps.href).then(() =>
                toast.success("Copied to clipboard."),
              );
            }
          }}
        >
          <LuCopy />
        </IconButton>
      )}
    </Flex>
  );
};

export default CopyLink;
