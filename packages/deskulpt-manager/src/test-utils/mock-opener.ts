/**
 * Mock opener plugin.
 */
export class MockOpener {
  private openedUrls: string[] = [];

  openUrl(url: string): Promise<void> {
    this.openedUrls.push(url);
    return Promise.resolve();
  }

  getOpenedUrls(): string[] {
    return [...this.openedUrls];
  }

  clear() {
    this.openedUrls = [];
  }
}
