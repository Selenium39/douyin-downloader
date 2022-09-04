# 抖音下载器

> 一个基于 Rust+Tauri+Vue 的抖音视频下载器

- 支持下载无水印视频
![image](https://user-images.githubusercontent.com/29670394/188313105-7458ada5-8a93-4935-9635-c20e91485979.png)

- 支持批量下载用户发布的所有视频(安装目录为Downloads目录)
![image](https://user-images.githubusercontent.com/29670394/188313576-3ed60a0a-6340-4f6b-a071-f64c504dbb06.png)

![image](https://user-images.githubusercontent.com/29670394/188315667-7d127fd8-b72a-44de-8252-08e2489136f3.png)


### 安装
```bash
npm install

# 调试
npm run tauri:serve

# 打包
npm run tauri:build
```

### 使用方法
- 单个视频链接
![image](https://user-images.githubusercontent.com/29670394/188315093-e2129df4-f614-4545-a88f-64aa7b8e6fca.png)

- 用户主页链接
![1628834372](https://user-images.githubusercontent.com/29670394/188315204-a1f4e7d2-32f6-449e-8e8e-41ef0176ca32.jpg)

### 说明
- 本项目仅供学习交流使用，不得用于商业用途
- 此项目仅在 Ubuntu 18.04 下测试通过，其他系统未测试(理论上支持跨平台)

### 参考
https://github.com/lecepin/douyin-downloader