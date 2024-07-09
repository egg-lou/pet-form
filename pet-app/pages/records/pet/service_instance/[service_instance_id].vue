<script setup lang="ts">
import { useRoute } from 'vue-router';
import { ServiceInstanceService } from '~/api/service-instance';
import { PetService } from '~/api/pet';
import type { ServiceInstanceType } from '~/types/service-type';
import type { Pet } from '~/types/pet-type';
import UpdateServiceInstanceForm from '~/components/form/update-service-instance-form.vue';
import UpdateSurgeryForm from '~/components/form/update-surgery-form.vue';
import AddGroomingForm from '~/components/form/add-grooming-form.vue';
import AddPreventiveForm from '~/components/form/add-preventive-form.vue';
import { toast } from '~/components/ui/toast';

const { service_instance_id } = useRoute().params;
const fetch = useRefetchStore();
const petService = new PetService();
const serviceInstanceService = new ServiceInstanceService();
const service = ref<ServiceInstanceType | null>(null);
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
    await getPet(response.data.pet_id);
};

const getPet = async (pet_id) => {
    const response = await petService.getPet(pet_id);
    pet.value = response.data.pet;
};

const isEditModeEnabled = ref(false);

const handleEdit = () => {
    isEditModeEnabled.value = !isEditModeEnabled.value;
};

const deleteGrooming = async (grooming_id: number) => {
    await serviceInstanceService
        .deleteGrooming(grooming_id)
        .then(() => {
            toast({
                title: '✅ Grooming deleted successfully',
                description: 'Grooming has been deleted successfully'
            });
            getServiceInstance();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description:
                    'An error occurred while deleting grooming.' + error.message
            });
        });
};

const deletePreventive = async (preventive_id: number) => {
    await serviceInstanceService
        .deletePreventiveCare(preventive_id)
        .then(() => {
            toast({
                title: '✅ Preventive Care deleted successfully',
                description: 'Preventive Care has been deleted successfully'
            });
            getServiceInstance();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description:
                    'An error occurred while deleting preventive care.' +
                    error.message
            });
        });
};
onMounted(() => {
    getServiceInstance();
});

watch(
    () => fetch.needRefetch,
    (newValue) => {
        if (newValue) {
            getServiceInstance();
            fetch.needRefetch = false;
        }
    }
);
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
                        <Switch @click="handleEdit" />
                    </div>
                </div>
                <div class="flex w-full gap-4">
                    <div class="w-full border-2 p-3">
                        <div class="flex justify-between">
                            <h4 class="text-lg font-semibold">
                                Service Instance Information:
                            </h4>
                            <div v-if="isEditModeEnabled">
                                <UpdateServiceInstanceForm />
                            </div>
                        </div>
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
                                {{ service?.requires_followup ? 'Yes' : 'No' }}
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
                    class="grid h-[35rem] w-full grid-cols-3 gap-3"
                    style="grid-template-columns: 3fr 1fr 1fr">
                    <div class="wrap p-3">
                        <div class="flex items-center justify-between p-2">
                            <h5 class="text-md font-semibold">Surgery:</h5>
                            <div
                                v-if="isEditModeEnabled"
                                class="flex gap-3">
                                <UpdateSurgeryForm />
                            </div>
                        </div>
                        <ScrollArea class="h-[28rem]">
                            <div
                                v-for="surgery in service?.surgery"
                                :key="surgery.surgery_id">
                                <div class="flex flex-col gap-2">
                                    <div class="w-full border-2 p-2">
                                        Name: {{ surgery.surgery_name }}
                                    </div>
                                    <div class="w-full border-2 p-2">
                                        Anesthesia Used:
                                        {{ surgery.anesthesia_used }}
                                    </div>
                                    <div class="flex h-[15rem] w-full gap-3">
                                        <div class="w-full border-2 p-2">
                                            Veterinarian Diagnosis:
                                            {{ surgery.veterinarian_diagnosis }}
                                        </div>
                                        <div class="w-full border-2 p-2">
                                            Complications:
                                            {{ surgery.complications }}
                                        </div>
                                    </div>
                                    <div class="w-full border-2 p-2">
                                        Outcome: {{ surgery.outcome }}
                                    </div>
                                    <div class="w-full border-2 p-2">
                                        Performed by: {{ surgery.vet.vet_name }}
                                    </div>
                                </div>
                            </div>
                        </ScrollArea>
                    </div>
                    <div class="wrap p-3">
                        <h5 class="text-md font-semibold">Preventive Care:</h5>
                        <ScrollArea class="h-[32rem]">
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
                                        <AlertDialog v-if="isEditModeEnabled">
                                            <AlertDialogTrigger>
                                                <Button variant="destructive"
                                                    >Delete</Button
                                                >
                                                <AlertDialogContent>
                                                    <AlertDialogHeader>
                                                        <AlertDialogTitle>
                                                            Are you sure you
                                                            want to delete this
                                                            item?
                                                        </AlertDialogTitle>
                                                        <AlertDialogDescription>
                                                            This action cannot
                                                            be undone.
                                                        </AlertDialogDescription>
                                                    </AlertDialogHeader>
                                                    <AlertDialogFooter>
                                                        <AlertDialogCancel>
                                                            Cancel
                                                        </AlertDialogCancel>
                                                        <AlertDialogAction
                                                            class="dark:text-accent-foreground"
                                                            @click="
                                                                deletePreventive(
                                                                    preventive.preventive_care_id
                                                                )
                                                            ">
                                                            Continue
                                                        </AlertDialogAction>
                                                    </AlertDialogFooter>
                                                </AlertDialogContent>
                                            </AlertDialogTrigger>
                                        </AlertDialog>
                                    </div>
                                </div>
                            </div>
                            <div v-if="isEditModeEnabled">
                                <AddPreventiveForm :pet_type="pet?.pet_type" />
                            </div>
                        </ScrollArea>
                    </div>
                    <div class="wrap p-3">
                        <h5 class="text-md font-semibold">Grooming:</h5>
                        <ScrollArea class="h-[32rem]">
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
                                    <AlertDialog v-if="isEditModeEnabled">
                                        <AlertDialogTrigger>
                                            <Button variant="destructive"
                                                >Delete</Button
                                            >
                                            <AlertDialogContent>
                                                <AlertDialogHeader>
                                                    <AlertDialogTitle>
                                                        Are you sure you want to
                                                        delete this item?
                                                    </AlertDialogTitle>
                                                    <AlertDialogDescription>
                                                        This action cannot be
                                                        undone.
                                                    </AlertDialogDescription>
                                                </AlertDialogHeader>
                                                <AlertDialogFooter>
                                                    <AlertDialogCancel>
                                                        Cancel
                                                    </AlertDialogCancel>
                                                    <AlertDialogAction
                                                        class="dark:text-accent-foreground"
                                                        @click="
                                                            deleteGrooming(
                                                                groom.grooming_id
                                                            )
                                                        ">
                                                        Continue
                                                    </AlertDialogAction>
                                                </AlertDialogFooter>
                                            </AlertDialogContent>
                                        </AlertDialogTrigger>
                                    </AlertDialog>
                                </div>
                            </div>
                            <div v-if="isEditModeEnabled">
                                <AddGroomingForm />
                            </div>
                        </ScrollArea>
                    </div>
                </div>
            </div>
        </Card>
    </div>
</template>

<style scoped>
.wrap {
    @apply border-2;
}
</style>
