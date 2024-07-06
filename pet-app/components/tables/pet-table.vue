<script setup lang="ts">
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
} from '~/components/ui/table';
import type { Pet } from '~/types/pet-type';

const { pets } = defineProps({
    pets: Array as () => Pet[]
});

const headers = ref<string[]>([
    'Name',
    'Type',
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
                <TableCell>{{ pet.pet_name }}</TableCell>
                <TableCell>{{ pet.pet_type }}</TableCell>
                <TableCell>{{ pet.owner_name }}</TableCell>
                <TableCell class="flex gap-3">
                    <nuxt-link :to="`/records/pet/${pet.pet_id}`">
                        <Button
                            variant="default"
                            class="dark:text-accent-foreground"
                            >View</Button
                        >
                    </nuxt-link>
                </TableCell>
            </TableRow>
        </TableBody>
    </Table>
</template>

<style scoped></style>
