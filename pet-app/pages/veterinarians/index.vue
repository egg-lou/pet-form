<script setup lang="ts">
import { VetService } from '~/api/vet';

const vetService = new VetService();

const fetchVets = async () => {
    const response = await vetService.getVets();
}

onMounted(async () => {
    await fetchVets();
});

</script>

<template>
    <div class="main">hello</div>
</template>
