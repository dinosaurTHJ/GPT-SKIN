import type {
  CodexInstallation,
  SmokeTestReport,
  RuntimeEnvironment,
  ThemePreviewReport,
  ThemeSummary,
} from "./types";

export const DEMO_RUNTIME_ENVIRONMENT: RuntimeEnvironment = {
  appVersion: "0.1.0",
  themeRuntimeVersion: "0.1.0",
  compatibility: {
    adapterId: "codex-2026-home-v1",
    revision: 12,
    source: "signedRemote",
  },
};

export const DEMO_THEMES: ThemeSummary[] = [
  {
    id: "studio.example.protocol-preview",
    name: "协议预览主题",
    description: "用于桌面界面预览的虚构主题数据，不包含主题代码或资源。",
    version: "1.0.0",
    author: "Example Studio",
    preview: {
      background: "#17191f",
      surface: "#242832",
      accent: "#8ca9ff",
    },
    builtIn: false,
    onlineSlug: "protocol-preview",
  },
];

export const DEMO_INSTALLATION: CodexInstallation = {
  appName: "ChatGPT",
  path: "/Applications/ChatGPT.app",
  executable: "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT",
  bundleId: "com.openai.codex",
  version: "26.707.91948",
};

export const DEMO_SMOKE_REPORT: SmokeTestReport = {
  appVersion: DEMO_INSTALLATION.version,
  browserVersion: "Chrome/140",
  port: 18926,
  targetTitle: "ChatGPT",
  targetUrl: "app://codex/",
  loopbackOnly: true,
  probeApplied: true,
  probeRemoved: true,
  durationMs: 860,
};

export function createDemoPreview(themeId: string): ThemePreviewReport {
  const theme = DEMO_THEMES.find((item) => item.id === themeId) ?? DEMO_THEMES[0];
  return {
    themeId,
    theme,
    source: "installed",
    appVersion: DEMO_INSTALLATION.version,
    port: 18926,
    appliedSlots: ["app.shell", "sidebar", "home.hero", "composer", "settings"],
    loopbackOnly: true,
  };
}
