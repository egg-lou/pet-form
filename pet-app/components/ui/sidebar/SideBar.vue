<script setup lang="ts">
import {Stethoscope, Archive, House, Plus} from "lucide-vue-next";
</script>

<template>
  <aside class="h-full border-r-2 w-full flex flex-col gap-8 items-center justify-start py-8 px-8 shadow-md">
    <nuxt-link to="/">
      <Button variant="ghost">
        <House  class="icon"/>
      </Button>
    </nuxt-link>

    <nuxt-link to="/registration" >
      <Button variant="ghost">
        <Plus class="icon"/>
      </Button>
    </nuxt-link>

    <nuxt-link to="/records" >
      <Button variant="ghost">
        <Archive class="icon"/>
      </Button>
    </nuxt-link>

    <nuxt-link to="/veterinarians" >
      <Button variant="ghost">
       <Stethoscope class="icon"/>
      </Button>
    </nuxt-link>


  </aside>
</template>

<style scoped>
.icon {
  @apply h-5 w-5;
}
</style>