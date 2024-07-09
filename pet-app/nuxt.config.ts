// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  ssr: false,

  routeRules: {
      '/':{prerender:true}
  },

  runtimeConfig: {
      public: {
          apiUrl: process.env.NUXT_PUBLIC_API_URL
      }
  },

  modules: [
      '@nuxt/eslint',
      '@nuxtjs/tailwindcss',
      'shadcn-nuxt',
      '@nuxtjs/color-mode',
      '@nuxt/image',
      '@pinia/nuxt'
  ],

  colorMode: {
      classSuffix: ''
  },

  shadcn: {
      prefix: '',
      componentDir: './components/ui'
  },

  css: ['~/assets/css/main.css'],

  postcss: {
      plugins: {
          tailwindcss: {},
          autoprefixer: {}
      }
  },

  components: true,
  compatibilityDate: '2024-07-09'
});