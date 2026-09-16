# ReTheme 主题开发规范 v1

本规范是 ReTheme 桌面端主题引擎、社区投稿和 AI 主题生成的共同依据。可运行模板见 [`theme-example/package`](theme-example/package)，完整插槽见 [`theme-slots.md`](theme-slots.md)，Banner 与生图提示词见 [`theme-banner-assets.md`](theme-banner-assets.md)，AI 固定流程见 [`theme-ai-workflow.md`](theme-ai-workflow.md)，Manifest 机器协议见 [`theme.schema.json`](theme.schema.json)。

## 1. 设计边界

ReTheme 主题是声明式资源包，只包含 JSON、CSS 和静态图片。引擎负责识别当前 ChatGPT 版本、将原生节点映射成稳定插槽、挂载受控图片、同步明暗与语言状态，并在停止、到期或异常时恢复现场。

主题负责视觉，不负责 ChatGPT 的结构和行为：

- 可以修改颜色、边框、圆角、阴影、透明度、背景、图标色、文字层级和装饰动画。
- 可以通过 Manifest 提供首页 Banner、会话窄 Banner、受控装饰图和双语文案。
- 不得执行 JavaScript，不得修改事件、ARIA 语义、业务状态或原生 DOM 顺序。
- 不得依赖 ChatGPT 压缩类名、文本内容、DOM 深度或版本专用属性。
- 不得覆盖字体。主题始终继承 ChatGPT 默认字体。
- 不得用绝对定位搬动首页、会话、输入框、设置页等结构区域。布局由引擎统一管理。

## 2. 运行模型

主题根节点始终带有以下状态：

```css
:root[data-ct-theme="studio.example.my-theme"]
:root[data-ct-color-scheme="light"]
:root[data-ct-color-scheme="dark"]
:root[data-ct-view="home"]
:root[data-ct-view="home-compact"]
:root[data-ct-view="conversation"]
:root[data-ct-view="other"]
```

视图含义：

| 状态 | 含义 | 主题应验证的内容 |
|:--|:--|:--|
| `home` | 空白首页 | 完整 Hero、提示、快捷卡片和输入框 |
| `home-compact` | 首页已输入内容或打开工作区面板 | 完整 Hero 与首页卡片由引擎隐藏；可出现会话窄 Banner |
| `conversation` | 已开始或历史会话 | 窄 Banner 位于消息滚动区上方，输入框保持原生定位 |
| `other` | 设置等其他页面 | 页面、设置、菜单插槽按实际 DOM 条件出现 |

这些状态由引擎判断。主题不能自行设置 `data-ct-*` 属性。

## 3. 目录结构

```text
my-theme/
├── manifest.json
├── styles/
│   ├── tokens.css
│   └── overrides.css
└── assets/
    ├── hero.webp
    ├── hero-foreground.webp
    └── conversation.webp
```

规则：

- 本地开发时选择包含 `manifest.json` 的目录。
- 社区投稿上传 ZIP，ZIP 根目录必须直接包含 `manifest.json`，不能多包一层目录。
- CSS 只能位于 `styles/`，图片只能位于 `assets/`。
- 路径必须使用 `/`、UTF-8 和规范相对路径；禁止绝对路径、`..`、反斜杠和盘符。
- 开发目录不能是 `.ctheme`。正式 `.ctheme` 只能由平台生成并由客户端下载。

## 4. 最短开发流程

1. 复制 [`theme-example/package`](theme-example/package)，并阅读旁边的注释版 Manifest。
2. 修改 `id`、`name`、`version`、作者和文案。
3. 先在 `tokens.css` 定义浅色与深色 token，再写插槽规则。
4. 在 ReTheme 的主题库中选择“加载本地主题”。
5. 验证中文/英文 × 浅色/深色，以及首页、紧凑首页、会话、设置和菜单。
6. 修复所有横向滚动、遮挡、闪烁、低对比和交互区域被覆盖问题。
7. 升级 SemVer，压缩源码目录为 ZIP，提交社区。

本地静态校验无需下载 ReTheme 源码或安装 Rust：

```bash
pnpm dlx @duxweb/retheme-theme-skill validate /absolute/path/to/my-theme
```

校验 ZIP：

```bash
pnpm dlx @duxweb/retheme-theme-skill validate /absolute/path/to/my-theme.zip
```

npm 包会自动调用当前系统对应的编译版 Rust 校验器，并始终在 stdout 输出 JSON：成功退出码为 `0`，协议失败为 `1`，命令用法错误为 `2`。桌面端、服务端和开发工具共享同一协议实现，不使用 WASM，也不在 JavaScript 或 PHP 中维护第二套校验规则。

## 5. Manifest 完整字段

Manifest 使用严格字段校验。任何未知字段、拼错字段、重复 `styles` 或重复 `slots` 都会直接失败，防止“加载成功但效果消失”。JSON 本身不支持注释；开发时参考 [`manifest.annotated.jsonc`](theme-example/manifest.annotated.jsonc)，实际文件必须是合法 JSON。

### 5.1 顶层字段

| 字段 | 必填 | 约束 | 说明 |
|:--|:--:|:--|:--|
| `schemaVersion` | 是 | 固定 `1` | 协议版本，不是主题版本 |
| `id` | 是 | 最长 128；小写字母、数字、`.`、`-`；至少一个 `.` | 全局唯一，发布后不可修改 |
| `name` | 是 | 1–120 字符 | 默认语言名称 |
| `description` | 是 | 1–240 字符 | 默认语言说明 |
| `version` | 是 | SemVer | 同一版本发布后不可覆盖内容 |
| `author` | 是 | 见下表 | 作者身份 |
| `testedCodexVersions` | 否 | 字符串数组 | 保留协议名；记录实际测试的 ChatGPT 版本，不锁定运行版本 |
| `styles` | 是 | 至少 1 项；不可重复 | 按数组顺序合并 CSS |
| `slots` | 是 | 白名单；不可重复 | 主题声明的能力清单，详见 5.2 |
| `permissions` | 是 | 必须为 `[]` | v1 不开放权限 |
| `preview` | 是 | 三个 `#RRGGBB` | 商店和客户端预览色 |
| `experience` | 是 | 见第 6 节 | 引擎创建的内容和受控资源 |
| `locales` | 否 | locale → 翻译对象 | 当前运行时选择 `zh-CN` 或 `en` |
| `access` | 平台 | 平台生成 | 源码 ZIP 不要填写 |
| `integrity` | 平台 | 固定 `integrity.json` | 源码 ZIP 不要填写 |
| `signature` | 平台 | 固定 `signature.ed25519` | 源码 ZIP 不要填写 |

`supportedCodexVersions` 仅为旧别名，新主题不要使用。目标应用统一称为 ChatGPT；字段名 `testedCodexVersions` 为 v1 协议兼容而保留。

`author`：

| 字段 | 约束 |
|:--|:--|
| `id` | 1–128 字节；仅小写字母、数字、`.`、`-` |
| `name` | 1–100 字符 |

### 5.2 `slots` 的真实语义

`slots` 是主题使用能力与受控资源的声明，不是运行时开关。引擎仍会给当前页面所有可识别节点标记稳定插槽，因此 CSS 能命中一个未声明的普通插槽；但新主题必须把实际使用的插槽完整列入 `slots`，原因是：

- `experience.assets` 引用的资源插槽必须先在 `slots` 声明。
- 审核工具和未来协议迁移依赖这份能力清单。
- AI 和维护者可由清单快速判断覆盖范围。

当前为兼容早期主题，不会强制扫描 CSS 并拒绝所有漏声明项。不要利用这一兼容行为。

### 5.3 `preview`

```json
"preview": {
  "background": "#EEF4FF",
  "surface": "#FFFFFF",
  "accent": "#526EE8"
}
```

三个颜色用于主题列表预览，不会自动生成 CSS。选择能代表浅色主题的背景、表面和强调色。

## 6. `experience` 与受控资源

### 6.1 首页 Hero

`homeHero` 必填，由引擎创建稳定 DOM 并放入首页正常文档流：

| 字段 | 约束 |
|:--|:--|
| `eyebrow` | 1–80 字符 |
| `title` | 1–120 字符 |
| `description` | 1–240 字符 |
| `asset` | 包内图片路径 |
| `fit` | `cover` 或 `contain` |
| `position` | `center`、`top`、`bottom`、`left`、`right` |
| `foreground` | 可选透明前景图 |
| `divider` | 可选 `{ label, asset? }`；label 1–80 字符 |

Hero 是 `home.stage` 的第一个流式子项。主题可以设置视觉高度或最小高度，但不要对 `home.hero` 设置 `position: fixed/absolute`、负布局偏移或把首页卡片移动到 Hero 内。

### 6.2 首页提示

`homePrompt` 可选：

```json
"homePrompt": { "title": "今天想创造什么？" }
```

提供后，引擎隐藏原生提示并挂载本地化标题；恢复主题时原生内容会复原。`title` 为 1–120 字符。

### 6.3 会话窄 Banner

`conversationBanner` 可选，字段与 Hero 相同但没有 `divider`。在真实会话中，引擎会创建：

```text
conversation.stage
├── conversation.header
│   └── conversation.header.content
│       └── conversation.banner
└── conversation.viewport
    └── 原生消息滚动区
```

窄 Banner 不在消息滚动容器内，不使用绝对定位，也不会遮挡历史消息。宽度由引擎跟随消息/输入框内容列。主题只定义高度、背景、文字和前景图视觉。

### 6.4 专用装饰字段

| Manifest 字段 | 创建的挂载 | 父节点 | 出现条件 |
|:--|:--|:--|:--|
| `composerSubmit` | `composer.submit.decoration` | 发送按钮 | 找到发送按钮且配置资源 |
| `composerDecoration` | `composer.decoration` | 输入框表面 | 找到输入框且配置资源 |
| `conversationSummaryDecoration` | `conversation.summary.decoration` | 右侧任务/摘要卡片 | 摘要卡片出现且配置资源 |
| `sidebarSectionDecoration` | `sidebar.section.decoration` | 项目分组区域 | 侧栏项目分组出现且配置资源 |

格式均为 `{ "asset": "assets/file.svg" }`。挂载默认 `pointer-events: none`，不得覆盖按钮命中区域。

### 6.5 通用图片插槽 `experience.assets`

每项基础格式为 `{ "slot", "asset" }`。需要按 ChatGPT 明暗模式切换图片时，可增加可选字段 `lightAsset` 和 `darkAsset`；引擎会原位切换资源，缺少对应变体时回退到 `asset`：

```json
{
  "slot": "app.background",
  "asset": "assets/background.webp",
  "lightAsset": "assets/background-light.webp",
  "darkAsset": "assets/background-dark.webp"
}
```

当前只允许：

`app.background`、`main.background`、`main.overlay`、`main.frame`、`sidebar.brand.icon`、`sidebar.brand.badge`、`sidebar.header.background`、`sidebar.header.decoration`、`sidebar.frame`、`home.card.background`、`home.card.arrow.asset`。

资源插槽必须同时列在顶层 `slots`。同一资源插槽只能声明一次。三种资源路径均须指向包内图片；复杂背景优先使用压缩后的 WebP。

### 6.6 角落装饰 `experience.decorations`

可省略，默认 `[]`。当前允许 `decoration.top-right` 和 `decoration.bottom-right`，父节点为主内容区。只适合不参与交互的小型氛围装饰；必须控制尺寸并保证 `pointer-events: none`。

## 7. 图片与本地静态服务

| 项目 | 限制 |
|:--|:--|
| 格式 | SVG、PNG、JPEG、WebP |
| 单图 | 最大 8 MiB |
| 非图片单文件 | 最大 1 MiB |
| ZIP | 最大 30 MiB |
| 解压总量 | 最大 60 MiB |
| 文件数 | 最大 256 |

图片由 ReTheme 为当前主题会话启动的 `127.0.0.1` 临时静态服务提供，URL 带会话令牌并在恢复主题时撤销。主题 CSS 禁止 `url()`，图片必须通过 Manifest 的受控字段声明；不需要 Base64。

SVG 会拒绝脚本、`foreignObject`、事件属性、外部 URL 和 JavaScript URL。复杂插画优先 WebP；需要无损缩放或简单图形时使用静态 SVG。

首页背景、会话窄 Banner、透明人物/物品前景的推荐尺寸、安全区、导出验收和中英文生图提示词见 [`theme-banner-assets.md`](theme-banner-assets.md)。

## 8. CSS 协议

### 8.1 强制根作用域

每一个顶层选择器都必须从精确主题根开始：

```css
:root[data-ct-theme="studio.example.my-theme"] [data-ct-slot="sidebar"] {
  background: var(--rt-sidebar-background) !important;
}
```

选择器列表中的每项都要带根：

```css
:root[data-ct-theme="studio.example.my-theme"] [data-ct-slot="home.card"],
:root[data-ct-theme="studio.example.my-theme"] [data-ct-slot="settings.card"] {
  border-color: var(--rt-border) !important;
}
```

`:is()`、`:where()` 等函数内部可使用逗号，引擎按 CSS AST 校验，不会错误拆分。但函数内部仍只能使用开放属性和平台类名。

### 8.2 允许的选择器材料

- 属性：`data-ct-theme`、`data-ct-color-scheme`、`data-ct-view`、`data-ct-slot`、`data-ct-mount`、`role="button"`、`role="switch"`。
- 平台类：仅 `ct-home-hero__copy`、`ct-home-hero__eyebrow`、`ct-home-hero__title`、`ct-home-hero__description`、`ct-home-hero__image`、`ct-decoration`。
- 伪类/伪元素：可以使用标准 CSS 伪类和 `::before`/`::after`，但生成内容不能替代业务文本。
- 禁止 ID 选择器、ChatGPT 内部类名、`data-app-*` 和未开放属性。

### 8.3 允许的规则

- 普通样式规则。
- `@media`。
- `@keyframes`，动画名必须以 `ct-` 开头。

禁止 CSS 嵌套、`@import`、外部 URL、`url()`、`expression()`、`javascript:` 和注入 `</style`。

### 8.4 字体与布局

禁止 `font`、`font-family`、包含 `font` 的自定义属性和 `--ct-font*`。可调整局部 `font-size`、`font-weight`、`line-height` 和 `letter-spacing`，但正文可读性优先。

以下结构属性通常由引擎拥有，不要覆盖：

- `home.layout`、`home.stage`、`home.content.region`、`composer.region` 的 flex 结构。
- `conversation.stage`、`conversation.header`、`conversation.viewport` 的定位与滚动。
- `settings`、`page` 的原生滚动容器。
- 输入框原生高度切换和按钮交互布局。

如果需求只能通过改变这些结构才能实现，应先扩展引擎协议，而不是在主题中写补丁。

## 9. Token 组织

先定义语义 token，再映射到插槽。不要在几十条规则中复制颜色：

```css
:root[data-ct-theme="studio.example.my-theme"] {
  --rt-app-background: #eef4ff;
  --rt-surface: rgba(255, 255, 255, 0.84);
  --rt-text: #18213d;
  --rt-text-muted: #5e6885;
  --rt-accent: #526ee8;
  --rt-accent-soft: rgba(82, 110, 232, 0.14);
  --rt-border: rgba(82, 110, 232, 0.22);
  --rt-radius-card: 14px;
  --rt-shadow-card: 0 12px 30px rgba(30, 50, 110, 0.1);
}

:root[data-ct-theme="studio.example.my-theme"][data-ct-color-scheme="dark"] {
  --rt-app-background: #101521;
  --rt-surface: rgba(28, 36, 55, 0.86);
  --rt-text: #edf2ff;
}
```

主题 token 使用自己的前缀，例如 `--rt-*`。不要重新定义引擎 `--ct-*`，除非使用下面明确开放的布局变量。

### 9.1 开放的引擎布局变量

| 变量 | 默认值 | 用途 |
|:--|:--|:--|
| `--ct-home-hero-max-width` | `1080px` | 首页 Hero 最大宽度 |
| `--ct-home-hero-gap` | `24px` | Hero 与下方内容间距 |
| `--ct-home-hero-top` | `0px` | 首页安全顶距附加量 |
| `--ct-home-hero-viewport-overflow` | `hidden` | 允许人物露头时可设 `visible` |
| `--ct-home-hero-media-overflow` | `hidden` | Hero 背景媒体裁剪 |
| `--ct-banner-foreground-width/right/bottom/z-index` | 见引擎默认 | 首页与会话前景图公共值 |
| `--ct-home-hero-foreground-*` | 回退公共值 | 首页前景图单独覆盖 |
| `--ct-conversation-banner-foreground-*` | 回退公共值 | 会话前景图单独覆盖 |
| `--ct-workspace-panel-z-index` | `35` | 工作区面板与装饰层关系 |

`--ct-titlebar-safe-top`、`--ct-conversation-content-*`、`--ct-conversation-header-safe-top`、`--ct-conversation-summary-width` 和 `--ct-home-card-count` 是引擎动态输出，只读，不要赋值。

## 10. 明暗主题

必须同时定义浅色和深色。默认 token 建议作为浅色，再用 `[data-ct-color-scheme="dark"]` 覆盖；也可反向，但必须明确。

检查：

- 正文与背景对比度至少 4.5:1，大字号至少 3:1。
- 边框、网格和背景动画在两种模式都可见但不抢正文。
- 选中、Hover、Focus、禁用状态在两种模式都可辨认。
- 半透明层在浅色不能变成灰脏遮罩，在深色不能丢失边界。
- 不要通过 `prefers-color-scheme` 推断 ChatGPT 模式；使用 `data-ct-color-scheme`。

## 11. 双语

默认字段提供一种完整语言，再在 `locales` 覆盖另一种。当前 ReTheme 将界面语言归一为 `zh-CN` 或 `en`；其他合法 locale 可以预留，但当前客户端不会主动选择。

可本地化字段：

- 主题 `name`、`description`。
- `homeHero` 的 `eyebrow`、`title`、`description`、`divider.label`。
- `homePrompt.title`。
- `conversationBanner` 的 `eyebrow`、`title`、`description`。

资源和布局不随语言重复声明。中文和英文都要检查溢出、换行和窄窗口。

## 12. 动画

动画优先只改变 `transform` 和 `opacity`，避免频繁触发布局和大面积滤镜重绘。所有动画名使用 `ct-` 前缀，并提供减少动态效果：

```css
@media (prefers-reduced-motion: reduce) {
  :root[data-ct-theme="studio.example.my-theme"] *,
  :root[data-ct-theme="studio.example.my-theme"] *::before,
  :root[data-ct-theme="studio.example.my-theme"] *::after {
    animation: none !important;
    transition-duration: 0.01ms !important;
  }
}
```

禁止通过动画改变元素布局位置、反复重建阴影或造成 Hover 状态在两个命中区域之间闪烁。装饰层必须 `pointer-events: none`。

## 13. 插槽使用原则

完整 164 项见 [`theme-slots.md`](theme-slots.md)。使用时遵循：

1. 先选最具体插槽，例如改图标用 `sidebar.item.icon`，不要给整个 `sidebar.item` 强制所有 SVG。
2. 状态插槽优先于伪类，例如 `sidebar.item.active`、`menu.item.checked`、`settings.control.checked`。
3. 条件插槽不存在是正常状态，不得假设每个页面都有任务卡片、终端或菜单。
4. 引擎挂载节点可安全装饰，但不要赋予交互或业务文字。
5. 原生节点只改视觉，不隐藏关键按钮、表单、消息、Focus Ring 或可访问文本。

## 14. 本地开发、试用与恢复

- 本地目录无需签名、解密或打包。
- 普通账号本地开发预览每次 10 分钟；Pro 可不限时。到期由运行时页面租约恢复主题。
- 退出账号、主动恢复主题、结束预览或关闭 ReTheme 会停止当前管理会话；页面租约是管理器异常退出时的安全恢复保障。
- 本地预览不会安装进在线主题库，不会生成 `.ctheme`。
- 修改文件后重新应用主题；不要依赖热更新保留旧 DOM 状态。

## 15. 验收矩阵

每个主题发布前至少完成：

| 维度 | 必测项 |
|:--|:--|
| 语言 | 中文、英文 |
| 模式 | 浅色、深色 |
| 平台 | macOS、Windows |
| 宽度 | 窄、中、宽、最大化；打开右侧工作区面板 |
| 首页 | 空白、输入草稿、展开快捷卡片、删除草稿 |
| 会话 | 新会话、历史会话、长消息滚动、右侧任务/摘要卡片 |
| 输入框 | 空、单行、多行、图片/附件、权限按钮、展开面板 |
| 侧栏 | 普通、选中、项目分组、折叠、Footer、拖动宽度 |
| 设置 | 各导航项、卡片、行、输入控件、Switch 开/关 |
| 菜单 | 普通、Hover/高亮、选中、快捷键、分割线 |
| 内容 | 用户/助手、代码、行内代码、Diff、终端 |
| 动态 | `prefers-reduced-motion`、Hover 不闪烁、滚动不回跳 |
| 恢复 | 切换主题、恢复主题、预览到期后无残留 |

发布阻断项：横向滚动、Banner 遮挡消息、输入框位置改变、按钮不能点击、文字低对比、设置页闪白持续存在、明暗或双语缺失、恢复后残留挂载/样式。

## 16. 社区发布

1. 删除未引用资源和系统隐藏文件。
2. 确认 ZIP 根直接包含 Manifest。
3. 更新 SemVer；已发布同版本不可替换内容。
4. 上传源码 ZIP，不上传 `.ctheme`。
5. 不要填写 `access`、`integrity`、`signature`。
6. 平台校验、审核、生成封面、规范化 Manifest、计算完整性索引并签名。
7. 客户端通过在线接口下载平台生成的 `.ctheme`，校验 Ed25519 签名后写入设备加密缓存。

签名用于完整性和发布来源验证；设备缓存加密用于提高直接提取成本。平台私钥、远程适配规则和 ChatGPT 版本选择器不属于主题源码。

## 17. 常见失败

| 报错/现象 | 根因 | 修复 |
|:--|:--|:--|
| Manifest unknown field | 字段拼写错误或使用未开放字段 | 对照 Schema；不要猜字段名 |
| 选择器未限定自身作用域 | 选择器列表某一项缺根 | 每一项都加精确主题根 |
| 非平台类名/属性 | 使用 ChatGPT 内部实现 | 改用稳定插槽 |
| CSS 包含禁止内容 | `url()`、外链、导入或脚本式表达式 | 资源移入 `assets/` 并在 Manifest 声明 |
| 资源插槽未在 slots 声明 | `experience.assets` 与能力清单不一致 | 将对应 slot 加入顶层 `slots` |
| 图片内容与扩展名不匹配 | 仅改后缀或文件损坏 | 重新正确导出 |
| Banner 被裁切 | 前景露头但 viewport 仍 hidden | 设置开放的 overflow token，不改结构定位 |
| 首页输入后错位 | 主题覆盖输入框高度/结构布局 | 删除布局规则，保留原生高度 |
| Hover 闪烁 | 伪元素或装饰层抢鼠标 | 添加 `pointer-events: none`，保持命中区稳定 |

## 18. 版本适配

主题只依赖稳定插槽。ChatGPT DOM 的版本差异由 ReTheme 的固定适配引擎与签名远程兼容数据处理；客户端根据 ChatGPT 和引擎版本选择适配记录。`testedCodexVersions` 只是回归记录，不是版本锁。

如果更新后某个插槽消失：

1. 先用其他主题确认是否为引擎映射问题。
2. 不要把新 ChatGPT 内部选择器写入主题。
3. 记录 ChatGPT 版本、平台、页面、缺失插槽和复现步骤。
4. 更新兼容数据或引擎，再用原主题验证。

## 19. 给 AI 的入口

不要让 AI 临时拼接本规范中的规则。安装完整 Skill 后，它会按需读取协议、插槽、QA、Banner 生图提示词和模板，并调用内置校验器：

```bash
pnpm dlx @duxweb/retheme-theme-skill install
```

重启 Codex 后，直接要求使用 `retheme-theme-development` Skill 创建、优化或检查主题即可，不需要下载 ReTheme 仓库。
