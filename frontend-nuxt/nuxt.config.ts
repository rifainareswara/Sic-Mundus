// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false, // SPA mode for now
  modules: [
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt',
  ],
  css: ['~/assets/css/main.css'],
  devtools: { enabled: true },
  telemetry: false,
  runtimeConfig: {
    apiProxyTarget: 'http://localhost:8006', // Default to local dev
  },
  nitro: {
    // routeRules removed, using server/api/[...].js
  },
  app: {
    head: {
      title: 'Sic Mundus Creatus Est',
      htmlAttrs: {
        lang: 'en',
        class: 'dark',
      },
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { hid: 'description', name: 'description', content: 'FAI — Smart time tracking for productive teams' },
      ],
      link: [
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap' },
      ]
    }
  }
})
