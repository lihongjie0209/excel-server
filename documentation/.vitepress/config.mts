import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Excel Server",
  description: "基于 Rust + Axum 的 Excel 生成服务",
  base: '/docs/',
  lang: 'zh-CN',
  ignoreDeadLinks: true,
  
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    logo: '/logo.svg',
    
    nav: [
      { text: '首页', link: '/' },
      { text: '指南', link: '/guide/getting-started' },
      { text: 'API 文档', link: '/api/overview' },
      { text: 'DSL 规范', link: '/dsl/overview' },
      {
        text: 'v0.2.0',
        items: [
          { text: '更新日志', link: '/changelog' },
          { text: '贡献指南', link: '/contributing' }
        ]
      }
    ],

    sidebar: {
      '/guide/': [
        {
          text: '入门指南',
          items: [
            { text: '快速开始', link: '/guide/getting-started' },
            { text: '安装部署', link: '/guide/installation' },
            { text: '配置说明', link: '/guide/configuration' },
            { text: '使用示例', link: '/guide/examples' }
          ]
        },
        {
          text: '核心功能',
          items: [
            { text: '文件持久化', link: '/guide/persistence' },
            { text: 'GET 下载接口', link: '/guide/get-download' },
            { text: '性能优化', link: '/guide/performance' }
          ]
        }
      ],
      '/api/': [
        {
          text: 'API 接口',
          items: [
            { text: '接口概览', link: '/api/overview' },
            { text: '生成接口', link: '/api/generate' },
            { text: '下载接口', link: '/api/download' },
            { text: '系统接口', link: '/api/system' }
          ]
        },
        {
          text: '客户端示例',
          items: [
            { text: 'cURL', link: '/api/clients/curl' },
            { text: 'JavaScript', link: '/api/clients/javascript' },
            { text: 'React', link: '/api/clients/react' },
            { text: 'Vue', link: '/api/clients/vue' }
          ]
        }
      ],
      '/dsl/': [
        {
          text: 'DSL 规范',
          items: [
            { text: '规范概述', link: '/dsl/overview' },
            { text: '文档属性', link: '/dsl/document-properties' },
            { text: '样式定义', link: '/dsl/styles' },
            { text: '工作表', link: '/dsl/worksheet' },
            { text: '单元格', link: '/dsl/cells' },
            { text: '数据表格', link: '/dsl/tables' },
            { text: '数据验证', link: '/dsl/validation' },
            { text: '条件格式', link: '/dsl/conditional-format' }
          ]
        }
      ]
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/yourusername/excel-server' }
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026-present Excel Server'
    },

    search: {
      provider: 'local'
    },

    outline: {
      level: [2, 3],
      label: '本页目录'
    },

    docFooter: {
      prev: '上一页',
      next: '下一页'
    },

    lastUpdated: {
      text: '最后更新于',
      formatOptions: {
        dateStyle: 'short',
        timeStyle: 'short'
      }
    },

    darkModeSwitchLabel: '外观',
    sidebarMenuLabel: '菜单',
    returnToTopLabel: '返回顶部',
    langMenuLabel: '多语言'
  }
})
