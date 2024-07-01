<script setup lang="ts">
  import Navbar from '~/components/ui/navbar/Navbar.vue';
  import Footer from '~/components/ui/footer/Footer.vue';
</script>

<template>
  <div>
    <Navbar />
    <div class="main container py-10">
      <slot />
    </div>
    <Footer />
  </div>
</template>

<style scoped>
.main {
    min-height: calc(100vh - 4rem - 3rem);
  }
</style>
