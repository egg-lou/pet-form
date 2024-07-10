<script setup lang="ts">
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
} from '~/components/ui/table';
import { ServiceInstanceService } from '~/api/service-instance';
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger
} from '~/components/ui/alert-dialog';
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';
import { toast } from '~/components/ui/toast';
const serviceInstanceService = new ServiceInstanceService();

const serviceInstances = ref([]);
const currentPage = ref(2);
const totalPage = ref(0);
const fetchServiceInstances = async () => {
    const response = await serviceInstanceService.getAllServiceInstances();
    console.log(response);
    serviceInstances.value = response.data.service_instances;
    totalPage.value = response.data.total_pages;
};

const nextPage = () => {
    currentPage.value++;
    fetchServiceInstances();
};

const prevPage = () => {
    currentPage.value--;
    fetchServiceInstances();
};

const headers = ref<string[]>([
    'ID',
    'Date',
    'Service Type',
    'Pet Name',
    'Pet Type',
    'Pet Breed',
    'Owner',
    'Actions'
]);

onMounted(async () => {
    await fetchServiceInstances();
});

const handleDelete = async (service_instance_id: string) => {
    await serviceInstanceService
        .deleteServiceInstance(service_instance_id)
        .then(() => {
            toast({
                title: '✅ Deleted Successfully',
                description: 'Service Instance has been deleted successfully'
            });
        })
        .catch((error) => {
            toast({
                title: '❌Error',
                description:
                    'Error occurred while deleting vet. Please try again later.' +
                    error
            });
        });
};
</script>

<template>
    <div class="main container">
        <Card>
            <div class="h-[80vh] p-10">
                <div class="flex items-center gap-4 pb-3">
                    <nuxt-img
                        src="/img/icon.png"
                        alt="Paws and Claws Vet Clinic"
                        width="50"
                        height="50" />
                    <h3 class="text-xl font-semibold">Service Instances</h3>
                </div>
                <ScrollArea class="h-[40rem]">
                    <Table>
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
                                    service.service_date
                                }}</TableCell>
                                <TableCell>{{
                                    service.service_type.join(', ')
                                }}</TableCell>
                                <TableCell>{{
                                    service.pet.pet_name
                                }}</TableCell>
                                <TableCell>{{
                                    service.pet.pet_type
                                }}</TableCell>
                                <TableCell>{{
                                    service.pet.pet_breed
                                }}</TableCell>
                                <TableCell>{{
                                    service.pet.owner_name
                                }}</TableCell>
                                <TableCell class="flex gap-3">
                                    <nuxt-link
                                        :to="`/records/pet/service_instance/${service.service_instance_id}`">
                                        <Button
                                            variant="default"
                                            class="dark:text-accent-foreground"
                                            >View</Button
                                        >
                                    </nuxt-link>
                                    <AlertDialog>
                                        <AlertDialogTrigger>
                                            <Button variant="destructive"
                                                >Delete</Button
                                            >
                                        </AlertDialogTrigger>
                                        <AlertDialogContent>
                                            <AlertDialogHeader>
                                                <AlertDialogTitle>
                                                    Are you sure you want to
                                                    delete this vet?
                                                </AlertDialogTitle>
                                                <AlertDialogDescription>
                                                    Deleting
                                                    <span class="font-bold">{{
                                                        service.service_instance_id
                                                    }}</span>
                                                    will remove all the data
                                                    associated with this vet.
                                                </AlertDialogDescription>
                                            </AlertDialogHeader>
                                            <AlertDialogFooter>
                                                <AlertDialogCancel>
                                                    Cancel
                                                </AlertDialogCancel>
                                                <AlertDialogAction
                                                    @click="
                                                        handleDelete(
                                                            service.service_instance_id
                                                        )
                                                    ">
                                                    Continue
                                                </AlertDialogAction>
                                            </AlertDialogFooter>
                                        </AlertDialogContent>
                                    </AlertDialog>
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </ScrollArea>
            </div>
        </Card>
    </div>
</template>

<style scoped></style>
