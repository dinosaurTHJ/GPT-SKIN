# Banner and Asset Generation

## Specifications

| Asset | Recommended | Minimum | Safe composition |
|:--|:--|:--|:--|
| Home Hero background | 1920×640, about 3:1 | 1440×480 | Keep critical content in central 60%; reserve calm copy side |
| Compact conversation background | 1920×300, about 6.4:1 | 1440×240 | Keep central 70% calm; reserve 12% top/bottom crop safety |
| Transparent character/object | 1200×800 or natural ratio | shortest side 600 | Align base to bottom; keep 4% transparent edge padding |
| Small ornament/badge | 2× expected display size | shortest side 128 | Keep 6% transparent edge padding |

These are source dimensions, not CSS height. The engine owns flow and responsive placement; the theme owns visual height/min-height.

Use WebP for complex backgrounds, transparent WebP/PNG for foregrounds, and safe SVG for simple static vectors. Keep each image under 8 MiB.

Do not bake required copy, ChatGPT controls, fake cards, menus, buttons, input fields, logos, watermarks, or unreadable pseudo-text into images. Manifest provides localized copy.

## Home Hero prompt — Chinese

```text
为 ReTheme 的 ChatGPT 桌面主题生成首页横幅背景。
主题概念：{主题概念}；视觉风格：{风格}；主色：{主色}；辅助色：{辅助色}。
横向 3:1 构图，1920×640。主体为 {主体}，位于 {主体位置}，主体完整、轮廓清晰。
中间 60% 为关键安全区，在 {文案位置} 留出安静、低纹理、低对比的文案区域。
背景有明确前中后景、克制的 {光效}、适合桌面软件的精致质感，边缘允许 cover 裁切。
只生成背景插画：无文字、无字母、无数字、无水印、无标志、无 UI、无输入框、无按钮、无菜单、无边框。
```

## Home Hero prompt — English

```text
Create a home hero background for a ReTheme theme for the ChatGPT desktop app.
Concept: {theme concept}. Art direction: {style}. Primary color: {primary color}. Supporting color: {supporting color}.
Use a wide 3:1 composition at 1920×640. Feature {subject} at {subject position}, fully readable with a clean silhouette.
Keep the central 60% as the critical safe area and reserve a calm, low-detail, low-contrast copy area at {copy position}.
Build clear foreground, midground, and background depth with restrained {lighting effect}; allow safe edge cropping for cover scaling.
Background artwork only: no text, letters, numbers, watermark, logo, UI, composer, buttons, menus, cards, or frames.
```

## Compact Banner prompt — Chinese

```text
为 ReTheme 的 ChatGPT 会话页生成窄横幅背景。
主题概念：{主题概念}；视觉风格：{风格}；主色：{主色}。
超宽 6.4:1 构图，1920×300。主体/环境元素 {主体} 位于 {主体位置}，中央 70% 保持安静、低纹理并可承载短标题。
上下各保留 12% 裁切安全区，背景层次清晰但对比克制，适合长期阅读场景。
无文字、无字母、无数字、无水印、无标志、无 UI、无消息气泡、无输入框、无按钮、无边框。
```

## Compact Banner prompt — English

```text
Create a compact conversation banner background for a ReTheme theme for the ChatGPT desktop app.
Concept: {theme concept}. Art direction: {style}. Primary color: {primary color}.
Use an ultra-wide 6.4:1 composition at 1920×300. Place {subject or environment} at {subject position}; keep the central 70% calm and low-detail for a short title.
Reserve 12% crop-safe space at both the top and bottom. Keep depth polished but contrast restrained for a long-form reading surface.
No text, letters, numbers, watermark, logo, UI, message bubbles, composer, buttons, or frames.
```

## Transparent foreground prompt — Chinese

```text
生成 ReTheme 横幅使用的透明前景素材。
主体：{主体}；视觉风格：{风格}；主色：{主色}；姿态/角度：{姿态}。
画布 1200×800，主体完整，轮廓清晰，底部与画布底边对齐，四周至少 4% 透明安全边。
光影方向为 {光影方向}，细节精致但适合缩小显示，边缘干净无杂色。
透明背景，只有单个主体；无文字、无水印、无标志、无 UI、无底座、无背景场景、无投影底板。
```

## Transparent foreground prompt — English

```text
Create a transparent foreground asset for a ReTheme banner.
Subject: {subject}. Art direction: {style}. Primary color: {primary color}. Pose/angle: {pose}.
Use a 1200×800 canvas. Keep the full subject visible with a clean silhouette, align its base to the bottom edge, and preserve at least 4% transparent padding around all other edges.
Light from {lighting direction}; retain polished detail that remains readable at smaller sizes and produce clean, color-fringe-free edges.
Transparent background and one isolated subject only: no text, watermark, logo, UI, pedestal, environment, backdrop, or shadow plate.
```

## Appearance strategy

Manifest v1 does not switch an asset URL by appearance. Prefer one neutral-luminance image and adjust opacity/filter/controlled overlay CSS for light and dark. If composition must differ completely, request a protocol extension instead of using CSS `url()`.

## Asset QA

- Verify real file format, dimensions, transparent channel, clean edges, and compression.
- Test copy readability in Chinese/English and light/dark.
- Test cover cropping at narrow, medium, wide, maximized, and workspace-panel widths.
- Ensure the compact Banner remains quiet above long conversations.
- Ensure foreground heads/weapons remain inside source bounds while theme overflow tokens allow intentional reveal.
