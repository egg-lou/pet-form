<script setup lang="ts">
import { VetService } from '~/api/vet';
import VetTable from '~/components/tables/vet-table.vue';
import { PlusIcon } from 'lucide-vue-next';
import { Dialog, DialogContent, DialogTrigger } from '~/components/ui/dialog';
import VetForm from '~/components/form/vet-form.vue';

const vetService = new VetService();
const vets = ref([]);
const fetchVets = async () => {
    const response = await vetService.getVets();
    vets.value = response.data.vets;
};
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
                    <Dialog>
                        <DialogTrigger as-child>
                            <div class="flex items-center">
                                <Button
                                    variant="outline"
                                    class="rounded-lg bg-blue-500 text-accent-foreground transition-all duration-300 hover:bg-blue-600">
                                    <PlusIcon class="h-6 w-6" />
                                </Button>
                            </div>
                        </DialogTrigger>
                        <DialogContent>
                            <VetForm
                                :mode="'add'"
                                @form-submitted="fetchVets" />
                        </DialogContent>
                    </Dialog>
                </div>
                <VetTable
                    :vets="vets"
                    @delete-vet="fetchVets" />
            </div>
        </Card>
    </div>
</template>
