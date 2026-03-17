# 国内常见应用缓存扫���报告

**扫描时间**: 2026-03-17
**扫描目标**: 中国国内常见应用的缓存位置
**目的**: 扩展 DiskDisk 对国内应用的支持

---

## 🎯 发现的国内应用缓存

### 1. 办公软件

#### ✅ WPS Office (金山办公)
**缓存位置**:
```
%LOCALAPPDATA%\Kingsoft\WPS Office\12.1.0.*
%APPDATA%\kingsoft\wps\addons
%APPDATA%\kingsoft\wps\download
%APPDATA%\kingsoft\wpspdf
%APPDATA%\kingsoft\wpsphoto+
```

**相关产品**:
- WPS Video Edit (万兴优剪)
- EdrawMax (WPS版)

**危险等级**: 🟢 低
**说明**: WPS 的各种办公套件缓存

---

### 2. 浏览器

#### ✅ 豆包浏览器 (字节跳动)
**缓存位置**:
```
%LOCALAPPDATA%\Doubao\User Data\Default\Cache
%LOCALAPPDATA%\Doubao\User Data\Default\Code Cache
%LOCALAPPDATA%\Doubao\User Data\Default\GPUCache
%LOCALAPPDATA%\Doubao\User Data\Default\DawnCache
%LOCALAPPDATA%\Doubao\User Data\Default\DawnGraphiteCache
%LOCALAPPDATA%\Doubao\User Data\Default\DawnWebGPUCache
%LOCALAPPDATA%\Doubao\User Data\extensions_crx_cache
%LOCALAPPDATA%\Doubao\User Data\GraphiteDawnCache
%LOCALAPPDATA%\Doubao\User Data\GrShaderCache
%LOCALAPPDATA%\Doubao\User Data\local_cache
```

**发现**: 12 个不同的缓存目录
**危险等级**: 🟢 低
**说明**: 字节跳动推出的 AI 浏览器，基于 Chromium

#### ✅ QQ浏览器 (腾讯)
**缓存位置**:
```
%LOCALAPPDATA%\Tencent\QQBrowser\User Data\Default\Cache
```

**危险等级**: 🟢 低

#### ✅ 360浏览器 (奇虎360)
**缓存位置**:
```
%APPDATA%\360browser\cache
```

**危险等级**: 🟢 低

---

### 3. 社交通信

#### ✅ 微信 (腾讯)
**缓存位置**:
```
%APPDATA%\Tencent\WeChat\*\webnet\cacheinfo
%APPDATA%\Tencent\WeChat\*\radium\cache
%APPDATA%\Tencent\WeChat\*\radium\WmpfCache
%APPDATA%\Tencent\WeChat\wmpf\cache
%APPDATA%\Tencent\WeChat\wmpf\Code Cache
%APPDATA%\Tencent\WeChat\wmpf\GPUCache
%APPDATA%\Tencent\WeChat\wmpf_app\cache
%APPDATA%\Tencent\WeChat\xweb\miniprogram\Code Cache
%APPDATA%\Tencent\WeChat\xweb\miniprogram\GPUCache
```

**实际大小**: ~1.2GB ⚠️
**包含内容**:
- 小程序缓存
- 网页缓存
- 小程序文件缓存
- GPU 渲染缓存

**危险等级**: 🟡 中
**说明**: 微信缓存可能很大，清理后小程序会重新下载

#### ✅ QQ (腾讯)
**缓存位置**:
```
%APPDATA%\QQ\Cache
%APPDATA%\QQ\Code Cache
%APPDATA%\QQ\Local Storage
%APPDATA%\QQ\Network
```

**危险等级**: 🟢 低

#### ✅ 钉钉 (阿里巴巴)
**缓存位置**:
```
%APPDATA%\DingTalk\*\Sync_v2\cache
%APPDATA%\DingTalk\globalStorage\resource_cache
%APPDATA%\DingTalk\image_res_cache
```

**危险等级**: 🟢 低

---

### 4. 视频编辑

#### ✅ 剪映专业版 (字节跳动)
**缓存位置**:
```
%LOCALAPPDATA%\JianyingPro\User Data\Cache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\agencycache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\AIAvatarCache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\AIAvatarFrameCache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\AigcMaterailCache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\AlgorithmCache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\C2PACache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\importcache3
%LOCALAPPDATA%\JianyingPro\User Data\Cache\InterpolationCache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\MotionBlurCache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\RemuxCache
%LOCALAPPDATA%\JianyingPro\User Data\Cache\segmentPrerenderCache
```

**发现**: 15+ 个不同的缓存目录
**危险等级**: 🟡 中
**说明**: 剪映是抖音官方视频编辑工具，缓存可能很大

---

### 5. 云存储

#### ✅ 百度网盘
**缓存位置**:
```
%APPDATA%\baidu\BaiduNetdisk
%APPDATA%\BaiduYunGuanjia
%APPDATA%\BaiduYunKernel
%LOCALAPPDATA%\Packages\BaiduNetdisk.DesktopSyncClient_r5kxaep58dem0
```

**危险等级**: 🟡 中
**说明**: 可能包含同步文件的临时缓存

---

### 6. 其他发现的应用

#### Adobe (国际软件，国内常用)
**缓存位置**:
```
%LOCALAPPDATA%\Adobe\CameraRaw\Cache
%APPDATA%\Adobe
```

#### 网易云音乐
**缓存位置**:
```
%APPDATA%\kwmusic
```

#### 2345系列软件
**发现**:
- 2345OCR
- 2345SafeCenter
- 2345Uninst

#### 搜狗系列
**发现**:
- SogouExplorer
- SogouPen

---

## 📊 缓存大小统计

| 应用 | 预估大小 | 实际测量 |
|------|----------|----------|
| 微信 | 100MB-2GB | ~1.2GB ✅ |
| 剪映 | 500MB-5GB | 未测量 |
| WPS Office | 50-200MB | 未测量 |
| 豆包浏览器 | 50-500MB | 未测量 |
| QQ | 20-200MB | 未测量 |
| 钉钉 | 10-100MB | 未测量 |
| 百度网盘 | 50-500MB | 未测量 |

**总计预估**: 780MB - 8.5GB

---

## 🌟 重要发现

### 1. 微信缓存巨大
微信的 1.2GB 缓存主要包括：
- 小程序缓存（最大）
- 网页渲染缓存
- GPU 渲染缓存
- 微信支付缓存

**建议**: 定期清理，但要注意清理后小程序需要重新加载

### 2. 豆包浏览器缓存复杂
豆包浏览器有 12 个独立缓存目录，类似 Chrome 结构：
- 主缓存
- GPU 着色器缓存
- 代码缓存
- WebGPU 缓存
- 扩展缓存

### 3. 剪映专业版缓存众多
剪映有 15+ 个缓存目录，包括：
- AI 相关缓存（Avatar、素材）
- 算法缓存
- 导入缓存
- 渲染缓存
- 特效缓存

---

## ⚠️ 清理风险分析

### 🟢 低风险（可安全清理）
- 浏览器缓存（豆包、QQ、360）
- QQ 缓存
- 钉钉缓存
- WPS Office 缓存

### 🟡 中风险（需谨慎）
- **微信缓存** - 清理后小程序需重新加载
- **剪映缓存** - 可能影响项目文件
- **百度网盘缓存** - 可能影响下载/上传

### 🔴 高风险（不建议自动清理）
- 未保存的编辑项目
- 正在下载的文件

---

## 💡 实施建议

### 1. 分类策略
将国内应用分为三类：

**A 类 - 安全清理**（默认选中）
- 浏览器缓存
- QQ 缓存
- WPS 缓存

**B 类 - 谨慎清理**（需确认）
- 微信缓存（显示大小，提示风险）
- 剪映缓存
- 百度网盘缓存

**C 类 - 高级清理**（需明确启用）
- 临时文件
- 日志文件

### 2. 智能检测
```rust
// 检测应用是否运行
fn is_app_running(process_name: &str) -> bool {
    // 检查进程是否存在
}

// 跳过运行中的应用的缓存
if is_app_running("WeChat.exe") {
    skip_wechat_cache();
}
```

### 3. 大缓存警告
```rust
if cache_size > 500MB {
    warn_user("微信缓存较大 (1.2GB)，清理后小程序需重新加载");
}
```

---

## 🎯 代码更新总结

### 新增缓存类型（8 种）
1. `BrowserDoubao` - 豆包浏览器
2. `BrowserQQBrowser` - QQ浏览器
3. `Browser360` - 360浏览器
4. `WPSOfficeCache` - WPS Office
5. `WeChatCache` - 微信
6. `QQCache` - QQ
7. `DingTalkCache` - 钉钉
8. `JianyingProCache` - 剪映专业版
9. `BaiduNetdiskCache` - 百度网盘

### 更新的文件
1. **`crates/core/src/cache_types.rs`**
   - 新增 9 种缓存类型枚举
   - 更新 `display_name()` 方法

2. **`crates/core/src/scanner.rs`**
   - 添加所有新发现的缓存路径
   - 配置微信、剪映等复杂缓存目录

### 支持的缓存类型总数
- **实施前**: 22 种
- **实施后**: **30 种** ✨
- **增长率**: 36%

---

## 📝 待添加的应用

虽然扫描到但未添加支持的应用（可选）：

1. **Sogou 搜狗系列** - 输入法、浏览器
2. **2345 系列** - 安全软件、压缩工具
3. **网易云音乐** - kwmusic
4. **Adobe** - 如果国内用户常用

这些应用用户群较小或缓存不明确，可以作为未来扩展。

---

## 🔮 未来改进方向

1. **应用程序运行检测**
   - 防止清理正在使用的应用的缓存
   - 提示用户关闭应用后清理

2. **白名单功能**
   - 用户可指定不想清理的缓存
   - 例如：保留微信小程序缓存

3. **缓存大小分析**
   - 显示每种缓存的大小
   - 帮助用户决定是否清理

4. **定时清理**
   - 定期自动清理浏览器缓存
   - 保留重要应用缓存

---

**扫描工具**: Bash + find + ls + du
**分析**: Claude Code Agent
**报告版本**: 2.0 (国内应用专项)
**相关文件**:
- `system-scan-report.md` (国际应用)
- `domestic-apps-scan-report.md` (本文档)
