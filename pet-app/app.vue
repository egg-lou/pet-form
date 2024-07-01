<script setup lang="ts">
import Toaster from '@/components/ui/toast/Toaster.vue';
useSeoMeta({
    title: 'Paws and Claws'
});
</script>

<template>
    <Toaster />
    <NuxtLayout>
        <NuxtPage />
    </NuxtLayout>
</template>
