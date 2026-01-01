# Excel Server 文档

本目录包含 Excel Server 的完整文档，使用 VitePress 构建。

## 开发

### 安装依赖

```bash
npm install
# 或
pnpm install
# 或
yarn install
```

### 本地开发

```bash
npm run docs:dev
```

访问 http://localhost:5173

### 构建静态文件

```bash
npm run docs:build
```

构建后的文件位于 `.vitepress/dist/` 目录。

### 预览构建结果

```bash
npm run docs:preview
```

## 目录结构

```
documentation/
├── .vitepress/
│   ├── config.mts          # VitePress 配置
│   └── dist/               # 构建输出目录
├── guide/                  # 指南文档
│   ├── getting-started.md
│   ├── installation.md
│   └── ...
├── api/                    # API 文档
│   ├── overview.md
│   ├── generate.md
│   └── ...
├── dsl/                    # DSL 规范文档
│   ├── overview.md
│   ├── styles.md
│   └── ...
├── public/                 # 静态资源
├── index.md               # 首页
└── package.json
```

## 部署

### 部署到 GitHub Pages

```bash
# 构建
npm run docs:build

# 部署到 gh-pages 分支
cd .vitepress/dist
git init
git add -A
git commit -m 'deploy'
git push -f git@github.com:yourusername/excel-server.git master:gh-pages
```

### 部署到 Vercel/Netlify

直接连接 GitHub 仓库，选择 `documentation` 目录作为根目录，构建命令为 `npm run docs:build`，输出目录为 `.vitepress/dist`。

## 贡献

欢迎改进文档！请：

1. Fork 项目
2. 创建特性分支
3. 提交变更
4. 推送到分支
5. 创建 Pull Request

