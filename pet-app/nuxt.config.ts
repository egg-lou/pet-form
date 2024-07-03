// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    devtools: { enabled: true },
    runtimeConfig: {
        public: {
            apiUrl: 'http://localhost:4000'
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
    components: true
});
