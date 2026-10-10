// Canonical cross-platform renderer. Run tools/sync-runtime-assets.mjs after editing.
((cssText, artDataUrl, themeConfig) => {
  const SELECTOR_CONTRACT = {"schema":"codex-dream-skin-selectors/1","selectors":[{"key":"shell-main","selector":"main:is(.main-surface, [data-app-shell-main-surface], [class*=\"_MainContentSurface_\"])","tier":"L1","scope":"all","required":true},{"key":"left-panel","selector":"aside.app-shell-left-panel","tier":"L1","scope":"all","required":true},{"key":"header-tint","selector":"header:is(.app-header-tint, [data-app-shell-header-edge-scroll], [class*=\"_Header_\"])","tier":"L1","scope":"all","required":true},{"key":"main-content-top-fade","selector":":is(.app-shell-main-content-top-fade, [class*=\"_MainContentTopFade_\"])","tier":"L2","scope":"all","required":false},{"key":"home-icon","selector":"[data-testid=\"home-icon\"]","tier":"L1","scope":"home","required":true},{"key":"home-route","selector":"[role=\"main\"]:has([data-testid=\"home-icon\"])","tier":"L1","scope":"home","required":true},{"key":"home-route-css","selector":"[role=\"main\"]","tier":"L1","scope":"home","required":true},{"key":"home-banners","selector":".home-banners","tier":"L2","scope":"home","required":false},{"key":"composer-chrome","selector":".composer-surface-chrome","tier":"L2","scope":"home+thread","required":false},{"key":"composer-toolbar","selector":".composer-surface-chrome [class*=\"_footer_\"]","tier":"L2","scope":"home+thread","required":false},{"key":"home-utility","selector":"[class*=\"_homeUtilityBar_\"]","tier":"L2","scope":"home","required":false},{"key":"game-source","selector":"[data-feature=\"game-source\"]","tier":"L2","scope":"home","required":false},{"key":"home-suggestions","selector":".group\\/home-suggestions","tier":"L2","scope":"home","required":false},{"key":"project-selector","selector":".group\\/project-selector","tier":"L2","scope":"home config","required":false},{"key":"markdown","selector":"[class*=\"_markdown\"]","tier":"L2","scope":"thread","required":false},{"key":"thread-surface","selector":".thread-scroll-container","tier":"L2","scope":"thread","required":false},{"key":"message","selector":":is([data-message-author-role], [data-local-conversation-user-anchor], [data-local-conversation-final-assistant])","tier":"L2","scope":"thread","required":false},{"key":"settings-panel","selector":"[data-settings-panel-slug=\"general-settings\"]","tier":"L2","scope":"settings","required":false},{"key":"appearance-radio","selector":"input[name=\"appearance-theme\"]","tier":"L2","scope":"settings","required":false},{"key":"overlay-menu","selector":"[role=\"menu\"]","tier":"L2","scope":"overlay","required":false},{"key":"overlay-dialog","selector":"[role=\"dialog\"]","tier":"L2","scope":"overlay","required":false},{"key":"overlay-popper","selector":"[data-radix-popper-content-wrapper]","tier":"L2","scope":"overlay","required":false}],"stableTestids":["app-shell-header-context-menu-surface","home-icon","theme-preview"]};
  const STATE_KEY = "__CODEX_DREAM_SKIN_STATE__";
  const DISABLED_KEY = "__CODEX_DREAM_SKIN_DISABLED__";
  const STYLE_REGISTRY_KEY = "__CODEX_DREAM_SKIN_STYLE_SHEETS__";
  const STYLE_ID = "codex-dream-skin-style";
  const VIDEO_ID = "codex-dream-skin-video";
  const SHELL_ATTR = "data-dream-shell";
  const PART_ATTR = "data-ds-part";
  const ROOT_ATTRS = [
    "data-dream-main-present", "data-dream-main-home", "data-dream-main-or-home",
    "data-dream-skin", SHELL_ATTR,
    "data-dream-art-wide", "data-dream-art-safe", "data-dream-task-mode",
    "data-dream-art-safe-area", "data-dream-art-task-mode", "data-dream-art-aspect",
    "data-dream-art-ready", "data-dream-media", "data-dream-base-state",
  ];
  const VERSION = __DREAM_SKIN_VERSION_JSON__;
  const STYLE_REVISION = __DREAM_SKIN_STYLE_REVISION_JSON__;
  const PAYLOAD_REVISION = __DREAM_SKIN_PAYLOAD_REVISION_JSON__;
  const THEME = themeConfig && typeof themeConfig === "object" ? themeConfig : {};
  const ART = THEME.art && typeof THEME.art === "object" ? THEME.art : {};
  const ART_METADATA = THEME.artMetadata && typeof THEME.artMetadata === "object"
    ? THEME.artMetadata : null;
  const MEDIA_TYPE = THEME.mediaType === "video" ? "video" : "image";
  const ANALYSIS_CACHE_KEY = "__CODEX_DREAM_SKIN_ANALYSIS_CACHE__";
  const THEME_VARIABLES = [
    "--ds-bg", "--ds-panel", "--ds-panel-2", "--ds-green", "--ds-lime", "--ds-on-accent",
    "--ds-cyan", "--ds-purple", "--ds-text", "--ds-muted", "--ds-line",
    "--ds-bg-rgb", "--ds-panel-rgb", "--ds-panel-2-rgb", "--ds-accent-rgb",
    "--ds-accent-alt-rgb", "--ds-secondary-rgb", "--ds-highlight-rgb",
    "--ds-text-rgb", "--ds-muted-rgb", "--ds-line-rgb",
    "--dream-art-focus-x", "--dream-art-focus-y", "--dream-art-position",
    "--dream-skin-focus-x", "--dream-skin-focus-y", "--dream-skin-art-position",
    "--dream-skin-name", "--dream-skin-tagline", "--dream-skin-project-prefix",
    "--dream-skin-project-label", "--dream-skin-brand-subtitle", "--dream-skin-status",
    "--dream-skin-quote", "--dream-skin-art",
    "--ds-theme-color-background", "--ds-theme-color-panel",
    "--ds-theme-color-panel-alt", "--ds-theme-color-accent",
    "--ds-theme-color-accent-alt", "--ds-theme-color-secondary",
    "--ds-theme-color-highlight", "--ds-theme-color-text",
    "--ds-theme-color-muted", "--ds-theme-color-line",
    "--ds-theme-font-family", "--ds-theme-font-scale",
    "--ds-theme-surface-radius", "--ds-theme-surface-opacity",
    "--ds-theme-surface-blur", "--ds-theme-surface-border-alpha",
    "--ds-theme-surface-shadow", "--ds-theme-image-opacity", "--ds-theme-image-focus-x",
    "--ds-theme-image-focus-y", "--ds-theme-image-zoom",
    "--ds-theme-image-dim", "--ds-theme-image-task-intensity",
    "--ds-theme-density-scale", "--ds-theme-motion-level",
  ];
  const selectorByKey = new Map(SELECTOR_CONTRACT.selectors.map((entry) => [entry.key, entry]));
  const stableTestidSelector = (testid) => SELECTOR_CONTRACT.stableTestids?.includes(testid)
    ? `[data-testid="${testid}"]` : null;
  const installToken = {};
  const existingAnalysisCache = window[ANALYSIS_CACHE_KEY];
  const analysisCache = existingAnalysisCache && typeof existingAnalysisCache.get === "function" &&
    typeof existingAnalysisCache.set === "function" ? existingAnalysisCache : new Map();
  window[ANALYSIS_CACHE_KEY] = analysisCache;
  let artAnalysis = typeof THEME.artKey === "string" ? analysisCache.get(THEME.artKey) ?? null : null;
  let analysisTimer = null;
  let rootObserver = null;
  let partObserver = null;
  let bodyReadyHandler = null;
  let styleMode = null;
  let styleNode = null;
  let styleSheet = null;
  let backgroundVideo = null;
  let profileMenuViewport = null;
  let profileMenuElement = null;
  let profileMenuResizeObserver = null;
  let threadClipScroll = null;
  let threadClipContent = null;
  let threadClipFooter = null;
  let threadClipResizeObserver = null;
  let threadClipFrame = 0;
  let threadClipOriginal = null;
  const diffStyleSnapshots = new Map();
  const now = () => typeof performance === "object" && typeof performance.now === "function"
    ? performance.now() : Date.now();
  const metrics = {
    ensureCalls: 0,
    rootPasses: 0,
    routePasses: 0,
    layoutReads: 0,
    attributeWrites: 0,
    styleWrites: 0,
    styleRepairs: 0,
    partPasses: 0,
    partWrites: 0,
    navigationEvents: 0,
    safetyPasses: 0,
    analysisRuns: 0,
    analysisCacheHits: artAnalysis ? 1 : 0,
    firstEnsureMs: null,
    analysisMs: null,
  };

  const previous = window[STATE_KEY];
  if (typeof previous?.cleanup === "function") previous.cleanup();
  window[DISABLED_KEY] = false;

  const existingStyleRegistry = window[STYLE_REGISTRY_KEY];
  const styleRegistry = existingStyleRegistry instanceof Set ? existingStyleRegistry : new Set();
  window[STYLE_REGISTRY_KEY] = styleRegistry;
  const artUrl = (() => {
    const comma = artDataUrl.indexOf(",");
    const mime = /^data:([^;,]+)/.exec(artDataUrl)?.[1] || "image/png";
    const binary = atob(artDataUrl.slice(comma + 1));
    const bytes = new Uint8Array(binary.length);
    for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
    return URL.createObjectURL(new Blob([bytes], { type: mime }));
  })();

  const ensureBackgroundVideo = () => {
    if (MEDIA_TYPE !== "video" || !document?.createElement) return null;
    if (backgroundVideo?.isConnected) return backgroundVideo;
    const existing = document.getElementById(VIDEO_ID);
    const video = existing instanceof HTMLVideoElement ? existing : document.createElement("video");
    video.id = VIDEO_ID;
    video.muted = true;
    video.loop = true;
    video.autoplay = true;
    video.playsInline = true;
    video.preload = "auto";
    video.setAttribute("aria-hidden", "true");
    if (video.src !== artUrl) video.src = artUrl;
    if (!video.isConnected) document.documentElement?.insertBefore(video, document.body || null);
    backgroundVideo = video;
    video.play().catch(() => {});
    return video;
  };

  const cssString = (value) => JSON.stringify(String(value ?? ""));

  const setStyleProperty = (root, name, value) => {
    if (root.style.getPropertyValue(name) !== value) {
      root.style.setProperty(name, value);
      metrics.styleWrites += 1;
    }
  };

  const setAttribute = (root, name, value) => {
    const normalized = String(value);
    if (root.getAttribute(name) !== normalized) {
      root.setAttribute(name, normalized);
      metrics.attributeWrites += 1;
    }
  };

  const parseRgb = (value) => {
    if (!value || value === "transparent") return null;
    const hex = String(value).trim().match(/^#([0-9a-f]{3,4}|[0-9a-f]{6}|[0-9a-f]{8})$/i);
    if (hex) {
      const digits = hex[1];
      const rgbHex = digits.length <= 4
        ? digits.slice(0, 3).split("").map((digit) => `${digit}${digit}`).join("")
        : digits.slice(0, 6);
      const alphaHex = digits.length === 4
        ? `${digits[3]}${digits[3]}`
        : digits.length === 8 ? digits.slice(6, 8) : "ff";
      const number = Number.parseInt(rgbHex, 16);
      return {
        r: number >> 16,
        g: (number >> 8) & 255,
        b: number & 255,
        alpha: Number.parseInt(alphaHex, 16) / 255,
      };
    }
    const m = String(value).trim().match(
      /^rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)(?:\s*,\s*([\d.]+))?\s*\)$/i,
    );
    if (!m) return null;
    return {
      r: Number(m[1]),
      g: Number(m[2]),
      b: Number(m[3]),
      alpha: m[4] === undefined ? 1 : Math.min(1, Math.max(0, Number(m[4]))),
    };
  };

  const clamp = (value, min, max) => Math.min(max, Math.max(min, value));

  const rgbString = (value) => {
    const rgb = parseRgb(value);
    return rgb ? [rgb.r, rgb.g, rgb.b]
      .map((channel) => Math.round(clamp(channel, 0, 255)))
      .join(" ") : null;
  };

  const rgbToHex = ({ r, g, b }) => `#${[r, g, b]
    .map((value) => clamp(Math.round(value), 0, 255).toString(16).padStart(2, "0"))
    .join("")}`;

  const relativeLuminance = ({ r, g, b }) => {
    const channels = [r, g, b].map((value) => {
      const normalized = clamp(value, 0, 255) / 255;
      return normalized <= 0.04045
        ? normalized / 12.92
        : ((normalized + 0.055) / 1.055) ** 2.4;
    });
    return 0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2];
  };

  const compositeColor = (value, background, alphaOverride = null) => {
    const foreground = parseRgb(value);
    if (!foreground) return background;
    const alpha = clamp(alphaOverride ?? foreground.alpha ?? 1, 0, 1);
    return {
      r: clamp(foreground.r, 0, 255) * alpha + background.r * (1 - alpha),
      g: clamp(foreground.g, 0, 255) * alpha + background.g * (1 - alpha),
      b: clamp(foreground.b, 0, 255) * alpha + background.b * (1 - alpha),
    };
  };

  const readableAccentInk = (accent, panel) => {
    // The send button sits on the composer surface, which renders panel RGB
    // at 94% regardless of the panel color's declared alpha. Compare against
    // both possible backdrop extremes so artwork cannot flip the decision.
    const luminances = [0, 255].map((backdrop) => {
      const surface = compositeColor(
        panel,
        { r: backdrop, g: backdrop, b: backdrop },
        0.94,
      );
      return relativeLuminance(compositeColor(accent, surface));
    });
    const whiteContrast = Math.min(...luminances.map((value) => 1.05 / (value + 0.05)));
    const blackContrast = Math.min(...luminances.map((value) => (value + 0.05) / 0.05));
    return whiteContrast >= blackContrast ? "rgb(255 255 255)" : "rgb(0 0 0)";
  };

  const rgbToHsl = ({ r, g, b }) => {
    const values = [r, g, b].map((value) => value / 255);
    const max = Math.max(...values);
    const min = Math.min(...values);
    const lightness = (max + min) / 2;
    if (max === min) return { h: 0, s: 0, l: lightness };
    const delta = max - min;
    const saturation = lightness > 0.5 ? delta / (2 - max - min) : delta / (max + min);
    let hue;
    if (max === values[0]) hue = (values[1] - values[2]) / delta + (values[1] < values[2] ? 6 : 0);
    else if (max === values[1]) hue = (values[2] - values[0]) / delta + 2;
    else hue = (values[0] - values[1]) / delta + 4;
    return { h: hue * 60, s: saturation, l: lightness };
  };

  const hslToRgb = ({ h, s, l }) => {
    const hue = ((h % 360) + 360) % 360 / 360;
    if (s === 0) {
      const neutral = Math.round(l * 255);
      return { r: neutral, g: neutral, b: neutral };
    }
    const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
    const p = 2 * l - q;
    const channel = (offset) => {
      let t = hue + offset;
      if (t < 0) t += 1;
      if (t > 1) t -= 1;
      if (t < 1 / 6) return p + (q - p) * 6 * t;
      if (t < 1 / 2) return q;
      if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
      return p;
    };
    return { r: channel(1 / 3) * 255, g: channel(0) * 255, b: channel(-1 / 3) * 255 };
  };

  const detectShellAppearance = () => {
    const root = document.documentElement;
    if (root?.classList?.contains("electron-dark")) return "dark";
    if (root?.classList?.contains("electron-light")) return "light";
    try { return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"; } catch {}
    return "light";
  };

  const makeAdaptivePalette = (sample, shell) => {
    const source = sample || { r: 108, g: 126, b: 136 };
    const hsl = rgbToHsl(source);
    const hue = hsl.s < 0.12 ? 214 : hsl.h;
    const saturation = clamp(hsl.s, 0.38, 0.72);
    const accent = hslToRgb({ h: hue, s: saturation, l: shell === "light" ? 0.42 : 0.66 });
    const accentAlt = hslToRgb({ h: hue + 12, s: saturation * 0.82, l: shell === "light" ? 0.52 : 0.73 });
    const secondary = hslToRgb({ h: hue - 24, s: saturation * 0.64, l: shell === "light" ? 0.56 : 0.62 });
    const highlight = hslToRgb({ h: hue + 24, s: saturation * 0.76, l: shell === "light" ? 0.36 : 0.58 });
    const neutral = (lightness, chroma = 0.08) => rgbToHex(hslToRgb({ h: hue, s: chroma, l: lightness }));
    return shell === "light" ? {
      background: neutral(0.965, 0.07),
      panel: neutral(0.987, 0.035),
      panelAlt: neutral(0.945, 0.09),
      accent: rgbToHex(accent),
      accentAlt: rgbToHex(accentAlt),
      secondary: rgbToHex(secondary),
      highlight: rgbToHex(highlight),
      text: neutral(0.13, 0.10),
      muted: neutral(0.42, 0.08),
      line: `rgba(${Math.round(accent.r)}, ${Math.round(accent.g)}, ${Math.round(accent.b)}, .24)`,
    } : {
      background: neutral(0.055, 0.045),
      panel: neutral(0.085, 0.04),
      panelAlt: neutral(0.125, 0.05),
      accent: rgbToHex(accent),
      accentAlt: rgbToHex(accentAlt),
      secondary: rgbToHex(secondary),
      highlight: rgbToHex(highlight),
      text: neutral(0.93, 0.025),
      muted: neutral(0.69, 0.03),
      line: `rgba(${Math.round(accent.r)}, ${Math.round(accent.g)}, ${Math.round(accent.b)}, .28)`,
    };
  };

  const resolvedShell = () => {
    if (THEME.appearance === "light" || THEME.appearance === "dark") return THEME.appearance;
    // Custom image themes use the sampled artwork luminance for the shell. The
    // first paint can use the native appearance; analyzeArt then re-applies the
    // live layer without restarting ChatGPT.
    if (artAnalysis?.shell === "light" || artAnalysis?.shell === "dark") return artAnalysis.shell;
    return detectShellAppearance();
  };

  const applyTheme = (root, shell) => {
    const declaredColors = THEME.colors && typeof THEME.colors === "object" ? THEME.colors : {};
    const legacyPalette = THEME.palette && typeof THEME.palette === "object" ? THEME.palette : {};
    // macOS themes use the full `colors` contract; older Windows themes used
    // `palette.accent`. Accept both while keeping one renderer source.
    const colors = Object.keys(declaredColors).length ? declaredColors : legacyPalette;
    const hasExplicitKeyList = Array.isArray(THEME.explicitColorKeys);
    const explicit = new Set(hasExplicitKeyList ? THEME.explicitColorKeys : []);
    if (!hasExplicitKeyList && (THEME.colorMode === "explicit" || !Object.hasOwn(THEME, "colorMode"))) {
      for (const key of Object.keys(declaredColors)) explicit.add(key);
    }
    if (typeof legacyPalette.accent === "string") explicit.add("accent");
    const adaptive = makeAdaptivePalette(artAnalysis?.accentRgb, shell);
    const legacyLight = (THEME.appearance === undefined || THEME.appearance === "auto")
      && THEME.colorMode !== "explicit" && shell === "light";
    const structural = new Set(["background", "panel", "panelAlt", "text", "muted"]);
    const pick = (name) => {
      // 深色图片必须使用浅色正文，即使旧主题包携带了深紫色 text/muted。
      // 浅色图片仍沿用原来的显式颜色与黑色正文，不改变现有浅色主题行为。
      const darkArtworkInk = shell === "dark" && (name === "text" || name === "muted");
      const allowExplicit = explicit.has(name) &&
        !(legacyLight && structural.has(name)) && !darkArtworkInk;
      return allowExplicit && typeof colors[name] === "string" ? colors[name] : adaptive[name];
    };
    const accent = pick("accent");
    const accentAlt = explicit.has("accentAlt") ? pick("accentAlt") : (explicit.has("accent") ? accent : adaptive.accentAlt);
    const variables = {
      "--ds-bg": pick("background"),
      "--ds-panel": pick("panel"),
      "--ds-panel-2": pick("panelAlt"),
      "--ds-green": accent,
      "--ds-lime": accentAlt,
      "--ds-cyan": pick("secondary"),
      "--ds-purple": pick("highlight"),
      "--ds-text": pick("text"),
      "--ds-muted": pick("muted"),
      "--ds-line": explicit.has("line") && typeof colors.line === "string" ? colors.line : adaptive.line,
    };

    for (const [name, value] of Object.entries(variables)) {
      if (typeof value === "string" && value) setStyleProperty(root, name, value);
    }
    if (explicit.has("accent")) {
      const accentInk = readableAccentInk(
        accent,
        variables["--ds-panel"],
      );
      if (accentInk) setStyleProperty(root, "--ds-on-accent", accentInk);
    }
    const publicColors = {
      "--ds-theme-color-background": variables["--ds-bg"],
      "--ds-theme-color-panel": variables["--ds-panel"],
      "--ds-theme-color-panel-alt": variables["--ds-panel-2"],
      "--ds-theme-color-accent": variables["--ds-green"],
      "--ds-theme-color-accent-alt": variables["--ds-lime"],
      "--ds-theme-color-secondary": variables["--ds-cyan"],
      "--ds-theme-color-highlight": variables["--ds-purple"],
      "--ds-theme-color-text": variables["--ds-text"],
      "--ds-theme-color-muted": variables["--ds-muted"],
      "--ds-theme-color-line": variables["--ds-line"],
    };
    for (const [name, value] of Object.entries(publicColors)) {
      if (typeof value === "string" && value) setStyleProperty(root, name, value);
    }
    setStyleProperty(root, "--ds-theme-surface-radius", "12px");
    setStyleProperty(root, "--ds-theme-surface-opacity", "1");
    setStyleProperty(root, "--ds-theme-surface-blur", "0px");
    setStyleProperty(root, "--ds-theme-font-family", "system");
    setStyleProperty(root, "--ds-theme-font-scale", "1");
    setStyleProperty(root, "--ds-theme-surface-border-alpha", "0.14");
    setStyleProperty(root, "--ds-theme-surface-shadow", "soft");
    const imageOpacity = typeof THEME.imageOpacity === "number" && Number.isFinite(THEME.imageOpacity)
      ? clamp(THEME.imageOpacity, 0.1, 1) : 0.8;
    setStyleProperty(root, "--ds-theme-image-opacity", String(Number(imageOpacity.toFixed(3))));
    setStyleProperty(root, "--ds-theme-image-zoom", "1");
    setStyleProperty(root, "--ds-theme-image-dim", "0");
    setStyleProperty(root, "--ds-theme-image-task-intensity", "0.35");
    setStyleProperty(root, "--ds-theme-density-scale", "standard");
    setStyleProperty(root, "--ds-theme-motion-level", "standard");
    const rgbVariables = {
      "--ds-bg-rgb": variables["--ds-bg"],
      "--ds-panel-rgb": variables["--ds-panel"],
      "--ds-panel-2-rgb": variables["--ds-panel-2"],
      "--ds-accent-rgb": variables["--ds-green"],
      "--ds-accent-alt-rgb": variables["--ds-lime"],
      "--ds-secondary-rgb": variables["--ds-cyan"],
      "--ds-highlight-rgb": variables["--ds-purple"],
      "--ds-text-rgb": variables["--ds-text"],
      "--ds-muted-rgb": variables["--ds-muted"],
      "--ds-line-rgb": variables["--ds-line"],
    };
    for (const [name, value] of Object.entries(rgbVariables)) {
      const rgb = rgbString(value);
      if (rgb) setStyleProperty(root, name, rgb);
    }
    setStyleProperty(root, "--dream-skin-name", cssString(THEME.name || "Codex Dream Skin"));
    setStyleProperty(root, "--dream-skin-tagline", cssString(THEME.tagline || "Make something wonderful."));
    setStyleProperty(root, "--dream-skin-quote", cssString(THEME.quote || "MAKE SOMETHING WONDERFUL"));
    setStyleProperty(root, "--dream-skin-brand-subtitle", cssString(
      THEME.brandSubtitle || "CODEX DREAM SKIN",
    ));
    setStyleProperty(root, "--dream-skin-status", cssString(THEME.statusText || "DREAM SKIN ONLINE"));
    setStyleProperty(root, "--dream-skin-project-prefix", cssString(THEME.projectPrefix || "选择项目 · "));
    setStyleProperty(root, "--dream-skin-project-label", cssString(THEME.projectLabel || "◉  选择项目"));
  };

  const applyArtMetadata = (root) => {
    const profile = artAnalysis || ART_METADATA;
    const inferredSafe = profile?.safeArea || "center";
    const safeArea = ART.safeArea && ART.safeArea !== "auto" ? ART.safeArea : inferredSafe;
    const canonicalSafe = ["left", "right", "center", "none"].includes(safeArea)
      ? safeArea : "center";
    const focusX = typeof ART.focusX === "number" ? ART.focusX
      : profile?.focusX ?? (safeArea === "left" ? 0.72 : safeArea === "right" ? 0.28 : 0.5);
    const focusY = typeof ART.focusY === "number" ? ART.focusY : profile?.focusY ?? 0.5;
    const taskMode = ART.taskMode && ART.taskMode !== "auto"
      ? ART.taskMode : profile?.taskMode || "ambient";
    const wide = profile?.wide || false;
    const aspect = profile?.aspect || "unknown";
    const focusXValue = `${(clamp(focusX, 0, 1) * 100).toFixed(2)}%`;
    const focusYValue = `${(clamp(focusY, 0, 1) * 100).toFixed(2)}%`;

    setAttribute(root, "data-dream-art-wide", wide ? "true" : "false");
    setAttribute(root, "data-dream-art-safe", canonicalSafe);
    setAttribute(root, "data-dream-task-mode", taskMode);
    setAttribute(root, "data-dream-art-safe-area", safeArea);
    setAttribute(root, "data-dream-art-task-mode", taskMode);
    setAttribute(root, "data-dream-art-aspect", aspect);
    setAttribute(root, "data-dream-art-ready", artAnalysis ? "true" : "false");
    setStyleProperty(root, "--dream-art-focus-x", focusXValue);
    setStyleProperty(root, "--dream-art-focus-y", focusYValue);
    setStyleProperty(root, "--dream-art-position", `${focusXValue} ${focusYValue}`);
    setStyleProperty(root, "--dream-skin-focus-x", focusXValue);
    setStyleProperty(root, "--dream-skin-focus-y", focusYValue);
    setStyleProperty(root, "--dream-skin-art-position", `${focusXValue} ${focusYValue}`);
    setStyleProperty(root, "--ds-theme-image-focus-x", String(Number(focusX.toFixed(4))));
    setStyleProperty(root, "--ds-theme-image-focus-y", String(Number(focusY.toFixed(4))));
  };

  const analyzeArt = () => new Promise((resolve) => {
    const startedAt = now();
    metrics.analysisRuns += 1;
    if (typeof window.Image !== "function" || !document?.createElement) {
      metrics.analysisMs = Number((now() - startedAt).toFixed(3));
      resolve(null);
      return;
    }
    const media = MEDIA_TYPE === "video" ? ensureBackgroundVideo() : new window.Image();
    if (!media) {
      metrics.analysisMs = Number((now() - startedAt).toFixed(3));
      resolve(null);
      return;
    }
    let settled = false;
    const finish = (value) => {
      if (settled) return;
      settled = true;
      if (analysisTimer) clearTimeout(analysisTimer);
      analysisTimer = null;
      metrics.analysisMs = Number((now() - startedAt).toFixed(3));
      resolve(value);
    };
    analysisTimer = setTimeout(() => finish(null), 6000);
    const analyzeLoadedMedia = () => {
      try {
        const sourceWidth = MEDIA_TYPE === "video" ? media.videoWidth : media.naturalWidth;
        const sourceHeight = MEDIA_TYPE === "video" ? media.videoHeight : media.naturalHeight;
        const ratio = sourceWidth / sourceHeight;
        if (!Number.isFinite(ratio) || ratio <= 0) throw new Error("Invalid image dimensions");
        const maxDimension = 96;
        const width = Math.max(16, Math.round(ratio >= 1 ? maxDimension : maxDimension * ratio));
        const height = Math.max(16, Math.round(ratio >= 1 ? maxDimension / ratio : maxDimension));
        const canvas = document.createElement("canvas");
        canvas.width = width;
        canvas.height = height;
        const context = canvas.getContext?.("2d", { willReadFrequently: true });
        if (!context) throw new Error("Canvas is unavailable");
        context.drawImage(media, 0, 0, width, height);
        const data = context.getImageData(0, 0, width, height).data;
        const samples = new Array(width * height);
        const bins = Array.from({ length: 24 }, () => ({ weight: 0, r: 0, g: 0, b: 0 }));
        let lightTotal = 0;
        let count = 0;

        for (let y = 0; y < height; y += 1) {
          for (let x = 0; x < width; x += 1) {
            const offset = (y * width + x) * 4;
            if (data[offset + 3] < 32) continue;
            const rgb = { r: data[offset], g: data[offset + 1], b: data[offset + 2] };
            const light = (0.2126 * rgb.r + 0.7152 * rgb.g + 0.0722 * rgb.b) / 255;
            const hsl = rgbToHsl(rgb);
            samples[y * width + x] = { light, saturation: hsl.s };
            lightTotal += light;
            count += 1;
            if (hsl.s >= 0.16 && hsl.l >= 0.16 && hsl.l <= 0.86) {
              const bin = bins[Math.min(23, Math.floor(hsl.h / 15))];
              const weight = hsl.s * (1 - Math.abs(hsl.l - 0.52) * 0.85);
              bin.weight += weight;
              bin.r += rgb.r * weight;
              bin.g += rgb.g * weight;
              bin.b += rgb.b * weight;
            }
          }
        }
        if (!count) throw new Error("Image has no visible pixels");
        const brightness = lightTotal / count;
        const information = (start, end) => {
          let total = 0;
          let totalSquared = 0;
          let edges = 0;
          let edgeCount = 0;
          let pixels = 0;
          for (let y = 0; y < height; y += 1) {
            for (let x = start; x < end; x += 1) {
              const sample = samples[y * width + x];
              if (!sample) continue;
              total += sample.light;
              totalSquared += sample.light * sample.light;
              pixels += 1;
              const previous = x > start ? samples[y * width + x - 1] : null;
              const above = y > 0 ? samples[(y - 1) * width + x] : null;
              if (previous) { edges += Math.abs(sample.light - previous.light); edgeCount += 1; }
              if (above) { edges += Math.abs(sample.light - above.light); edgeCount += 1; }
            }
          }
          const mean = pixels ? total / pixels : 0;
          const variance = pixels ? Math.max(0, totalSquared / pixels - mean * mean) : 1;
          return Math.sqrt(variance) * 0.58 + (edgeCount ? edges / edgeCount : 1) * 0.42;
        };
        const zoneWidth = Math.max(1, Math.floor(width * 0.38));
        const leftInformation = information(0, zoneWidth);
        const rightInformation = information(width - zoneWidth, width);
        let safeArea = "center";
        if (leftInformation < rightInformation * 0.86) safeArea = "left";
        else if (rightInformation < leftInformation * 0.86) safeArea = "right";

        let saliencyTotal = 0;
        let saliencyX = 0;
        let saliencyY = 0;
        for (let y = 0; y < height; y += 1) {
          for (let x = 0; x < width; x += 1) {
            const sample = samples[y * width + x];
            if (!sample) continue;
            const previous = x > 0 ? samples[y * width + x - 1] : null;
            const above = y > 0 ? samples[(y - 1) * width + x] : null;
            const edge = (previous ? Math.abs(sample.light - previous.light) : 0) +
              (above ? Math.abs(sample.light - above.light) : 0);
            const weight = 0.01 + Math.abs(sample.light - brightness) * 0.48 +
              sample.saturation * 0.34 + edge * 0.28;
            saliencyTotal += weight;
            saliencyX += (x + 0.5) / width * weight;
            saliencyY += (y + 0.5) / height * weight;
          }
        }
        let focusX = saliencyTotal ? saliencyX / saliencyTotal : 0.5;
        let focusY = saliencyTotal ? saliencyY / saliencyTotal : 0.5;
        if (safeArea === "left") focusX = Math.max(0.64, focusX);
        if (safeArea === "right") focusX = Math.min(0.36, focusX);
        focusX = clamp(focusX, 0.12, 0.88);
        focusY = clamp(focusY, 0.18, 0.82);

        const accentBin = bins.reduce((best, candidate) => candidate.weight > best.weight ? candidate : best, bins[0]);
        const accentRgb = accentBin.weight > 0 ? {
          r: accentBin.r / accentBin.weight,
          g: accentBin.g / accentBin.weight,
          b: accentBin.b / accentBin.weight,
        } : null;
        const aspect = ratio >= 2.25 ? "ultrawide" : ratio >= 1.45 ? "wide"
          : ratio >= 1.08 ? "landscape" : ratio >= 0.9 ? "square" : "portrait";
        finish({
          width: sourceWidth,
          height: sourceHeight,
          ratio,
          wide: ratio >= 1.75,
          aspect,
          brightness,
          shell: brightness >= 0.58 ? "light" : "dark",
          safeArea,
          focusX,
          focusY,
          taskMode: ratio >= 2.25 ? "banner" : "ambient",
          accentRgb,
        });
      } catch {
        finish(null);
      }
    };
    if (MEDIA_TYPE === "video") {
      media.addEventListener("error", () => finish(null), { once: true });
      if (media.readyState >= 2 && media.videoWidth > 0 && media.videoHeight > 0) {
        queueMicrotask(analyzeLoadedMedia);
      } else {
        media.addEventListener("loadeddata", analyzeLoadedMedia, { once: true });
      }
    } else {
      media.onerror = () => finish(null);
      media.onload = analyzeLoadedMedia;
      media.src = artUrl;
    }
  });

  const installStyle = () => {
    try {
      if (!("adoptedStyleSheets" in document) || typeof CSSStyleSheet !== "function") {
        throw new Error("Constructable stylesheets are unavailable");
      }
      const sheet = new CSSStyleSheet();
      if (typeof sheet.replaceSync !== "function") throw new Error("replaceSync is unavailable");
      sheet.replaceSync(cssText);
      const retained = [...document.adoptedStyleSheets]
        .filter((candidate) => !styleRegistry.has(candidate));
      document.adoptedStyleSheets = [...retained, sheet];
      styleRegistry.clear();
      styleRegistry.add(sheet);
      document.getElementById(STYLE_ID)?.remove();
      styleSheet = sheet;
      styleMode = "adopted";
      return;
    } catch {
      styleSheet = null;
    }

    styleNode = document.getElementById(STYLE_ID) || document.createElement("style");
    styleNode.id = STYLE_ID;
    styleNode.textContent = cssText;
    if (!styleNode.parentElement) (document.head || document.documentElement).appendChild(styleNode);
    styleMode = "style";
  };

  const ensureStyle = () => {
    if (styleMode === "adopted" && styleSheet) {
      const current = [...document.adoptedStyleSheets];
      if (!current.includes(styleSheet)) {
        document.adoptedStyleSheets = [...current, styleSheet];
        metrics.styleRepairs += 1;
      }
      return;
    }
    if (styleNode && document.getElementById(STYLE_ID) !== styleNode) {
      document.getElementById(STYLE_ID)?.remove();
      (document.head || document.documentElement).appendChild(styleNode);
      metrics.styleRepairs += 1;
    }
  };

  installStyle();

  const applyRootState = (root) => {
    metrics.rootPasses += 1;
    ensureStyle();
    const shell = resolvedShell();
    setAttribute(root, "data-dream-skin", "active");
    setAttribute(root, SHELL_ATTR, shell);
    setAttribute(root, "data-dream-media", MEDIA_TYPE);
    setStyleProperty(root, "--dream-skin-art", MEDIA_TYPE === "video" ? "none" : `url("${artUrl}")`);
    if (MEDIA_TYPE === "video") ensureBackgroundVideo();
    applyTheme(root, shell);
    applyArtMetadata(root);
    return shell;
  };

  // 新版缓存未激活的 Workspace；只把实际展示的节点作为当前路由信号。
  const isRenderedNode = (node) => {
    if (!node?.isConnected || !node.getClientRects().length) return false;
    const style = getComputedStyle(node);
    return style.display !== "none" && style.visibility !== "hidden" &&
      style.visibility !== "collapse" && style.contentVisibility !== "hidden";
  };
  const selectorHit = (key) => {
    const selector = selectorByKey.get(key)?.selector;
    if (!selector) return false;
    try {
      return [...document.querySelectorAll(selector)].some((node) => {
        if (isRenderedNode(node)) return true;
        if (key !== "home-icon") return false;
        // 皮肤会隐藏首页图标；用可见且已标记的首页容器确认锚点，排除缓存页面。
        const homeSelector = selectorByKey.get("home-route")?.selector;
        const home = homeSelector ? node.closest(homeSelector) : null;
        return home?.getAttribute("data-dream-home-hero") === "true" && isRenderedNode(home);
      });
    } catch { return false; }
  };

  const stableTestidHit = (testid) => {
    const selector = stableTestidSelector(testid);
    if (!selector) return false;
    try { return [...document.querySelectorAll(selector)].some(isRenderedNode); } catch { return false; }
  };

  const partNodes = new Set();
  // 用稳定标记替代全页 :has()，终端频繁增删节点时无需重算整棵界面。
  const surfaceRouteNodes = new Set();
  const SURFACE_ROUTE_ATTRS = [
    "data-dream-surface-route", "data-dream-home-standalone", "data-dream-home-hero",
    "data-dream-home-banners", "data-dream-code-wrapper", "data-dream-settings-content",
    "data-dream-browser-toolbar", "data-dream-turn-diff", "data-dream-turn-diff-header",
    "data-dream-home-utility", "data-dream-thread-search", "data-dream-title-input",
    "data-dream-project-strip", "data-dream-editor-dialog", "data-dream-tooltip-wrapper",
    "data-dream-navigation-rail", "data-dream-selected-tab", "data-dream-command-popover",
  ];
  // 这些关系只在界面结构变化时计算，避免原生终端插入样式表时反复执行 :has()。
  const SURFACE_RELATIONS = [
    ["data-dream-browser-toolbar", '[class~="bg-surface"]', '[class~="@container/browser-toolbar"]'],
    ["data-dream-turn-diff", '[class*="bg-surface-elevated-secondary/50"]', '[class*="turn-diff"]'],
    ["data-dream-turn-diff-header", '[class~="bg-surface-elevated-secondary/50"]', '[class~="group/turn-diff-header"]'],
    ["data-dream-home-utility", '[role="main"]', '[class*="_homeUtilityBar_"]'],
    ["data-dream-thread-search", "div.sticky", 'input[type="text"]'],
    ["data-dream-title-input", "div.no-drag", ':scope > input[type="text"]'],
    ["data-dream-project-strip", "div", ':scope > .horizontal-scroll-fade-mask [class~="group/project-selector"]'],
    ["data-dream-editor-dialog", '[role="dialog"], [aria-modal="true"]', '.monaco-editor, .monaco-diff-editor, .editor-widget, diffs-container'],
    ["data-dream-tooltip-wrapper", "[data-radix-popper-content-wrapper], [data-radix-popper-content]", '[role="tooltip"]'],
    ["data-dream-navigation-rail", "nav", '[data-thread-user-message-navigation-rail-list="true"]'],
    ["data-dream-selected-tab", '[data-tab-id][class~="group/tab"]', '[role="tab"][aria-selected="true"]'],
  ];
  const userMessageRailBindings = new Map();
  const queryAll = (selector) => {
    if (!selector) return [];
    try { return [...document.querySelectorAll(selector)]; } catch { return []; }
  };
  const selectorNodes = (key) => queryAll(selectorByKey.get(key)?.selector);
  const genericNodes = (selector) => queryAll(selector)
    .filter((node) => node && typeof node.setAttribute === "function");
  const genericInputNodes = () => genericNodes(
    'textarea, [contenteditable="true"], [role="textbox"]',
  ).filter((node) => isRenderedNode(node) && !node.closest?.('[role="dialog"], [aria-modal="true"]'));
  const resolvedMainNode = () => {
    const exact = selectorNodes("shell-main").find(isRenderedNode);
    if (exact) return exact;
    for (const input of genericInputNodes()) {
      const main = input.closest?.('main, [role="main"]');
      if (main && typeof main.setAttribute === "function") return main;
    }
    return genericNodes('main, [role="main"]')
      .find((node) => !node.closest?.('[role="dialog"], [aria-modal="true"]')) ?? null;
  };
  const fallbackMainNodes = () => selectorNodes("shell-main").length
    ? [] : [resolvedMainNode()].filter(Boolean);
  const fallbackSidebarNodes = () => {
    if (selectorNodes("left-panel").length) return [];
    const main = resolvedMainNode();
    const mainParent = main?.parentElement;
    if (!main || !mainParent) return [];
    const candidate = genericNodes('aside, nav[aria-label]')
      .filter((node) => !main.contains?.(node))
      .filter((node) => !node.closest?.('[role="dialog"], [aria-modal="true"]'))
      .find((node) => node.parentElement === mainParent
        || node.parentElement?.parentElement === mainParent
        || node.parentElement === mainParent.parentElement);
    return candidate ? [candidate] : [];
  };
  const fallbackComposerNodes = () => selectorNodes("composer-chrome").length
    ? [] : (() => {
      const main = resolvedMainNode();
      for (const input of genericInputNodes()) {
        if (main && !main.contains?.(input)) continue;
        const owner = input.closest?.(
          '[data-testid*="composer" i], [data-testid*="prompt" i], ' +
          '[class*="composer" i], [class*="prompt" i]',
        );
        if (owner && (!main || main.contains?.(owner))) return [owner];
      }
      return [];
    })();
  const addPart = (desired, part, nodes) => {
    for (const node of nodes) {
      if (node && typeof node.setAttribute === "function" && !desired.has(node)) {
        desired.set(node, part);
      }
    }
  };
  const removeUserMessageRailBinding = (rail, binding) => {
    rail.removeEventListener?.("pointerdown", binding.onPointerDown);
    rail.removeEventListener?.("pointermove", binding.onPointerMove);
    rail.removeEventListener?.("pointerup", binding.onPointerEnd);
    rail.removeEventListener?.("pointercancel", binding.onPointerEnd);
    rail.removeAttribute?.("data-dream-skin-scrubbing");
    userMessageRailBindings.delete(rail);
  };
  const refreshUserMessageNavigation = () => {
    const rails = new Set(queryAll('[data-thread-user-message-navigation-rail-list="true"]'));
    for (const [rail, binding] of userMessageRailBindings) {
      if (!rails.has(rail) || !rail.isConnected) removeUserMessageRailBinding(rail, binding);
    }
    for (const rail of rails) {
      if (userMessageRailBindings.has(rail)) continue;
      const binding = { pointerId: null, lastButton: null, startY: 0 };
      const activateNearestButton = (clientY) => {
        const buttons = [...rail.querySelectorAll('button[data-thread-user-message-navigation-item-id]')];
        if (!buttons.length) return;
        let nearest = null;
        let nearestDistance = Number.POSITIVE_INFINITY;
        for (const button of buttons) {
          const rect = button.getBoundingClientRect();
          const distance = Math.abs(clientY - (rect.top + rect.height / 2));
          if (distance < nearestDistance) {
            nearest = button;
            nearestDistance = distance;
          }
        }
        if (!nearest || nearest === binding.lastButton) return;
        binding.lastButton = nearest;
        nearest.click();
      };
      binding.onPointerDown = (event) => {
        if (event.button !== 0) return;
        binding.pointerId = event.pointerId;
        binding.startY = event.clientY;
        binding.lastButton = null;
        rail.setPointerCapture?.(event.pointerId);
        rail.setAttribute?.("data-dream-skin-scrubbing", "true");
        event.preventDefault();
        activateNearestButton(event.clientY);
      };
      binding.onPointerMove = (event) => {
        if (binding.pointerId !== event.pointerId) return;
        activateNearestButton(event.clientY);
      };
      binding.onPointerEnd = (event) => {
        if (binding.pointerId !== event.pointerId) return;
        try { rail.releasePointerCapture?.(event.pointerId); } catch {}
        binding.pointerId = null;
        binding.lastButton = null;
        rail.removeAttribute?.("data-dream-skin-scrubbing");
      };
      rail.addEventListener?.("pointerdown", binding.onPointerDown);
      rail.addEventListener?.("pointermove", binding.onPointerMove);
      rail.addEventListener?.("pointerup", binding.onPointerEnd);
      rail.addEventListener?.("pointercancel", binding.onPointerEnd);
      userMessageRailBindings.set(rail, binding);
    }
  };
  const refreshSurfaceRoutes = () => {
    const mains = selectorNodes("shell-main");
    const homes = [...partNodes].filter((node) => node.getAttribute(PART_ATTR) === "home");
    const homeHeroes = queryAll('[role="main"]').filter((node) => node.querySelector('[data-testid="home-icon"]'));
    const bannerParents = queryAll(".home-banners").map((node) => node.parentElement).filter(Boolean);
    const settingsMains = queryAll("main").filter((node) => node.querySelector("[data-settings-panel-slug]"));
    // 代码块父层的 div:has(> pre) 会让终端中的每个 span 都检查祖先链。
    const codeWrappers = new Set(queryAll('pre, [data-markdown-copy="code-block"]')
      .map((node) => node.parentElement).filter((node) => node?.tagName === "DIV"));
    const markers = new Map();
    const mark = (node, name, value = "true") => {
      if (!markers.has(node)) markers.set(node, new Map());
      markers.get(node).set(name, value);
    };
    for (const node of mains) mark(node, "data-dream-surface-route", node.querySelector('[role="main"]') ? "home" : "thread");
    for (const node of homes) mark(node, "data-dream-home-standalone", String(
      !node.parentElement?.querySelector(`[${PART_ATTR}="main"]`),
    ));
    for (const [nodes, name] of [
      [homeHeroes, "data-dream-home-hero"], [bannerParents, "data-dream-home-banners"],
      [settingsMains, "data-dream-settings-content"], [codeWrappers, "data-dream-code-wrapper"],
    ]) for (const node of nodes) mark(node, name);
    for (const [name, candidateSelector, descendantSelector] of SURFACE_RELATIONS) {
      for (const node of queryAll(candidateSelector)) {
        if (node.querySelector(descendantSelector)) mark(node, name);
      }
    }
    // 搜索项目等命令菜单只在外壳铺底，避免 listbox 与外层叠成深色块。
    for (const node of queryAll("[cmdk-root]")) {
      const shell = node.closest('[role="dialog"], [data-radix-popper-content], [data-slot="popover-content"]') ?? node.parentElement;
      if (shell) mark(shell, "data-dream-command-popover");
    }
    for (const node of surfaceRouteNodes) {
      // 缓存页面复用同一个节点时也移除失效标记，防止设置样式残留到会话。
      for (const name of SURFACE_ROUTE_ATTRS) if (!markers.get(node)?.has(name)) node.removeAttribute(name);
    }
    surfaceRouteNodes.clear();
    setAttribute(document.documentElement, "data-dream-main-present", String(mains.length > 0));
    setAttribute(document.documentElement, "data-dream-main-or-home", String(
      homes.length > 0 || [...partNodes].some((node) => node.getAttribute(PART_ATTR) === "main"),
    ));
    setAttribute(document.documentElement, "data-dream-main-home", String(mains.some(
      (node) => Boolean(node.querySelector('[role="main"]')) && isRenderedNode(node),
    )));
    for (const [node, attributes] of markers) {
      for (const [name, value] of attributes) setAttribute(node, name, value);
      surfaceRouteNodes.add(node);
    }
  };

  const refreshParts = () => {
    metrics.partPasses += 1;
    const desired = new Map();
    addPart(desired, "root", [document.documentElement]);
    addPart(desired, "sidebar", [...selectorNodes("left-panel"), ...fallbackSidebarNodes()]);
    addPart(desired, "header", selectorNodes("header-tint"));
    // Route-specific parts win when a generic shell collapses home and main
    // onto the same element.
    addPart(desired, "home", selectorNodes("home-route"));
    addPart(desired, "main", [...selectorNodes("shell-main"), ...fallbackMainNodes()]);
    addPart(desired, "project-list", selectorNodes("project-selector"));
    addPart(desired, "thread", selectorNodes("thread-surface"));
    addPart(desired, "message", selectorNodes("message"));
    addPart(desired, "composer", [...selectorNodes("composer-chrome"), ...fallbackComposerNodes()]);
    addPart(desired, "composer-toolbar", selectorNodes("composer-toolbar"));
    addPart(desired, "dialog", selectorNodes("overlay-dialog"));
    const homeHero = selectorNodes("game-source")[0] ??
      selectorNodes("home-icon")[0]?.parentElement;
    addPart(desired, "home-hero", homeHero ? [homeHero] : []);

    for (const node of partNodes) {
      if (!desired.has(node)) {
        node.removeAttribute?.(PART_ATTR);
        metrics.partWrites += 1;
      }
    }
    partNodes.clear();
    for (const [node, part] of desired) {
      if (node.getAttribute?.(PART_ATTR) !== part) {
        node.setAttribute(PART_ATTR, part);
        metrics.partWrites += 1;
      }
      partNodes.add(node);
    }
    refreshSurfaceRoutes();
    refreshUserMessageNavigation();
  };

  // The review pane is rendered by <diffs-container> inside an open shadow
  // root. Document-level CSS cannot reach its pre/code/line surfaces, so make
  // the minimal colour changes directly when the review surface is refreshed.
  // Do not observe inline styles here: Monaco virtualization mutates them at a
  // high frequency, and repainting the full shadow tree from that observer can
  // starve Electron's renderer.
  const setDiffStyle = (node, property, value) => {
    if (!node?.style) return;
    let snapshot = diffStyleSnapshots.get(node);
    if (!snapshot) {
      snapshot = new Map();
      diffStyleSnapshots.set(node, snapshot);
    }
    if (!snapshot.has(property)) {
      snapshot.set(property, {
        value: node.style.getPropertyValue(property),
        priority: node.style.getPropertyPriority(property),
      });
    }
    if (node.style.getPropertyValue(property) !== value ||
      node.style.getPropertyPriority(property) !== "important") {
      node.style.setProperty(property, value, "important");
    }
  };

  const styleDiffRoot = (shadowRoot) => {
    const lightShell = document.documentElement?.getAttribute("data-dream-shell") === "light";
    const palette = lightShell
      ? {
        surface: "transparent",
        separator: "transparent",
        header: "transparent",
        active: "transparent",
        text: "rgb(30 32 36)",
        muted: "rgb(99 105 116)",
        selection: "rgb(37 99 235 / .38)",
        selectionText: "rgb(15 23 42)",
      }
      : {
        surface: "transparent",
        separator: "transparent",
        header: "transparent",
        active: "transparent",
        text: "rgb(235 238 244)",
        muted: "rgb(163 171 184)",
        selection: "rgb(96 165 250 / .48)",
        selectionText: "rgb(248 250 252)",
      };
    let scrollbarStyle = shadowRoot.querySelector("style[data-dream-skin-diff-scrollbars]");
    if (!scrollbarStyle) {
      scrollbarStyle = document.createElement("style");
      scrollbarStyle.setAttribute("data-dream-skin-diff-scrollbars", "");
      shadowRoot.append(scrollbarStyle);
    }
    const scrollbarCss = `
      :host { color-scheme: ${lightShell ? "light" : "dark"}; }
      /* 普通代码行还用伪元素铺底，清除它才能透出面板背后的皮肤。 */
      [data-line-type='context']::after {
        background: transparent !important;
      }
      /* 行号节点随滚动重新创建，规则也必须覆盖尚未挂载的行号与装饰层。 */
      [data-gutter],
      [data-gutter] [data-line-type='context'],
      [data-gutter]::before,
      [data-gutter]::after {
        background: transparent !important;
      }
      [data-gutter] [data-line-number-content] {
        color: ${palette.muted} !important;
        -webkit-text-fill-color: ${palette.muted} !important;
      }
      :host :is(.monaco-scrollable-element, .overflow-guard, [data-scrollable]) {
        scrollbar-color: rgb(112 120 135 / .72) transparent;
      }
      :host :is(.monaco-scrollable-element, .overflow-guard, [data-scrollable])::-webkit-scrollbar {
        width: 10px; height: 10px;
      }
      :host :is(.monaco-scrollable-element, .overflow-guard, [data-scrollable])::-webkit-scrollbar-track {
        background: transparent;
      }
      :host :is(.monaco-scrollable-element, .overflow-guard, [data-scrollable])::-webkit-scrollbar-thumb {
        background: rgb(112 120 135 / .62);
        background-clip: padding-box;
        border: 3px solid transparent;
        border-radius: 999px;
      }
      :host::selection {
        color: ${palette.selectionText} !important;
        -webkit-text-fill-color: ${palette.selectionText} !important;
        background: ${palette.selection} !important;
      }
      :host .monaco-editor .selected-text,
      :host .monaco-editor .selectionHighlight,
      :host .monaco-editor .wordHighlight,
      :host .monaco-editor .wordHighlightStrong {
        background-color: ${palette.selection} !important;
      }
    `;
    // 菜单或路由刷新时复用相同样式，避免重复解析并使整个 shadow tree 失效。
    if (scrollbarStyle.textContent !== scrollbarCss) scrollbarStyle.textContent = scrollbarCss;
    const set = (selector, background, color = palette.text) => {
      for (const node of shadowRoot.querySelectorAll(selector)) {
        setDiffStyle(node, "background", background);
        setDiffStyle(node, "background-image", "none");
        setDiffStyle(node, "color", color);
      }
    };
    // 文件预览使用 data-file，差异视图使用 data-diff，二者共享透明正文底色。
    set("pre[data-diff], pre[data-file], code[data-code], [data-gutter], [data-content], [data-separator-wrapper], [data-line-type='context']", palette.surface);
    set("[data-separator], [data-separator-content]", palette.separator, palette.muted);
    set(":is(header, [role='toolbar'], [role='tablist'], [data-diff-header], [data-file-header], [data-toolbar], [data-tabs])", palette.header);
    set(":is([role='tab'][aria-selected='true'], [data-active='true'], [aria-current='true'])", palette.active);
    set("[data-line-type='change-addition']", "rgb(64 201 119 / .16)");
    set("[data-line-type='change-deletion']", "rgb(246 117 118 / .16)");
    for (const node of shadowRoot.querySelectorAll("span[style*='color:#999']")) {
      setDiffStyle(node, "color", palette.muted);
    }
  };

  const refreshDiffSurfaces = () => {
    for (const host of queryAll("diffs-container")) {
      const shadowRoot = host.shadowRoot;
      if (!shadowRoot) continue;
      const lightShell = document.documentElement?.getAttribute("data-dream-shell") === "light";
      setDiffStyle(host, "background", "transparent");
      setDiffStyle(host, "color", lightShell ? "rgb(30 32 36)" : "rgb(235 238 244)");
      styleDiffRoot(shadowRoot);
    }
  };

  const restoreDiffSurfaces = () => {
    for (const host of queryAll("diffs-container")) {
      host.shadowRoot?.querySelector("style[data-dream-skin-diff-scrollbars]")?.remove();
    }
    for (const [node, snapshot] of diffStyleSnapshots) {
      for (const [property, previous] of snapshot) {
        if (previous.value) node.style.setProperty(property, previous.value, previous.priority);
        else node.style.removeProperty(property);
      }
    }
    diffStyleSnapshots.clear();
  };

  const removeParts = () => {
    for (const [rail, binding] of userMessageRailBindings) {
      removeUserMessageRailBinding(rail, binding);
    }
    for (const node of partNodes) node.removeAttribute?.(PART_ATTR);
    partNodes.clear();
    for (const node of queryAll(`[${PART_ATTR}]`)) node.removeAttribute?.(PART_ATTR);
  };

  const scopeMatches = (scope, baseState, overlay) => {
    const active = new Set([baseState]);
    if (baseState !== "settings") active.add("all");
    if (overlay) active.add("overlay");
    const tokens = String(scope || "all").toLowerCase().match(/[a-z]+/g) || ["all"];
    return tokens.some((token) => token !== "config" && active.has(token));
  };

  const clearProfileMenuCover = () => {
    profileMenuResizeObserver?.disconnect();
    profileMenuResizeObserver = null;
    profileMenuElement?.removeAttribute("data-dream-skin-profile-menu");
    profileMenuElement = null;
    profileMenuViewport?.removeAttribute("data-dream-skin-profile-menu-cover");
    profileMenuViewport?.style.removeProperty("--ds-profile-menu-cover-bottom");
    profileMenuViewport = null;
  };

  const refreshProfileMenuCover = () => {
    const sidebar = document.querySelector("aside.app-shell-left-panel");
    // 新版图标栏也有 vertical-scroll-fade-mask，优先选择会话列表的原生标记。
    const viewport = sidebar?.querySelector("[data-app-action-sidebar-scroll]") ??
      sidebar?.querySelector(".sidebar-navigation .vertical-scroll-fade-mask, .vertical-scroll-fade-mask");
    const sidebarRect = sidebar?.getBoundingClientRect();
    const trigger = sidebar && [...sidebar.querySelectorAll('button[aria-haspopup="menu"][aria-expanded="true"]')]
      .find((button) => button.getBoundingClientRect().bottom >= sidebarRect.bottom - 64);
    const triggerRect = trigger?.getBoundingClientRect();
    const menus = triggerRect ? [...document.querySelectorAll('[role="menu"]')] : [];
    // Radix 菜单通过 aria-labelledby 关联按钮；不能再要求菜单完全落在侧栏内。
    const linkedMenu = trigger?.id && menus.find((candidate) =>
      (candidate.getAttribute("aria-labelledby") || "").split(/\s+/).includes(trigger.id));
    const menu = linkedMenu || menus.find((candidate) => {
      const rect = candidate.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0 &&
        rect.left < sidebarRect.right + 12 && rect.right > sidebarRect.left - 12 &&
        Math.abs(rect.bottom - triggerRect.top) <= triggerRect.height + 24;
    });
    if (!viewport || !menu) {
      if (profileMenuViewport) clearProfileMenuCover();
      return;
    }
    const coveredHeight = Math.max(0,
      Math.ceil(viewport.getBoundingClientRect().bottom - menu.getBoundingClientRect().top));
    if (coveredHeight < 1) {
      if (profileMenuViewport) clearProfileMenuCover();
      return;
    }
    if (profileMenuViewport !== viewport || profileMenuElement !== menu) {
      clearProfileMenuCover();
      profileMenuViewport = viewport;
      profileMenuElement = menu;
      if (typeof ResizeObserver === "function") {
        profileMenuResizeObserver = new ResizeObserver(refreshProfileMenuCover);
        profileMenuResizeObserver.observe(menu);
        profileMenuResizeObserver.observe(viewport);
      }
    }
    viewport.style.setProperty("--ds-profile-menu-cover-bottom", `${coveredHeight}px`);
    viewport.setAttribute("data-dream-skin-profile-menu-cover", "true");
    menu.setAttribute("data-dream-skin-profile-menu", "true");
  };

  const detectScope = () => {
    const overlay = selectorHit("overlay-menu") || selectorHit("overlay-dialog") ||
      selectorHit("overlay-popper");
    let baseState = "thread";
    // 新版设置导航使用 general/profile/usage/billing 等 slug，不能仅识别旧版 general-settings。
    const hasSettingsNavigation = queryAll("[data-settings-panel-slug]").some(isRenderedNode);
    // 独立个人资料页没有设置导航，使用原生资料热图和加载骨架识别页面。
    const hasProfileContent = queryAll(
      '[class~="grid-flow-col"][class~="grid-rows-[repeat(7,minmax(1px,1fr))]"], [class*="_profileLoadingBlock_"]',
    ).some(isRenderedNode);
    if (hasSettingsNavigation || hasProfileContent || selectorHit("settings-panel") || selectorHit("appearance-radio") ||
      stableTestidHit("theme-preview")) baseState = "settings";
    else if (selectorHit("home-icon") || selectorHit("home-route")) baseState = "home";
    else if (!selectorHit("shell-main") && !document.querySelector('main, [role="main"]')) baseState = "settings";
    const missingL1 = SELECTOR_CONTRACT.selectors
      .filter((entry) => entry.tier === "L1" && entry.required &&
        scopeMatches(entry.scope, baseState, overlay) && !selectorHit(entry.key))
      .map((entry) => entry.key);
    return {
      state: overlay ? "overlay" : baseState,
      baseState,
      overlay,
      // Settings replaces (or partially replaces) the app shell on macOS and
      // can retain a shell on Windows.  It is therefore always an L0 scope;
      // never treat the absence of the home/thread L1 anchors as a failure.
      level: baseState === "settings" || missingL1.length ? "L0" : "L1",
      missingL1,
    };
  };

  const updateThreadClip = () => {
    threadClipFrame = 0;
    if (!threadClipScroll?.isConnected || !threadClipContent?.isConnected ||
      !threadClipFooter?.isConnected) return;
    const content = threadClipContent.getBoundingClientRect();
    const footer = threadClipFooter.getBoundingClientRect();
    // 输入框上方预留约 80px 的透明遮挡区，对齐会话底部的完整交互区域。
    const cutoff = footer.top - 80;
    const overlap = Math.max(0, Math.min(content.height, content.bottom - cutoff + 1));
    const clip = overlap > 0 ? `inset(0px 0px ${overlap.toFixed(2)}px 0px)` : null;
    if (clip) {
      if (threadClipContent.style.clipPath !== clip) threadClipContent.style.clipPath = clip;
    } else if (threadClipOriginal?.value) {
      threadClipContent.style.setProperty(
        "clip-path", threadClipOriginal.value, threadClipOriginal.priority,
      );
    } else {
      threadClipContent.style.removeProperty("clip-path");
    }
  };
  const scheduleThreadClip = () => {
    if (!threadClipFrame) threadClipFrame = requestAnimationFrame(updateThreadClip);
  };
  const clearThreadClip = () => {
    if (threadClipFrame) cancelAnimationFrame(threadClipFrame);
    threadClipFrame = 0;
    threadClipScroll?.removeEventListener("scroll", scheduleThreadClip);
    window.removeEventListener("resize", scheduleThreadClip);
    threadClipResizeObserver?.disconnect();
    if (threadClipContent && threadClipOriginal) {
      if (threadClipOriginal.value) threadClipContent.style.setProperty(
        "clip-path", threadClipOriginal.value, threadClipOriginal.priority,
      );
      else threadClipContent.style.removeProperty("clip-path");
    }
    threadClipScroll = null;
    threadClipContent = null;
    threadClipFooter = null;
    threadClipResizeObserver = null;
    threadClipOriginal = null;
  };
  const bindThreadClip = () => {
    const active = document.documentElement?.getAttribute("data-dream-skin") === "active" &&
      document.documentElement?.getAttribute("data-dream-art-wide") === "true";
    const scroll = active ? [...document.querySelectorAll(".thread-scroll-container")].find(isRenderedNode) : null;
    const footer = scroll?.querySelector('[data-thread-scroll-footer="true"]');
    const content = scroll?.querySelector("[data-thread-user-message-navigation-content]") ??
      scroll?.querySelector('[data-thread-find-target="conversation"]');
    if (scroll === threadClipScroll && content === threadClipContent &&
      footer === threadClipFooter) {
      if (scroll) scheduleThreadClip();
      return;
    }
    clearThreadClip();
    if (!scroll || !content || !footer) return;
    threadClipScroll = scroll;
    threadClipContent = content;
    threadClipFooter = footer;
    threadClipOriginal = {
      value: content.style.getPropertyValue("clip-path"),
      priority: content.style.getPropertyPriority("clip-path"),
    };
    scroll.addEventListener("scroll", scheduleThreadClip, { passive: true });
    window.addEventListener("resize", scheduleThreadClip, { passive: true });
    if (typeof ResizeObserver === "function") {
      threadClipResizeObserver = new ResizeObserver(scheduleThreadClip);
      threadClipResizeObserver.observe(scroll, { box: "border-box" });
      threadClipResizeObserver.observe(content, { box: "border-box" });
      threadClipResizeObserver.observe(footer, { box: "border-box" });
    }
    scheduleThreadClip();
  };

  const refreshScope = ({ refreshMarkers = true } = {}) => {
    metrics.routePasses += 1;
    if (refreshMarkers) refreshSurfaceRoutes();
    const scope = detectScope();
    setAttribute(document.documentElement, "data-dream-base-state", scope.baseState);
    refreshProfileMenuCover();
    bindThreadClip();
    const state = window[STATE_KEY];
    if (state?.installToken === installToken) state.scope = scope;
    return scope;
  };

  // 顶部额度栏只调用客户端已有的只读接口，登录凭据仍由原生请求层管理。
  const USAGE_BAR_ID = "codex-dream-skin-usage-status";
  const usageBar = {
    element: null, fields: null, timer: null, controller: null, apiPromise: null,
    running: false, stopped: false, lastAttempt: 0,
  };

  // 将有效服务端时间转换为本地时间；缺失或无效时间不显示为已过期。
  const usageDate = (value) => {
    if (typeof value !== "string" && typeof value !== "number") return null;
    const date = new Date(typeof value === "number" ? value * 1000 : value);
    return Number.isFinite(date.getTime()) ? date : null;
  };
  const usageDateText = (date) => date ? new Intl.DateTimeFormat("zh-CN", {
    year: "numeric", month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit",
    hour12: false,
  }).format(date).replaceAll("/", "-") : "未提供";

  // 按窗口时长识别额度，避免不同套餐的 primary/secondary 顺序导致错配。
  const usageWindowText = (limits, seconds) => {
    const window = [limits?.primary_window, limits?.secondary_window]
      .find((entry) => entry?.limit_window_seconds === seconds);
    const used = window?.used_percent;
    if (typeof used !== "number" || !Number.isFinite(used) || used < 0 || used > 100) {
      return { text: "未获取", reset: "未获取", title: "当前账户的额度信息尚未获取" };
    }
    const remaining = Math.round((100 - used) * 10) / 10;
    const reset = usageDateText(usageDate(window.reset_at));
    return { text: `${remaining}%`, reset, title: `额度重置：${reset}` };
  };

  // 根据当前安装的模块地址寻找桌面请求实例，不依赖易变的压缩导出名称。
  const getUsageApi = async () => {
    if (!usageBar.apiPromise) {
      usageBar.apiPromise = (async () => {
        let sharedUrl = [...document.querySelectorAll('link[rel="modulepreload"][href]')]
          .map((link) => link.href).find((url) => /\/app-shared-[\w-]+\.js$/.test(url));
        if (!sharedUrl) {
          const entry = document.querySelector('script[type="module"][src]');
          if (!entry || new URL(entry.src).origin !== location.origin) throw new Error("模块不可用");
          const response = await fetch(entry.src, { signal: usageBar.controller?.signal });
          if (!response.ok) throw new Error("模块不可用");
          const path = (await response.text()).match(/\.\/app-shared-[\w-]+\.js/);
          if (path) sharedUrl = new URL(path[0], entry.src).href;
        }
        if (!sharedUrl || new URL(sharedUrl).origin !== location.origin) throw new Error("模块不可用");
        const exports = await import(sharedUrl);
        const api = Object.values(exports).find((value) => {
          if (!value || typeof value.safeGet !== "function" ||
              typeof value.getRequestTarget !== "function") return false;
          try {
            return value.getRequestTarget("/wham/usage").headers?.originator === "Codex Desktop";
          } catch { return false; }
        });
        if (!api) throw new Error("桌面请求实例不可用");
        return api;
      })().catch((error) => { usageBar.apiPromise = null; throw error; });
    }
    return usageBar.apiPromise;
  };

  // 只更新自身节点，用 textContent 呈现服务端数据，不插入 HTML。
  const setUsageField = (name, text, title = text) => {
    const field = usageBar.fields?.[name];
    if (!field) return;
    if (field.textContent !== text) field.textContent = text;
    if (field.title !== title) field.title = title;
  };
  const showUsageUnavailable = () => {
    setUsageField("five", "5 小时剩余：未获取");
    setUsageField("week", "周额度剩余：未获取");
    setUsageField("resets", "重置次数：未获取");
    setUsageField("expires", "过期时间：未获取");
  };

  // 复用原生使用情况和重置券接口；失败后清除旧值，防止账户切换时误显示。
  const refreshUsageBar = async () => {
    // 首次加载即读取一次，后台窗口之后暂停轮询，恢复可见时再刷新。
    if (usageBar.stopped || usageBar.running ||
        (document.visibilityState === "hidden" && usageBar.lastAttempt > 0)) return;
    usageBar.running = true;
    usageBar.lastAttempt = Date.now();
    const controller = new AbortController();
    usageBar.controller = controller;
    const timeout = setTimeout(() => controller.abort(), 15000);
    try {
      const api = await getUsageApi();
      if (usageBar.stopped || controller.signal.aborted) return;
      const [usageResult, creditsResult] = await Promise.allSettled([
        api.safeGet("/wham/usage", {
          signal: controller.signal,
          additionalHeaders: { "OAI-App-Brand": "codex", "x-openai-codex-pricing-chooser": "1" },
        }),
        api.safeGet("/wham/rate-limit-reset-credits", { signal: controller.signal }),
      ]);
      if (usageBar.stopped) return;
      showUsageUnavailable();
      if (usageResult.status === "fulfilled") {
        const limits = usageResult.value?.rate_limit;
        const five = usageWindowText(limits, 18000);
        const week = usageWindowText(limits, 604800);
        const fiveText = `5 小时剩余：${five.text}（${five.reset} 刷新）`;
        const weekText = `周额度剩余：${week.text}（${week.reset} 刷新）`;
        setUsageField("five", fiveText, `${fiveText}；${five.title}`);
        setUsageField("week", weekText, `${weekText}；${week.title}`);
      }
      if (creditsResult.status === "fulfilled") {
        const credits = creditsResult.value;
        const count = credits?.available_count;
        if (Number.isSafeInteger(count) && count >= 0) {
          setUsageField("resets", `重置次数：${count}`);
          // 仅列出尚未使用且未过期的重置券，按到期时间排列并合并相同时间。
          const groups = new Map();
          for (const credit of Array.isArray(credits.credits) ? credits.credits : []) {
            if (credit?.status !== "available") continue;
            const date = usageDate(credit.expires_at);
            if (date && date.getTime() <= Date.now()) continue;
            const key = date ? date.getTime() : Infinity;
            groups.set(key, (groups.get(key) ?? 0) + 1);
          }
          const times = [...groups.entries()].sort(([a], [b]) => a - b)
            .map(([time, quantity]) => `${usageDateText(Number.isFinite(time) ? new Date(time) : null)}${quantity > 1 ? `（${quantity} 次）` : ""}`);
          const expiry = count === 0 ? "无可用重置" : times.join("、") || "未提供";
          setUsageField("expires", `过期时间：${expiry}`, `重置券过期时间（本地时间）：${expiry}`);
        }
      }
    } catch {
      if (!usageBar.stopped) showUsageUnavailable();
    } finally {
      clearTimeout(timeout);
      usageBar.running = false;
      if (usageBar.controller === controller) usageBar.controller = null;
    }
  };

  // 红框位于原生标题栏：避让左侧菜单、导航按钮和右侧窗口控制按钮。
  const positionUsageBar = () => {
    const host = usageBar.element;
    if (!host) return;
    const header = [...document.querySelectorAll("header")].find((element) => {
      const rect = element.getBoundingClientRect();
      return rect.top < 2 && rect.height > 20 && rect.height < 64 && rect.width > innerWidth * .8;
    });
    if (!header) { host.hidden = true; return; }
    let area;
    try { area = navigator.windowControlsOverlay?.getTitlebarAreaRect(); } catch {}
    const headerRect = header.getBoundingClientRect();
    const height = area?.height > 0 ? Math.min(area.height, headerRect.height) : headerRect.height;
    const right = area?.width > 0 ? area.x + area.width : headerRect.right - 140;
    let left = Math.max(headerRect.left + 12, 160);
    for (const element of document.querySelectorAll('[role="menubar"], header button')) {
      const rect = element.getBoundingClientRect();
      if (rect.width > 0 && rect.top < height && rect.bottom > 0 && rect.right < right * .5) {
        left = Math.max(left, rect.right + 12);
      }
    }
    const width = Math.max(0, right - left - 12);
    host.hidden = width < 260;
    const styles = { left: `${left}px`, top: `${Math.max(0, (height - 28) / 2)}px`, width: `${width}px` };
    for (const [name, value] of Object.entries(styles)) {
      if (host.style[name] !== value) host.style[name] = value;
    }
  };

  // 回到窗口或页面可见时补充刷新；同一分钟内不重复请求。
  const resumeUsageBar = () => {
    positionUsageBar();
    if (Date.now() - usageBar.lastAttempt >= 60000) void refreshUsageBar();
  };
  const ensureUsageBar = () => {
    if (!document.body || usageBar.stopped) return;
    if (!usageBar.element) {
      const host = document.createElement("div");
      host.id = USAGE_BAR_ID;
      host.setAttribute("role", "group");
      host.setAttribute("aria-label", "账户额度与重置券状态");
      const panel = document.createElement("div");
      panel.className = "ds-usage-panel";
      usageBar.fields = {};
      for (const name of ["five", "week", "resets", "expires"]) {
        const field = document.createElement("span");
        field.dataset.usageField = name;
        usageBar.fields[name] = field;
        panel.append(field);
      }
      host.append(panel);
      usageBar.element = host;
      showUsageUnavailable();
      window.addEventListener("resize", positionUsageBar, { passive: true });
      window.addEventListener("focus", resumeUsageBar);
      document.addEventListener("visibilitychange", resumeUsageBar);
      navigator.windowControlsOverlay?.addEventListener("geometrychange", positionUsageBar);
      usageBar.timer = setInterval(() => { void refreshUsageBar(); }, 60000);
      void refreshUsageBar();
    }
    if (!usageBar.element.isConnected) document.body.append(usageBar.element);
    positionUsageBar();
  };

  // 恢复默认或重新换肤时撤销请求、计时器和节点，不留下重复状态栏。
  const stopUsageBar = () => {
    usageBar.stopped = true;
    clearInterval(usageBar.timer);
    usageBar.controller?.abort();
    window.removeEventListener("resize", positionUsageBar);
    window.removeEventListener("focus", resumeUsageBar);
    document.removeEventListener("visibilitychange", resumeUsageBar);
    navigator.windowControlsOverlay?.removeEventListener("geometrychange", positionUsageBar);
    usageBar.element?.remove();
    usageBar.element = null;
    usageBar.fields = null;
    usageBar.apiPromise = null;
  };

  const ensure = ({ root: rootPass = true, scope: scopePass = false, parts: partPass = false } = {}) => {
    if (window[DISABLED_KEY]) return;
    const root = document.documentElement;
    if (!root) return;
    metrics.ensureCalls += 1;
    ensureUsageBar();
    if (rootPass) applyRootState(root);
    if (partPass) {
      refreshParts();
      // Diff 根节点是昂贵的 shadow DOM，仅在路由或内容结构变化后刷新。
      refreshDiffSurfaces();
    }
    if (rootPass || partPass) bindThreadClip();
    if (scopePass) refreshScope({ refreshMarkers: !partPass });
  };

  const cleanup = () => {
    const state = window[STATE_KEY];
    if (state?.installToken !== installToken) return false;
    stopUsageBar();
    window[DISABLED_KEY] = true;
    const root = document.documentElement;
    for (const name of ROOT_ATTRS) root?.removeAttribute(name);
    for (const attribute of [...(root?.attributes || [])]) {
      if (attribute.name.startsWith("data-dream-")) root.removeAttribute(attribute.name);
    }
    for (const name of THEME_VARIABLES) root?.style.removeProperty(name);
    for (const property of [...(root?.style || [])]) {
      if (property.startsWith("--dream-") || property.startsWith("--ds-")) {
        root.style.removeProperty(property);
      }
    }
    removeParts();
    for (const node of surfaceRouteNodes) {
      for (const name of SURFACE_ROUTE_ATTRS) node.removeAttribute(name);
    }
    surfaceRouteNodes.clear();
    restoreDiffSurfaces();
    clearProfileMenuCover();
    clearThreadClip();
    state?.rootObserver?.disconnect();
    state?.partObserver?.disconnect();
    if (bodyReadyHandler && typeof document.removeEventListener === "function") {
      document.removeEventListener("DOMContentLoaded", bodyReadyHandler);
    }
    if (state?.timer) clearInterval(state.timer);
    if (state?.scheduler?.timeout) clearTimeout(state.scheduler.timeout);
    if (analysisTimer) clearTimeout(analysisTimer);
    if (state?.mediaHandler && state?.mediaQuery) {
      try { state.mediaQuery.removeEventListener("change", state.mediaHandler); } catch {}
    }
    if (state?.navigationHandler && state?.navigation) {
      try { state.navigation.removeEventListener("navigate", state.navigationHandler); } catch {}
    }
    if (state?.visibilityHandler && typeof document.removeEventListener === "function") {
      document.removeEventListener("visibilitychange", state.visibilityHandler);
    }
    if (styleSheet) {
      try {
        document.adoptedStyleSheets = [...document.adoptedStyleSheets]
          .filter((candidate) => candidate !== styleSheet);
      } catch {}
      styleRegistry.delete(styleSheet);
    }
    styleNode?.remove();
    if (document.getElementById(STYLE_ID) === styleNode) document.getElementById(STYLE_ID)?.remove();
    if (styleRegistry.size === 0) delete window[STYLE_REGISTRY_KEY];
    const video = state?.backgroundVideo || backgroundVideo || document.getElementById(VIDEO_ID);
    if (video) {
      try {
        video.pause();
        video.removeAttribute("src");
        video.load();
        video.remove();
      } catch {}
    }
    if (state?.artUrl) URL.revokeObjectURL(state.artUrl);
    delete window[STATE_KEY];
    return true;
  };

  const scheduler = { timeout: null, root: false, scope: false, parts: false };
  const flushScheduledEnsure = () => {
    if (scheduler.timeout) clearTimeout(scheduler.timeout);
    scheduler.timeout = null;
    const pending = { root: scheduler.root, scope: scheduler.scope, parts: scheduler.parts };
    scheduler.root = false;
    scheduler.scope = false;
    scheduler.parts = false;
    ensure(pending);
  };
  const scheduleEnsure = ({ root = false, scope = false, parts = false } = {}, delay = 64) => {
    scheduler.root ||= root;
    scheduler.scope ||= scope;
    scheduler.parts ||= parts;
    if (scheduler.timeout) return;
    scheduler.timeout = setTimeout(flushScheduledEnsure, delay);
  };
  const STRUCTURAL_NODE_SELECTOR = [
    "main", "aside", "header", "nav", "dialog", "[role='dialog']", "[role='menu']",
    "[role='listbox']", "[data-radix-popper-content-wrapper]", "[data-floating-ui-portal]",
    "[data-thread-find-target]", "[data-settings-panel-slug]", "[data-testid='home-icon']",
    "[data-ds-part]", "diffs-container", "form", "textarea", "pre", "input[type='text']",
    "[data-markdown-copy='code-block']", "[role='tooltip']", "[class*='turn-diff']",
    "[class~='@container/browser-toolbar']", ".monaco-editor", ".editor-widget",
    "[class*='_homeUtilityBar_']", ".home-banners", "[class~='group/project-selector']", "[cmdk-root]"
  ].join(",");
  const isStructuralNode = (node) => node instanceof Element &&
    (node.matches(STRUCTURAL_NODE_SELECTOR) || Boolean(node.querySelector(STRUCTURAL_NODE_SELECTOR)));
  const hasStructuralMutation = (records) => records.some((record) =>
    [...record.addedNodes, ...record.removedNodes].some(isStructuralNode));
  // 模型菜单由 CSS 直接换肤，不需要重新扫描所有消息与 diff shadow root。
  const MENU_NODE_SELECTOR = "[role='menu'], [role='listbox'], [data-radix-popper-content-wrapper], [data-floating-ui-portal]";
  const CONTENT_NODE_SELECTOR = "main, aside, form, textarea, [role='dialog'], [data-settings-panel-slug], [data-testid='home-icon'], diffs-container";
  const hasContentStructureMutation = (records) => records.some((record) => {
    if (record.target instanceof Element && record.target.closest(MENU_NODE_SELECTOR) &&
      !record.target.closest("[role='dialog']")) return false;
    return [...record.addedNodes, ...record.removedNodes].some((node) => {
      if (!isStructuralNode(node)) return false;
      const hasContent = node.matches(CONTENT_NODE_SELECTOR) || node.querySelector(CONTENT_NODE_SELECTOR);
      const isMenu = node.matches(MENU_NODE_SELECTOR) || node.querySelector(MENU_NODE_SELECTOR);
      return Boolean(hasContent || !isMenu);
    });
  });
  if (typeof MutationObserver === "function") {
    rootObserver = new MutationObserver(() => scheduleEnsure({ root: true }));
    // SPA 路由变化会反映为 DOM 变动；只对会影响界面结构的节点重新标记。
    partObserver = new MutationObserver((records) => {
      // 切换缓存页面只修改 Workspace 的 style，未必增删 DOM 节点。
      if (records.some((record) => record.type === "attributes" &&
        record.target instanceof Element && record.target.matches('[class*="_Workspace_"]'))) {
        scheduleEnsure({ scope: true, parts: true }, 0);
      }
      if (records.some((record) => record.type === "attributes" &&
        record.attributeName === "aria-selected" && record.target instanceof Element &&
        record.target.matches('[role="tab"]'))) {
        scheduleEnsure({ scope: true }, 0);
      }
      if (hasStructuralMutation(records)) {
        scheduleEnsure({ scope: true, parts: hasContentStructureMutation(records) }, 80);
      }
    });
  }

  let mediaQuery = null;
  let mediaHandler = null;
  try {
    mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaHandler = () => scheduleEnsure({ root: true });
  } catch {}

  const navigationApi = window.navigation && typeof window.navigation.addEventListener === "function"
    ? window.navigation : null;
  const navigationHandler = navigationApi ? () => {
    metrics.navigationEvents += 1;
    scheduleEnsure({ scope: true, parts: true }, 180);
  } : null;
  const visibilityHandler = () => {
    if (document.visibilityState !== "visible") return;
    metrics.safetyPasses += 1;
    scheduleEnsure({ root: true, scope: true, parts: true }, 0);
  };

  window[STATE_KEY] = {
    ensure,
    cleanup,
    rootObserver,
    partObserver,
    timer: null,
    scheduler,
    mediaQuery,
    mediaHandler,
    navigation: navigationApi,
    navigationHandler,
    visibilityHandler,
    artUrl,
    backgroundVideo,
    installToken,
    styleMode,
    styleNode,
    styleSheet,
    styleRevision: STYLE_REVISION,
    analysis: artAnalysis,
    artMetadata: ART_METADATA,
    scope: null,
    selectorsSchema: SELECTOR_CONTRACT.schema,
    metrics,
    version: VERSION,
    themeId: THEME.id || "custom",
    revision: PAYLOAD_REVISION,
    detectShellAppearance,
  };
  const firstEnsureStartedAt = now();
  ensure({ root: true, parts: true });
  const initialScope = refreshScope();
  metrics.firstEnsureMs = Number((now() - firstEnsureStartedAt).toFixed(3));

  const observeAttributes = (node) => {
    if (!rootObserver || !node) return;
    rootObserver.observe(node, {
      attributes: true,
      attributeFilter: ["class", "data-theme", "data-appearance", "data-color-mode"],
    });
  };
  const observePartTree = (node) => {
    if (!partObserver || !node) return;
    partObserver.observe(node, { childList: true, subtree: true,
      attributes: true, attributeFilter: ["style", "hidden", "inert", "aria-selected"] });
  };
  observeAttributes(document.documentElement);
  const observeBody = () => {
    observeAttributes(document.body);
    observePartTree(document.body);
  };
  if (document.body) observeBody();
  else if (typeof document.addEventListener === "function") {
    bodyReadyHandler = () => {
      if (!window[DISABLED_KEY]) {
        observeBody();
        scheduleEnsure({ scope: true, parts: true }, 0);
      }
    };
    document.addEventListener("DOMContentLoaded", bodyReadyHandler, { once: true });
  }
  if (mediaHandler && mediaQuery && typeof mediaQuery.addEventListener === "function") {
    mediaQuery.addEventListener("change", mediaHandler);
  }
  if (navigationHandler && navigationApi) {
    navigationApi.addEventListener("navigate", navigationHandler);
  }
  if (typeof document.addEventListener === "function") {
    document.addEventListener("visibilitychange", visibilityHandler);
  }
  const analysisPromise = artAnalysis ? Promise.resolve(null) : analyzeArt();
  window[STATE_KEY].analysisTimer = analysisTimer;
  analysisPromise.then((analysis) => {
    const state = window[STATE_KEY];
    if (!analysis || state?.installToken !== installToken || window[DISABLED_KEY]) return;
    artAnalysis = analysis;
    state.analysis = analysis;
    if (typeof THEME.artKey === "string") {
      analysisCache.set(THEME.artKey, analysis);
      while (analysisCache.size > 8) analysisCache.delete(analysisCache.keys().next().value);
    }
    ensure({ root: true });
  }).catch(() => {});
  return {
    installed: true,
    version: VERSION,
    themeId: THEME.id || "custom",
    revision: PAYLOAD_REVISION,
    shell: resolvedShell(),
    scope: initialScope,
    styleMode,
    analysis: artAnalysis,
  };
})(__DREAM_SKIN_CSS_JSON__, __DREAM_SKIN_ART_JSON__, __DREAM_SKIN_THEME_JSON__)
