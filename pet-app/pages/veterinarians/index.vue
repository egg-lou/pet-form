<script setup lang="ts">
import { VetService } from '~/api/vet';
import VetTable from '~/components/tables/vet-table.vue';
import VetForm from '~/components/form/vet-form.vue';
import { useRefetchStore } from '~/stores/refetch';
import { ChevronRight, ChevronLeft } from 'lucide-vue-next';

const vetService = new VetService();
const vets = ref([]);
const currentPage = ref(1);
const totalPage = ref(0);
const fetchVets = async () => {
    const response = await vetService.getVets(currentPage.value);
    vets.value = response.data.vets;
    totalPage.value = response.data.total_pages;
};

const refetch = useRefetchStore();
const nextPage = () => {
    currentPage.value++;
    fetchVets();
};

const prevtPage = () => {
    currentPage.value--;
    fetchVets();
};

watch(
    () => refetch.needRefetch,
    (newValue) => {
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
    <div class="main container">
        <Card>
            <div class="h-[80vh] p-10">
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
        <div class="flex items-center justify-center gap-4 py-4">
            <Button
                :disabled="currentPage === 1"
                @click="prevtPage"
                ><ChevronLeft class="icon"
            /></Button>
            <Button
                :disabled="currentPage === totalPage"
                @click="nextPage"
                ><ChevronRight class="icon"
            /></Button>
        </div>
    </div>
</template>
