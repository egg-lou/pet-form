<script setup lang="ts">
const { pet_id } = useRoute().params;
</script>

<template>
  <div class="main container">
     <Card>
        <div class="h-[85vh] p-10">
          {{ pet_id }}
        </div>
     </Card>
  </div>
</template>

<style scoped>

</style>