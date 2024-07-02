<script setup lang="ts">
import Navbar from '~/components/ui/navbar/Navbar.vue';
import SideBar from "~/components/ui/sidebar/SideBar.vue";
</script>

<template>
    <div class="flex">
      <div class="w-[3.5rem] ">
        <SideBar />
      </div>
      <div class="w-full">
        <Navbar />
        <slot />
      </div>
    </div>
</template>
