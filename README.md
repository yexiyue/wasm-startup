### wasm-startup

**一个用于快速启动rust wasm的脚手架工具**

别名原神启动



#### 用法

#### 1.安装

```bash
cargo install wasm-startup
```

#### 2.使用

**创建rust wasm项目**

```bash
wasm-startup new
```

**创建rust 并且带vite 原生ts 实时测试环境**

```bash
wasm-startup new -v
```

**同时可以使用-n 参数命名vite项目**

```bash
wasm-startup new -v -n test
```

**打包项目成wasm**

```bash
wasm-startup build
#等价于
wasm-pack build -t web
```

