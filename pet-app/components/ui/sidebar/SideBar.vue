<script setup lang="ts">
import { Stethoscope, Archive, House, Plus } from 'lucide-vue-next';
</script>

<template>
    <aside
        class="flex h-full w-full flex-col items-center justify-start gap-8 border-r-2 px-8 py-8 shadow-md">
        <nuxt-link to="/">
            <Button variant="ghost">
                <House class="icon" />
            </Button>
        </nuxt-link>

        <nuxt-link to="/registration">
            <Button variant="ghost">
                <Plus class="icon" />
            </Button>
        </nuxt-link>

        <nuxt-link to="/records">
            <Button variant="ghost">
                <Archive class="icon" />
            </Button>
        </nuxt-link>

        <nuxt-link to="/veterinarians">
            <Button variant="ghost">
                <Stethoscope class="icon" />
            </Button>
        </nuxt-link>
    </aside>
</template>

<style scoped>
.icon {
    @apply h-5 w-5;
}
</style>
