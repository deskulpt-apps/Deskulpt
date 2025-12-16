export interface DownloadOption {
  value: string;
  label: string;
  download?: string;
  code?: string;
}

export function getWindowsDownloadOptions(version: string): DownloadOption[] {
  return [
    {
      value: "msi-x64",
      label: "Intel/AMD (.msi, x64)",
      download: `Deskulpt_${version}_x64_en-US.msi`,
    },
    {
      value: "exe-x64",
      label: "Intel/AMD (.exe, x64)",
      download: `Deskulpt_${version}_x64-setup.exe`,
    },
  ];
}

export function getMacOSDownloadOptions(version: string): DownloadOption[] {
  return [
    {
      value: "dmg-arm64",
      label: "Apple Silicon (.dmg, ARM64)",
      download: `Deskulpt_${version}_aarch64.dmg`,
    },
    {
      value: "x64",
      label: "Intel (.dmg, x64)",
      download: `Deskulpt_${version}_x64.dmg`,
    },
  ];
}

export function getLinuxDownloadOptions(version: string): DownloadOption[] {
  return [
    {
      value: "script",
      label: "Installer Script",
      code: "curl -f https://deskulpt-apps.github.io/install.sh | sh",
    },
    {
      value: "deb-x64",
      label: "Debian/Ubuntu (.deb, x64)",
      download: `Deskulpt_${version}_amd64.deb`,
    },
    {
      value: "rpm-x64",
      label: "RHEL/Fedora/SUSE (.rpm, x64)",
      download: `Deskulpt-${version}-1.x86_64.rpm`,
    },
    {
      value: "appimage-x64",
      label: "AppImage (x64)",
      download: `Deskulpt_${version}_amd64.AppImage`,
    },
    {
      value: "portable-x64",
      label: "Portable (.tar.gz, x64)",
      download: "Deskulpt_x64.app.tar.gz",
    },
    {
      value: "portable-arm64",
      label: "Portable (.tar.gz, aarch64)",
      download: "Deskulpt_aarch64.app.tar.gz",
    },
  ];
}
