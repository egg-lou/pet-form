<script setup lang="ts">
import { OwnerService } from '~/api/owner';
import { PetService} from "~/api/pet";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from '~/components/ui/table';
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogTrigger,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogCancel,
  AlertDialogAction
} from '~/components/ui/alert-dialog';
import type { Owner } from '~/types/owner-type';
import type { Pet } from '~/types/pet-type';
import { toast } from '~/components/ui/toast';
import { useRefetchStore } from '~/stores/refetch';
import UpdateOwnerForm from '~/components/form/update-owner-form.vue';
import PetForm from "~/components/form/pet-form.vue";
const ownerService = new OwnerService();
const  petService = new PetService();

const fetch = useRefetchStore();
const { owner_id } = useRoute().params;
const owner = ref<Owner | null>(null);
const pets = ref<Pet[]>([]);

const isDataReady = ref(false);

const fetchOwnerAndPets = async () => {
    const response = await ownerService.getOwnerAndPets(owner_id);
    owner.value = response.data.owner;
    pets.value = response.data.pets;
    isDataReady.value = true;
};

const headers = ref<string[]>([
  'Name',
  'Type',
  'Breed',
  'Actions'
]);

const deletePet = async (pet_id: string) => {
    await petService
        .deletePet(pet_id)
        .then(() => {
            toast({
                title: '✅ Deleted Successfully',
                description: 'Pet has been deleted successfully'
            });
            fetch.triggerRefetch();
        })
        .catch((error) => {
            toast({
                title: '❌Error',
                description:
                    'Error occurred while deleting pet. Please try again later.' +
                    error
            });
        });
}


watch(
    () => fetch.needRefetch,
    (newValue) => {
        if (newValue) {
            fetchOwnerAndPets();
            fetch.needRefetch = false;
        }
    }
);

onMounted(async () => {
    await fetchOwnerAndPets();
});
</script>

<template>
    <div class="main container">
        <Card>
            <div class="flex h-[85vh] w-full flex-col gap-5 p-10">
                <div class="flex flex-col gap-4 border-2 p-5">
                    <div class="flex items-center justify-between">
                        <h3 class="text-xl font-semibold">
                            Owner Information:
                        </h3>
                        <div class="flex gap-3">
                            <UpdateOwnerForm />
                        </div>
                    </div>
                    <div class="flex gap-10 px-10 py-3">
                        <div class="flex flex-col gap-4">
                            <p>
                                <span class="font-semibold">ID:</span>
                                {{ owner?.owner_id }}
                            </p>
                            <p>
                                <span class="font-semibold">Name: </span>
                                {{ owner?.owner_name }}
                            </p>
                            <p>
                                <span class="font-semibold">Email: </span
                                >{{ owner?.owner_email }}
                            </p>
                        </div>
                        <div class="flex flex-col gap-4">
                            <p>
                                <span class="font-semibold">Phone Number:</span>
                                {{ owner?.owner_phone_number }}
                            </p>
                            <p>
                                <span class="font-semibold">Address: </span
                                >{{ owner?.owner_address }}
                            </p>
                        </div>
                    </div>
                </div>
                <div class="border-2 p-5">
                    <div class="flex items-center justify-between">
                        <h3 class="text-xl font-semibold">Pets:</h3>
                      <PetForm :owner_id="owner_id"/>
                    </div>
                    <ScrollArea class="h-[25rem] w-full px-10 py-1">
                        <h3
                            v-if="pets.length === 0"
                            class="text-center">
                            No registered pets.
                        </h3>
                      <Table>
                        <TableHeader>
                          <TableRow>
                            <TableHead
v-for="(header, index) in headers"
                            :key="index">
                              {{header}}
                            </TableHead>
                          </TableRow>
                        </TableHeader>
                          <TableBody>
                            <TableRow v-for="pet in pets" :key="pet.pet_id">
                              <TableCell>{{ pet.pet_name}}</TableCell>
                              <TableCell>{{ pet.pet_type}}</TableCell>
                              <TableCell>{{ pet.pet_breed}}</TableCell>
                              <TableCell class="flex gap-3">
                                <PetForm
                                  :mode="'update'"
                                  :pet_data="pet"
                                />
                                <AlertDialog>
                                  <AlertDialogTrigger>
                                    <Button variant="destructive">Delete</Button>
                                  </AlertDialogTrigger>
                                  <AlertDialogContent>
                                    <AlertDialogHeader>
                                      <AlertDialogTitle>
                                        Are you sure you want to delete this pet?
                                      </AlertDialogTitle>
                                      <AlertDialogDescription>
                                        Deleting
                                        <span class="font-bold">{{
                                          pet.pet_name
                                        }}</span>
                                        will remove all the data associated with
                                        this pet.
                                      </AlertDialogDescription>
                                    </AlertDialogHeader>
                                    <AlertDialogFooter>
                                      <AlertDialogCancel> Cancel </AlertDialogCancel>
                                      <AlertDialogAction
                                        @click="deletePet(pet.pet_id)"
                                      >
                                        Delete
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
            </div>
        </Card>
    </div>
</template>

<style scoped></style>
