# Dioxus Gloo Web Worker Example

[Chinese / 中文](#dioxus-gloo-web-worker-示例)

This is a minimal example of a Dioxus app using [gloo-worker](https://crates.io/crates/gloo-worker). It demonstrates how to configure a Dioxus project to support WebAssembly using a Gloo Web Worker.

This example uses the oneshot macro from gloo-worker to create a Worker instance. Other methods of creating a Gloo Worker can be set up in a similar way. This example is based on Dioxus 0.6.

## Note

You need to modify the project name in the assets/worker.js file to match your project's name. If the register function has been renamed in the src files, then the register function in the worker.js file should also be modified accordingly.

This worker.js shim script is placed in the assets folder. It is used to load the Web Worker and should be referenced in the src files when creating a worker using gloo-worker.

## Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

# dioxus gloo Web Worker 示例

这是一个在 Dioxus 中使用 [gloo-worker](https://crates.io/crates/gloo-worker) 的最小示例。它演示了如何在 Dioxus 项目中配置支持 WebAssembly 的 Gloo Web Worker。

此示例使用了 gloo-worker 的 oneshot 宏来创建 Worker 实例。其他创建 Gloo Worker 的方法也可以采用类似的方式来设置。该示例基于 Dioxus 0.6。

## 注意

你需要修改 assets/worker.js 文件中的项目名称为你的项目名称。如果在 src 文件中重命名了 register 函数，那么 worker.js 文件中的 register 函数也应相应修改。

这个 worker.js 垫层脚本放置在了 assets 文件夹中，用于加载 Web Worker。在 src 文件中使用 gloo-worker 创建 worker 时，应引用该脚本。

## 启动应用

在项目根目录下运行以下命令，以默认平台开始开发：

```bash
dx serve
```
