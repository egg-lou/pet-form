<script setup lang="ts">
import { OwnerService } from '~/api/owner';
import { PetService } from '~/api/pet';

const ownerService = new OwnerService();
const petService = new PetService();

const fetchOwners = async () => {
    const response = await ownerService.getOwners();
    console.log(response);
};

const fetchPets = async () => {
    const response = await petService.getPets();
    console.log(response);
};

onMounted(() => {
    fetchOwners();
    fetchPets();
});
</script>

<template>
    <div class="main container">
        <h1>Records</h1>
    </div>
</template>
