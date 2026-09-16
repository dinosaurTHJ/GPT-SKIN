# ReTheme AI 主题开发工作流

本文是 AI/Skill 的确定性执行流程。协议细节以 [`theme-development.md`](theme-development.md)、[`theme-slots.md`](theme-slots.md)、[`theme-banner-assets.md`](theme-banner-assets.md) 和 [`theme.schema.json`](theme.schema.json) 为准。

## 1. 开始前必须获得的输入

- 主题名称、唯一 `id`、版本、作者 ID 与作者名。
- 视觉方向：关键词、参考图、目标气质、主色和禁用风格。
- 默认语言，以及中文和英文文案。
- 要覆盖的页面：至少首页、会话、输入框、侧栏、设置、菜单。
- 可使用的本地图片素材和每张图的语义。
- 是否需要 Hero 前景、窄 Banner、卡片背景、侧栏品牌、输入框或任务卡片装饰。

信息不足时只询问会改变视觉或协议结果的问题。不得猜作者身份、主题 ID 或素材版权。

## 2. 读取顺序

1. 完整读取主题开发规范。
2. 从插槽表筛选本主题需要的区域。
3. 生成或编辑 Hero、窄 Banner、透明前景前，读取 Banner 与图片规范。
4. 读取 Schema 中相关对象，不凭记忆写字段。
5. 复制 `theme-example/package`；不要从现有复杂主题复制布局补丁。
6. 查看用户提供的参考图与素材尺寸。

## 3. 产出顺序

### 阶段 A：视觉 token

先给出语义 token 表，至少包含：

- app/main/sidebar/page 背景。
- surface、surface-hover、surface-active。
- 主/次文字、图标、禁用色。
- accent、accent-soft、focus、selection。
- border、border-strong、separator。
- card/composer/menu/settings 的圆角和阴影。
- light/dark 两套值。

禁止设置字体族。确认 token 后再写 CSS。

### 阶段 B：资源计划

为每张图片写清：文件名、格式、用途、Manifest 字段/资源插槽、透明背景需求和最大显示尺寸。只有 Manifest 受控资源可进入运行时；CSS 不得使用 `url()`。

### 阶段 C：Manifest

1. 从注释模板理解字段，再生成无注释 `manifest.json`。
2. 只写 Schema 允许字段。
3. `styles` 和 `slots` 去重。
4. `experience.assets` 的每个 slot 同时加入顶层 `slots`。
5. 提供 `en` 和 `zh-CN` 所需文案；当前客户端只主动选择这两种。
6. 源码主题不写 `access`、`integrity`、`signature`。

### 阶段 D：CSS

1. `tokens.css` 只放主题根和明暗 token。
2. `overrides.css` 按 app → sidebar → main/page → home → conversation → composer → settings → menu → content → animation 排序。
3. 每个选择器从精确主题根开始。
4. 优先具体插槽和状态插槽。
5. 不改结构布局；不隐藏业务节点；装饰层 `pointer-events: none`。
6. 动画只使用 `ct-` 前缀，并添加 reduced-motion。

### 阶段 E：静态检查

- JSON 可解析且满足 Schema/引擎校验。
- 所有文件路径存在、大小合规、扩展名与内容一致。
- CSS 无 `url()`、外链、字体、内部类、ID、`data-app-*`。
- CSS 每个使用的 slot 都写入 Manifest。
- 无未引用图片、重复样式或重复插槽。

每次修改 Manifest、CSS 或图片后运行 Skill 自带的共享校验器，不得用其他脚本替代协议结论：

```bash
node /absolute/path/to/retheme-theme-development/scripts/validate-theme.mjs /absolute/path/to/theme
```

提交前还要将源码压成根目录直接包含 `manifest.json` 的 ZIP，并使用同一脚本再校验一次。Skill 由 `pnpm dlx @duxweb/retheme-theme-skill install` 安装，不依赖 ReTheme 源码、Rust 或 Cargo。

### 阶段 F：真实验收

完成以下四象限：中文浅色、中文深色、英文浅色、英文深色。每个象限至少检查：

1. Home 空白态。
2. Home 输入草稿后的 `home-compact`。
3. 新会话与历史会话，滚动长消息。
4. 输入框空/单行/多行/附件/展开面板。
5. 右侧工作区或任务摘要卡片。
6. 设置导航、卡片、表单、Switch。
7. 菜单普通/高亮/选中/分割线。
8. 窄、中、宽、最大化窗口；macOS 与 Windows。
9. 切换主题和恢复主题后无残留。

## 4. AI 禁止行为

- 不修改 ReTheme 引擎来迁就单一主题视觉。
- 不写 ChatGPT 内部选择器、文本匹配、多语言 `aria-label` 匹配或 DOM 层级选择器。
- 不用绝对定位修复首页、Banner、会话、任务卡片或输入框布局。
- 不给输入框编辑区强制高度、内背景或额外 padding。
- 不设置字体，不使用 Emoji 代替素材图标。
- 不把卡片移入 Hero，不改变原生卡片点击/展开结构。
- 不把“在当前截图看起来正确”当成验收完成。
- 不打包 `.ctheme`，不生成平台签名字段。

## 5. 问题归属判断

| 现象 | 先判断为主题问题 | 先判断为引擎问题 |
|:--|:--|:--|
| 只有一个主题异常 | 是 | 否 |
| 所有主题同一 ChatGPT 版本都缺相同插槽 | 否 | 是 |
| 颜色、圆角、透明度、动画不理想 | 是 | 否 |
| Banner 被挂到错误 DOM 或恢复后消失 | 否 | 是 |
| 输入框因主题 CSS 高度规则错位 | 是 | 否 |
| 页面切换后插槽未重新标记 | 否 | 是 |
| 某平台原生画布/标题栏差异 | 先用平台规则核对 | 多主题复现则是 |

修复引擎前必须用最小参考主题和另一个已知正常主题复现。修复主题时不得改引擎选择器映射。

## 6. 最终交付清单

- `manifest.json`：无注释、合法、字段完整。
- `styles/tokens.css`：浅色/深色 token。
- `styles/overrides.css`：按区域组织且有必要注释。
- `assets/`：只有已引用、已压缩的安全图片。
- 测试记录：平台、ChatGPT 版本、语言、模式、窗口宽度和页面矩阵。
- 已知限制：只记录无法在当前协议安全实现的内容。
- 发布信息：版本、变更说明、主题预览图。

完成标准不是“能加载”，而是协议校验通过、矩阵通过、无布局副作用、可完整恢复。
