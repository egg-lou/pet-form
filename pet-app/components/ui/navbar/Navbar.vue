<script setup lang="ts">
import ThemeSwitcher from '~/components/ui/theme-switcher/ThemeSwitcher.vue';
</script>

<template>
    <nav
        class="sticky top-0 flex h-16 w-full items-center justify-between px-8 py-12">
        <div class="flex items-center gap-4">
            <img
                src="/img/icon.png"
                alt="Paws and Claws"
                class="h-8 w-8" >
            <h1 class="text-lg font-semibold">Paws and Claws</h1>
        </div>
        <div class="flex items-center gap-4">
            <ThemeSwitcher />
        </div>
    </nav>
</template>

<style scoped></style>
