<script setup lang="ts">
import { ServiceInstanceService } from '~/api/service-instance';
import { PetService } from '~/api/pet';
import type { Pet } from '~/types/pet-type';
import { DateFormatter } from '@internationalized/date';
import type { ServiceContext } from '~/types/service-type';
import DateRange from '~/components/calendars/date-range.vue';
import { useDateRangeStore } from '~/stores/dateRange';
import { toast } from '~/components/ui/toast';
import UpdatePetForm from '~/components/form/update-pet-form.vue';
const petService = new PetService();
const serviceInstance = new ServiceInstanceService();
const { pet_id } = useRoute().params;

const fetch = useRefetchStore();
const pet = ref<Pet | null>(null);
const serviceInstances = ref<ServiceContext[]>([]);

const dateRange = ref({
    start_date: '',
    end_date: ''
});
const fetchPet = async () => {
    const response = await petService.getPet(pet_id);
    pet.value = response.data.pet;
};
const fetchServiceInstances = async () => {
    const response = await serviceInstance.getPetHistories(
        pet_id,
        dateRange.value.start_date,
        dateRange.value.end_date
    );
    serviceInstances.value = response.data.service_instances;
};

const headers = ref<string[]>([
    'ID',
    'Types',
    'Date',
    'Requires Follow Up',
    'Follow Up Date',
    'Actions'
]);

const df = new DateFormatter('en-US', {
    dateStyle: 'long'
});
const dateStore = useDateRangeStore();
onMounted(() => {
    fetchServiceInstances();
    fetchPet();
});
watch(
    () => [dateStore.start, dateStore.end],
    async ([newStart, newEnd]) => {
        dateRange.value = {
            start_date: newStart,
            end_date: newEnd
        };

        await fetchServiceInstances();
    },
    { deep: true }
);
watch(
    () => fetch.needRefetch,
    (newValue) => {
        if (newValue) {
            fetchPet();
            fetchServiceInstances();
            fetch.needRefetch = false;
        }
    }
);

const handleDelete = async (service_instance_id: string) => {
    await serviceInstance
        .deleteServiceInstance(service_instance_id)
        .then(() => {
            toast({
                title: '✅ Deleted Successfully',
                description: 'Record has been deleted successfully'
            });
            fetchServiceInstances();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description: 'Failed to delete record' + error.message
            });
        });
};
</script>

<template>
    <div class="main px-10 md:px-20">
        <Card>
            <div class="flex h-[87vh] w-full flex-col gap-5 p-10">
                <div class="flex flex-col gap-4 border-2 p-5">
                    <div class="flex items-center justify-between">
                        <h3 class="text-xl font-semibold">Pet Information</h3>
                        <UpdatePetForm />
                    </div>

                    <div class="flex gap-10 px-10 py-3">
                        <div class="flex flex-col gap-4">
                            <p>
                                <span class="font-semibold">ID:</span>
                                {{ pet?.pet_id }}
                            </p>
                            <p>
                                <span class="font-semibold">Name:</span>
                                {{ pet?.pet_name }}
                            </p>
                        </div>
                        <div class="flex flex-col gap-4">
                            <p>
                                <span class="font-semibold">Type:</span>
                                {{ pet?.pet_type }}
                            </p>
                            <p>
                                <span class="font-semibold">Breed:</span>
                                {{ pet?.pet_breed }}
                            </p>
                        </div>

                        <div class="flex flex-col gap-4">
                            <p>
                                <span class="font-semibold">Color:</span>
                                {{ pet?.pet_color }}
                            </p>
                            <p>
                                <span class="font-semibold">Weight:</span>
                                {{ pet?.pet_weight }}
                            </p>
                        </div>
                        <div class="flex flex-col gap-4">
                            <p>
                                <span class="font-semibold">Birth Date:</span>
                                {{
                                    pet?.pet_birth_date
                                        ? df.format(
                                              new Date(pet?.pet_birth_date)
                                          )
                                        : 'N/A'
                                }}
                            </p>
                            <p>
                                <span class="font-semibold">Owner:</span>
                                {{ pet?.owner_name }}
                            </p>
                        </div>
                    </div>
                </div>
                <div class="border-2 p-5">
                    <div class="flex items-center justify-between">
                        <h3 class="text-xl font-semibold">Histories:</h3>
                        <div class="flex items-center justify-center gap-4">
                            <DateRange v-model="dateRange" />
                            <nuxt-link
                                :to="`/records/pet/add_service/${pet_id}`">
                                <Button class="dark:text-accent-foreground"
                                    >Add Record</Button
                                >
                            </nuxt-link>
                        </div>
                    </div>
                    <ScrollArea class="h-[30rem] w-full px-10 py-1">
                        <h3
                            v-if="serviceInstances.length === 0"
                            class="text-center">
                            No records found.
                        </h3>
                        <Table v-if="serviceInstances.length > 0">
                            <TableHeader>
                                <TableRow>
                                    <TableHead
                                        v-for="(header, index) in headers"
                                        :key="index">
                                        {{ header }}
                                    </TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow
                                    v-for="service in serviceInstances"
                                    :key="service.service_instance_id">
                                    <TableCell>{{
                                        service.service_instance_id
                                    }}</TableCell>
                                    <TableCell>{{
                                        service.service_type.join(', ')
                                    }}</TableCell>
                                    <TableCell>{{
                                        df.format(
                                            new Date(service.service_date)
                                        )
                                    }}</TableCell>
                                    <TableCell>{{
                                        service.requires_followup ? 'Yes' : 'No'
                                    }}</TableCell>
                                    <TableCell>{{
                                        service.followup_date
                                            ? df.format(
                                                  new Date(
                                                      service.followup_date
                                                  )
                                              )
                                            : 'N/A'
                                    }}</TableCell>
                                    <TableCell>
                                        <div class="flex gap-3">
                                            <nuxt-link
                                                :to="`/records/pet/service_instance/${service.service_instance_id}`">
                                                <Button
                                                    class="dark:text-accent-foreground"
                                                    >View</Button
                                                >
                                            </nuxt-link>
                                            <AlertDialog>
                                                <AlertDialogTrigger>
                                                    <Button
                                                        variant="destructive"
                                                        >Delete</Button
                                                    >
                                                </AlertDialogTrigger>
                                                <AlertDialogContent>
                                                    <AlertDialogHeader>
                                                        <AlertDialogTitle
                                                            >Delete
                                                            Record</AlertDialogTitle
                                                        >
                                                        <AlertDialogDescription>
                                                            Are you sure you
                                                            want to delete this
                                                            record?
                                                        </AlertDialogDescription>
                                                    </AlertDialogHeader>
                                                    <AlertDialogFooter>
                                                        <AlertDialogCancel
                                                            >Cancel</AlertDialogCancel
                                                        >
                                                        <AlertDialogAction
                                                            @click="
                                                                handleDelete(
                                                                    service.service_instance_id
                                                                )
                                                            "
                                                            >Delete</AlertDialogAction
                                                        >
                                                    </AlertDialogFooter>
                                                </AlertDialogContent>
                                            </AlertDialog>
                                        </div>
                                    </TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </ScrollArea>
                </div>
            </div>
        </Card>
    </div>
</template>

<style scoped></style>
