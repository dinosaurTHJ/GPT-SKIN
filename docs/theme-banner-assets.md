# ReTheme Banner 与视觉资源规范

本文定义首页 Hero、会话窄 Banner 和透明前景资源的生产规格。主题协议与字段约束见 [`theme-development.md`](theme-development.md)，可运行示例见 [`theme-example/package`](theme-example/package)。

## 1. 通用原则

- 图片只承担氛围和装饰，不承担必需业务文案。名称、标题和说明必须写入 Manifest，才能随 `zh-CN` / `en` 切换。
- 背景图要为文字、输入框和卡片保留低噪声安全区；不要把关键主体放满整个画面。
- 不在图片中绘制 ChatGPT 控件、输入框、按钮、菜单、边框或假文字。
- 同一资源需要同时适应窗口裁切和 `cover` 缩放；关键内容不能贴近四边。
- 图片不能使用外部 URL。导出到 `assets/`，再通过 Manifest 的受控资源字段引用。
- 复杂插画优先 WebP；需要透明通道的前景使用透明 WebP 或 PNG；简单几何图形可用安全 SVG。
- 每张图片最大 8 MiB。提交前删除元数据并按肉眼无明显损失的质量压缩。

## 2. 规格总表

| 类型 | 推荐画布 | 最低画布 | 构图安全区 | 导出建议 |
|:--|:--|:--|:--|:--|
| 首页 Hero 背景 | `1920 × 640`，约 `3:1` | `1440 × 480` | 关键内容放中间 60%，文字侧保持低噪声 | WebP；无透明通道 |
| 会话窄 Banner 背景 | `1920 × 300`，约 `6.4:1` | `1440 × 240` | 关键内容放中间 70%，上下各留 12% | WebP；无透明通道 |
| 透明人物/物品前景 | `1200 × 800` 或按主体比例 | 最短边 600 | 主体四周留 4% 透明边；底部对齐 | 透明 WebP/PNG |
| 小型装饰/徽标 | 按显示尺寸的 2 倍导出 | 最短边 128 | 四周留 6% 透明边 | SVG、透明 WebP/PNG |

推荐尺寸是源素材尺寸，不是主题 CSS 高度。引擎将 Banner 放在稳定文档流中，主题通过插槽 CSS 定义视觉高度或 `min-height`。

## 3. 首页 Hero 背景

对应 `experience.homeHero.asset`，挂载到 `home.hero.media.asset`。

- 推荐将视觉主体放在右侧 35% 或左侧 35%，另一侧留给 Manifest 文案。
- 若画面中央有主体，主体周围必须有足够的低对比留白，避免与标题和说明竞争。
- `fit: "cover"` 适合铺满背景；`fit: "contain"` 只适合资源自身留有完整背景且不能裁切的情况。
- 主题可以给 `home.hero` 设置视觉高度或最小高度，但不得用绝对定位把 Hero 浮在卡片上方。
- 不把首页快捷卡片、输入框或分割线烘焙进图片。

### 中文提示词模板

```text
为 ReTheme 的 ChatGPT 桌面主题生成首页横幅背景。
主题概念：{主题概念}；视觉风格：{风格}；主色：{主色}；辅助色：{辅助色}。
横向 3:1 构图，1920×640。主体为 {主体}，位于 {主体位置}，主体完整、轮廓清晰。
中间 60% 为关键安全区，在 {文案位置} 留出安静、低纹理、低对比的文案区域。
背景有明确前中后景、克制的 {光效}、适合桌面软件的精致质感，边缘允许 cover 裁切。
只生成背景插画：无文字、无字母、无数字、无水印、无标志、无 UI、无输入框、无按钮、无菜单、无边框。
```

### English prompt template

```text
Create a home hero background for a ReTheme theme for the ChatGPT desktop app.
Concept: {theme concept}. Art direction: {style}. Primary color: {primary color}. Supporting color: {supporting color}.
Use a wide 3:1 composition at 1920×640. Feature {subject} at {subject position}, fully readable with a clean silhouette.
Keep the central 60% as the critical safe area and reserve a calm, low-detail, low-contrast copy area at {copy position}.
Build clear foreground, midground, and background depth with restrained {lighting effect}; allow safe edge cropping for cover scaling.
Background artwork only: no text, letters, numbers, watermark, logo, UI, composer, buttons, menus, cards, or frames.
```

## 4. 会话窄 Banner 背景

对应 `experience.conversationBanner.asset`，挂载到 `conversation.banner.media.asset`。

- 画面要比首页 Hero 更安静，不能抢夺消息阅读焦点。
- 主体适合放在左右边缘，中央保留标题安全区。
- 顶部和底部至少各留 12% 安全区，避免窄高度裁切主体头部或底部装饰。
- 会话 Banner 位于消息滚动区上方，由引擎对齐消息内容列；主题不要使用 `position: fixed/absolute` 搬动它。
- 若使用透明人物露头，背景图只保留环境，人物另做 `foreground`。

### 中文提示词模板

```text
为 ReTheme 的 ChatGPT 会话页生成窄横幅背景。
主题概念：{主题概念}；视觉风格：{风格}；主色：{主色}。
超宽 6.4:1 构图，1920×300。主体/环境元素 {主体} 位于 {主体位置}，中央 70% 保持安静、低纹理并可承载短标题。
上下各保留 12% 裁切安全区，背景层次清晰但对比克制，适合长期阅读场景。
无文字、无字母、无数字、无水印、无标志、无 UI、无消息气泡、无输入框、无按钮、无边框。
```

### English prompt template

```text
Create a compact conversation banner background for a ReTheme theme for the ChatGPT desktop app.
Concept: {theme concept}. Art direction: {style}. Primary color: {primary color}.
Use an ultra-wide 6.4:1 composition at 1920×300. Place {subject or environment} at {subject position}; keep the central 70% calm and low-detail for a short title.
Reserve 12% crop-safe space at both the top and bottom. Keep depth polished but contrast restrained for a long-form reading surface.
No text, letters, numbers, watermark, logo, UI, message bubbles, composer, buttons, or frames.
```

## 5. 透明人物或物品前景

对应 `homeHero.foreground` 或 `conversationBanner.foreground`，挂载到对应的 `*.foreground.asset`。

- 必须是真透明背景，不要白底、黑底、棋盘格或残余色边。
- 主体底部建议落在画布底边，便于使用 `bottom` token 对齐 Banner 底部。
- 人物头顶、武器和飘带至少留 4% 透明边，避免源图自身裁切。
- 不在前景图中加入大面积发光背景；光斑和环境效果优先放背景图或 CSS 伪元素。
- 同一张前景可由首页和会话分别设置宽度、右偏移、底偏移；不要为了两个场景复制仅尺寸不同的图片。

### 中文提示词模板

```text
生成 ReTheme 横幅使用的透明前景素材。
主体：{主体}；视觉风格：{风格}；主色：{主色}；姿态/角度：{姿态}。
画布 1200×800，主体完整，轮廓清晰，底部与画布底边对齐，四周至少 4% 透明安全边。
光影方向为 {光影方向}，细节精致但适合缩小显示，边缘干净无杂色。
透明背景，只有单个主体；无文字、无水印、无标志、无 UI、无底座、无背景场景、无投影底板。
```

### English prompt template

```text
Create a transparent foreground asset for a ReTheme banner.
Subject: {subject}. Art direction: {style}. Primary color: {primary color}. Pose/angle: {pose}.
Use a 1200×800 canvas. Keep the full subject visible with a clean silhouette, align its base to the bottom edge, and preserve at least 4% transparent padding around all other edges.
Light from {lighting direction}; retain polished detail that remains readable at smaller sizes and produce clean, color-fringe-free edges.
Transparent background and one isolated subject only: no text, watermark, logo, UI, pedestal, environment, backdrop, or shadow plate.
```

## 6. 明暗模式策略

Manifest v1 的单个资源字段不会根据明暗模式切换 URL，因此不要承诺运行时自动换图。

优先做法：使用一张中性明度图片，通过浅色/深色 CSS 调整其 `opacity`、`filter`、遮罩层和周围表面色。生成时避免纯白高光和大片纯黑。

若两种模式必须使用完全不同的构图，应先请求协议扩展；不要在 CSS 中使用 `url()` 绕过 Manifest。

## 7. 导出与验收

1. 检查尺寸、透明通道和文件格式，不要只改扩展名。
2. 压缩后在 100% 和实际缩放尺寸检查色带、锯齿、透明毛边与文字安全区。
3. 在浅色、深色、中文、英文下检查 Manifest 文案可读性。
4. 检查窄、中、宽、最大化窗口，以及打开右侧工作区面板后的裁切。
5. 检查会话页长消息滚动，Banner 不得遮挡或跟随消息滚动。
6. 开启 `prefers-reduced-motion`，资源仍应保持完整可读。

验收失败时先判断是资源构图、主题 CSS 还是引擎挂载问题，不用绝对定位补偿错误素材。
