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
import {
  AlertDialog, AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
  AlertDialogHeader, AlertDialogTitle,
  AlertDialogTrigger
} from "~/components/ui/alert-dialog";
import {toast} from "~/components/ui/toast";
import { OwnerService} from "~/api/owner";

const ownerService = new OwnerService();
const { owners } = defineProps({
    owners: Array as () => Owner[]
});

const headers = ref<string[]>([
    'ID',
    'Name',
    'Email Address',
    'Actions'
]);
const handleDelete = async (owner_id:string) => {
  await ownerService
      .deleteOwner(owner_id)
      .then(() => {
        toast({
          title: '✅ Deleted Successfully',
          description: 'Owner has been deleted successfully'
        });
        const fetch = useRefetchStore();
        fetch.triggerRefetch();
      })
      .catch((error) => {
        toast({
          title: '❌Error',
          description:
              'Error occurred while deleting owner. Please try again later.' +
              error
        });
      });
};
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
                <TableCell>{{ owner.owner_email }}</TableCell>
                <TableCell class="flex gap-3">
                    <nuxt-link :to="`/records/owner/${owner.owner_id}`">
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
                        <AlertDialogTitle
                        >Delete Owner</AlertDialogTitle
                        >
                        <AlertDialogDescription>
                          Are you sure you want to delete
                          <span class="font-semibold">{{
                              owner?.owner_name
                            }}</span
                          >?
                        </AlertDialogDescription>
                      </AlertDialogHeader>
                      <AlertDialogFooter>
                        <AlertDialogCancel
                        >Cancel</AlertDialogCancel
                        >
                        <AlertDialogAction
                            class="bg-red-700 dark:text-accent-foreground"
                            @click="handleDelete(owner.owner_id)"
                        >Continue</AlertDialogAction
                        >
                      </AlertDialogFooter>
                    </AlertDialogContent>
                  </AlertDialog>
                </TableCell>
            </TableRow>
        </TableBody>
    </Table>
</template>

<style scoped></style>
