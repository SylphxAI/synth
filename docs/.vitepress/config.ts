import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Synth',
  description: "The world's fastest AST processor - 50-3000x faster than unified",

  // Ignore dead links for now (pages to be added)
  ignoreDeadLinks: true,

  head: [['link', { rel: 'icon', href: '/favicon.ico' }]],

  themeConfig: {
    logo: '/logo.svg',

    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'API', link: '/api/synth' },
      { text: 'Examples', link: '/examples/ast-chunking' },
      {
        text: 'Packages',
        items: [
          { text: '@sylphx/synth', link: '/api/synth' },
          { text: '@sylphx/synth-md', link: '/api/synth-md' },
          { text: '@sylphx/synth-js', link: '/api/synth-js' },
          { text: '@sylphx/synth-html', link: '/api/synth-html' },
          { text: '@sylphx/synth-json', link: '/api/synth-json' },
          { text: 'All Packages...', link: '/api/' },
        ],
      },
      {
        text: 'v0.1.2',
        items: [
          { text: 'Changelog', link: '/changelog' },
          { text: 'GitHub', link: 'https://github.com/SylphxAI/synth' },
          { text: 'npm', link: 'https://www.npmjs.com/org/sylphx' },
        ],
      },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Introduction',
          items: [
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'Why Synth?', link: '/guide/why-synth' },
          ],
        },
        {
          text: 'Core Concepts',
          items: [
            { text: 'Tree & Nodes', link: '/guide/core-concepts' },
            { text: 'Traversal', link: '/guide/traversal' },
            { text: 'Query Index', link: '/guide/querying' },
            { text: 'Plugins', link: '/guide/plugins' },
          ],
        },
        {
          text: 'Advanced',
          items: [
            { text: 'Incremental Parsing', link: '/guide/incremental' },
            { text: 'Performance', link: '/guide/performance' },
            { text: 'Architecture', link: '/guide/architecture' },
          ],
        },
      ],
      '/api/': [
        {
          text: 'Core',
          items: [{ text: '@sylphx/synth', link: '/api/synth' }],
        },
        {
          text: 'Parsers',
          items: [
            { text: '@sylphx/synth-md', link: '/api/synth-md' },
            { text: '@sylphx/synth-js', link: '/api/synth-js' },
            { text: '@sylphx/synth-html', link: '/api/synth-html' },
            { text: '@sylphx/synth-json', link: '/api/synth-json' },
            { text: '@sylphx/synth-yaml', link: '/api/synth-yaml' },
            { text: '@sylphx/synth-css', link: '/api/synth-css' },
            { text: '@sylphx/synth-sql', link: '/api/synth-sql' },
            { text: '@sylphx/synth-graphql', link: '/api/synth-graphql' },
          ],
        },
        {
          text: 'Tools',
          items: [
            { text: '@sylphx/synth-js-format', link: '/api/synth-js-format' },
            { text: '@sylphx/synth-js-minify', link: '/api/synth-js-minify' },
            { text: '@sylphx/synth-lint', link: '/api/synth-lint' },
            { text: '@sylphx/synth-metrics', link: '/api/synth-metrics' },
            { text: '@sylphx/synth-docs', link: '/api/synth-docs' },
          ],
        },
      ],
      '/examples/': [
        {
          text: 'Examples',
          items: [
            { text: 'AST Chunking', link: '/examples/ast-chunking' },
            { text: 'Code Analysis', link: '/examples/code-analysis' },
            { text: 'Linting', link: '/examples/linting' },
            { text: 'Transformation', link: '/examples/transformation' },
            { text: 'Multi-Language', link: '/examples/multi-language' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/SylphxAI/synth' },
      { icon: 'npm', link: 'https://www.npmjs.com/org/sylphx' },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024 SylphxAI',
    },

    search: {
      provider: 'local',
    },

    editLink: {
      pattern: 'https://github.com/SylphxAI/synth/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },
  },
})
