import { beforeEach, describe, expect, it, vi } from "vitest";
import { renderWithProviders, screen } from "../test-utils/test-helpers";
import { mockClipboard } from "../test-utils/tauri-mocks";
import CopyLink from "./CopyLink";

describe("CopyLink", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockClipboard.clear();
  });

  it("renders link text", () => {
    renderWithProviders(
      <CopyLink href="https://example.com">Example Link</CopyLink>,
    );

    expect(screen.getByText("Example Link")).toBeInTheDocument();
  });

  it("renders copy button when href is provided", () => {
    renderWithProviders(
      <CopyLink href="https://example.com">Example Link</CopyLink>,
    );

    expect(screen.getByTitle("Copy link")).toBeInTheDocument();
  });

  it("does not render copy button when href is not provided", () => {
    renderWithProviders(<CopyLink>Example Link</CopyLink>);

    expect(screen.queryByTitle("Copy link")).not.toBeInTheDocument();
  });
});
