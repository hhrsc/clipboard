# Clipboard UI Visual Specification

> 阶段：Reference analysis only  
> 目标：为后续 pixel-perfect / 1:1 实现提供可执行基线  
> 本文只记录参考图中可见的事实与基于像素的推算，不包含设计优化或 UI 实现。

## 1. 参考图与页面映射

| 页面 | 原始文件 | 像素尺寸 | 比例 |
|---|---|---:|---:|
| Images | `codex-clipboard-e8387639-822f-413f-b1d1-4034751253c2.png` | 1586 × 992 | 1.5988:1 |
| Passwords | `codex-clipboard-21c5d4d0-05b0-42c8-9dfa-d855ff40a1f2.png` | 1586 × 992 | 1.5988:1 |
| Settings | `codex-clipboard-72062dc5-5629-4a32-8ccf-b173f56c2906.png` | 1586 × 992 | 1.5988:1 |
| Recent | `codex-clipboard-479aec65-f42e-4f35-82cc-0a230bb7107d.png` | 1586 × 992 | 1.5988:1 |

四张图均为同一桌面应用窗口、同一 viewport 和同一视觉状态族。基线必须按 **1586 × 992 CSS px** 对齐。截图没有提供其他尺寸下的结果，因此不得从这些图片推断移动端、平板端或窄窗口重排规则。

坐标约定：左上角为 `(0, 0)`，矩形使用 `x / y / width / height`。文中 `≈` 表示位图推算值，误差通常为 ±1–3 px；没有 `≈` 的值来自清晰边界测量。

## 2. 测量置信度

| 标记 | 含义 |
|---|---|
| 精确 | 可从连续边界、分割线或图片尺寸直接测得 |
| 高 | 可见轮廓清晰，预计误差不超过 2 px |
| 中 | 文字抗锯齿、生成图纹理或软边导致误差约 3–5 px |
| 推断 | 位图无法反推出源值，例如字体文件名、真实 alpha、阴影参数 |

字体家族、字重内部命名、透明度和阴影核无法从扁平 PNG 中精确恢复。本文会给出视觉最接近的实现候选，但这些候选不得被表述为源设计的已知事实。

## 3. 全局窗口与安全区域

### 3.1 Window frame

| 属性 | 规格 | 置信度 |
|---|---:|---|
| viewport | `1586 × 992` | 精确 |
| 外边框 | `1px solid` 暖灰 | 高 |
| 外圆角 | ≈ `10–12px` | 中 |
| 窗口背景 | 暖白、无透明穿透 | 高 |
| 页面滚动 | 截图中根窗口不滚动；内部列表独立滚动 | 高 |
| overflow | 根容器裁切到窗口圆角 | 高 |

安全区域：所有页面内容位于 `x=1..1584, y=1..990`。任何可点击内容不得越过外边框。左侧主内容安全边距通常为 30–32px；主工作区安全边距通常为 31–34px；窗口控制区占据右上角约 `x=1410..1585, y=0..46`。

### 3.2 Custom title bar / window controls

参考图没有应用图标和窗口标题，只显示 Windows 控制符号。

| 控件 | 视觉中心 | 图标约尺寸 | 点击区推算 |
|---|---:|---:|---:|
| Minimize | `(1434, 23)` | `14 × 1` | ≈ `46 × 44` |
| Maximize | `(1493, 23)` | `13 × 13` | ≈ `46 × 44` |
| Close | `(1554, 23)` | `15 × 15` | ≈ `46 × 44` |

三个控件绝对定位在右上；未显示 hover 状态。图标颜色为接近 `#20201D` 的深墨色，线宽约 1.3–1.5px。

Recent、Passwords、Settings 有明确的主内容顶栏：`x=306, y=0, width=1280, height=92`，下边界为 1px 分隔线。Images 页面保留相同窗口控制区，但页面标题直接进入两列内容，不能擅自添加 Recent 页式横向顶栏。

## 4. 全局栅格

### 4.1 固定侧栏

| 属性 | 值 |
|---|---:|
| x | `0` |
| y | `0` |
| width | `306px` |
| height | `992px` |
| 右分隔线 | `x=306`, `1px` |
| 左内容 padding | `30–31px`（导航选中底为例外，使用 15px） |
| 右内容 padding | `29–30px` |

侧栏占 viewport 的 19.29%。其右边界必须在全部四页保持同一像素列。

### 4.2 页面分栏

| 页面 | 中间区 | 右侧区 | 分隔线 |
|---|---|---|---:|
| Recent | `x=307..783`, ≈ `477px` | `x=784..1585`, ≈ `802px` | `x=783/784` |
| Passwords | `x=307..806`, ≈ `500px` | `x=807..1585`, ≈ `779px` | `x=807` |
| Images | `x=307..1100`, ≈ `794px` | `x=1101..1585`, ≈ `485px` | `x=1101` |
| Settings | 单主内容区 `x=307..1585` | 无二级竖分栏 | 无 |

这些分栏比例按页面任务变化，不得为了组件复用强行统一。

## 5. 色彩系统

以下颜色来自空白区域和大面积实色区域的像素采样；生成图有约 ±2 RGB 的细微纸面噪点。

| Token | 采样/推算值 | 用途 |
|---|---|---|
| `--window-bg` | ≈ `#FAF9F7` | 主工作区、顶栏 |
| `--sidebar-bg` | ≈ `#F6F3F0` | 固定侧栏 |
| `--surface-focus` | ≈ `#FDFCFB` | 输入框、内容框、按钮浅表面 |
| `--nav-active-bg` | ≈ `#F3E8E1` | 一级导航选中态 |
| `--row-selected-bg` | Recent ≈ `#F6EAE3`; Passwords ≈ `#F5EAE6` | 列表选中态 |
| `--ink` | ≈ `#292824` | 标题与正文主色 |
| `--ink-soft` | ≈ `#4E4B46` | 次级正文 |
| `--muted` | ≈ `#77726B` | 时间、计数、说明文字 |
| `--line` | ≈ `#EAE5E0` | 全局分隔线 |
| `--line-strong` | ≈ `#D8D1CA` | 输入框和按钮边框 |
| `--accent` | ≈ `#BD5A3E` | 选中图标、主按钮、强调文字 |
| `--accent-dark` | ≈ `#B64E36` | 主按钮较深区域/按压态推断 |
| `--danger` | ≈ `#C45538` | 删除文字与危险图标 |
| `--danger-surface` | ≈ `#FAF4F1` | Danger zone 背景 |
| `--disabled-fill` | ≈ `#E8BDAA` | Settings 的 disabled Delete |

主背景不是纯白；不得替换为 `#FFFFFF`。侧栏与主区的亮度差很小，但可见。激活底色为陶土色的极浅 tint，不是灰色或粉色渐变。

## 6. 字体与排版

### 6.1 字体族

| 角色 | 位图事实 | 实现候选（推断） |
|---|---|---|
| Display serif | 高反差、编辑感衬线；用于品牌、页面标题和重点正文 | 初始候选为 `Georgia`；实现阶段对比确认 `Times New Roman` 的字形和宽度更接近。按像素实测校准字号和字面宽度，不把候选字体当成已确认的原字体。 |
| UI sans | 中性、开放字面；用于导航、输入、按钮、时间和说明 | `Segoe UI` / `Arial` 类；优先以截图对比决定，不得仅因系统默认而选用 |
| Monospace | Recent 的代码条目 | `Consolas` 类；字面较窄 |

### 6.2 全局字号表

| 用途 | font-size | weight | line-height | letter-spacing |
|---|---:|---:|---:|---:|
| 品牌 `Clipboard` | ≈ `40px` | `400` | ≈ `46px` | ≈ `-1.2px` |
| Settings 大标题 | ≈ `40px` | `400` | ≈ `48px` | ≈ `-1px` |
| Selected clip 大正文标题 | ≈ `31–32px` | `400` | ≈ `40px` | ≈ `-0.4px` |
| 页面/面板标题 | ≈ `21–24px` | `400–500` | ≈ `28px` | ≈ `-0.2px` |
| 导航文字 | ≈ `18px` | `400–500` | ≈ `24px` | `0` |
| 列表主文字 | ≈ `17–18px` | `400–500` | ≈ `24px` | `0` |
| 表单 label | ≈ `17px` | `400` | ≈ `22px` | `0` |
| 按钮文字 | ≈ `16–17px` | `400–500` | ≈ `22px` | `0` |
| 正文说明 | ≈ `16px` | `400` | ≈ `22px` | `0` |
| 时间/元数据 | ≈ `15px` | `400` | ≈ `20px` | `0` |
| Tag | ≈ `11px` | `500` | ≈ `16px` | ≈ `0.2px` |
| 代码内容 | ≈ `14–15px` | `400` | ≈ `18px` | ≈ `-0.1px` |

所有文字抗锯齿正常、opacity 1。Muted 视觉通过较浅文本色实现，不应同时使用低 opacity 造成二次变浅。

## 7. 边框、圆角、阴影与材质

| 元素 | border | radius | shadow | blur |
|---|---|---:|---|---:|
| Window | 1px warm gray | 10–12px | 无可辨识投影 | 0 |
| 一级选中导航 | 无 | 7–8px | none | 0 |
| Search | 1px `--line-strong` | 26px | none | 0 |
| 普通 input/select | 1px `--line-strong` | 7–9px | none | 0 |
| Outline button | 1px `--line-strong` | 6–7px | none | 0 |
| Primary button | 无明显描边 | 6–7px | 极轻，≈ `0 2px 5px rgba(100,45,25,.10)`（推断） | 0 |
| 内容大框 | 1px `--line-strong` | 7–8px | none | 0 |
| Image thumbnail | 无 | 8–9px | none | 0 |
| Selected image thumb | 2px accent + ≈3px 内部亮间隔 | 10–11px | none | 0 |
| Danger zone | 1px 浅陶土边框 | 8px | none | 0 |

主按钮像素存在轻微上下色差，视觉上可实现为极弱的陶土色明暗变化；若实现纯色在对比图中更接近，则优先纯色。参考图没有玻璃、backdrop blur 或明显 elevation shadow。

## 8. 共享 Sidebar specification

### 8.1 品牌

- 边界框约：`x=31, y=47, width=164, height=41`。
- 文本：`Clipboard`，Display serif，≈40px/46px，深墨色。
- 无 logo 图形、无副标题。

### 8.2 一级导航

- 容器约：`x=15, y=130, width=277`。
- 每项：`277 × 46px`。
- 项间垂直 gap：≈9px。
- 内部：左 padding ≈19px；图标 `23–24px`；图标到文字 gap ≈17px；文字起点 `x≈74px`。
- Active：浅陶土底、7–8px 圆角；图标与文字均为 accent。
- Inactive：透明底；图标为灰棕，文字为 ink。
- 页面顺序固定：Recent / Images / Passwords / Settings。

### 8.3 Collections

- 标题行约从 `y=382–386px` 开始，左右 `x=31..264`。
- 标题 `Collections`：≈17px serif/sans 混合观感；plus 图标中心约 `x=257px`，尺寸 16px。
- Collection row：`x=15, width=277, height≈47px`。
- 行图标：≈22px；文字起点 `x≈70px`；计数右对齐到 `x≈266px`。
- 行间无卡片 gap，视觉节奏约 51px。
- Active collection 使用与一级导航相同的浅陶土底和 accent 文本。
- `New collection` 位于列表之后，图标/文字 accent，无背景框。

### 8.4 Sidebar footer

- 普通页面底部计数文本约：`x=31, y=949..970`。
- 右侧下拉箭头中心约 `(271, 956)`，尺寸 ≈12px。
- Passwords 页额外有 `Lock vault` outline button：`x=30, y=829, width=247, height=53`；footer 计数仍位于底部。
- Sidebar 本身固定；截图中不随主列表滚动。

## 9. 共享 Top search

用于 Recent、Passwords、Settings：

- `x=339, y=22, width≈430, height=52`。
- `border-radius≈26px`。
- Search icon：`18–19px`，左边界约 `x=364px`。
- 输入文字起点：`x≈395px`。
- Placeholder：16px UI sans，muted。
- 顶栏底分隔线：`y=91/92`。

Placeholder 分别为：

- Recent / Settings：`Search your clipboard`
- Passwords：`Search passwords`

Images 页搜索框不放在顶栏：见第 11 节。

## 10. Recent 页面

### 10.1 层级

```text
Window
├─ Sidebar
└─ Main
   ├─ Topbar + Search + Window controls
   ├─ Clip list column
   │  ├─ Header
   │  └─ Scrollable rows
   └─ Selected clip inspector
      ├─ Header actions
      ├─ Selected title
      ├─ Collection + copy actions
      ├─ Content box + metadata footer
      ├─ Quick actions
      └─ Delete action
```

### 10.2 Clip list column

| 元素 | x | y | width | height |
|---|---:|---:|---:|---:|
| Column | 307 | 92 | 477 | 900 |
| Header | 307 | 92 | 477 | ≈67 |
| Header title | 339 | ≈116 | — | ≈27 |
| Filter icon | ≈728 | ≈117 | 16 | 16 |
| Selected row | 323 | 159 | ≈439 | ≈82 |
| List scrollbar | ≈768 | ≈160 | 7 | ≈169 thumb |

- 列表左右内边距：左 16px，右约 22px（含 scrollbar）。
- 普通 row 高度约 75–78px；分隔线 1px。
- Selected row 左侧有 2px accent 竖线，背景 `--row-selected-bg`，右侧没有圆角卡片阴影。
- 主文本最多两行；时间右对齐，距离右边约 17px。
- Tag 位于主文本下方，outline 灰边，圆角约 4px，高约 20px。
- 代码条目使用 monospace；普通文字保持 UI sans。

### 10.3 Selected clip inspector

| 元素 | x | y | width | height |
|---|---:|---:|---:|---:|
| Inspector | 784 | 92 | 802 | 900 |
| 内部内容区 | 815 | — | 738 | — |
| `Selected clip` | 815 | ≈116 | — | ≈27 |
| Pin icon button | 1460 | 106 | 43 | 43 |
| More icon button | 1510 | 106 | 43 | 43 |
| 大标题 | 815 | 181 | ≈680 | ≈65 |
| Collection select | 815 | 331 | 244 | 46 |
| Copy all | 1377 | 287 | 176 | 44 |
| Copy selected | 1377 | 339 | 176 | 42 |
| Content box | 815 | 399 | 738 | 367 |
| Content body | 815 | 399 | 738 | 318 |
| Metadata footer | 815 | 717 | 738 | 49 |
| Quick actions | 815 | 809 | 738 | 76 |
| Delete clip | 1405 | 909 | 148 | 44 |

- Inspector 左右 padding 均约 31–32px。
- 大标题使用 31–32px Display serif，约 40px line-height。
- Content body 内 padding ≈17px；正文约 21–22px serif，line-height ≈28px。
- Content box footer 由 1px 上分隔线隔开；左元数据和右时间垂直居中。
- Quick actions 是单一 outline 容器，不是三张 card；内部按钮横排，gap≈10px。
- Delete 为右下 outline danger button，不能替换为圆形图标。

## 11. Images 页面

### 11.1 层级与主分栏

```text
Window
├─ Sidebar
├─ Image library (x=307..1100)
│  ├─ Title
│  ├─ Search + date filter
│  ├─ Today grid
│  └─ Yesterday grid
└─ Selected image inspector (x=1101..1585)
   ├─ Title
   ├─ Large preview
   ├─ Metadata
   └─ Copy / Delete / Clear actions
```

### 11.2 Image library

| 元素 | x | y | width | height |
|---|---:|---:|---:|---:|
| Library | 307 | 1 | 794 | 990 |
| Title | 343 | ≈47 | — | ≈31 |
| Search | 342 | 98 | 530 | 50 |
| Date filter | 883 | 98 | 190 | 50 |
| `Today` label | 343 | ≈181 | — | ≈24 |
| Grid start | 339 | 216 | 734 | — |
| `Yesterday` label | 343 | ≈705 | — | ≈24 |

Today grid：4 列；列 x 约为 `339, 527, 713, 900`。单图可见尺寸约 `174 × 216px`，列 gap 13–14px，行 gap 13px。第二行 y≈446。图片 radius≈8px，`object-fit: cover`。

Selected thumbnail 外框约 `181 × 223px`，2px accent border，图片与外框之间有约 3px 暖白间隔。不得通过 box-shadow 模拟边框。

Yesterday 第一行约从 `y=739px` 开始，保持相同列宽和 gap；底部由 viewport 裁切，不应为“完整显示所有图片”而压缩尺寸。

### 11.3 Selected image inspector

| 元素 | x | y | width | height |
|---|---:|---:|---:|---:|
| Inspector | 1101 | 1 | 485 | 990 |
| 内部左右 padding | 26 | — | — | — |
| Title | 1127 | ≈47 | — | ≈28 |
| Preview | 1127 | 98 | 427 | 480 |
| Metadata block | 1127 | ≈606 | 427 | ≈126 |
| Copy image | 1127 | 758 | 427 | 42 |
| Delete image | 1127 | 810 | 427 | 42 |
| Divider | 1127 | 878 | 427 | 1 |
| Clear images | 1127 | 905 | 427 | 43 |

- Preview aspect ratio ≈0.8896；thumbnail aspect ratio ≈0.806。两处使用同一原图但裁切不同，均为 centered `object-fit: cover`。
- Metadata 为四行，两列；左侧 icon 18–20px、label 起点约 x=1164；值右对齐到 x=1553。
- Copy image 为满宽 primary。
- Delete image 与 Clear images 均为满宽 outline danger；Clear images 与前组由 1px divider + 26px 间距隔离。

### 11.4 图片资产目录

参考图包含 12 个独立缩略图主题；实现时必须准备真实 raster 资产，不能使用渐变或空 placeholder：

1. 白色陶瓷花瓶与碗（选中图，同时用于大预览）
2. 窗边笔记本电脑工作台
3. 山脉与湖泊
4. 米色扶手椅与壁灯
5. 纸质笔记本与黑笔
6. 林间木栈道
7. 棕色陶瓷杯
8. 书本上的小花瓶
9. 暖色厨房
10. 逆光绿叶
11. 海面日落
12. 桌面与装饰画

## 12. Passwords 页面

### 12.1 主分栏

| 区域 | x | y | width | height |
|---|---:|---:|---:|---:|
| Topbar | 307 | 1 | 1279 | 91 |
| Vault list | 307 | 92 | 500 | 900 |
| Item detail | 807 | 92 | 779 | 900 |

### 12.2 Vault list

| 元素 | x | y | width | height |
|---|---:|---:|---:|---:|
| List header | 307 | 92 | 500 | 76 |
| `Password vault` | 339 | ≈123 | — | ≈25 |
| Add password | 609 | 113 | 134 | 39 |
| Filter icon | ≈759 | ≈124 | 18 | 18 |
| Selected row | 323 | 168 | 462 | 103 |
| Scrollbar | ≈794 | 169 | 7 | ≈100 thumb |

- List 左右 padding：16px / 22px。
- Row 高≈103px，底部 1px divider。
- Selected row 有 2px accent 左边线、浅陶土背景、右侧小型 more button。
- 每行内部：标题 18px；用户名和 masked password 14–15px；时间位于右下；more button 38×38px 左右。
- Masked password 使用圆点，不显示明文。

### 12.3 Item detail

| 元素 | x | y | width | height |
|---|---:|---:|---:|---:|
| 内容左右边界 | 838..1550 | — | 712 | — |
| Header title | 838 | ≈124 | — | ≈25 |
| Edit button | 1457 | 111 | 43 | 43 |
| More button | 1508 | 111 | 43 | 43 |
| Title field | 838 | 212 | 712 | 52 |
| Username field | 838 | 323 | 712 | 53 |
| Password field | 838 | 435 | 712 | 53 |
| Fixed footer | 807 | 882 | 779 | 110 |
| Footer lock | 837 | 904 | 175 | 54 |
| Save changes | 1360 | 903 | 190 | 55 |

- Label 与输入间距约 10–12px；field 之间垂直节奏约 59px。
- Username/Password 的 copy action 位于 field 内右侧独立 outline 区域，不能放到 field 外。
- Password field 的 eye icon 位于 copy button 左侧约 28px。
- Footer 固定在 detail 底部，1px 顶分隔线；按钮垂直居中。
- Primary Save 与 Add password 使用同一陶土色按钮系统。

## 13. Settings 页面

### 13.1 页面区域

| 元素 | x | y | width | height |
|---|---:|---:|---:|---:|
| Main content | 307 | 92 | 1279 | 900 |
| Content column | 338 | 132 | ≈1110 | — |
| `General settings` | 338 | 134 | — | ≈48 |
| Row 1 | 338 | ≈213 | ≈1110 | 94 |
| Row 2 | 338 | 307 | ≈1110 | 110 |
| Row 3 | 338 | 418 | ≈1110 | 109 |
| Row 4 | 338 | 528 | ≈1110 | 109 |
| Danger zone | 338 | 666 | 1110 | 164 |

内容不占满窗口右侧；最大内容宽度约 1110px。不得把设置行拉伸到 `x=1554`。

### 13.2 Settings rows

- Row 使用 1px 底分隔线；无卡片背景、无独立圆角。
- 左侧标题约 21–22px serif，说明 16px muted，标题到说明 gap≈5px。
- 控件统一靠右，但每行宽度不同：
  - Capture toggle 约 `x=1362, y=239, 51 × 28px`；`On` 位于右侧约 14px。
  - Shortcut control 约 `x=1156, y=339, 292 × 49px`。
  - Import / Export：各约 `133 × 49px`，gap≈20px，整体右对齐。
  - Open website：约 `184 × 49px`，右对齐。
- 快捷键按键帽为独立小方框，不能渲染成一段普通文本。

### 13.3 Danger zone

- `x=338, y=666, width≈1110, height=164px`。
- Border 1px 浅陶土；radius≈8px；背景比主区略暖。
- 内 padding ≈25px。
- `Danger zone` 标题 accent，≈21px。
- 操作标题距上标题约 29px；说明紧随其下。
- Confirmation input：`x≈1041, y=751, width≈269, height=47px`。
- Disabled Delete：`x≈1322, y=751, width≈101, height=47px`，浅陶土填充、白色低对比文字。
- 该 disabled 状态是参考图事实；初始页面不得显示 enabled 红色删除按钮。

## 14. 图标规范

- 全局是圆角线性图标，视觉尺寸通常 18–24px。
- 主导航图标约 23–24px；stroke≈1.5px。
- 行内/元数据图标约 18–20px；stroke≈1.4–1.6px。
- Icon button 可视框通常 42–43px 方形，radius≈7px。
- 图标与文字 baseline 视觉居中，而非单纯几何居中。
- 位图无法识别原始图标库；后续必须选取轮廓、端点、比例最接近的真实 icon library。不得手绘近似 SVG，也不得用 emoji 或文本字符代替。

## 15. 对齐规则

1. 所有主页面与 Sidebar 在 `x=306/307` 共用同一垂直基线。
2. Recent/Passwords/Settings 的搜索框左边界统一为 `x=339`。
3. Sidebar 品牌、Collections 标题、footer 统一对齐 `x≈31`。
4. Sidebar 一级导航与 active collection 的背景统一为 `x=15..292`。
5. Recent inspector、Passwords detail 的内容左边界分别是分栏线后 31px。
6. Images inspector 左右 padding 约 26px，比其他 inspector 更窄。
7. 表单 label、field、footer action 共享相同内容左右边界。
8. 所有列表时间右对齐，不随正文长度漂移。
9. Settings 控件右侧统一落在约 `x=1448`，而不是窗口最右边。
10. 分隔线严格为 1px，不使用 box-shadow 替代。

## 16. 固定、绝对与滚动元素

| 元素 | 行为 |
|---|---|
| Window controls | absolute/fixed 于窗口右上 |
| Sidebar | 固定整高，不随内容滚动 |
| Topbar | Recent/Passwords/Settings 固定于主内容顶部 |
| Recent clip list | 中间列内部独立纵向滚动 |
| Password vault list | 中间列内部独立纵向滚动 |
| Password detail footer | 固定于右侧 detail 底部 |
| Image library | 仅主图库区域纵向滚动；右侧 inspector 保持独立 |
| Settings | 截图高度内无滚动证据；内容自然放置 |

滚动条可见宽度约 6–7px；track 透明，thumb 为浅灰棕、圆角约 4px。

## 17. 重复组件清单

后续实现应复用结构但不得统一掉页面特有尺寸：

1. `AppWindow`
2. `WindowControls`
3. `Sidebar`
4. `PrimaryNavItem`
5. `CollectionRow`
6. `SidebarFooter`
7. `TopSearch`
8. `SectionTitle`
9. `IconButton`
10. `PrimaryButton`
11. `OutlineButton`
12. `DangerButton`
13. `TextField`
14. `SelectField`
15. `ScrollableList`
16. `SelectableRow`
17. `MetadataRow`
18. `SettingsRow`
19. `DangerZone`
20. `InternalScrollbar`

可复用 token：颜色、字体角色、1px line、按钮高度、icon button。不可盲目复用：页面列宽、图库 padding、列表 row 高、右侧 inspector 宽度。

## 18. 页面状态事实

| 页面 | 截图状态 |
|---|---|
| Recent | Recent active；All clips active；第一条 clip selected；pin 未确认状态；普通内容 inspector |
| Images | Images active；第一张 Today 图片 selected；右侧显示该图片；Clear images enabled |
| Passwords | Passwords active；vault unlocked；第一条 password selected；字段为查看态；Save changes 可见 |
| Settings | Settings active；capture On；shortcut 为 Ctrl+Shift+V；Delete disabled，等待输入 DELETE |

后续做视觉对比时必须复现同一状态，不能拿 empty/loading/不同选择态截图比较。

## 19. 不确定项与禁止推断

以下内容无法由静态位图确认，实施前不得自行补充为设计事实：

- 精确字体文件名及 variable font axis。
- hover、pressed、focus、loading、error、modal 状态。
- 小于 1586×992 时的响应式布局。
- 动画时长、easing、页面切换方式。
- 图片原始文件、完整未裁切画面和色彩配置。
- 图标库名称。
- 阴影/纸面纹理是否为真实设计层，还是生成图的像素噪点。
- 顶栏是否可拖拽、双击最大化等原生行为（视觉上是 custom title bar）。

出现这些问题时，应优先保持当前截图的静态视觉结果，不得借“最佳实践”主动重设计。

## 20. Pixel-parity 验收基线（供后续阶段使用）

本阶段不执行实现或 QA。后续实现阶段应至少按以下顺序核对：

1. 在 `1586 × 992` 同尺寸 viewport 截图。
2. 分别复现 Recent / Images / Passwords / Settings 的参考状态。
3. 先检查全局锚点：`x=306` sidebar、`y=92` topbar、各页面第二分栏线。
4. 再检查组件外框：search、rows、fields、preview、footer、danger zone。
5. 最后检查字体、颜色、图标和 1–3px 微差。
6. 使用 overlay/difference 比较，不凭肉眼记忆判断。
7. 未达到同 viewport、同状态、同素材时，不得宣称 pixel-perfect。

## 21. 第一阶段边界

- 已完成：四张参考图读取、像素尺寸确认、结构/位置/尺寸/间距/颜色/排版/边框/圆角/状态/重复组件规范化。
- 未执行：Svelte/HTML/CSS 修改、组件实现、素材生成、依赖安装、开发服务器、浏览器视觉 QA、Tauri 构建。
- 本文是下一阶段的唯一实现规格；若参考图与本文存在冲突，以参考图像素为准。
