<script setup lang="ts">
import Navbar from '~/components/ui/navbar/Navbar.vue';
import Footer from '~/components/ui/footer/Footer.vue';
</script>

<template>
    <div>
        <Navbar />
        <slot />
        <Footer />
    </div>
</template>
