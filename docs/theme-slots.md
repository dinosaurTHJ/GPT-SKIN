# ReTheme v1 稳定插槽目录

本目录对应运行时唯一白名单 `retheme_theme_protocol::ALLOWED_SLOTS`，共 164 项。Manifest 的 `slots` 只能使用这里的名称。节点在当前页面不存在是正常情况；主题不能为了让插槽出现而查找 ChatGPT 内部类名或自行创建业务 DOM。

## 阅读规则

- **原生**：ChatGPT 自己的节点，引擎只添加 `data-ct-slot`。只改视觉，不改语义、DOM 顺序、交互、滚动或可访问性。
- **引擎**：ReTheme 创建的受控挂载。可做视觉与无交互装饰，但不得增加业务行为；装饰应保持 `pointer-events: none`。
- **混合**：引擎为原生区域建立稳定外层或按适配结果选择原生节点。仍由引擎拥有布局。
- “禁止”列列出该插槽最重要的局部限制。所有插槽同时禁止字体族、外部 URL、ChatGPT 内部类名、隐藏关键功能和横向溢出。

## App 与标题栏

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `app.shell` | 原生根 | 文档根 | 主题运行期间始终存在 | 全局颜色、CSS token | 改根布局、滚动或字体 |
| `app.background` | 引擎资源挂载 | `app.shell` 最底层 | `experience.assets` 配置该资源 | 全局背景图、透明度 | 捕获事件、盖住主界面 |
| `titlebar` | 原生 | `app.shell` | 平台标题/菜单栏可识别时 | 背景、边框、透明度 | 改拖拽区、窗口按钮与高度 |

## 侧栏

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `sidebar` | 原生 | `app.shell` | 侧栏可见 | 背景、右边框、阴影 | 改宽度算法或定位 |
| `sidebar.scroll` | 原生 | `sidebar` | 找到侧栏滚动容器 | 滚动区背景、滚动条色 | 禁用或接管滚动 |
| `sidebar.resize` | 原生 | `sidebar` 边缘 | 可调整侧栏宽度时 | Hover 色、命中提示 | 改命中宽度或拖动逻辑 |
| `sidebar.resize.indicator` | 原生/引擎标记 | `sidebar.resize` | 拖动指示节点存在 | 线色、透明度 | 搬动拖动边界 |
| `sidebar.header` | 原生 | `sidebar` | 顶部品牌/工作区区域存在 | 背景、间距内的视觉 | 改高度与业务布局 |
| `sidebar.header.icon` | 原生 | `sidebar.header` | Header 有图标 | 图标色、尺寸上限 | 替换点击行为 |
| `sidebar.header.label` | 原生 | `sidebar.header` | Header 有标题 | 字号、字重、颜色 | 用伪元素替代标题 |
| `sidebar.header.background` | 引擎资源挂载 | `sidebar.header` 底层 | 配置对应资源 | 背景图、适配方式 | 盖住标题或按钮 |
| `sidebar.header.decoration` | 引擎资源挂载 | `sidebar.header` | 配置对应资源 | Logo 周围装饰 | 接收鼠标事件 |
| `sidebar.brand` | 原生 | `sidebar.header` | 品牌/工作区入口存在 | 表面、边框、图标关系 | 改入口行为 |
| `sidebar.brand.icon` | 引擎资源挂载 | `sidebar.brand` | 配置品牌图标 | 自定义 Logo、尺寸 | 使用外部图或 Emoji |
| `sidebar.brand.badge` | 引擎资源挂载 | `sidebar.brand` | 配置品牌徽标 | 小徽章、丝带 | 遮住品牌名称 |
| `sidebar.frame` | 引擎资源挂载 | `sidebar` 底层 | 配置侧栏框架资源 | 纹理、边框装饰 | 参与侧栏尺寸计算 |
| `sidebar.section` | 原生 | `sidebar.scroll` | 侧栏分组存在 | 分组间距、背景 | 重排分组 |
| `sidebar.section.projects` | 原生 | `sidebar.scroll` | 项目分组存在 | 项目区背景、上边界 | 隐藏项目列表 |
| `sidebar.section.header` | 原生 | `sidebar.section` | 分组有标题行 | 标题行背景、圆角 | 改折叠行为 |
| `sidebar.section.toggle` | 原生 | `sidebar.section.header` | 分组可折叠 | 图标颜色、Hover | 改命中区、旋转状态逻辑 |
| `sidebar.section.label` | 原生 | `sidebar.section.header` | 分组有文字标题 | 颜色、字号、字重 | 改文案内容 |
| `sidebar.section.actions` | 原生 | `sidebar.section.header` | 标题行有操作区 | 操作区颜色、间距 | 隐藏新建等操作 |
| `sidebar.section.action` | 原生 | `sidebar.section.actions` | 操作按钮存在 | 按钮表面、Hover | 改按钮尺寸到不可点 |
| `sidebar.section.action.icon` | 原生 | `sidebar.section.action` | 操作按钮有图标 | 图标色、线宽感 | 替换语义图标 |
| `sidebar.section.decoration` | 引擎资源挂载 | `sidebar.section.projects` | 配置专用资源 | 项目区丝带/挂件 | 覆盖分组操作 |
| `sidebar.item` | 原生 | `sidebar.scroll` / section | 普通导航、会话或项目项存在 | 背景、圆角、Hover | 改行布局和点击范围 |
| `sidebar.item.icon` | 原生 | `sidebar.item` | 普通项有图标 | 图标颜色、尺寸 | 强制隐藏 SVG |
| `sidebar.item.label` | 原生 | `sidebar.item` | 普通项有标签 | 文字色、截断视觉 | 取消必要省略导致溢出 |
| `sidebar.item.active` | 原生状态 | `sidebar.scroll` / section | 当前项被选中 | 选中背景、边框、高光 | 用 Hover 模拟选中状态 |
| `sidebar.item.active.icon` | 原生状态 | `sidebar.item.active` | 选中项有图标 | 选中图标色 | 改图标结构 |
| `sidebar.item.active.label` | 原生状态 | `sidebar.item.active` | 选中项有标签 | 选中文字色、字重 | 隐藏当前项名称 |
| `sidebar.footer` | 原生 | `sidebar` 底部 | Footer 存在 | 背景、顶边界、内间距视觉 | 改固定/滚动归属 |
| `sidebar.footer.item` | 原生 | `sidebar.footer` | 设置/账号等入口存在 | 行背景、Hover、圆角 | 改入口顺序 |
| `sidebar.footer.icon` | 原生 | `sidebar.footer.item` | Footer 项有图标 | 图标色、大小 | 替换交互语义 |
| `sidebar.footer.label` | 原生 | `sidebar.footer.item` | Footer 项有文字 | 文字色、字号 | 覆盖本地化文案 |
| `sidebar.footer.brand` | 引擎挂载 | `sidebar.footer` | ReTheme 状态条可挂载时 | 品牌行背景、圆角 | 增加独立边框破坏原布局 |
| `sidebar.footer.brand.label` | 引擎 | `sidebar.footer.brand` | 品牌行存在 | ReTheme 名称字号、颜色 | 修改账号/主题状态 |
| `sidebar.footer.brand.timer` | 引擎 | `sidebar.footer.brand` | 试用主题有倒计时 | 倒计时颜色、徽标感 | 隐藏到期信息 |
| `sidebar.footer.brand.pro` | 引擎 | `sidebar.footer.brand` | Pro 身份显示时 | Pro 徽标颜色、表面 | 伪造权益状态 |
| `sidebar.footer.brand.version` | 引擎 | `sidebar.footer.brand` | 运行时选择显示版本时 | 次要文字颜色 | 伪造版本信息 |

## 主区、页面与菜单

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `main` | 原生 | `app.shell` | 主内容存在 | 基础背景、文字色 | 改主区定位和滚动 |
| `main.fade` | 原生 | `main` 边缘 | 原生渐变/遮罩可识别时 | 透明度、背景、隐藏视觉遮罩 | 改交互命中 |
| `main.content.frame` | 原生 | `main` | 内容框架存在 | 边框、阴影、透明度 | 改内容宽度计算 |
| `main.background` | 引擎资源挂载 | `main` 最底层 | 配置对应资源 | 主区背景图 | 盖住页面内容 |
| `main.overlay` | 引擎资源挂载 | `main.background` 上层 | 配置对应资源 | 网格、光斑、纹理 | 高对比影响阅读 |
| `main.frame` | 引擎资源挂载 | `main` 上层 | 配置对应资源 | 四周装饰框 | 捕获事件、挤压布局 |
| `page` | 原生 | `main` | 非首页页面可识别时 | 页面背景、文字色 | 改页面滚动结构 |
| `page.surface` | 原生 | `page` | 页面画布/表面存在 | 表面色、边框、阴影 | 强制固定宽高 |
| `page.header` | 原生 | `page` | 页面 Header 存在 | 背景、底边框、透明度 | 改按钮位置或 Header 高度 |
| `page.content` | 原生 | `page` | 页面内容存在 | 内容背景、文字色 | 改原生布局/滚动 |
| `menu` | 原生 | 浮层根 | 下拉/上下文菜单打开 | 背景、边框、阴影、圆角 | 改菜单定位与层级 |
| `menu.item` | 原生 | `menu` | 菜单项存在 | 普通表面、文字色 | 改命中高度到不可用 |
| `menu.item.active` | 原生状态 | `menu` | Hover/键盘高亮项 | 高亮背景、文字色 | 造成 Hover 闪烁 |
| `menu.item.checked` | 原生状态 | `menu` | 已选项存在 | 选中色、图标色 | 隐藏选中提示 |
| `menu.icon` | 原生 | `menu.item` | 菜单项有图标 | 图标色、大小 | 用 Emoji 替代 |
| `menu.label` | 原生 | `menu.item` | 菜单项有标签 | 文字色、字号 | 覆盖本地化文字 |
| `menu.shortcut` | 原生 | `menu.item` | 快捷键提示存在 | 次要文字色 | 隐藏快捷键 |
| `menu.separator` | 原生 | `menu` | 菜单分割线存在 | 颜色、粗细、边距视觉 | 改成可交互元素 |

## 输入框 Composer

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `composer` | 原生 | `composer.region` / 页面 | 输入框存在 | 外层背景、边框、圆角、高光 | 设置固定高度、额外结构 padding |
| `composer.backdrop` | 原生 | `composer` 底层 | 原生渐变/背景层存在 | 背景、透明度、去除多余渐变 | 改输入框定位 |
| `composer.context` | 原生 | `composer` | 附件/工具上下文条存在 | 实色/半透明背景、边界 | 改行高或折行逻辑 |
| `composer.context.item` | 原生 | `composer.context` | 上下文项存在 | 背景、圆角、间距视觉 | 删除项或改事件 |
| `composer.context.item.icon` | 原生 | `composer.context.item` | 上下文项有图标 | 图标色、尺寸 | 隐藏状态图标 |
| `composer.context.item.label` | 原生 | `composer.context.item` | 上下文项有标签 | 文字色、字号 | 覆盖文案 |
| `composer.editor` | 原生 contenteditable | `composer` | 编辑区存在 | 文字色、Caret、占位符色 | 背景填充、固定高、额外 padding |
| `composer.action` | 原生 | `composer` | 附件/工具按钮存在 | 按钮背景、Hover、圆角 | 改按钮布局或命中范围 |
| `composer.action.icon` | 原生 | `composer.action` | 操作按钮有图标 | 图标色、尺寸 | 替换业务图标 |
| `composer.action.label` | 原生 | `composer.action` | 操作按钮有标签 | 标签色、字号 | 隐藏必要标签 |
| `composer.permission` | 原生 | `composer` | 权限/模式按钮存在 | 背景、边框、选中视觉 | 伪造权限状态 |
| `composer.permission.icon` | 原生 | `composer.permission` | 权限按钮有图标 | 图标色、尺寸 | 改权限行为 |
| `composer.permission.label` | 原生 | `composer.permission` | 权限按钮有文字 | 文字色、字号 | 覆盖当前模式文字 |
| `composer.submit` | 原生 | `composer` | 发送按钮存在 | 背景、图标色、圆角、高光 | 改禁用/发送行为 |
| `composer.submit.icon` | 原生 | `composer.submit` | 发送图标存在 | 图标色、尺寸 | 隐藏提交状态 |
| `composer.submit.decoration` | 引擎资源挂载 | `composer.submit` | 配置 `composerSubmit` | 按钮小装饰 | 盖住图标或捕获点击 |
| `composer.decoration` | 引擎资源挂载 | `composer` | 配置 `composerDecoration` | 边框中央挂件 | 改输入区尺寸 |
| `composer.panel` | 原生 | `composer` 附近 | 工具/模式面板展开 | 背景、边框、阴影 | 改弹层定位 |
| `composer.panel.item` | 原生 | `composer.panel` | 面板项存在 | 背景、Hover、圆角 | 改选择行为 |
| `composer.panel.icon` | 原生 | `composer.panel.item` | 面板项有图标 | 图标色、大小 | Emoji 替换 |
| `composer.panel.separator` | 原生 | `composer.panel` | 分割线存在 | 线色、透明度 | 改结构间距 |

## 首页 Hero 与快捷卡片

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `home.hero` | 引擎挂载 | `home.stage` 首项 | `home` 且配置必填 Hero | 高度、表面、边框、圆角 | 绝对/固定定位、负偏移 |
| `home.hero.viewport` | 引擎 | `home.hero` | Hero 存在 | 裁切策略、圆角 | 改流式占位高度 |
| `home.hero.copy` | 引擎 | `home.hero.viewport` | Hero 存在 | 文案区域宽度、对齐、层级 | 脱离 Hero 文档流 |
| `home.hero.eyebrow` | 引擎 | `home.hero.copy` | eyebrow 有内容 | 小标题色、字距 | 用图片替代本地化文字 |
| `home.hero.title` | 引擎 | `home.hero.copy` | title 有内容 | 标题色、字号、行高 | 固定单行导致溢出 |
| `home.hero.description` | 引擎 | `home.hero.copy` | description 有内容 | 次要文字色、宽度 | 低对比或裁切 |
| `home.hero.media` | 引擎 | `home.hero.viewport` 底层 | Hero 存在 | 透明度、滤镜、裁切 | 盖住文案 |
| `home.hero.media.asset` | 引擎图片 | `home.hero.media` | 背景资源加载成功 | object-fit/position 视觉 | 改资源 URL |
| `home.hero.foreground` | 引擎 | `home.hero.viewport` | 配置 foreground | 前景宽度、位置、层级 | 参与布局或捕获事件 |
| `home.hero.foreground.asset` | 引擎图片 | `home.hero.foreground` | 前景资源加载成功 | 尺寸、滤镜 | 拉伸失真、改 URL |
| `home.hero.divider` | 引擎 | `home.hero` / 下缘 | 配置 divider | 分割区域颜色、间距视觉 | 改 Hero 与卡片顺序 |
| `home.hero.divider.icon` | 引擎 | `home.hero.divider` | divider 配置 asset | 小图标/装饰尺寸 | 替代 label |
| `home.hero.divider.label` | 引擎 | `home.hero.divider` | divider 有 label | 标签色、字号 | 隐藏本地化标签 |
| `home.hero.divider.line` | 引擎 | `home.hero.divider` | divider 存在 | 线色、渐变、高光 | 撑开页面宽度 |
| `home.layout` | 混合 | `main` | 首页或紧凑首页识别成功 | 只读背景与 token | 改 flex、尺寸与排列 |
| `home.content.region` | 混合 | `home.layout` | 首页原生内容分支存在 | 区域表面、颜色 | 改原生流式布局 |
| `home.stage` | 混合 | `home.content.region` | 首页识别成功 | 只读视觉背景 | 改 flex 方向、定位 |
| `home.brand` | 原生 | `home.stage` | 原生品牌/Logo 区存在 | 颜色、透明度、隐藏 Logo 视觉 | 影响首页尺寸占位 |
| `home.prompt` | 原生 | `home.stage` | 首页提示区域存在 | 文字区颜色、间距视觉 | 改区域结构 |
| `home.prompt.title` | 原生或引擎替换 | `home.prompt` | 原生标题或 `homePrompt` 存在 | 字号、颜色、字重 | 用伪元素硬编码文案 |
| `home.cards` | 原生 | `home.stage` | 快捷建议区存在 | 区域间距、背景 | 移入 Hero 或改点击结构 |
| `home.cards.layout` | 原生 | `home.cards` | 外层布局可识别时 | 外层最大宽度视觉 | 改页面定位 |
| `home.cards.grid` | 原生 | `home.cards.layout` / cards | 卡片网格可识别时 | Gap 与列视觉 token | 强制溢出或固定总宽 |
| `home.card` | 原生 | `home.cards.grid` | 快捷卡片存在 | 背景、边框、圆角、Hover | 固定高度、绝对定位 |
| `home.card.background` | 引擎资源挂载 | 每个 `home.card` 底层 | 配置对应资源 | 卡片纹理、透明度 | 盖住内容、捕获点击 |
| `home.card.content` | 原生 | `home.card` | 卡片内容容器可识别 | 内容颜色、层级 | 改内容排列 |
| `home.card.icon` | 原生 | `home.card.content` | 卡片图标容器存在 | 图标容器背景、大小 | 改卡片布局 |
| `home.card.icon.glyph` | 原生 SVG | `home.card.icon` | SVG 图标存在 | 实际图标 stroke/fill 色 | 给外圈代替图标着色 |
| `home.card.label` | 原生 | `home.card.content` | 卡片标签存在 | 文字色、字号、行高 | 固定高度造成重叠 |
| `home.card.arrow` | 原生 | `home.card` | 原生箭头存在 | 箭头容器背景、透明度 | 改点击行为 |
| `home.card.arrow.glyph` | 原生 SVG | `home.card.arrow` | SVG 箭头存在 | 实际箭头颜色、大小 | 隐藏方向提示 |
| `home.card.arrow.asset` | 引擎资源挂载 | `home.card.arrow` | 配置对应资源 | 自定义箭头素材 | 同时遮住原生箭头与状态 |
| `composer.region` | 混合 | `home.layout` / 页面底部 | 首页或会话输入区存在 | 区域背景透明度 | 改输入框定位、宽度算法 |

## 会话、代码与终端

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `conversation.stage` | 混合 | `main` | 已开始或历史会话 | 背景、只读视觉 token | 改 flex、定位或滚动 |
| `conversation.header` | 引擎挂载 | `conversation.stage` 首项 | 配置窄 Banner | Header 透明度、间距视觉 | 绝对定位或挤压摘要栏 |
| `conversation.header.content` | 引擎 | `conversation.header` | 窄 Banner 存在 | 最大宽度、视觉对齐 token | 手算媒体查询偏移 |
| `conversation.viewport` | 原生/混合 | `conversation.stage` | 会话消息区存在 | 背景、滚动条色 | 改滚动、锚点或高度 |
| `conversation.summary.region` | 原生 | `conversation.stage` 旁侧 | 右侧任务/摘要栏出现 | 区域背景、边界 | 改栏宽与定位 |
| `conversation.summary` | 原生 | `conversation.summary.region` | 摘要卡片存在 | 卡片背景、双边框、高光 | 改任务交互 |
| `conversation.summary.decoration` | 引擎资源挂载 | `conversation.summary` | 配置专用资源 | 上边框挂件 | 挤压标题、捕获事件 |
| `conversation` | 原生 | `conversation.viewport` | 消息列表可识别 | 列背景、文字 token | 改消息列宽与滚动 |
| `conversation.user` | 原生 | `conversation` | 用户消息存在 | 用户消息表面、文字色 | 隐藏内容或改变语义 |
| `conversation.assistant` | 原生 | `conversation` | 助手消息存在 | 助手文字、链接与表面 | 改内容结构 |
| `conversation.banner` | 引擎 | `conversation.header.content` | 配置会话 Banner | 高度、背景、边框、圆角 | 浮动遮挡消息 |
| `conversation.banner.copy` | 引擎 | `conversation.banner` | Banner 存在 | 文案区宽度、对齐 | 脱离 Banner |
| `conversation.banner.eyebrow` | 引擎 | `conversation.banner.copy` | eyebrow 非空 | 小标题色、字距 | 硬编码语言 |
| `conversation.banner.title` | 引擎 | `conversation.banner.copy` | title 非空 | 标题色、字号 | 固定单行裁切 |
| `conversation.banner.description` | 引擎 | `conversation.banner.copy` | description 非空 | 次要色、字号 | 低对比 |
| `conversation.banner.media` | 引擎 | `conversation.banner` 底层 | Banner 存在 | 透明度、滤镜 | 盖住文案 |
| `conversation.banner.media.asset` | 引擎图片 | `conversation.banner.media` | 资源加载成功 | fit/position 视觉 | 改 URL |
| `conversation.banner.foreground` | 引擎 | `conversation.banner` | 配置 foreground | 前景宽度、底对齐、露头 | 参与文档流尺寸 |
| `conversation.banner.foreground.asset` | 引擎图片 | 前景挂载 | 前景资源加载成功 | 尺寸、滤镜 | 拉伸或捕获事件 |
| `code` | 原生 | 消息内容 | 代码块存在 | 背景、边框、语法整体色调 | 破坏复制/滚动 |
| `code.inline` | 原生 | 消息文字 | 行内代码存在 | 背景、文字色、圆角 | 改基线导致行高跳动 |
| `diff` | 原生 | 消息/工具输出 | Diff 存在 | 增删背景、边界 | 隐藏差异语义 |
| `terminal` | 原生 | 工具输出 | 终端卡片存在 | 外框、标题表面 | 改终端尺寸行为 |
| `terminal.viewport` | 原生 | `terminal` | 终端内容区存在 | 背景、文字色、滚动条 | 禁用选择或滚动 |

## 设置

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `settings` | 原生 | `main` | 设置页面打开 | 页面背景、文字 token | 改设置页布局 |
| `settings.header` | 原生 | `settings` | 设置 Header 存在 | 背景、底边框、透明度 | 隐藏返回/关闭按钮 |
| `settings.sidebar` | 原生 | `settings` | 设置左栏存在 | 背景、右边界 | 改栏宽和滚动 |
| `settings.nav` | 原生 | `settings.sidebar` | 设置导航存在 | 导航区背景、间距视觉 | 重排导航 |
| `settings.nav.item` | 原生 | `settings.nav` | 普通导航项存在 | 背景、文字色、圆角 | 改命中范围 |
| `settings.nav.item.active` | 原生状态 | `settings.nav` | 当前导航项 | 选中背景、文字色 | 隐藏当前状态 |
| `settings.content` | 原生 | `settings` | 内容列存在 | 内容背景、文字色 | 改滚动容器 |
| `settings.surface` | 原生 | `settings.content` | 外层画布存在 | 表面色、透明度 | 固定尺寸 |
| `settings.frame` | 原生 | `settings.surface` | 框架层存在 | 边框、圆角、阴影 | 改平台布局差异 |
| `settings.canvas` | 原生 | `settings.frame` | 内画布存在 | 页面底色、背景图叠加感 | 强制白色大容器 |
| `settings.toolbar` | 原生 | `settings.canvas` | 顶部工具栏存在 | 背景、边界、透明度 | 改工具栏占位高度 |
| `settings.body` | 原生 | `settings.canvas` | 设置正文存在 | 正文背景、文字色 | 改布局与滚动 |
| `settings.section` | 原生 | `settings.body` | 设置分区存在 | 区块间距视觉、背景 | 合并/重排分区 |
| `settings.section.title` | 原生 | `settings.section` | 分区标题存在 | 标题色、字号、字重 | 覆盖标题文案 |
| `settings.card` | 原生 | `settings.section` | 设置卡片存在 | 背景、边框、圆角、阴影 | 改卡片宽高布局 |
| `settings.row` | 原生 | `settings.card` / section | 设置项行存在 | 行背景、Hover | 改控件与文案排列 |
| `settings.row.title` | 原生 | `settings.row` | 行标题存在 | 标题色、字号 | 隐藏标签 |
| `settings.row.description` | 原生 | `settings.row` | 行说明存在 | 次要色、行高 | 低对比或截断 |
| `settings.row.separator` | 原生 | `settings.card` | 行间分割线存在 | 线色、透明度 | 改成大间距 |
| `settings.control` | 原生 | `settings.row` | 输入/选择/按钮控件存在 | 边框、背景、Focus | 隐藏 Focus、改值 |
| `settings.control.checked` | 原生状态 | `settings.row` | 控件处于选中状态 | 强调色、选中背景 | 伪造状态 |
| `settings.switch` | 原生 | `settings.control` | Switch 存在 | 外部尺寸比例视觉 | 改点击和状态属性 |
| `settings.switch.checked` | 原生状态 | `settings.control.checked` | Switch 开启 | 选中高光 | 与关闭状态不可辨认 |
| `settings.switch.track` | 原生 | `settings.switch` | Switch 轨道存在 | 关闭背景、边框 | 改轨道布局 |
| `settings.switch.track.checked` | 原生状态 | `settings.switch.checked` | Switch 开启 | 开启背景、边框 | 覆盖 Thumb |
| `settings.switch.thumb` | 原生 | `settings.switch.track` | Switch 圆点存在 | 圆点颜色、阴影 | 改位移动画逻辑 |

## 角落装饰

| 插槽 | 节点 | 父级 | 出现条件 | 推荐修改 | 禁止 |
|:--|:--|:--|:--|:--|:--|
| `decoration.top-right` | 引擎资源挂载 | `main` 装饰层 | `experience.decorations` 配置 | 右上氛围挂件、透明度、动画 | 覆盖标题栏/按钮或捕获事件 |
| `decoration.bottom-right` | 引擎资源挂载 | `main` 装饰层 | `experience.decorations` 配置 | 右下氛围挂件、透明度、动画 | 覆盖输入框或工作区面板 |

## Manifest 资源插槽补充

只有下列普通插槽可通过 `experience.assets` 挂载图片：

`app.background`、`main.background`、`main.overlay`、`main.frame`、`sidebar.brand.icon`、`sidebar.brand.badge`、`sidebar.header.background`、`sidebar.header.decoration`、`sidebar.frame`、`home.card.background`、`home.card.arrow.asset`。

它们必须同时写入顶层 `slots`，每个资源插槽只能声明一次。专用字段 `composerSubmit`、`composerDecoration`、`conversationSummaryDecoration` 和 `sidebarSectionDecoration` 不放在 `experience.assets`。

## 归属判断

- 单一主题在某插槽颜色、边框、透明度或动画异常：先修主题。
- 多个主题在同一 ChatGPT 版本缺同一个原生插槽：先修兼容数据或引擎。
- 引擎挂载位置错误、切页后丢失、恢复后残留：修引擎，不在主题里写位置补丁。
- 输入框、消息滚动、首页卡片或设置布局被某主题改变：删除该主题的结构规则。

更新运行时白名单时，必须同时更新 Schema、本目录、中文/英文开发指南、Skill 插槽参考和协议一致性测试。
