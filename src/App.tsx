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
  openOnlineStore,
  restoreOfficialTheme,
  setImageOpacity,
} from "./desktop-api";
import type { ImageAsset, LiveThemeStatus } from "./types";

type Page = "themes" | "settings" | "store";

const DEFAULT_IMAGE_LIBRARY_ROOT = "C:\\image";

function Icon({ name, size = 18 }: { name: string; size?: number }) {
  const paths: Record<string, ReactNode> = {
    themes: <><path d="M12 3a9 9 0 1 0 0 18h1.4a1.6 1.6 0 0 0 .5-3.1 1.6 1.6 0 0 1 .5-3.1H17a4 4 0 0 0 4-4A7.8 7.8 0 0 0 12 3Z" /><circle cx="7.5" cy="10" r=".8" fill="currentColor" /><circle cx="10" cy="6.8" r=".8" fill="currentColor" /><circle cx="14" cy="6.6" r=".8" fill="currentColor" /></>,
    settings: <><circle cx="12" cy="12" r="3" /><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.9l.1.1-2.8 2.8-.1-.1a1.7 1.7 0 0 0-1.9-.3 1.7 1.7 0 0 0-1 1.6v.2h-4V21a1.7 1.7 0 0 0-1-1.6 1.7 1.7 0 0 0-1.9.3l-.1.1L4.2 17l.1-.1a1.7 1.7 0 0 0 .3-1.9A1.7 1.7 0 0 0 3 14H2.8v-4H3a1.7 1.7 0 0 0 1.6-1 1.7 1.7 0 0 0-.3-1.9L4.2 7 7 4.2l.1.1a1.7 1.7 0 0 0 1.9.3A1.7 1.7 0 0 0 10 3v-.2h4V3a1.7 1.7 0 0 0 1 1.6 1.7 1.7 0 0 0 1.9-.3l.1-.1L19.8 7l-.1.1a1.7 1.7 0 0 0-.3 1.9 1.7 1.7 0 0 0 1.6 1h.2v4H21a1.7 1.7 0 0 0-1.6 1Z" /></>,
    monitor: <><rect x="3" y="4" width="18" height="13" rx="2" /><path d="M8 21h8M12 17v4" /></>,
    refresh: <><path d="M20 6v5h-5" /><path d="M19 11a7 7 0 1 0-2 7" /></>,
    folder: <><path d="M3 6.5A1.5 1.5 0 0 1 4.5 5H10l2 2h7.5A1.5 1.5 0 0 1 21 8.5v8A1.5 1.5 0 0 1 19.5 18h-15A1.5 1.5 0 0 1 3 16.5v-10Z" /></>,
    arrow: <><path d="M5 12h14" /><path d="m13 6 6 6-6 6" /></>,
    store: <><path d="M4 9h16l-1 11H5L4 9Z" /><path d="M6 9V6.5A2.5 2.5 0 0 1 8.5 4h7A2.5 2.5 0 0 1 18 6.5V9" /><path d="M8 13v3M12 13v3M16 13v3" /></>,
  };
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.7" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      {paths[name]}
    </svg>
  );
}

function Logo() {
  return <span className="logo-mark" aria-hidden="true"><i /><i /><i /></span>;
}

function formatImageSize(size: number) {
  if (size >= 1024 * 1024) return `${(size / 1024 / 1024).toFixed(1)} MB`;
  return `${Math.max(1, Math.round(size / 1024))} KB`;
}

function isVideoAsset(asset: ImageAsset) {
  return asset.extension === "mp4" || asset.extension === "webm";
}

function ImageCard({ asset, active, applying, onApply }: { asset: ImageAsset; active: boolean; applying: boolean; onApply: () => void }) {
  return (
    <article className={`image-card ${active ? "is-active" : ""}`}>
      <button className="image-card-cover" disabled={applying} onClick={onApply} title={`应用 ${asset.name}`}>
        {isDesktopRuntime() ? isVideoAsset(asset)
          ? <video src={convertFileSrc(asset.path)} muted loop playsInline preload="metadata" aria-hidden="true" />
          : <img src={convertFileSrc(asset.path)} alt="" loading="lazy" />
          : <span>{asset.name}</span>}
        <span className="image-card-overlay">{active ? "正在使用" : applying ? "应用中…" : "点击应用"}</span>
      </button>
      <div className="image-card-meta">
        <div><strong title={asset.name}>{asset.name}</strong><small>{asset.category} · {formatImageSize(asset.size)}</small></div>
        <button className={`pill ${active ? "is-live" : ""}`} disabled={applying} onClick={onApply}>{active ? "使用中" : applying ? "应用中…" : "应用"}</button>
      </div>
    </article>
  );
}

function ThemesPage({ assets, rootPath, status, search, setSearch, applyingPath, onApply, onRestore, onChooseRoot, onRefresh, rootChanging, error }: {
  assets: ImageAsset[];
  rootPath: string;
  status: LiveThemeStatus;
  search: string;
  setSearch: (value: string) => void;
  applyingPath: string;
  onApply: (asset: ImageAsset) => void;
  onRestore: () => void;
  onChooseRoot: () => void;
  onRefresh: () => void;
  rootChanging: boolean;
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

  return (
    <section className="page themes-page" aria-labelledby="themes-title">
      <div className="page-heading inline-heading">
        <div><p className="eyebrow">LOCAL THEMES</p><h1 id="themes-title">本地主题</h1><p>首次应用会建立一次调试会话；之后切换主题不重启 ChatGPT。</p></div>
        <button className="button primary" onClick={() => void window.__openChatGPT?.()}><Icon name="monitor" size={16} />打开 ChatGPT</button>
      </div>

      <div className="page-toolbar">
        <label className="toolbar-search"><span aria-hidden="true">⌕</span><input type="search" placeholder="搜索本地背景…" aria-label="搜索本地背景" value={search} onChange={(event) => setSearch(event.target.value)} /></label>
        <span className="toolbar-spacer" />
        <button className="button secondary" onClick={onRestore}><Icon name="refresh" size={15} />恢复官方</button>
      </div>

      <div className="themes-scroll">
        {error && <div className="notice is-error">{error}</div>}
        <section className="local-image-library" aria-labelledby="local-images-title">
          <div className="section-head image-library-head">
            <div><h2 id="local-images-title">{rootPath} 本地背景</h2><small>已扫描 {assets.length} 个背景资源，包含所有子目录</small></div>
            <div className="image-library-actions">
              <button className="button ghost" disabled={rootChanging} onClick={onChooseRoot}>{rootChanging ? "保存中…" : "更换目录"}</button>
              <select value={category} onChange={(event) => setCategory(event.target.value)} aria-label="背景分类">
                {categories.map((item) => <option value={item} key={item}>{item}</option>)}
              </select>
              <button className="button ghost" onClick={onRefresh}>重新扫描</button>
            </div>
          </div>
          <div className="image-grid">
            {visibleAssets.map((asset) => <ImageCard key={asset.id} asset={asset} active={status.imagePath === asset.path} applying={applyingPath === asset.path} onApply={() => onApply(asset)} />)}
          </div>
          {visibleAssets.length < filtered.length && <button className="button ghost load-more" onClick={() => setDisplayLimit((limit) => limit + 24)}>加载更多（剩余 {filtered.length - visibleAssets.length} 张）</button>}
          {!filtered.length && <p className="image-empty">当前目录没有匹配的背景资源。</p>}
        </section>
      </div>
    </section>
  );
}

function StorePage({ onOpenStore }: { onOpenStore: () => void }) {
  return (
    <section className="page settings-page" aria-labelledby="store-title">
      <div className="page-heading"><p className="eyebrow">DREAMSKIN GALLERY</p><h1 id="store-title">在线商店</h1><p>在线商店入口保留；本地媒体换肤仍然可以独立运行。</p></div>
      <div className="settings-scroll">
        <div className="settings-group">
          <h2>DreamSkin 在线主题库</h2>
          <div className="setting-row">
            <div><strong>浏览在线主题</strong><p>在浏览器中打开 DreamSkin Gallery，不会关闭或重启当前 ChatGPT。</p></div>
            <button className="button primary" onClick={onOpenStore}><Icon name="store" size={15} />打开在线商店<Icon name="arrow" size={15} /></button>
          </div>
        </div>
        <div className="settings-group">
          <h2>当前换肤方式</h2>
          <div className="setting-row"><div><strong>本地媒体主题</strong><p>从“本地主题”中点击背景即可实时切换，恢复官方外观也不需要重启。</p></div><span className="setting-state"><i />本地可用</span></div>
        </div>
      </div>
    </section>
  );
}

function SettingsPage({ rootPath, rootChanging, onChooseRoot, onOpenChatGPT, onRestore, active, opacity, onOpacityChange }: { rootPath: string; rootChanging: boolean; onChooseRoot: () => void; onOpenChatGPT: () => void; onRestore: () => void; active: boolean; opacity: number; onOpacityChange: (value: number) => void }) {
  return (
    <section className="page settings-page" aria-labelledby="settings-title">
      <div className="page-heading"><p className="eyebrow">LOCAL SERVICE</p><h1 id="settings-title">设置</h1><p>所有主题文件和配置都保存在本机。</p></div>
      <div className="settings-scroll">
        <div className="settings-group">
          <h2>本地背景目录</h2>
          <div className="setting-row"><div><strong>扫描目录</strong><p className="path-text">{rootPath}</p><p>会递归读取 JPG、PNG、WebP、GIF、MP4 和 WebM。</p></div><button className="button ghost" disabled={rootChanging} onClick={onChooseRoot}><Icon name="folder" size={15} />更换目录</button></div>
        </div>
        <div className="settings-group">
          <h2>ChatGPT 页面</h2>
          <div className="setting-row"><div><strong>当前连接</strong><p>{active ? "本地主题已直接注入当前页面" : "尚未应用本地主题"}</p></div><button className="button secondary" onClick={onOpenChatGPT}><Icon name="monitor" size={15} />打开 ChatGPT</button></div>
          <div className="setting-row"><div><strong>恢复官方外观</strong><p>移除本地样式，不关闭或重启 ChatGPT。</p></div><button className="button secondary" onClick={onRestore}><Icon name="refresh" size={15} />恢复</button></div>
          <div className="setting-row"><div><strong>自动明暗</strong><p>按当前背景首帧采样亮度自动选择深色或浅色外观，切换过程不重启 ChatGPT。</p></div><span className="setting-state"><i />自动</span></div>
          <div className="setting-row opacity-setting"><div><strong>背景透明度</strong><p>调整背景媒体显示强度，当前主题会实时更新。</p></div><label className="opacity-control"><input type="range" min="0.1" max="1" step="0.01" value={opacity} onChange={(event) => onOpacityChange(Number(event.target.value))} aria-label="背景透明度" /><output>{Math.round(opacity * 100)}%</output></label></div>
        </div>
        <div className="settings-group">
          <h2>服务范围</h2>
          <div className="setting-row"><div><strong>本地换肤 + 在线商店</strong><p>媒体主题和实时注入在本机完成；在线商店入口保留，账号与云端同步不参与本地换肤。</p></div><span className="setting-state"><i />可用</span></div>
        </div>
      </div>
    </section>
  );
}

declare global {
  interface Window {
    __openChatGPT?: () => Promise<void>;
  }
}

function App() {
  const [page, setPage] = useState<Page>("themes");
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
  const opacityTimer = useRef<number | null>(null);

  async function refreshImages() {
    setLoading(true);
    setError("");
    try {
      setAssets(await listImageAssets());
    } catch (reason) {
      setError(String(reason));
    } finally {
      setLoading(false);
    }
  }

  async function openChatGPT() {
    setError("");
    try {
      await openChatGPTWindow();
      setChatgptReady(true);
    } catch (reason) {
      setError(String(reason));
    }
  }

  async function openStore() {
    setError("");
    try {
      await openOnlineStore();
    } catch (reason) {
      setError(String(reason));
    }
  }

  useEffect(() => {
    window.__openChatGPT = openChatGPT;
    void getImageLibraryRoot().then(setRootPath).catch((reason) => setError(String(reason)));
    void getImageOpacity().then(setOpacity).catch((reason) => setError(String(reason)));
    void refreshImages();
    return () => {
      delete window.__openChatGPT;
      if (opacityTimer.current !== null) window.clearTimeout(opacityTimer.current);
    };
  }, []);

  async function apply(asset: ImageAsset) {
    setApplyingPath(asset.path);
    setError("");
    try {
      const next = await applyImageTheme(asset.path, "zh-CN", opacity);
      setStatus(next);
      setChatgptReady(true);
    } catch (reason) {
      setError(String(reason));
    } finally {
      setApplyingPath("");
    }
  }

  function changeOpacity(value: number) {
    const next = Math.max(0.1, Math.min(1, value));
    setOpacity(next);
    if (opacityTimer.current !== null) window.clearTimeout(opacityTimer.current);
    opacityTimer.current = window.setTimeout(() => {
      void setImageOpacity(next).catch((reason) => setError(String(reason)));
    }, 180);
  }

  async function restore() {
    setError("");
    try {
      await restoreOfficialTheme();
      setStatus({ active: false });
    } catch (reason) {
      setError(String(reason));
    }
  }

  async function chooseRoot() {
    setRootChanging(true);
    setError("");
    try {
      const selected = await chooseImageLibraryRoot();
      if (selected) {
        setRootPath(selected);
        await refreshImages();
      }
    } catch (reason) {
      setError(String(reason));
    } finally {
      setRootChanging(false);
    }
  }

  return (
    <main className="app-shell">
      <aside className="app-sidebar">
        <button className="brand" onClick={() => setPage("themes")} aria-label="返回本地主题"><Logo /><span><strong>ChatGPT Skin</strong><small>本地换肤工作室</small></span></button>
        <nav className="side-nav" aria-label="主导航">
          <div className="nav-section">
            <span className="nav-label">本地服务</span>
            <button className={`nav-item ${page === "themes" ? "is-active" : ""}`} onClick={() => setPage("themes")}><Icon name="themes" size={16} />本地主题<em className="nav-badge">{assets.length}</em></button>
            <button className={`nav-item ${page === "settings" ? "is-active" : ""}`} onClick={() => setPage("settings")}><Icon name="settings" size={16} />设置</button>
          </div>
          <div className="nav-section">
            <span className="nav-label">在线服务</span>
            <button className={`nav-item ${page === "store" ? "is-active" : ""}`} onClick={() => setPage("store")}><Icon name="store" size={16} />在线商店</button>
          </div>
        </nav>
        <div className="sidebar-foot">
          <div className={`codex-chip ${chatgptReady ? "is-ok" : ""}`}><i />{chatgptReady ? "ChatGPT 页面已打开" : "ChatGPT 页面未打开"}</div>
          <button className="button secondary" onClick={() => void openChatGPT()}><Icon name="monitor" size={15} />{chatgptReady ? "显示 ChatGPT" : "打开 ChatGPT"}</button>
        </div>
      </aside>

      <div className="app-content">
        {page === "themes" && <ThemesPage assets={assets} rootPath={rootPath} status={status} search={search} setSearch={setSearch} applyingPath={applyingPath} onApply={(asset) => void apply(asset)} onRestore={() => void restore()} onChooseRoot={() => void chooseRoot()} onRefresh={() => void refreshImages()} rootChanging={rootChanging} error={error} />}
        {page === "settings" && <SettingsPage rootPath={rootPath} rootChanging={rootChanging} onChooseRoot={() => void chooseRoot()} onOpenChatGPT={() => void openChatGPT()} onRestore={() => void restore()} active={status.active} opacity={opacity} onOpacityChange={changeOpacity} />}
        {page === "store" && <StorePage onOpenStore={() => void openStore()} />}
      </div>

      <footer className="app-statusbar">
        <span><i className={chatgptReady ? "is-ok" : ""} /> 本地服务</span>
        <span>{status.active ? "主题已实时应用" : "官方外观"}</span>
        <span>不重启 · 在线商店保留</span>
        <span>ChatGPT Skin</span>
      </footer>
    </main>
  );
}

export default App;
