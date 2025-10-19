/**
 * Convert an unknown error to a string for logging and displaying.
 */
export function stringifyError(err: unknown) {
  if (typeof err === "string") {
    return err;
  }
  if (err instanceof Error) {
    return err.stack ?? err.message;
  }
  return "Unknown error that is neither a string nor an Error instance";
}
