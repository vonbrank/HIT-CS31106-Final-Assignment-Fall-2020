# HIT CS31106 Final Assignment Fall 2020

## 项目简介

**Contactify** 是一款控制台通讯录小程序，是哈工大2020秋《高级语言程序设计》课程大作业的重构版本。

[旧版本](https://github.com/vonbrank/HIT-CS31106-Final-Assignment-Fall-2020/tree/legacy-cpp-version) 使用 C++ 编写，架构较为混乱；现使用 Rust 进行重构，采用 MVC 架构模式。

![MVC Architecture](https://s1.ax1x.com/2023/04/07/ppT9A61.png)

> 图仅作 MVC 架构模式示例，项目并非 Web 应用

功能方面，允许用户创建多个电话簿，为每个电话簿创建多个联系人。创建的电话簿/联系人信息、应用设置等自动保存至 `%APPDATA%/Contactify` 下。

电话簿、联系人信息的填写支持几种表单验证功能：
- 必填字段判空
- 电话号码合法性检测
- 电话簿、联系人名称重复性检测

渲染方面，项目内部实现了一个简单的排版渲染器，可以将数据渲染为 ASCII 字符排版，显示在终端中，支持：
- 文字居左、居中、居右对齐
- 列表渲染与选中项目高亮
- 页面切换与状态缓存

![Contactify Demo](https://vonbrank-images.oss-cn-hangzhou.aliyuncs.com/20231206-Contactify/contactify-demo.gif)

键位映射：
- `↑` `↓` 切换列表选中条目
- `←` `→` 更改条目的值（如果条目是 key-value 形式的话）
- `Enter` 进入选中条目/确认表单
- `Esc` 退出当前页面