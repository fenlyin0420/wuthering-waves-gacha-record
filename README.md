<div align="center">
    <img width="40%" src="https://github.com/user-attachments/assets/81fdd5b1-53db-4941-9ca2-996a1d9be284" />

# wuthering-waves-gacha-record
</div>
一个使用 Rust 制作的小工具，支持 Windows 10 (x64) 及以上的系统。

通过读取游戏目录下的日志文件，自动获取抽卡统计信息。

## 特性
- 同时支持国服 / 国际服
- 自动获取游戏目录
- 更新记录无需启动游戏
- 支持同时记录多个不同服务器的用户抽卡信息
- 主题切换
- 程序更新提醒

## 数据目录
程序运行时会在当前目录下生成`data`文件夹，用于缓存和存储用户数据及抽卡历史数据,每个用户一个文件夹

获取新数据时会将原数据备份，放在`data/{uid}/backup`文件夹下。

## 使用方式
首次启动：
1. 启动游戏
2. 进入抽卡记录页
3. 打开软件自动获取抽卡统计信息

追加最新的抽卡记录（无需登录游戏）：
1. 在`选择用户`下拉框选择需要追加记录的用户 UID
2. 点击`获取数据更新`

添加新用户：
1. 游戏中使用新用户打开抽卡记录页
2. 点击`获取新用户`按钮等待片刻

## 下载
[Github 下载](https://github.com/fenlyin0420/wuthering-waves-gacha-record/releases/tag/v0.9.0)

## 截图
![img](https://github.com/fenlyin0420/wuthering-waves-gacha-record/raw/master/images/light-theme.png)
![img](https://github.com/fenlyin0420/wuthering-waves-gacha-record/raw/master/images/dark-theme.png)

