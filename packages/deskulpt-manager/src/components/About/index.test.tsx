import { describe, expect, it } from "vitest";
import { renderWithProviders, screen } from "../../test-utils/test-helpers";
import About from "./index";

describe("About Tab", () => {
  it("renders Deskulpt title and description", async () => {
    renderWithProviders(<About />);

    await screen.findByText("Deskulpt");
    expect(
      screen.getByText("A cross-platform desktop customization tool"),
    ).toBeInTheDocument();
  });

  it("renders version information", async () => {
    renderWithProviders(<About />);

    await screen.findByText("Version");
  });

  it("renders authors information", async () => {
    renderWithProviders(<About />);

    await screen.findByText("Authors");
    expect(
      screen.getByText("The Deskulpt Development Team"),
    ).toBeInTheDocument();
  });

  it("renders repository link", async () => {
    renderWithProviders(<About />);

    await screen.findByText("Repository");
    expect(screen.getByText(/deskulpt-apps\/Deskulpt/)).toBeInTheDocument();
  });

  it("renders homepage link", async () => {
    renderWithProviders(<About />);

    await screen.findByText("Homepage");
    expect(screen.getByText("deskulpt-apps.github.io")).toBeInTheDocument();
  });
});
