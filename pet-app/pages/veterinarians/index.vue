<script setup lang="ts">
import { VetService } from '~/api/vet';
import VetTable from '~/components/tables/vet-table.vue';

const vetService = new VetService();
const vets = ref([]);
const fetchVets = async () => {
    const response = await vetService.getVets();
    console.log(response.data.vets)
    vets.value = response.data.vets;
}

onMounted(async () => {
    await fetchVets();
});

</script>

<template>
    <div class="main">
        <Card>
            <div class="h-[85vh] p-10">
              <div class="flex items-center gap-4 py-3">
                <nuxt-img
                    src="/img/icon.png"
                    alt="Paws and Claws Vet Clinic"
                    width="50"
                    height="50" />
                <h3 class="text-xl font-semibold">Veterinarians</h3>
              </div>
               <VetTable :vets="vets"/>
            </div>
        </Card>
    </div>
</template>
