/**
 * Mock clipboard manager.
 */
export class MockClipboard {
  private text = "";

  writeText(text: string): Promise<void> {
    this.text = text;
    return Promise.resolve();
  }

  readText(): Promise<string> {
    return Promise.resolve(this.text);
  }

  clear() {
    this.text = "";
  }
}
