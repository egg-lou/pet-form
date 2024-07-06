<script setup lang="ts">
import { OwnerService } from '~/api/owner';
import { PetService } from '~/api/pet';
import { ChevronLeft, ChevronRight, Search } from 'lucide-vue-next';
import PetTable from '~/components/tables/pet-table.vue';
import OwnerTable from '~/components/tables/owner-table.vue';
import OwnerForm from "~/components/form/owner-form.vue";

const ownerService = new OwnerService();
const petService = new PetService();
const activeTab = ref('owners');
const owners = ref([]);
const pets = ref([]);
const search = ref('');
const currentPage = ref(1);
const totalPage = ref(0);

const fetchOwners = async (searchQuery = '', pageNumber = 1) => {
    const response = await ownerService.getOwners(searchQuery, pageNumber);
    owners.value = response.data.owners;
    totalPage.value = response.data.total_pages;
};

const fetchPets = async (searchQuery = '', pageNumber = 1) => {
    const response = await petService.getPets(searchQuery, pageNumber);
    pets.value = response.data.pets;
};

const fetchData = async () => {
    if (activeTab.value === 'owners') {
        await fetchOwners(search.value, currentPage.value);
    } else {
        await fetchPets(search.value, currentPage.value);
    }
};

const nextPage = () => {
    currentPage.value++;
    fetchData();
};

const prevPage = () => {
    currentPage.value--;
    fetchData();
};
const fetch = useRefetchStore();
watch(activeTab, fetchData);

watch(() => fetch.needRefetch, (newValue) => {
    if (newValue) {
        fetchData();
        fetch.needRefetch = false;
    }
})

onMounted(async () => {
    await fetchData();
});
</script>

<template>
    <div class="main container">
        <Card>
            <div class="h-[80vh] p-10">
                <div class="mb-2 flex items-center justify-between px-3 py-3">
                  <div class="flex items-center justify-between w-full">
                    <div class="flex items-center gap-4">
                        <nuxt-img
                            src="/img/icon.png"
                            alt="Paws and Claws Vet Clinic"
                            width="50"
                            height="50" />
                        <h3
                            v-if="activeTab === 'owners'"
                            class="text-xl font-semibold">
                        Owners
                        </h3>
                        <h3
                            v-else
                            class="text-xl font-semibold">
                            Pets
                        </h3>
                    </div>
                    <OwnerForm v-if="activeTab === 'owners'"/>
                    </div>
                </div>
                <Tabs
                    v-model="activeTab"
                    default-value="owners">
                    <div class="flex items-center gap-3">
                        <TabsList class="w-1/4 shadow-md">
                            <TabsTrigger
                                value="owners"
                                class="w-full">
                                Owners
                            </TabsTrigger>
                            <TabsTrigger
                                value="pets"
                                class="w-full">
                                Pets
                            </TabsTrigger>
                        </TabsList>
                        <div
                            class="relative flex w-3/4 items-center justify-start shadow-md">
                            <Search class="absolute left-2 h-5 w-5" />
                            <Input
                                v-model="search"
                                class="pl-8"
                                placeholder="Search"
                                type="search"
                                @input="fetchData" />
                        </div>
                    </div>
                    <TabsContent
                        value="owners"
                        class="relative w-full py-5">
                        <OwnerTable :owners="owners" />
                    </TabsContent>
                    <TabsContent
                        value="pets"
                        class="h-[100%] w-full py-5">
                        <PetTable :pets="pets" />
                    </TabsContent>
                </Tabs>
            </div>
        </Card>
        <div class="flex items-center justify-center gap-4 py-4">
            <Button
                :disabled="currentPage === 1"
                @click="prevPage"
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
