<script setup lang="ts">
import { VetService } from '~/api/vet';
import VetTable from '~/components/tables/vet-table.vue';
import VetForm from '~/components/form/vet-form.vue';
import { useRefetchStore } from '~/stores/refetch';

const vetService = new VetService();
const vets = ref([]);
const fetchVets = async () => {
    const response = await vetService.getVets();
    vets.value = response.data.vets;
};

const refetch = useRefetchStore();

watch(
    () => refetch.needRefetch,
    (newValue, oldValue) => {
        if (newValue) {
            fetchVets();
            refetch.needRefetch = false;
        }
    }
);
onMounted(async () => {
    await fetchVets();
});
</script>

<template>
    <div class="main">
        <Card>
            <div class="h-[85vh] p-10">
                <div class="flex items-center justify-between px-3 py-3">
                    <div class="flex items-center gap-4">
                        <nuxt-img
                            src="/img/icon.png"
                            alt="Paws and Claws Vet Clinic"
                            width="50"
                            height="50" />
                        <h3 class="text-xl font-semibold">Veterinarians</h3>
                    </div>
                    <VetForm :mode="'add'" />
                </div>
                <VetTable
                    :vets="vets"
                    @delete-vet="fetchVets" />
            </div>
        </Card>
    </div>
</template>
