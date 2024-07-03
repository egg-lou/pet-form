<script setup lang="ts">
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
} from '~/components/ui/table';
import {
    DialogContent,
    DialogTrigger,
    Dialog,
    DialogClose
} from '~/components/ui/dialog';
import type { Vet } from '~/types/vet-type';
import { VetService } from '~/api/vet';
import { toast } from '~/components/ui/toast';
import VetForm from '~/components/form/vet-form.vue';
const emit = defineEmits(['deleteVet']);
const vetService = new VetService();
const { vets } = defineProps({
    vets: Array as () => Vet[]
});

const handleDelete = async (vet_id: string) => {
    await vetService
        .deleteVet(vet_id)
        .then(() => {
            toast({
                title: '✅ Deleted Successfully',
                description: 'Vet has been deleted successfully'
            });
            const refetch = useRefetchStore();
            refetch.triggerRefetch();
        })
        .finally(() => {
            emit('deleteVet');
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

const headers = ref<string[]>([
    'Vet ID',
    'Name',
    'Email',
    'Phone Number',
    'License Number',
    'Actions'
]);
</script>

<template>
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
                v-for="vet in vets"
                :key="vet.vet_id">
                <TableCell>{{ vet.vet_id }}</TableCell>
                <TableCell>{{ vet.vet_name }}</TableCell>
                <TableCell>{{ vet.vet_email }}</TableCell>
                <TableCell>{{ vet.vet_phone_number }}</TableCell>
                <TableCell>{{ vet.vet_license_number }}</TableCell>
                <TableCell class="flex gap-3">
                    <VetForm
                        :mode="'update'"
                        :vet_data="vet" />
                    <Dialog>
                        <DialogTrigger>
                            <Button variant="destructive">Delete</Button>
                        </DialogTrigger>
                        <DialogContent>
                            <div
                                class="flex flex-col items-center justify-center gap-4">
                                <p>Are you sure you want to delete this vet?</p>
                                <p>{{ vet.vet_name }}</p>
                                <DialogClose as-child>
                                    <Button
                                        variant="outline"
                                        class="w-full"
                                        >Cancel</Button
                                    >
                                </DialogClose>
                                <Button
                                    variant="destructive"
                                    class="w-full"
                                    @click="handleDelete(vet.vet_id)"
                                    >Confirm</Button
                                >
                            </div>
                        </DialogContent>
                    </Dialog>
                </TableCell>
            </TableRow>
        </TableBody>
    </Table>
</template>

<style scoped></style>
