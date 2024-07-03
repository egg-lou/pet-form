<script setup lang="ts">
import { useRoute } from 'vue-router';
import { Stethoscope, Archive, House, Plus } from 'lucide-vue-next';

const route = useRoute();
</script>

<template>
    <aside
        class="flex h-full w-full flex-col items-center justify-start gap-8 px-8 py-8 shadow-md bg-white/60 dark:bg-black/60">
        <nuxt-link to="/">
            <Button variant="ghost" :class="{'active-link': route.path === '/'}">
                <House class="icon" />
            </Button>
        </nuxt-link>

        <nuxt-link to="/registration">
            <Button variant="ghost" :class="{'active-link': route.path === '/registration'}">
                <Plus class="icon" />
            </Button>
        </nuxt-link>

        <nuxt-link to="/records">
            <Button variant="ghost" :class="{'active-link': route.path === '/records'}">
                <Archive class="icon" />
            </Button>
        </nuxt-link>

        <nuxt-link to="/veterinarians">
            <Button variant="ghost" :class="{'active-link': route.path === '/veterinarians'}">
                <Stethoscope class="icon" />
            </Button>
        </nuxt-link>
    </aside>
</template>

<style scoped>
.icon {
    @apply h-5 w-5;
}

Button {
  @apply rounded-lg
}

.active-link {
    @apply bg-blue-400 transition-all duration-700 dark:bg-blue-500;
}
</style>
