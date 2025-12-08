/**
 * Mock clipboard manager.
 */
export class MockClipboard {
  private text = "";

  writeText(text: string): Promise<void> {
    this.text = text;
  }

  readText(): Promise<string> {
    return this.text;
  }

  clear() {
    this.text = "";
  }
}
