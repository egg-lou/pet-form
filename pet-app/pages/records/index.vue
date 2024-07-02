<script setup lang="ts">
import { OwnerService } from '~/api/owner';
import { PetService } from '~/api/pet';
import { Search } from 'lucide-vue-next';
import PetTable from '~/components/tables/pet-table.vue';
import OwnerTable from '~/components/tables/owner-table.vue';

const ownerService = new OwnerService();
const petService = new PetService();
const activeTab = ref('owners');
const owners = ref([]);
const pets = ref([]);

const fetchOwners = async () => {
    const response = await ownerService.getOwners();
    owners.value = response.data.owners;
};

const fetchPets = async () => {
    const response = await petService.getPets();
    pets.value = response.data.pets;
};

const fetchData = async () => {
    if (activeTab.value === 'owners') {
        await fetchOwners();
    } else {
        await fetchPets();
    }
};

watch(activeTab, fetchData);

onMounted(async () => {
    await fetchData();
});
</script>

<template>
    <div class="main">
        <Card>
            <div class="h-[85vh] p-10">
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
                                class="pl-8"
                                placeholder="Search" />
                        </div>
                    </div>
                    <TabsContent
                        value="owners"
                        class="h-[100%] w-full py-5">
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
    </div>
</template>
