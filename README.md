# HIT CS31106 Final Assignment Fall 2020

## 项目简介

**Contactify** 是一款控制台通讯录小程序，是哈工大2020秋《高级语言程序设计》课程大作业的重构版本。

旧版本使用 C++ 编写，架构较为混乱；现使用 Rust 进行重构，采用 MVC 架构模式。

![MVC Architecture](https://s1.ax1x.com/2023/04/07/ppT9A61.png)

项目内部实现了一个简单的排版渲染器，可以将 Model 的数据渲染为 ascii 字符排版，显示在终端中。

```txt
+---------------------+
| Contactify          |
|---------------------|
|+ New contacts book +|
| Contacts books list |
|      Settings       |
|        About        |
|                     |
|                     |
|                     |
|                     |
|                     |
|                     |
|   © VonBrank 2023   |
+---------------------+
```
