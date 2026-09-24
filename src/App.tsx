import { useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import {
  applyImageTheme,
  chooseImageLibraryRoot,
  getImageOpacity,
  getImageLibraryRoot,
  isDesktopRuntime,
  listImageAssets,
  openChatGPTWindow,
  restoreOfficialTheme,
  setImageOpacity,
} from "./desktop-api";
import type { ImageAsset, LiveThemeStatus } from "./types";

const DEFAULT_IMAGE_LIBRARY_ROOT = "C:\\image";

function Icon({ name, size = 18 }: { name: string; size?: number }) {
  const paths: Record<string, ReactNode> = {
    monitor: <><rect x="3" y="4" width="18" height="13" rx="2" /><path d="M8 21h8M12 17v4" /></>,
    refresh: <><path d="M20 6v5h-5" /><path d="M19 11a7 7 0 1 0-2 7" /></>,
    folder: <path d="M3 6.5A1.5 1.5 0 0 1 4.5 5H10l2 2h7.5A1.5 1.5 0 0 1 21 8.5v8A1.5 1.5 0 0 1 19.5 18h-15A1.5 1.5 0 0 1 3 16.5v-10Z" />,
    search: <><circle cx="10.8" cy="10.8" r="6.3" /><path d="m16 16 4.2 4.2" /></>,
    sparkle: <><path d="m12 3 1.6 5.4L19 10l-5.4 1.6L12 17l-1.6-5.4L5 10l5.4-1.6L12 3Z" /><path d="m19 15 .8 2.2L22 18l-2.2.8L19 21l-.8-2.2L16 18l2.2-.8L19 15Z" /></>,
  };
  return <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.7" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">{paths[name]}</svg>;
}

function Logo() {
  return <span className="logo-mark" aria-hidden="true"><i /><i /><i /></span>;
}

function formatImageSize(size: number) {
  if (size >= 1024 * 1024) return `${(size / 1024 / 1024).toFixed(1)} MB`;
  return `${Math.max(1, Math.round(size / 1024))} KB`;
}

function formatError(reason: unknown) {
  const message = reason instanceof Error ? reason.message : String(reason);
  if (/ChatGPT is already open without a verified skin connection|Codex is open without a verified Dream Skin CDP endpoint/i.test(message)) {
    return "检测到 ChatGPT / Codex 已在运行，但当前窗口没有 Skin Studio 所需的本地调试连接。为保护现有会话，程序没有强制关闭或重启它。请先保存工作并正常退出 ChatGPT / Codex，再回到这里点击“打开 ChatGPT”；连接后切换主题无需重启。";
  }
  return message.replace(/^Error:\s*/i, "");
}

function isVideoAsset(asset: ImageAsset) {
  return asset.extension === "mp4" || asset.extension === "webm";
}

function VideoPreview({ asset }: { asset: ImageAsset }) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [failed, setFailed] = useState(false);
  const source = convertFileSrc(asset.path);

  useEffect(() => {
    const video = videoRef.current;
    if (!video || failed) return;
    let disposed = false;
    const stop = () => {
      video.pause();
      if (video.hasAttribute("src")) {
        video.removeAttribute("src");
        video.load();
      }
    };
    const play = () => {
      if (disposed) return;
      if (video.getAttribute("src") !== source) {
        video.setAttribute("src", source);
        video.load();
      }
      const playback = video.play();
      if (playback) void playback.catch(() => {});
    };
    if (typeof IntersectionObserver === "undefined") {
      play();
      return () => { disposed = true; stop(); };
    }
    const observer = new IntersectionObserver(([entry]) => {
      if (entry?.isIntersecting) play();
      else stop();
    }, { rootMargin: "120px 0px" });
    observer.observe(video);
    return () => { disposed = true; observer.disconnect(); stop(); };
  }, [failed, source]);

  if (failed) return <span className="preview-fallback">{asset.name}</span>;
  return <video ref={videoRef} muted loop playsInline preload="none" aria-hidden="true" onError={() => setFailed(true)} />;
}

function ImageCard({ asset, active, applying, disabled, onApply }: { asset: ImageAsset; active: boolean; applying: boolean; disabled: boolean; onApply: () => void }) {
  return (
    <article className={`image-card ${active ? "is-active" : ""}`}>
      <button className="image-card-cover" disabled={disabled} onClick={onApply} title={`应用 ${asset.name}`}>
        {isDesktopRuntime() ? isVideoAsset(asset)
          ? <VideoPreview asset={asset} />
          : <img src={convertFileSrc(asset.path)} alt="" loading="lazy" />
          : <span className="preview-fallback">{asset.name}</span>}
        <span className="image-card-overlay">{active ? "正在使用" : applying ? "应用中…" : "点击应用"}</span>
      </button>
      <div className="image-card-meta">
        <div className="image-card-copy"><strong title={asset.name}>{asset.name}</strong><small>{asset.category} · {formatImageSize(asset.size)}</small></div>
        <button className={`pill ${active ? "is-live" : ""}`} disabled={disabled} onClick={onApply}>{active ? "使用中" : applying ? "应用中…" : "应用"}</button>
      </div>
    </article>
  );
}

function HomePage({
  assets, rootPath, status, search, setSearch, opacity, onOpacityChange, onApply, onChooseRoot, onRefresh,
  onRestore, onOpenChatGPT, applyingPath, actionBusy, rootChanging, loading, chatgptReady, error,
}: {
  assets: ImageAsset[];
  rootPath: string;
  status: LiveThemeStatus;
  search: string;
  setSearch: (value: string) => void;
  opacity: number;
  onOpacityChange: (value: number) => void;
  onApply: (asset: ImageAsset) => void;
  onChooseRoot: () => void;
  onRefresh: () => void;
  onRestore: () => void;
  onOpenChatGPT: () => void;
  applyingPath: string;
  actionBusy: string;
  rootChanging: boolean;
  loading: boolean;
  chatgptReady: boolean;
  error: string;
}) {
  const [category, setCategory] = useState("全部");
  const [displayLimit, setDisplayLimit] = useState(24);
  const categories = useMemo(() => ["全部", ...Array.from(new Set(assets.map((asset) => asset.category))).sort((a, b) => a.localeCompare(b, "zh-CN"))], [assets]);
  const keyword = search.trim().toLowerCase();
  const filtered = assets.filter((asset) => {
    const matchesCategory = category === "全部" || asset.category === category;
    const matchesKeyword = !keyword || `${asset.name} ${asset.category}`.toLowerCase().includes(keyword);
    return matchesCategory && matchesKeyword;
  });
  useEffect(() => setDisplayLimit(24), [category, keyword, assets.length]);
  const visibleAssets = filtered.slice(0, displayLimit);
  const busy = Boolean(actionBusy);

  return (
    <main className="app-shell">
      <header className="app-header">
        <div className="brand-lockup"><Logo /><div><strong>ChatGPT Skin</strong><span>本地主题工作室</span></div></div>
        <div className="header-actions">
          <span className={`connection-chip ${chatgptReady ? "is-ready" : ""}`}><i />{chatgptReady ? "ChatGPT 已就绪" : "本地主题"}</span>
          <button className="button primary launch-button" disabled={busy} onClick={onOpenChatGPT}>
            <Icon name="monitor" size={16} />{actionBusy === "opening" ? "正在启动…" : chatgptReady ? "显示 ChatGPT" : "打开 ChatGPT"}
          </button>
        </div>
      </header>

      <section className="settings-strip" aria-label="主页设置">
        <div className="setting-card library-setting">
          <div className="setting-card-icon"><Icon name="folder" size={18} /></div>
          <div className="setting-card-content"><span className="setting-label">背景目录</span><strong title={rootPath}>{rootPath}</strong><small>自动扫描子目录中的图片、动图和视频</small></div>
          <button className="button secondary compact" disabled={busy || rootChanging} onClick={onChooseRoot}>{rootChanging ? "保存中…" : "更换目录"}</button>
        </div>
        <div className="setting-card opacity-setting">
          <div className="setting-card-icon"><Icon name="sparkle" size={18} /></div>
          <div className="opacity-content">
            <div className="setting-line"><div><span className="setting-label">背景透明度</span><small>应用到当前主题</small></div><output>{Math.round(opacity * 100)}%</output></div>
            <input type="range" min="0.1" max="1" step="0.01" value={opacity} onChange={(event) => onOpacityChange(Number(event.target.value))} aria-label="背景透明度" />
          </div>
        </div>
        <div className="setting-card appearance-setting">
          <span className="appearance-indicator"><i /></span>
          <div className="setting-card-content"><span className="setting-label">外观模式</span><strong>自动明暗</strong><small>{status.active ? "主题已实时应用" : "随背景亮度自动适配"}</small></div>
          <button className="button ghost compact restore-button" disabled={busy} onClick={onRestore}><Icon name="refresh" size={14} />恢复官方</button>
        </div>
      </section>

      <section className="library-toolbar" aria-label="主题库工具">
        <div className="library-title"><h1>本地主题</h1><span>{loading ? "扫描中…" : `${filtered.length} 个主题`}</span></div>
        <label className="toolbar-search"><Icon name="search" size={16} /><input type="search" placeholder="搜索主题名称…" aria-label="搜索本地主题" value={search} onChange={(event) => setSearch(event.target.value)} /></label>
        <select value={category} onChange={(event) => setCategory(event.target.value)} aria-label="背景分类">
          {categories.map((item) => <option value={item} key={item}>{item}</option>)}
        </select>
        <button className="button secondary compact refresh-button" disabled={loading} onClick={onRefresh}><Icon name="refresh" size={15} />{loading ? "扫描中" : "重新扫描"}</button>
      </section>

      <section className="gallery-scroll" aria-label="本地主题列表">
        {error && <div className="notice is-error" role="alert"><strong>操作未完成</strong><span>{error}</span></div>}
        {!filtered.length && <div className="empty-state"><span className="empty-icon"><Icon name="sparkle" size={22} /></span><strong>{loading ? "正在读取主题" : assets.length ? "没有找到匹配主题" : "这个目录里还没有主题"}</strong><p>{loading ? "请稍候，正在读取本地媒体文件。" : assets.length ? "试试其他关键词或分类。" : "更换背景目录，或将图片、GIF、MP4 放入当前目录。"}</p></div>}
        <div className="image-grid">
          {visibleAssets.map((asset) => <ImageCard key={asset.id} asset={asset} active={status.imagePath === asset.path} applying={applyingPath === asset.path} disabled={busy} onApply={() => onApply(asset)} />)}
        </div>
        {visibleAssets.length < filtered.length && <button className="button secondary load-more" onClick={() => setDisplayLimit((limit) => limit + 24)}>加载更多 · 剩余 {filtered.length - visibleAssets.length} 个</button>}
      </section>

      <footer className="app-footer"><span><i className={chatgptReady ? "is-ready" : ""} />{chatgptReady ? "桌面连接正常" : "仅在本机运行"}</span><span>支持图片 · GIF · MP4 · WebM</span></footer>
    </main>
  );
}

function App() {
  const [assets, setAssets] = useState<ImageAsset[]>([]);
  const [rootPath, setRootPath] = useState(DEFAULT_IMAGE_LIBRARY_ROOT);
  const [status, setStatus] = useState<LiveThemeStatus>({ active: false });
  const [search, setSearch] = useState("");
  const [applyingPath, setApplyingPath] = useState("");
  const [rootChanging, setRootChanging] = useState(false);
  const [loading, setLoading] = useState(false);
  const [chatgptReady, setChatgptReady] = useState(false);
  const [opacity, setOpacity] = useState(0.8);
  const [error, setError] = useState("");
  const [actionBusy, setActionBusy] = useState("");
  const actionLock = useRef(false);
  const opacityTimer = useRef<number | null>(null);

  async function refreshImages() {
    setLoading(true);
    try {
      setAssets(await listImageAssets());
    } catch (reason) {
      setError(formatError(reason));
    } finally {
      setLoading(false);
    }
  }

  async function runExclusive(label: string, operation: () => Promise<void>) {
    if (actionLock.current) return;
    actionLock.current = true;
    setActionBusy(label);
    setError("");
    try {
      await operation();
    } catch (reason) {
      setError(formatError(reason));
    } finally {
      actionLock.current = false;
      setActionBusy("");
    }
  }

  function openChatGPT() {
    return runExclusive("opening", async () => {
      await openChatGPTWindow();
      setChatgptReady(true);
    });
  }

  useEffect(() => {
    void getImageLibraryRoot().then(setRootPath).catch((reason) => setError(formatError(reason)));
    void getImageOpacity().then(setOpacity).catch((reason) => setError(formatError(reason)));
    void refreshImages();
    return () => { if (opacityTimer.current !== null) window.clearTimeout(opacityTimer.current); };
  }, []);

  function apply(asset: ImageAsset) {
    setApplyingPath(asset.path);
    void runExclusive(`apply:${asset.path}`, async () => {
      const next = await applyImageTheme(asset.path, "zh-CN", opacity);
      setStatus(next);
      setChatgptReady(true);
    }).finally(() => setApplyingPath(""));
  }

  function changeOpacity(value: number) {
    const next = Math.max(0.1, Math.min(1, value));
    setOpacity(next);
    if (opacityTimer.current !== null) window.clearTimeout(opacityTimer.current);
    opacityTimer.current = window.setTimeout(() => {
      void setImageOpacity(next).catch((reason) => setError(formatError(reason)));
    }, 220);
  }

  function restore() {
    void runExclusive("restoring", async () => {
      await restoreOfficialTheme();
      setStatus({ active: false });
    });
  }

  async function chooseRoot() {
    if (actionLock.current) return;
    setRootChanging(true);
    setError("");
    try {
      const selected = await chooseImageLibraryRoot();
      if (selected) {
        setRootPath(selected);
        await refreshImages();
      }
    } catch (reason) {
      setError(formatError(reason));
    } finally {
      setRootChanging(false);
    }
  }

  return <HomePage
    assets={assets}
    rootPath={rootPath}
    status={status}
    search={search}
    setSearch={setSearch}
    opacity={opacity}
    onOpacityChange={changeOpacity}
    onApply={apply}
    onChooseRoot={() => void chooseRoot()}
    onRefresh={() => void refreshImages()}
    onRestore={restore}
    onOpenChatGPT={() => void openChatGPT()}
    applyingPath={applyingPath}
    actionBusy={actionBusy}
    rootChanging={rootChanging}
    loading={loading}
    chatgptReady={chatgptReady}
    error={error}
  />;
}

export default App;
