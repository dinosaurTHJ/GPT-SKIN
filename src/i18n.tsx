import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from "react";
import type { AccountThemeSummary, AppLocale, ThemeSummary } from "./types";

export type LocalePreference = "system" | AppLocale;

const LOCALE_KEY = "retheme.locale";

const zhCN = {
  "nav.workspace": "工作台",
  "nav.accountSection": "账户",
  "nav.overview": "概览",
  "nav.themes": "主题库",
  "nav.favorites": "云端收藏",
  "nav.account": "账号",
  "nav.devices": "设备",
  "nav.settings": "设置",
  "nav.store": "在线商店",
  "nav.main": "主导航",
  "brand.subtitle": "主题引擎",
  "brand.backOverview": "返回 ReTheme 概览",
  "common.system": "跟随系统",
  "common.chinese": "简体中文",
  "common.english": "English",
  "common.light": "浅色",
  "common.dark": "深色",
  "common.install": "安装",
  "common.installing": "安装中",
  "common.installed": "已安装",
  "common.apply": "应用",
  "common.applying": "应用中",
  "common.uninstall": "卸载",
  "common.uninstalling": "卸载中",
  "common.loading": "加载中…",
  "common.restoring": "恢复中…",
  "common.online": "在线",
  "common.offline": "离线",
  "common.localDevice": "本机",
  "heartbeat.signedOut.title": "未登录",
  "heartbeat.signedOut.detail": "登录后同步账号数据",
  "heartbeat.online.title": "设备在线",
  "heartbeat.online.detail": "账号已连接，数据会自动同步",
  "heartbeat.grace.title": "暂时离线",
  "heartbeat.grace.detail": "网络暂不可达，主题仍可继续使用",
  "heartbeat.replaced.title": "设备已替换",
  "heartbeat.replaced.detail": "请重新登录连接当前设备",
  "heartbeat.offline.title": "账号离线",
  "heartbeat.offline.detail": "请连接网络后重新登录",
  "overview.subtitle": "当前主题、ChatGPT 连接与账号状态",
  "overview.tier": "账号等级 {tier}",
  "overview.currentTheme": "当前主题",
  "overview.running": "运行中",
  "overview.official": "官方界面",
  "overview.officialTheme": "ChatGPT 官方主题",
  "overview.noTheme": "未注入任何主题样式",
  "overview.restore": "恢复主题",
  "overview.choose": "选择主题",
  "overview.app": "应用",
  "overview.waitingDetection": "等待检测",
  "overview.appReady": "应用已识别，适配器可用",
  "overview.detectingApp": "正在检查本机应用",
  "overview.accountStatus": "账号状态",
  "overview.diagnostic": "安全诊断",
  "overview.allPassed": "全部通过",
  "overview.diagnosing": "诊断中…",
  "overview.pending": "待运行",
  "overview.diagnosticPassed": "回环限定 · 完整撤销 · {duration} ms",
  "overview.diagnosticDetail": "仅连接 127.0.0.1，可完整撤销",
  "overview.runDiagnostic": "运行诊断",
  "overview.desktopApp": "桌面应用",
  "overview.themeEngine": "主题引擎",
  "overview.localRuntime": "本地运行时",
  "overview.support": "支持 ReTheme",
  "overview.supportAction": "前往 GitHub 为 ReTheme 点 Star",
  "overview.openSource": "开源项目",
  "overview.compatibility": "兼容规则",
  "overview.remoteRevision": "远程 r{revision}",
  "overview.builtInRules": "内置规则",
  "overview.reading": "读取中",
  "overview.waitingMatch": "等待匹配",
  "themes.search": "搜索主题…",
  "themes.localImagesTitle": "{path} 本地图片",
  "themes.localImagesHint": "已扫描 {count} 张图片，点击即可应用到 ChatGPT",
  "themes.imageCategory": "图片分类",
  "themes.rescanImages": "重新扫描",
  "themes.chooseImageDirectory": "选择图片目录",
  "themes.changeImageDirectory": "更换目录",
  "themes.applyImageLabel": "应用图片主题 {name}",
  "themes.noImages": "{path} 下没有找到支持的图片",
  "themes.localTitle": "选择包含 manifest.json 的主题目录；本地开发主题可不限时运行",
  "themes.loadLocal": "加载本地主题",
  "themes.restoreOfficial": "恢复主题",
  "themes.installSuccess": "{name} 已{action}，签名验证通过",
  "themes.updated": "更新",
  "themes.added": "安装",
  "themes.installedTitle": "已安装主题",
  "themes.communityHint": "社区主题免费使用，在线安装需登录",
  "themes.signInToInstall": "请先登录 ReTheme 后安装在线主题",
  "themes.chatGPTDebugRequired": "当前 ChatGPT 没有本地调试通道。为避免重开或重启 ChatGPT，ReTheme 不会启动隔离副本；必须在 ChatGPT 启动前启用调试参数，之后切换主题不会重启进程。",
  "themes.uninstallLabel": "卸载主题 {name}",
  "themes.restoreLabel": "恢复 {name} 主题",
  "themes.applyLabel": "应用主题 {name}",
  "themes.inUse": "使用中",
  "themes.more": "在 retheme.app 发现更多创作者主题",
  "themes.browseStore": "浏览在线商店 →",
  "themes.confirmUninstall": "确定卸载“{name}”吗？本机主题文件将被删除。",
  "themes.viewOnline": "查看线上主题 {name}",
  "favorites.subtitle": "Pro 云同步收藏与使用记录",
  "favorites.empty": "还没有线上收藏，前往社区找到喜欢的主题。",
  "favorites.syncing": "正在同步线上收藏…",
  "favorites.noAccess": "暂无权益使用",
  "favorites.view": "查看云端收藏",
  "favorites.recent": "最近使用",
  "account.subtitle": "管理登录状态、云同步与设备",
  "account.signedIn": "已登录",
  "account.signedOut": "未登录",
  "account.current": "当前账号",
  "account.loginReTheme": "登录",
  "account.proSupporter": "Pro 赞助者",
  "account.free": "免费账号",
  "account.syncAfterLogin": "登录后同步账号数据",
  "account.deviceOnline": "设备在线",
  "account.currentOffline": "当前离线",
  "account.proSupporterUpper": "PRO 赞助者",
  "account.status": "账号状态",
  "account.thanks": "感谢支持",
  "account.connected": "已连接",
  "account.proDetail": "云同步、多设备登录，并赠送 Pro 纪念主题",
  "account.communityFree": "社区主题仍可免费不限时使用",
  "account.manageData": "登录后管理账号数据",
  "account.manage": "管理账号",
  "account.identity": "账号身份",
  "account.identityHint": "登录后查看账号信息",
  "account.currentDevice": "当前设备",
  "account.proBenefit": "Pro 赞助权益",
  "account.communityThemes": "社区主题",
  "account.proBenefitDetail": "云同步、多设备与纪念主题赠品",
  "account.communityDetail": "全部主题可免费不限时使用",
  "account.proActive": "Pro 赞助者身份已激活",
  "account.notLoggedIn": "尚未登录 ReTheme",
  "account.unrestricted": "，主题使用不受登录或 Pro 状态限制",
  "account.login": "登录账号",
  "devices.subtitle": "查看账号登录过的设备与最近在线记录",
  "devices.proEnabled": "Pro 多设备登录已启用",
  "devices.proAvailable": "Pro 账号可多设备登录",
  "devices.proEnabledDetail": "多台设备可同时登录，并自动同步设备记录。",
  "devices.proAvailableDetail": "升级后即可多设备同时登录并同步记录。",
  "devices.proBenefit": "PRO 权益",
  "devices.loginDevices": "登录设备",
  "devices.historyCount": "{count} 台历史设备",
  "devices.current": "当前设备",
  "devices.active": "在线设备",
  "devices.history": "历史设备",
  "devices.firstLogin": "首次登录",
  "devices.lastOnline": "最后在线",
  "devices.signedOut": "已下线",
  "devices.empty": "还没有设备登录记录。",
  "devices.syncing": "正在同步设备记录…",
  "devices.viewHistory": "查看设备历史",
  "devices.signInToView": "登录后查看设备记录。",
  "settings.subtitle": "管理 ReTheme 的桌面行为、更新和本机主题服务",
  "settings.general": "通用",
  "settings.imageLibrary": "图片目录",
  "settings.imageLibraryDetail": "扫描该目录及其子目录中的图片",
  "settings.changeImageDirectory": "更换目录",
  "settings.language": "语言",
  "settings.languageDetail": "默认跟随系统，也可以手动选择界面语言",
  "settings.languageMode": "界面语言",
  "settings.appearance": "外观",
  "settings.appearanceDetail": "ReTheme 窗口的明暗模式，跟随系统或手动指定",
  "settings.appearanceMode": "外观模式",
  "settings.launchAtLogin": "开机自动启动",
  "settings.launchAtLoginDetail": "登录系统后在托盘中启动 ReTheme",
  "settings.hideToTray": "关闭窗口时隐藏到托盘",
  "settings.hideToTrayDetail": "退出请使用托盘菜单中的“退出 ReTheme”",
  "settings.autoDetect": "启动后自动检测 ChatGPT",
  "settings.autoDetectDetail": "检查安装版本与主题适配状态",
  "settings.update": "更新",
  "settings.autoUpdate": "自动检查更新",
  "settings.autoUpdateDetail": "仅安装通过 ReTheme 签名验证的更新",
  "settings.currentVersion": "当前版本",
  "settings.stableChannel": "ReTheme {version} · 稳定通道",
  "settings.latestVersion": "ReTheme {version} · 已是最新版本",
  "settings.latest": "已是最新",
  "settings.checkNow": "检查更新",
  "settings.checking": "检查中…",
  "settings.available": "发现 ReTheme {version}",
  "settings.installUpdate": "安装并重启",
  "settings.installing": "正在安装…",
  "settings.themeService": "主题服务",
  "settings.accountSync": "登录后自动同步账号数据",
  "settings.restoreOfficial": "恢复主题",
  "settings.themeRunning": "当前有主题运行，操作后会立即撤销样式",
  "settings.noThemeRunning": "当前没有运行中的主题",
  "settings.restoreNow": "恢复主题",
  "settings.diagnostics": "诊断",
  "settings.appPath": "ChatGPT 路径",
  "settings.appNotFound": "尚未检测到 ChatGPT",
  "settings.redetect": "重新检测",
  "oauth.completing": "正在完成第三方登录…",
  "oauth.invalidCallback": "第三方登录回调参数不完整，请重新登录",
  "sidebar.connected": "ChatGPT · 已连接",
  "sidebar.detecting": "正在检测 ChatGPT…",
  "sidebar.disconnected": "ChatGPT 未连接",
  "status.localChannel": "本地主题通道",
  "status.demoData": "演示数据",
  "status.localPreview": "本地预览",
  "status.themeRunning": "主题运行中",
  "status.account": "账号 {status}",
  "dialog.close": "关闭账号窗口",
  "dialog.loginSuccess": "登录成功",
  "dialog.registerSuccess": "注册并登录成功",
  "dialog.debugCode": "开发验证码：{code}",
  "dialog.codeSent": "验证码已发送到邮箱",
  "dialog.cdkSuccess": "CDK 兑换成功",
  "dialog.oauthOpened": "已在浏览器打开授权页面，完成后会自动返回 ReTheme",
  "dialog.apiConfigMissing": "此版本未配置 ReTheme 服务签名，GitHub 登录暂不可用。请使用官方发布版，或配置生产密钥后重新构建。",
  "dialog.device": "设备",
  "dialog.inactive": "未激活",
  "dialog.connection": "连接",
  "dialog.redeemCdk": "兑换 CDK",
  "dialog.redeem": "兑换",
  "dialog.logout": "退出账号",
  "dialog.loginTitle": "登录 ReTheme",
  "dialog.registerTitle": "注册 ReTheme",
  "dialog.lead": "账号用于同步主题权益与当前设备状态",
  "dialog.github": "使用 GitHub 登录",
  "dialog.linuxdo": "使用 Linux DO 登录",
  "dialog.emailAlternative": "或使用邮箱",
  "dialog.loginTab": "登录",
  "dialog.registerTab": "注册",
  "dialog.email": "邮箱",
  "dialog.emailCode": "邮箱验证码",
  "dialog.sixDigitCode": "6 位验证码",
  "dialog.send": "发送",
  "dialog.password": "密码",
  "dialog.passwordHint": "至少 8 位",
  "dialog.processing": "处理中…",
  "dialog.registerAndLogin": "注册并登录",
  "file.localThemeTitle": "选择本地主题开发目录",
  "error.browserDownload": "浏览器预览不能下载在线主题",
} as const;

export type TranslationKey = keyof typeof zhCN;
type Messages = Record<TranslationKey, string>;

const en: Messages = {
  "nav.workspace": "Workspace", "nav.accountSection": "Account", "nav.overview": "Overview", "nav.themes": "Themes", "nav.favorites": "Cloud Favorites", "nav.account": "Account", "nav.devices": "Devices", "nav.settings": "Settings", "nav.store": "Online Store", "nav.main": "Main navigation",
  "brand.subtitle": "Engine", "brand.backOverview": "Back to ReTheme overview",
  "common.system": "System", "common.chinese": "简体中文", "common.english": "English", "common.light": "Light", "common.dark": "Dark", "common.install": "Install", "common.installing": "Installing", "common.installed": "Installed", "common.apply": "Apply", "common.applying": "Applying", "common.uninstall": "Uninstall", "common.uninstalling": "Uninstalling", "common.loading": "Loading…", "common.restoring": "Restoring…", "common.online": "Online", "common.offline": "Offline", "common.localDevice": "This Mac",
  "heartbeat.signedOut.title": "Signed out", "heartbeat.signedOut.detail": "Sign in to sync account data", "heartbeat.online.title": "Device online", "heartbeat.online.detail": "Your account is connected and data sync is active", "heartbeat.grace.title": "Temporarily offline", "heartbeat.grace.detail": "The network is unavailable; your theme remains active", "heartbeat.replaced.title": "Device replaced", "heartbeat.replaced.detail": "Sign in again to connect this device", "heartbeat.offline.title": "Account offline", "heartbeat.offline.detail": "Connect to the internet and sign in again",
  "overview.subtitle": "Current theme, ChatGPT connection, and account status", "overview.tier": "Account tier {tier}", "overview.currentTheme": "Current theme", "overview.running": "Running", "overview.official": "Official UI", "overview.officialTheme": "Official ChatGPT theme", "overview.noTheme": "No theme styles are injected", "overview.restore": "Restore theme", "overview.choose": "Choose theme", "overview.app": "App", "overview.waitingDetection": "Waiting for detection", "overview.appReady": "App detected; adapter ready", "overview.detectingApp": "Checking the local app", "overview.accountStatus": "Account status", "overview.diagnostic": "Safety diagnostics", "overview.allPassed": "All checks passed", "overview.diagnosing": "Diagnosing…", "overview.pending": "Not run", "overview.diagnosticPassed": "Loopback only · Fully reversible · {duration} ms", "overview.diagnosticDetail": "Connects only to 127.0.0.1 and is fully reversible", "overview.runDiagnostic": "Run diagnostics", "overview.desktopApp": "Desktop app", "overview.themeEngine": "Theme engine", "overview.localRuntime": "Local runtime", "overview.support": "Support ReTheme", "overview.supportAction": "Star ReTheme on GitHub", "overview.openSource": "Open source", "overview.compatibility": "Compatibility rules", "overview.remoteRevision": "Remote r{revision}", "overview.builtInRules": "Built-in rules", "overview.reading": "Loading", "overview.waitingMatch": "Waiting for match",
  "themes.search": "Search themes…", "themes.localImagesTitle": "Local images from {path}", "themes.localImagesHint": "{count} images scanned · click one to apply it to ChatGPT", "themes.imageCategory": "Image category", "themes.rescanImages": "Rescan", "themes.chooseImageDirectory": "Choose image directory", "themes.changeImageDirectory": "Change directory", "themes.applyImageLabel": "Apply image theme {name}", "themes.noImages": "No supported images were found under {path}", "themes.localTitle": "Choose a theme folder containing manifest.json; local development themes can run without a time limit", "themes.loadLocal": "Load local theme", "themes.restoreOfficial": "Restore theme", "themes.installSuccess": "{name} was {action}; signature verified", "themes.updated": "updated", "themes.added": "installed", "themes.installedTitle": "Installed themes", "themes.communityHint": "Community themes are free; sign-in is required to install", "themes.signInToInstall": "Sign in to ReTheme before installing an online theme", "themes.chatGPTDebugRequired": "ChatGPT has no local debugging channel. ReTheme will not open an isolated copy or restart ChatGPT; the debug flags must be enabled before ChatGPT starts. Theme switching is live after that.", "themes.uninstallLabel": "Uninstall theme {name}", "themes.restoreLabel": "Restore theme {name}", "themes.applyLabel": "Apply theme {name}", "themes.inUse": "In use", "themes.more": "Discover more creator themes on retheme.app", "themes.browseStore": "Browse online store →", "themes.confirmUninstall": "Uninstall “{name}”? Its local theme files will be deleted.", "themes.viewOnline": "View online theme {name}",
  "favorites.subtitle": "Pro cloud sync for favorites and usage history", "favorites.empty": "No cloud favorites yet. Visit the community to find a theme you love.", "favorites.syncing": "Syncing cloud favorites…", "favorites.noAccess": "This benefit is not available", "favorites.view": "View cloud favorites", "favorites.recent": "Recently used",
  "account.subtitle": "Manage sign-in, cloud sync, and devices", "account.signedIn": "Signed in", "account.signedOut": "Signed out", "account.current": "Current account", "account.loginReTheme": "Sign in", "account.proSupporter": "Pro supporter", "account.free": "Free account", "account.syncAfterLogin": "Sign in to sync account data", "account.deviceOnline": "Device online", "account.currentOffline": "Currently offline", "account.proSupporterUpper": "PRO SUPPORTER", "account.status": "Account status", "account.thanks": "Thank you", "account.connected": "Connected", "account.proDetail": "Cloud sync, multi-device sign-in, and a commemorative Pro theme", "account.communityFree": "Community themes remain free with unlimited use", "account.manageData": "Sign in to manage account data", "account.manage": "Manage account", "account.identity": "Account identity", "account.identityHint": "Sign in to view account details", "account.currentDevice": "Current device", "account.proBenefit": "Pro supporter benefits", "account.communityThemes": "Community themes", "account.proBenefitDetail": "Cloud sync, multiple devices, and a commemorative theme", "account.communityDetail": "All community themes are free with unlimited use", "account.proActive": "Pro supporter status is active", "account.notLoggedIn": "Not signed in to ReTheme", "account.unrestricted": "; theme use is not restricted by sign-in or Pro status", "account.login": "Sign in",
  "devices.subtitle": "View devices that used this account and their recent activity", "devices.proEnabled": "Pro multi-device sign-in is enabled", "devices.proAvailable": "Pro supports multiple devices", "devices.proEnabledDetail": "Sign in on multiple devices and sync device history automatically.", "devices.proAvailableDetail": "Upgrade to sign in on multiple devices and sync their history.", "devices.proBenefit": "PRO BENEFIT", "devices.loginDevices": "Signed-in devices", "devices.historyCount": "{count} historical devices", "devices.current": "Current device", "devices.active": "Online device", "devices.history": "Historical device", "devices.firstLogin": "First sign-in", "devices.lastOnline": "Last online", "devices.signedOut": "Signed out", "devices.empty": "No device sign-in history yet.", "devices.syncing": "Syncing device history…", "devices.viewHistory": "View device history", "devices.signInToView": "Sign in to view device history.",
  "settings.subtitle": "Manage ReTheme desktop behavior, updates, and local theme services", "settings.general": "General", "settings.imageLibrary": "Image directory", "settings.imageLibraryDetail": "Scan images in this directory and its subdirectories", "settings.changeImageDirectory": "Change directory", "settings.language": "Language", "settings.languageDetail": "Follow the system by default or choose an interface language", "settings.languageMode": "Interface language", "settings.appearance": "Appearance", "settings.appearanceDetail": "Follow the system light/dark mode or choose one manually", "settings.appearanceMode": "Appearance mode", "settings.launchAtLogin": "Launch at login", "settings.launchAtLoginDetail": "Start ReTheme in the tray after signing in to your computer", "settings.hideToTray": "Hide to tray when closing the window", "settings.hideToTrayDetail": "Use “Quit ReTheme” in the tray menu to exit", "settings.autoDetect": "Detect ChatGPT after launch", "settings.autoDetectDetail": "Check its installed version and theme compatibility", "settings.update": "Updates", "settings.autoUpdate": "Check for updates automatically", "settings.autoUpdateDetail": "Only install updates verified by ReTheme signatures", "settings.currentVersion": "Current version", "settings.stableChannel": "ReTheme {version} · Stable channel", "settings.latestVersion": "ReTheme {version} · Up to date", "settings.latest": "Up to date", "settings.checkNow": "Check for updates", "settings.checking": "Checking…", "settings.available": "ReTheme {version} is available", "settings.installUpdate": "Install and restart", "settings.installing": "Installing…", "settings.themeService": "Theme service", "settings.accountSync": "Sync account data automatically after sign-in", "settings.restoreOfficial": "Restore theme", "settings.themeRunning": "A theme is running; this immediately removes its styles", "settings.noThemeRunning": "No theme is currently running", "settings.restoreNow": "Restore theme", "settings.diagnostics": "Diagnostics", "settings.appPath": "ChatGPT path", "settings.appNotFound": "ChatGPT has not been detected", "settings.redetect": "Detect again",
  "oauth.completing": "Completing third-party sign-in…", "oauth.invalidCallback": "The sign-in callback is incomplete. Please try again.",
  "sidebar.connected": "ChatGPT · Connected", "sidebar.detecting": "Detecting ChatGPT…", "sidebar.disconnected": "ChatGPT not connected", "status.localChannel": "Local theme channel", "status.demoData": "Demo data", "status.localPreview": "Local preview", "status.themeRunning": "Theme running", "status.account": "Account {status}",
  "dialog.close": "Close account dialog", "dialog.loginSuccess": "Signed in", "dialog.registerSuccess": "Registered and signed in", "dialog.debugCode": "Development code: {code}", "dialog.codeSent": "A verification code was sent to your email", "dialog.cdkSuccess": "CDK redeemed", "dialog.oauthOpened": "Authorization opened in your browser. ReTheme will resume automatically when complete.", "dialog.apiConfigMissing": "This build has no ReTheme service signature configured, so GitHub sign-in is unavailable. Use an official release or rebuild with production credentials.", "dialog.device": "Device", "dialog.inactive": "Inactive", "dialog.connection": "Connection", "dialog.redeemCdk": "Redeem CDK", "dialog.redeem": "Redeem", "dialog.logout": "Sign out", "dialog.loginTitle": "Sign in to ReTheme", "dialog.registerTitle": "Create a ReTheme account", "dialog.lead": "Your account syncs theme benefits and device status", "dialog.github": "Continue with GitHub", "dialog.linuxdo": "Continue with Linux DO", "dialog.emailAlternative": "or use email", "dialog.loginTab": "Sign in", "dialog.registerTab": "Register", "dialog.email": "Email", "dialog.emailCode": "Email verification code", "dialog.sixDigitCode": "6-digit code", "dialog.send": "Send", "dialog.password": "Password", "dialog.passwordHint": "At least 8 characters", "dialog.processing": "Processing…", "dialog.registerAndLogin": "Register and sign in",
  "file.localThemeTitle": "Choose local theme development folder", "error.browserDownload": "Online themes cannot be downloaded in browser preview",
};

const messages: Record<AppLocale, Messages> = { "zh-CN": zhCN, en };

function systemLocale(): AppLocale {
  return navigator.languages.some((language) => language.toLowerCase().startsWith("zh")) ? "zh-CN" : "en";
}

function storedPreference(): LocalePreference {
  const stored = window.localStorage.getItem(LOCALE_KEY);
  return stored === "zh-CN" || stored === "en" ? stored : "system";
}

function interpolate(message: string, values?: Record<string, string | number>): string {
  if (!values) return message;
  return message.replace(/\{(\w+)\}/g, (placeholder, key: string) => values[key] === undefined ? placeholder : String(values[key]));
}

export function translate(key: TranslationKey, values?: Record<string, string | number>, language = document.documentElement.lang): string {
  const locale: AppLocale = language.toLowerCase().startsWith("zh") ? "zh-CN" : "en";
  return interpolate(messages[locale][key], values);
}

type I18nContextValue = {
  locale: AppLocale;
  preference: LocalePreference;
  setPreference: (preference: LocalePreference) => void;
  t: (key: TranslationKey, values?: Record<string, string | number>) => string;
};

const I18nContext = createContext<I18nContextValue | null>(null);

export function I18nProvider({ children }: { children: ReactNode }) {
  const [preference, setPreference] = useState<LocalePreference>(storedPreference);
  const [detectedLocale, setDetectedLocale] = useState<AppLocale>(systemLocale);
  const locale = preference === "system" ? detectedLocale : preference;

  useEffect(() => {
    const update = () => setDetectedLocale(systemLocale());
    window.addEventListener("languagechange", update);
    return () => window.removeEventListener("languagechange", update);
  }, []);

  useEffect(() => {
    window.localStorage.setItem(LOCALE_KEY, preference);
    document.documentElement.lang = locale;
  }, [locale, preference]);

  const value = useMemo<I18nContextValue>(() => ({
    locale,
    preference,
    setPreference,
    t: (key, values) => interpolate(messages[locale][key], values),
  }), [locale, preference]);

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): I18nContextValue {
  const value = useContext(I18nContext);
  if (!value) throw new Error("useI18n must be used inside I18nProvider");
  return value;
}

export function localizeTheme(theme: ThemeSummary, locale: AppLocale): ThemeSummary {
  const base = locale.split("-")[0];
  const translated = theme.locales?.[locale] ?? theme.locales?.[base];
  return translated ? {
    ...theme,
    name: translated.name?.trim() || theme.name,
    description: translated.description?.trim() || theme.description,
  } : theme;
}

export function localizeAccountTheme(theme: AccountThemeSummary, locale: AppLocale): AccountThemeSummary {
  const base = locale.split("-")[0];
  const translated = theme.locales?.[locale] ?? theme.locales?.[base];
  return translated ? {
    ...theme,
    name: translated.name?.trim() || theme.name,
    description: translated.description?.trim() || theme.description,
  } : theme;
}
