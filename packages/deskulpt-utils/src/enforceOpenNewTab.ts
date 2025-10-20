import { openUrl } from "@tauri-apps/plugin-opener";

/**
 * Enfore all anchor elements to open in a new tab.
 *
 * This ensures all hyperlinks are opened using the default browser app instead
 * of in the webview window, even if target is not "_blank".
 */
export function enforceOpenNewTab() {
  // Adapted from:
  // https://github.com/tauri-apps/plugins-workspace/blob/a6b854032d0b10f0f17c4ffa6bdf4a05429e05fb/plugins/opener/guest-js/init.ts
  const handleClick = (event: MouseEvent) => {
    if (
      event.defaultPrevented ||
      event.button !== 0 ||
      event.metaKey ||
      event.altKey
    ) {
      return;
    }

    const a = event
      .composedPath()
      .find((el) => el instanceof Node && el.nodeName.toUpperCase() === "A") as
      | HTMLAnchorElement
      | undefined;
    if (!a || !a.href) {
      return; // No early return even if a.target !== "_blank"
    }

    const url = new URL(a.href);
    if (
      url.origin === window.location.origin ||
      ["http:", "https:", "mailto:", "tel:"].every((p) => url.protocol !== p)
    ) {
      return;
    }

    event.preventDefault();
    openUrl(url.toString());
  };

  window.addEventListener("click", handleClick);
}
