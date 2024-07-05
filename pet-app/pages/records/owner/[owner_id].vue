<script setup lang="ts">
import { OwnerService } from '~/api/owner';
import type {Owner} from "~/types/owner-type";
import type {Pet} from "~/types/pet-type";
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
import {toast} from "~/components/ui/toast";
import { useRefetchStore} from "~/stores/refetch";
import UpdateOwnerForm from "~/components/form/update-owner-form.vue";
const ownerService = new OwnerService();
const router = useRouter();

const { owner_id } = useRoute().params;
const owner = ref<Owner | null>(null);
const pets = ref<Pet[]>([]);

const refetch = useRefetchStore();
const isDataReady = ref(false);

const fetchOwnerAndPets = async () => {
    const response = await ownerService.getOwnerAndPets(owner_id);
    owner.value = response.data.owner;
    pets.value = response.data.pets;
    isDataReady.value = true;
};

const handleDelete = async () => {
    await ownerService.deleteOwner(owner_id).then(() => {
      toast({
        title: '✅ Deleted Successfully',
        description: 'Owner has been deleted successfully'
      });
    })
        .finally(() => {
          router.push('/records');
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

watch(
    () => refetch.needRefetch,
    (newValue) => {
        if (newValue) {
            fetchOwnerAndPets();
            refetch.needRefetch = false;
        }
    }
);

onMounted(async () => {
      fetchOwnerAndPets();
});
</script>

<template>
  <div class="main container">
     <Card>
        <div class="h-[85vh] p-10 w-full flex flex-col gap-5"  >
          <div class="flex flex-col gap-4 border-2 p-5">
            <div class="flex justify-between items-center">
              <h3 class="font-semibold text-xl">Owner Information: </h3>
              <div class="flex gap-3">
                <UpdateOwnerForm :owner_id="owner_id"/>
                <AlertDialog>
                  <AlertDialogTrigger>
                    <Button variant="destructive">Delete</Button>
                  </AlertDialogTrigger>
                    <AlertDialogContent>
                        <AlertDialogHeader>
                            <AlertDialogTitle>Delete Owner</AlertDialogTitle>
                            <AlertDialogDescription>
                              Are you sure you want to delete <span class="font-semibold">{{owner?.owner_name}}</span>?
                            </AlertDialogDescription>
                        </AlertDialogHeader>
                      <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction class="dark:text-accent-foreground bg-red-700" @click="handleDelete">Continue</AlertDialogAction>
                      </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialog>
              </div>
            </div>
            <div class="flex  gap-10 py-3 px-10">
              <div class="flex flex-col gap-4">
                <p><span class="font-semibold">ID:</span> {{owner?.owner_id}}</p>
                <p><span class="font-semibold">Name: </span> {{owner?.owner_name}}</p>
                <p><span class="font-semibold">Email: </span>{{owner?.owner_email}}</p>

              </div>
              <div class="flex  flex-col gap-4">
             <p><span class="font-semibold">Phone Number:</span> {{owner?.owner_phone_number}}</p>
              <p><span class="font-semibold">Address: </span>{{owner?.owner_address}}</p>
              </div>
            </div>
          </div>
          <div class="border-2 p-5">
            <div class="flex items-center justify-between">
              <h3 class="font-semibold text-xl">Pets: </h3>
              <Button variant="default" class="dark:text-accent-foreground">Add a Pet</Button>
            </div>
          <ScrollArea class="h-[25rem] w-full px-10 py-1">
            <h3 v-if="pets.length === 0" class="text-center">No registered pets.</h3>
              <div v-for="(pet, index) in pets" :key="pet.pet_id"  class="py-4">
                <div class="flex items-center justify-around border-2 py-3">
                  <h3 class="text-md font-semibold">Pet Number #{{index + 1}}</h3>
                  <p>Name: {{pet.pet_name}}</p>
                  <p>Breed: {{pet.pet_breed}}</p>
                  <p>Type: {{pet.pet_type}}</p>
                  <div class="flex items-center gap-4">
                    <nuxt-link :to="`/records/pet/${pet.pet_id}`">
                      <Button variant="default" class="dark:text-accent-foreground">View</Button>
                    </nuxt-link>
                    <Button variant="destructive">Delete</Button>
                  </div>
                </div>
              </div>
          </ScrollArea>
          </div>
        </div>
     </Card>
  </div>
</template>

<style scoped>

</style>