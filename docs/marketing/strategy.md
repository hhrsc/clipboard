# Clipboard 宣传增长方案

更新日期：2026-09-01

## 一句话定位

Clipboard 是一款本地优先的 Windows 剪贴板工具。它保存文字和图片历史，并允许用户自由选取需要的文字，复制为纯文本、Markdown、HTML 或单行文本。

## 第一批用户

优先面向每天大量复制内容、又不想把剪贴板上传云端的人：

- 程序员、产品经理和设计师
- 研究生、写作者和内容编辑
- 客服、运营和经常整理资料的办公用户
- 对 Win+V 功能不够满意的 Windows 用户

## 真实案例给出的启发

### Screen Studio

创始人早期持续公开制作过程，一条展示产品的视频带来约 1000 个点赞和 3 笔销售。随后持续发布和正式发布带来稳定销售，但过度依赖 Twitter 也成为后来的增长风险。

可复用点：真实产品演示比抽象口号有效；公开开发能建立信任；必须同时建设搜索和内容渠道。

来源：https://www.indiehackers.com/post/how-buildinpublic-brought-the-first-10-customers-for-screen-studio-and-later-became-the-main-marketing-challenge-ea7c6f0eb1

### Quicker

Quicker 把动作分享和社区内容做成增长飞轮。官网展示 8000 多个共享动作和 500 多万次安装。

可复用点：工具产品最强的传播物不是品牌口号，而是可复用的具体用法。Clipboard 应持续发布工作流示例，并让用户能提交需求和反馈。

来源：https://getquicker.net/

### Maccy

Maccy 长期坚持“轻量、开源、简单”的定位，官网、GitHub 和应用商店共同承接下载。

可复用点：小工具不需要讲宏大故事。定位越短，越容易被记住和转述。

来源：https://maccy.app/ 及 https://github.com/p0deje/Maccy

### 同类 Windows 工具的 V2EX 发布

一篇近期的 Windows 剪贴板工具帖子通过问题背景、功能清单、键盘操作表和下载链接获得约 1500 次浏览。

可复用点：垂直社区接受开发者发布，但内容必须具体、诚实，并持续回复更新。

来源：https://hk.v2ex.com/t/1229540

### Microsoft Store 小工具案例

一位独立开发者公开的数据为 422 次商店页面访问和 169 次安装。它不是行业平均值，但说明小工具也能从商店搜索获得全球自然流量。

可复用点：上架商店、完善截图和关键词，并为不同渠道使用可区分的下载入口。

来源：https://elma.sh/blog/microsoft-store-app-launch-169-installs

## 渠道优先级

### 第一层：信任和可追踪下载

- 官网：产品演示、隐私说明、版本说明、SHA-256、源码和反馈入口
- GitHub Releases：版本化安装包与下载次数
- GitHub 仓库：公开可检查的源码和更新记录

### 第二层：高意图社区

- V2EX：开发者故事和真实使用场景
- 小众软件、少数派：Windows 效率工具介绍
- GitHub Discussions 或 Issues：收集功能需求和问题

### 第三层：可重复的短内容

- 抖音：15 到 25 秒，先展示痛点，再展示一次完整操作
- Bilibili：60 到 180 秒，演示安装、快捷键、文字格式化复制和图片历史
- 小红书：4 到 6 张图，围绕“Win+V 不够用时怎么办”

### 第四层：长期分发

- Microsoft Store：搜索发现、信任和安装体验
- 百度与 Google：站点地图、搜索收录和围绕具体问题的内容页

## 首轮内容主题

1. 复制过一段重要文字，下一次复制后找不到了
2. 一段文字只想复制其中几句
3. 从网页复制文字，想直接得到 Markdown
4. 截图和图片复制太多，想按时间找回来
5. 不愿把剪贴板历史上传云端
6. Win+V 能用，但分类、搜索和格式转换不够顺手

## 数据基线

每周记录以下数据：

- 官网访问量和主要来源
- GitHub Release 安装包下载次数
- 社区帖子浏览、收藏、评论和点击
- 短视频前 3 秒留存、完播率、主页访问和链接点击
- Issues 数量、有效需求数量和安装问题数量

未接入分析工具前，以 GitHub Release 下载次数、平台帖子数据和 Issues 为基础。后续若接入网站分析，优先选择隐私友好方案，并先更新隐私说明。

执行 `npm run marketing:metrics` 会把官网状态、Release 资源下载量、仓库 Stars、Forks 和待处理 Issues 追加到 `docs/marketing/metrics-history.jsonl`。每次渠道发布前记录一次，发布后 24 小时、72 小时和 7 天各记录一次，避免只看单个时间点。

官网部署且 IndexNow 校验文件可以公开访问后，执行 `npm run marketing:indexnow`，主动通知支持 IndexNow 的搜索引擎发现官网、隐私页与搜索指南。接口接受请求只代表收到 URL 通知，不保证收录或排名。

IndexNow 接入依据：[Bing 官方接入说明](https://www.bing.com/indexnow/getstarted)与 [Bing URL Submission 指南](https://www.bing.com/webmasters/help/URL-Submission-62f2860b)。提交历史保存在 `docs/marketing/indexnow-submissions.jsonl`。

## 30 天目标

- 完成可信下载链路和隐私说明
- 发布 1 篇开发者社区首发帖
- 发布 6 条短演示内容
- 完成 Microsoft Store 上架材料
- 获得第一批真实反馈，至少整理出 5 个高频使用场景

目标不是刷一个漂亮播放量，而是建立“内容被看见、官网被访问、安装包被下载、用户会反馈”的完整闭环。
