<script setup lang="ts">
import { useRoute } from 'vue-router';
import { ServiceInstanceService } from '~/api/service-instance';
import { PetService } from '~/api/pet';
import type { ServiceInstance } from '~/types/service-type';
import type { Pet } from '~/types/pet-type';

const { service_instance_id } = useRoute().params;
const petService = new PetService();
const serviceInstanceService = new ServiceInstanceService();
const pet_id = ref('');
const service = ref<ServiceInstance | null>(null);
const pet = ref<Pet | null>(null);
const formatDate = (dateString) => {
    if (!dateString) return 'N/A';
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
};
const getServiceInstance = async () => {
    const response =
        await serviceInstanceService.getServiceInstance(service_instance_id);
    service.value = response.data;
    console.log(response.data.surgery);
    await getPet(response.data.pet_id);
};

const getPet = async (pet_id) => {
    const response = await petService.getPet(pet_id);
    pet.value = response.data.pet;
};

onMounted(() => {
    getServiceInstance();
});
</script>

<template>
    <div class="main px-10">
        <Card>
            <div class="flex h-[88vh] flex-col gap-3 p-5">
                <div class="flex w-full items-center justify-between px-4 py-2">
                    <h3 class="text-md font-semibold">
                        Service ID: {{ service?.service_instance_id }}
                    </h3>
                    <div class="flex items-center gap-3">
                        Edit Form:
                        <Switch />
                    </div>
                </div>
                <div class="flex w-full gap-4">
                    <div class="w-full border-2 p-3">
                        <h4 class="text-lg font-semibold">
                            Service Instance Information:
                        </h4>
                        <div>
                            <p>
                                <span class="font-semibold">Type:</span>
                                {{ service?.service_type.join(', ') }}
                            </p>
                            <p>
                                <span class="font-semibold">Date:</span>
                                {{ formatDate(service?.service_date) }}
                            </p>
                            <p>
                                <span class="font-semibold"
                                    >General Diagnosis:</span
                                >
                                {{ service?.general_diagnosis }}
                            </p>
                            <p>
                                <span class="font-semibold">Reason:</span>
                                {{ service?.service_reason }}
                            </p>
                            <p>
                                <span class="font-semibold"
                                    >Requires Follow Up:</span
                                >
                                {{ service?.requires_followup }}
                            </p>
                            <p>
                                <span class="font-semibold"
                                    >Follow Up Date:</span
                                >
                                {{ formatDate(service?.followup_date) }}
                            </p>
                        </div>
                    </div>
                    <div class="w-full border-2 p-3">
                        <h4 class="text-lg font-semibold">Pet Information:</h4>
                        <div>
                            <p>
                                <span class="font-semibold">Name:</span>
                                {{ pet?.pet_name }}
                            </p>
                            <p>
                                <span class="font-semibold">Type:</span>
                                {{ pet?.pet_type }}
                            </p>
                            <p>
                                <span class="font-semibold">Breed:</span
                                >{{ pet?.pet_breed }}
                            </p>
                            <p>
                                <span class="font-semibold">Owner Name:</span>
                                {{ pet?.owner_name }}
                            </p>
                        </div>
                    </div>
                </div>
                <div
                    class="grid h-[40rem] w-full grid-cols-3 gap-3"
                    style="grid-template-columns: 3fr 1fr 1fr">
                    <div class="wrap p-3">
                        <h5 class="text-md font-semibold">Surgery:</h5>
                        <ScrollArea class="h-[20rem]">
                            <div
                                v-for="surgery in service?.surgery"
                                :key="surgery.surgery_id">
                                <div>
                                    <div>Name: {{ surgery.surgery_name }}</div>
                                    <div>
                                        Veterinarian Diagnosis:
                                        {{ surgery.veterinarian_diagnosis }}
                                    </div>
                                    <div>
                                        Anesthesia Used:
                                        {{ surgery.anesthesia_used }}
                                    </div>
                                    <div>
                                        Complications:
                                        {{ surgery.complications }}
                                    </div>
                                    <div>Name: {{ surgery.outcome }}</div>
                                    <div>
                                        Performed by: {{ surgery.vet.vet_name }}
                                    </div>
                                </div>
                            </div>
                        </ScrollArea>
                    </div>
                    <div class="wrap p-3">
                        <h5 class="text-md font-semibold">Preventive Care:</h5>
                        <ScrollArea class="h-[20rem]">
                            <div
                                v-for="preventive in service?.preventive_care"
                                :key="preventive.preventive_care_id"
                                class="py-2">
                                <div>
                                    <div
                                        class="flex items-center justify-center gap-2 border-2 p-3">
                                        <div
                                            class="flex w-full flex-col items-start justify-center">
                                            <p>{{ preventive.treatment }}</p>
                                            <p>
                                                by:
                                                {{ preventive.vet.vet_name }}
                                            </p>
                                        </div>
                                        <Button variant="destructive"
                                            >Delete</Button
                                        >
                                    </div>
                                </div>
                            </div>
                            <Button
                                variant="outline"
                                class="w-full dark:text-accent-foreground"
                                >Add</Button
                            >
                        </ScrollArea>
                    </div>
                    <div class="wrap p-3">
                        <h5 class="text-md font-semibold">Grooming:</h5>
                        <ScrollArea class="h-[20rem]">
                            <div
                                v-for="groom in service?.grooming"
                                :key="groom.grooming_id"
                                class="py-2">
                                <div
                                    class="flex items-center justify-center gap-2 border-2 p-3">
                                    <div
                                        class="flex w-full items-center justify-start">
                                        {{ groom.grooming_type }}
                                    </div>
                                    <Button variant="destructive"
                                        >Delete</Button
                                    >
                                </div>
                            </div>
                            <Button
                                variant="outline"
                                class="w-full dark:text-accent-foreground"
                                >Add</Button
                            >
                        </ScrollArea>
                    </div>
                </div>
            </div>
        </Card>
    </div>
</template>

<style scoped>
.wrap {
    @apply border-2 border-dashed;
}
</style>
