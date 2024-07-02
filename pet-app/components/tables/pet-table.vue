<script setup lang="ts">
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
} from '~/components/ui/table';

interface Pet {
    pet_id: number;
    pet_name: string;
    pet_type: string;
    pet_color: string;
    pet_breed: string;
    pet_weight: number;
    owner_name: string;
}

const { pets } = defineProps({
    pets: Array as () => Pet[]
});

const headers = ref<string[]>([
    'Pet ID',
    'Name',
    'Type',
    'Color',
    'Breed',
    'Weight (kgs)',
    'Owner Name',
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
                v-for="pet in pets"
                :key="pet.pet_id">
                <TableCell>{{ pet.pet_id }}</TableCell>
                <TableCell>{{ pet.pet_name }}</TableCell>
                <TableCell>{{ pet.pet_type }}</TableCell>
                <TableCell>{{ pet.pet_color }}</TableCell>
                <TableCell>{{ pet.pet_breed }}</TableCell>
                <TableCell>{{ pet.pet_weight }}</TableCell>
                <TableCell>{{ pet.owner_name }}</TableCell>
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
