<script setup lang="ts">
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
} from '~/components/ui/table';

interface Vet {
    vet_id: string;
    vet_name: string;
    vet_email: string;
    vet_phone_number: string;
    vet_license_number: string;
}

const { vets } = defineProps({
    vets: Array as () => Vet[]
});

const headers = ref<string[]>([
    'Vet ID',
    'Name',
    'Email',
    'Phone Number',
    'License Number',
    'Actions'
]);

console.log(vets);
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
                    <Button variant="outline">View</Button>
                    <Button variant="outline">Edit</Button>
                    <Button variant="destructive">Delete</Button>
                </TableCell>
            </TableRow>
        </TableBody>
    </Table>
</template>

<style scoped></style>
