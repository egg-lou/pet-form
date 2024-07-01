<script setup lang="ts">
import ThemeSwitcher from '~/components/ui/theme-switcher/ThemeSwitcher.vue';
</script>

<template>
    <nav
        class="sticky top-0 flex h-16 w-full items-center justify-between border-b-2 bg-transparent px-6 backdrop-blur md:px-16 dark:border-slate-700 border-slate-400">
        <div class="flex items-center gap-4">
            <nuxt-img
                src="/img/icon.png"
                alt="Paws and Claws"
                class="h-8 w-8" />
            <h1 class="text-2xl font-semibold">Paws and Claws</h1>
        </div>
        <div>
            <ThemeSwitcher />
        </div>
    </nav>
</template>

<style scoped></style>
