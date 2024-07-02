<script setup lang="ts">
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
} from '~/components/ui/table';

interface Owner {
    owner_id: number;
    owner_phone_number: string;
    owner_email: string;
    owner_address: string;
}

const { owners } = defineProps({
    owners: Array as () => Owner[]
});

const headers = ref<string[]>([
    'Owner ID',
    'Phone Number',
    'Email Address',
    'Address',
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
                v-for="owner in owners"
                :key="owner.owner_id">
                <TableCell>{{ owner.owner_id }}</TableCell>
                <TableCell>{{ owner.owner_phone_number }}</TableCell>
                <TableCell>{{ owner.owner_email }}</TableCell>
                <TableCell>{{ owner.owner_address }}</TableCell>
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
