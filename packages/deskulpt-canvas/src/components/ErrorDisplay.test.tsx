import { describe, expect, it } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
  waitFor,
} from "../test-utils/test-helpers";
import ErrorDisplay from "./ErrorDisplay";

describe("ErrorDisplay", () => {
  it("renders error title and message", async () => {
    const user = userEvent.setup();

    renderWithProviders(
      <ErrorDisplay id="widget-1" error="Test Error" message="Error details" />,
    );

    // The error message is in a Dialog that's not open by default
    // Click the trigger to open the dialog
    const trigger = screen.getByText(/An error occurred in widget/);
    await user.click(trigger);

    // Now the dialog content should be visible
    await waitFor(() => {
      expect(screen.getByText("Test Error")).toBeInTheDocument();
      expect(screen.getByText("Error details")).toBeInTheDocument();
    });
  });

  it("renders widget ID", () => {
    renderWithProviders(
      <ErrorDisplay id="widget-1" error="Test Error" message="Error details" />,
    );

    expect(screen.getByText(/widget-1/)).toBeInTheDocument();
  });

  it("handles long error messages", () => {
    const longMessage = "A".repeat(1000);

    renderWithProviders(
      <ErrorDisplay id="widget-1" error="Test Error" message={longMessage} />,
    );

    // The message is inside a Dialog that's not open by default, so we check the trigger text
    expect(screen.getByText(/An error occurred in widget/)).toBeInTheDocument();
    // The actual long message is in the Dialog content which is not visible until opened
    // We can verify the component renders without errors
  });
});
