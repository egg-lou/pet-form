<script setup lang="ts">
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
} from '~/components/ui/table';
import type { Owner } from '~/types/owner-type';

const { owners } = defineProps({
    owners: Array as () => Owner[]
});

const headers = ref<string[]>([
    'Owner ID',
    'Name',
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
              <TableCell>{{ owner.owner_name }}</TableCell>
                <TableCell>{{ owner.owner_phone_number }}</TableCell>
                <TableCell>{{ owner.owner_email }}</TableCell>
                <TableCell>{{ owner.owner_address }}</TableCell>
                <TableCell class="flex gap-3">
                  <nuxt-link :to="`/records/owner/${owner.owner_id}`">
                    <Button variant="default" class="dark:text-accent-foreground">View</Button>
                  </nuxt-link>
                </TableCell>
            </TableRow>
        </TableBody>
    </Table>
</template>

<style scoped></style>
