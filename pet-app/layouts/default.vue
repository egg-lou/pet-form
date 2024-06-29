<script setup lang="ts">
import Navbar from '~/components/ui/navbar/Navbar.vue';
import Footer from '~/components/ui/footer/Footer.vue';
</script>

<template>
    <Navbar />
    <div class="h-lvh">
        <slot />
    </div>
    <Footer />
</template>

<style scoped></style>
