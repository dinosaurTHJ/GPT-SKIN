import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import App from "./App";
import { DEMO_INSTALLATION, DEMO_THEMES } from "./demo-data";
import { I18nProvider } from "./i18n";

function renderApp() {
  return render(<I18nProvider><App /></I18nProvider>);
}

const desktop = vi.hoisted(() => ({
  applyTheme: vi.fn(),
  chooseAndPreviewLocalTheme: vi.fn(),
  checkForUpdate: vi.fn(),
  getRuntimeEnvironment: vi.fn(),
  getAccountStatus: vi.fn(),
  getAccountSync: vi.fn(),
  getThemePreviewStatus: vi.fn(),
  isDesktopRuntime: vi.fn(),
  listThemes: vi.fn(),
  downloadOnlineTheme: vi.fn(),
  restoreOfficialTheme: vi.fn(),
  uninstallTheme: vi.fn(),
  installation: {
    appName: "ChatGPT",
    path: "/Applications/ChatGPT.app",
    executable: "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT",
    bundleId: "com.openai.codex",
    version: "26.707.91948",
  },
  themes: [
    {
      id: "studio.example.protocol-preview",
      name: "Protocol Preview",
      description: "Protocol fixture",
      version: "1.0.0",
      author: "Example Studio",
      preview: { background: "#111310", surface: "#20241c", accent: "#c9ff41" },
      builtIn: false,
      onlineSlug: "protocol-preview",
    },
  ],
  account: {
    authenticated: true,
    email: "hello@retheme.app",
    pro: true,
    deviceName: "Test Mac",
    deviceGeneration: 1,
    heartbeatState: "online",
    entitlements: [],
    trials: [],
  },
  openWebsite: vi.fn(),
  openGitHubRepository: vi.fn(),
  startOAuthLogin: vi.fn(),
  completeOAuthLogin: vi.fn(),
  syncThemeLocale: vi.fn(),
}));

vi.mock("./desktop-api", () => ({
    applyTheme: desktop.applyTheme,
    checkForUpdate: desktop.checkForUpdate,
    chooseAndPreviewLocalTheme: desktop.chooseAndPreviewLocalTheme,
    detectCodex: vi.fn().mockResolvedValue(desktop.installation),
    downloadOnlineTheme: desktop.downloadOnlineTheme,
    getAccountStatus: desktop.getAccountStatus,
    getAccountSync: desktop.getAccountSync,
    getRuntimeEnvironment: desktop.getRuntimeEnvironment,
    getThemePreviewStatus: desktop.getThemePreviewStatus,
    isDesktopRuntime: desktop.isDesktopRuntime,
    listThemes: desktop.listThemes,
    loginAccount: vi.fn(),
    startOAuthLogin: desktop.startOAuthLogin,
    completeOAuthLogin: desktop.completeOAuthLogin,
    logoutAccount: vi.fn(),
    openWebsite: desktop.openWebsite,
    openGitHubRepository: desktop.openGitHubRepository,
    redeemCdk: vi.fn(),
    registerAccount: vi.fn(),
    requestRegisterCode: vi.fn(),
    restoreOfficialTheme: desktop.restoreOfficialTheme,
    runSmokeTest: vi.fn(),
    syncThemeLocale: desktop.syncThemeLocale,
    syncTrayLocale: vi.fn(),
    uninstallTheme: desktop.uninstallTheme,
  }));

vi.mock("@tauri-apps/plugin-deep-link", () => ({
  getCurrent: vi.fn().mockResolvedValue(null),
  onOpenUrl: vi.fn().mockResolvedValue(vi.fn()),
}));

describe("ReTheme desktop shell", () => {
  beforeEach(() => {
    window.localStorage.setItem("retheme.locale", "zh-CN");
    desktop.isDesktopRuntime.mockReturnValue(false);
    desktop.checkForUpdate.mockReset().mockResolvedValue(null);
    desktop.getThemePreviewStatus.mockReset().mockResolvedValue(null);
    desktop.listThemes.mockReset().mockResolvedValue(desktop.themes);
    desktop.downloadOnlineTheme.mockReset();
    desktop.getAccountStatus.mockReset().mockResolvedValue(desktop.account);
    desktop.getAccountSync.mockReset().mockResolvedValue({
      devices: [
        { id: "test-mac", name: "Test Mac", current: true, active: true, registeredAt: "2026-07-10T01:00:00Z", lastSeenAt: "2026-07-18T01:00:00Z" },
        { id: "test-win", name: "Test Windows", current: false, active: false, registeredAt: "2026-07-09T01:00:00Z", lastSeenAt: "2026-07-17T00:30:00Z" },
      ],
      themes: {
        favorites: [{ slug: "midnight-orbit", name: "午夜轨道", description: "深夜主题", locales: { en: { name: "Midnight Orbit", description: "A late-night theme" } }, version: "1.0.0", author: { id: "retheme", name: "ReTheme" }, preview: { background: "#12131a", surface: "#202331", accent: "#8ca9ff" } }],
        used: [{ slug: "protocol-preview", name: "协议预览", locales: { en: { name: "Protocol Preview" } }, version: "1.0.0", author: { id: "example", name: "Example Studio" } }],
      },
    });
    desktop.openWebsite.mockReset().mockResolvedValue(undefined);
    desktop.openGitHubRepository.mockReset().mockResolvedValue(undefined);
    desktop.startOAuthLogin.mockReset().mockResolvedValue(undefined);
    desktop.completeOAuthLogin.mockReset().mockResolvedValue(desktop.account);
    desktop.getRuntimeEnvironment.mockResolvedValue({
      appVersion: "0.1.0",
      themeRuntimeVersion: "0.1.0",
      compatibility: {
        adapterId: "codex-2026-home-v2",
        revision: 12,
        source: "signedRemote",
      },
    });
    desktop.applyTheme.mockResolvedValue({
      themeId: DEMO_THEMES[0].id,
      theme: DEMO_THEMES[0],
      source: "installed",
      appVersion: DEMO_INSTALLATION.version,
      port: 18926,
      appliedSlots: ["app.shell"],
      loopbackOnly: true,
    });
    desktop.restoreOfficialTheme.mockResolvedValue(true);
    desktop.uninstallTheme.mockReset().mockResolvedValue(true);
    desktop.chooseAndPreviewLocalTheme.mockReset().mockResolvedValue(null);
  });

  afterEach(cleanup);

  test("switches all top-level pages", async () => {
    renderApp();
    for (const [label, heading] of [["主题库", "已安装主题"], ["云端收藏", "云端收藏"], ["账号", "账号"], ["设备", "设备"], ["设置", "设置"], ["概览", "概览"]]) {
      fireEvent.click(screen.getByRole("button", { name: label }));
      expect(await screen.findByRole("heading", { name: heading })).toBeInTheDocument();
    }
  });

  test("applies and restores an installed theme", async () => {
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "主题库" }));
    const apply = await screen.findByRole("button", { name: "应用主题 Protocol Preview" });
    await waitFor(() => expect(apply).toBeEnabled());
    fireEvent.click(apply);
    await waitFor(() => expect(desktop.applyTheme).toHaveBeenCalledWith("studio.example.protocol-preview", "zh-CN"));
    const restore = await screen.findByRole("button", { name: "恢复 Protocol Preview 主题" });
    fireEvent.click(restore);
    await waitFor(() => expect(desktop.restoreOfficialTheme).toHaveBeenCalledOnce());
    expect(await screen.findByRole("button", { name: "应用主题 Protocol Preview" })).toBeInTheDocument();
  });

  test("uninstalls an installed theme and refreshes the library", async () => {
    const confirm = vi.spyOn(window, "confirm").mockReturnValue(true);
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "主题库" }));

    fireEvent.click(await screen.findByRole("button", { name: "卸载主题 Protocol Preview" }));

    await waitFor(() => expect(desktop.uninstallTheme).toHaveBeenCalledWith("studio.example.protocol-preview"));
    await waitFor(() => expect(desktop.listThemes).toHaveBeenCalledTimes(2));
    expect(confirm).toHaveBeenCalledWith("确定卸载“Protocol Preview”吗？本机主题文件将被删除。");
  });

  test("loads a local theme without a time limit", async () => {
    desktop.isDesktopRuntime.mockReturnValue(true);
    const localPreview = {
      themeId: "dev.example.local",
      theme: {
        id: "dev.example.local",
        name: "本地调试主题",
        description: "Local development theme",
        version: "0.1.0",
        author: "Developer",
        preview: { background: "#111111", surface: "#222222", accent: "#ff6699" },
        builtIn: false,
      },
      source: "localDevelopment",
      appVersion: desktop.installation.version,
      port: 18926,
      appliedSlots: ["app.shell"],
      loopbackOnly: true,
    };
    let currentPreview: typeof localPreview | null = null;
    desktop.chooseAndPreviewLocalTheme.mockImplementation(async () => {
      currentPreview = localPreview;
      return localPreview;
    });
    desktop.getThemePreviewStatus.mockImplementation(async () => currentPreview);

    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "主题库" }));
    const loadLocal = await screen.findByRole("button", { name: "加载本地主题" });
    await waitFor(() => expect(loadLocal).toBeEnabled());
    await waitFor(() => expect(loadLocal).toHaveAttribute("title", expect.stringContaining("不限时运行")));
    fireEvent.click(loadLocal);

    expect(await screen.findByText("本地预览 · 本地调试主题")).toBeInTheDocument();
    expect(desktop.chooseAndPreviewLocalTheme).toHaveBeenCalledWith("zh-CN");
  });

  test("keeps local themes unlimited without Pro", async () => {
    desktop.getAccountStatus.mockResolvedValue({ ...desktop.account, pro: false });
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "主题库" }));

    const loadLocal = await screen.findByRole("button", { name: "加载本地主题" });
    await waitFor(() => expect(loadLocal).toHaveAttribute("title", expect.stringContaining("不限时运行")));
  });

  test("does not expose ctheme package sideloading", async () => {
    desktop.getAccountStatus.mockResolvedValue({ ...desktop.account, pro: false });
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "主题库" }));

    expect(await screen.findByRole("button", { name: "加载本地主题" })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "安装主题包" })).not.toBeInTheDocument();
  });

  test("clears the active theme after ChatGPT App exits", async () => {
    desktop.isDesktopRuntime.mockReturnValue(true);
    desktop.getThemePreviewStatus
      .mockResolvedValueOnce({
        themeId: desktop.themes[0].id,
        theme: desktop.themes[0],
        source: "installed",
        appVersion: desktop.installation.version,
        port: 18926,
        appliedSlots: ["app.shell"],
        loopbackOnly: true,
      })
      .mockResolvedValue(null);

    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "主题库" }));
    expect(await screen.findByRole("button", { name: "恢复 Protocol Preview 主题" })).toBeInTheDocument();
    await waitFor(
      () => expect(screen.getByRole("button", { name: "应用主题 Protocol Preview" })).toBeEnabled(),
      { timeout: 2_500 },
    );
  });

  test("shows the signed-in account state", async () => {
    renderApp();
    expect(await screen.findByLabelText("账号等级 PRO")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "账号" }));
    expect(await screen.findByRole("heading", { name: "账号" })).toBeInTheDocument();
    expect(screen.getAllByText("hello@retheme.app").length).toBeGreaterThanOrEqual(2);
    expect(screen.getByText("Pro 赞助者")).toBeInTheDocument();
    expect(screen.getByText("感谢支持")).toBeInTheDocument();
    expect(screen.getByText("云同步、多设备与纪念主题赠品")).toBeInTheDocument();
    expect(screen.getByText("Test Mac · 在线")).toBeInTheDocument();
    expect(screen.queryByRole("heading", { name: "登录设备" })).not.toBeInTheDocument();
  });

  test("offers GitHub and Linux DO login for signed-out accounts", async () => {
    desktop.getAccountStatus.mockResolvedValue({ ...desktop.account, authenticated: false, email: undefined, pro: false, heartbeatState: "offline" });
    renderApp();
    const accountButton = (await screen.findByText("登录")).closest("button");
    expect(accountButton).not.toBeNull();
    expect(within(accountButton!).queryByText("未登录")).not.toBeInTheDocument();
    fireEvent.click(accountButton!);

    fireEvent.click(screen.getByRole("button", { name: "使用 GitHub 登录" }));
    await waitFor(() => expect(desktop.startOAuthLogin).toHaveBeenCalledWith("github"));
    fireEvent.click(screen.getByRole("button", { name: "使用 Linux DO 登录" }));
    await waitFor(() => expect(desktop.startOAuthLogin).toHaveBeenCalledWith("linuxdo"));
  });

  test("shows device history on the devices page", async () => {
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "设备" }));

    const banner = await screen.findByRole("button", { name: /Pro 多设备登录已启用/ });
    fireEvent.click(banner);
    expect(desktop.openWebsite).toHaveBeenCalledWith("/pricing");
    expect(await screen.findByText("2 台历史设备")).toBeInTheDocument();
    expect(await screen.findByText("Test Mac")).toBeInTheDocument();
    expect(await screen.findByText("Test Windows")).toBeInTheDocument();
    expect(screen.getByText("已下线")).toBeInTheDocument();
  });

  test("shows device history for signed-in free accounts", async () => {
    desktop.getAccountStatus.mockResolvedValue({ ...desktop.account, pro: false });
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "设备" }));

    expect(await screen.findByRole("button", { name: /Pro 账号可多设备登录/ })).toBeInTheDocument();
    expect(await screen.findByText("2 台历史设备")).toBeInTheDocument();
    expect(await screen.findByText("Test Mac")).toBeInTheDocument();
    expect(await screen.findByText("Test Windows")).toBeInTheDocument();
  });

  test("shows Pro cloud favorites in the theme library", async () => {
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: /云端收藏/ }));
    expect(await screen.findByRole("heading", { name: "云端收藏" })).toBeInTheDocument();
    expect((await screen.findAllByText("午夜轨道")).length).toBeGreaterThanOrEqual(2);
    expect(screen.getByText("最近使用")).toBeInTheDocument();
    expect(screen.getByText("协议预览")).toBeInTheDocument();
  });

  test("localizes cloud theme names in English", async () => {
    window.localStorage.setItem("retheme.locale", "en");
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "Cloud Favorites" }));

    expect((await screen.findAllByText("Midnight Orbit")).length).toBeGreaterThanOrEqual(2);
    expect(screen.getByText("Protocol Preview")).toBeInTheDocument();
    expect(screen.queryByText("午夜轨道")).not.toBeInTheDocument();
  });

  test("localizes the required sign-in message for online installation", async () => {
    desktop.downloadOnlineTheme.mockRejectedValue(new Error("RETHEME_AUTH_REQUIRED"));
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "云端收藏" }));
    fireEvent.click(await screen.findByRole("button", { name: "安装" }));

    expect(await screen.findByText("请先登录 ReTheme 后安装在线主题")).toBeInTheDocument();
  });

  test("localizes the required sign-in message in English", async () => {
    window.localStorage.setItem("retheme.locale", "en");
    desktop.downloadOnlineTheme.mockRejectedValue(new Error("RETHEME_AUTH_REQUIRED"));
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "Cloud Favorites" }));
    fireEvent.click(await screen.findByRole("button", { name: "Install" }));

    expect(await screen.findByText("Sign in to ReTheme before installing an online theme")).toBeInTheDocument();
  });

  test("shows a free account without purchase prompts", async () => {
    desktop.getAccountStatus.mockResolvedValue({ ...desktop.account, pro: false });
    renderApp();
    expect(await screen.findByLabelText("账号等级 FREE")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "账号" }));

    expect(await screen.findByText("免费账号")).toBeInTheDocument();
    expect(screen.getByText("全部主题可免费不限时使用")).toBeInTheDocument();
    expect(screen.queryByText(/待购买|前往购买|¥29|\$6/)).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "设备" }));
    expect(await screen.findByRole("button", { name: /Pro 账号可多设备登录/ })).toBeInTheDocument();
    expect(await screen.findByText("2 台历史设备")).toBeInTheDocument();
    expect(screen.getByText("Test Mac")).toBeInTheDocument();
    expect(screen.getByText("Test Windows")).toBeInTheDocument();
    expect(desktop.getAccountSync).toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "账号" }));
    fireEvent.click(screen.getAllByRole("button", { name: /管理账号/ })[0]);
    expect(await screen.findByRole("dialog", { name: "hello@retheme.app" })).toBeInTheDocument();
  });

  test("shows locked cloud favorites without Pro", async () => {
    desktop.getAccountStatus.mockResolvedValue({ ...desktop.account, authenticated: false, email: undefined, pro: false, heartbeatState: "offline" });
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "云端收藏" }));

    expect(await screen.findByRole("heading", { name: "云端收藏" })).toBeInTheDocument();
    const favorites = screen.getByRole("button", { name: "查看云端收藏" });
    fireEvent.click(favorites);
    expect(screen.getByText("暂无权益使用")).toBeInTheDocument();
    expect(desktop.getAccountSync).not.toHaveBeenCalled();
  });

  test("shows product-facing runtime and compatibility versions", async () => {
    desktop.isDesktopRuntime.mockReturnValue(true);
    renderApp();

    expect(await screen.findAllByText("v0.1.0", { selector: "strong" })).toHaveLength(1);
    const support = screen.getByRole("button", { name: "前往 GitHub 为 ReTheme 点 Star" });
    expect(within(support).getByText("GitHub Star")).toBeInTheDocument();
    fireEvent.click(support);
    expect(desktop.openGitHubRepository).toHaveBeenCalledOnce();
    expect(screen.getByText("远程 r12")).toBeInTheDocument();
    expect(screen.getByText("codex-2026-home-v2")).toBeInTheDocument();
    expect(screen.getByText("本地主题通道")).toBeInTheDocument();
    expect(screen.getByText(/ChatGPT .*已连接/)).toBeInTheDocument();
    expect(screen.queryByText(/CDP/i)).not.toBeInTheDocument();
  });

  test("toggles desktop preferences", async () => {
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "设置" }));
    const launch = await screen.findByRole("switch", { name: "开机自动启动" });
    expect(launch).toHaveAttribute("aria-checked", "false");
    fireEvent.click(launch);
    expect(launch).toHaveAttribute("aria-checked", "true");
  });

  test("shows feedback when the app is up to date", async () => {
    desktop.isDesktopRuntime.mockReturnValue(true);
    desktop.checkForUpdate.mockResolvedValue(null);
    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "设置" }));
    fireEvent.click(await screen.findByRole("button", { name: "检查更新" }));

    expect(await screen.findByText("ReTheme 0.1.0 · 已是最新版本")).toBeInTheDocument();
  });

  test("restores the active theme before installing an update", async () => {
    desktop.isDesktopRuntime.mockReturnValue(true);
    desktop.restoreOfficialTheme.mockClear();
    desktop.getThemePreviewStatus.mockResolvedValue({
      themeId: desktop.themes[0].id,
      theme: desktop.themes[0],
      source: "installed",
      appVersion: desktop.installation.version,
      port: 18926,
      appliedSlots: ["app.shell"],
      loopbackOnly: true,
    });
    const install = vi.fn().mockResolvedValue(undefined);
    desktop.checkForUpdate.mockResolvedValue({
      currentVersion: "0.1.0",
      version: "0.1.1",
      body: "Test update",
      install,
    });

    renderApp();
    fireEvent.click(screen.getByRole("button", { name: "设置" }));
    const installUpdate = await screen.findByRole("button", { name: "安装并重启" });
    expect(screen.getByText("发现 ReTheme 0.1.1")).toBeInTheDocument();
    expect(screen.getByText("Test update")).toBeInTheDocument();
    fireEvent.click(installUpdate);

    await waitFor(() => expect(desktop.restoreOfficialTheme).toHaveBeenCalledOnce());
    expect(install).toHaveBeenCalledOnce();
    expect(desktop.restoreOfficialTheme.mock.invocationCallOrder[0]).toBeLessThan(install.mock.invocationCallOrder[0]);
  });
});
