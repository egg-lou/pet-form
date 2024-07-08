<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm } from 'vee-validate';
import { toast } from '~/components/ui/toast';
import { OwnerService } from '~/api/owner';
import { DialogClose } from '~/components/ui/dialog';
import { useRouter } from 'vue-router';
import { PlusIcon } from 'lucide-vue-next';
const router = useRouter();
const ownerService = new OwnerService();
const ownerSchema = toTypedSchema(
    z.object({
        owner_name: z.string().min(5).max(80),
        owner_email: z.string().email(),
        owner_phone_number: z.string().min(10).max(15),
        owner_address: z.string().min(10).max(255)
    })
);

const { isFieldDirty, handleSubmit } = useForm({
    validationSchema: ownerSchema
});

let ownerId: string;

const onSubmit = handleSubmit((values) => {
    ownerService
        .addOwner(values)
        .then((response) => {
            ownerId = response.data.owner.owner_id;
            toast({
                title: '✅ Owner added successfully',
                description: 'Owner has been added successfully'
            });
            router.push({ path: `/records/owner/${ownerId}` });
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description: 'Failed to add owner' + error.message
            });
        });
});

const resetForm = () => {};
</script>

<template>
    <Dialog>
        <DialogTrigger>
            <Button
                variant="outline"
                class="rounded-lg bg-blue-500 text-accent-foreground transition-all duration-300 hover:bg-blue-600">
                <PlusIcon class="h-6 w-6" />
            </Button>
        </DialogTrigger>
        <DialogContent>
            <h3 class="text-lg font-semibold">Owner Information</h3>
            <form
                class="flex w-full flex-col items-center justify-center space-y-6 py-4"
                @submit="onSubmit">
                <FormField
                    v-slot="{ componentField }"
                    name="owner_name"
                    :validate-on-blur="!isFieldDirty('owner_name')">
                    <FormItem class="w-full">
                        <FormLabel>Name: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Owner Name"
                                v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <FormField
                    v-slot="{ componentField }"
                    name="owner_email"
                    :validate-on-blur="!isFieldDirty('owner_email')">
                    <FormItem class="w-full">
                        <FormLabel>Email: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Owner Email"
                                v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <FormField
                    v-slot="{ componentField }"
                    name="owner_phone_number"
                    :validate-on-blur="!isFieldDirty('owner_phone_number')">
                    <FormItem class="w-full">
                        <FormLabel>Phone Number: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Owner Phone Number"
                                v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <FormField
                    v-slot="{ componentField }"
                    name="owner_address"
                    :validate-on-blur="!isFieldDirty('owner_address')">
                    <FormItem class="w-full">
                        <FormLabel>Address: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Owner Address"
                                v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <div class="flex gap-4 self-end">
                    <Button
                        type="reset"
                        class="self-end"
                        variant="destructive"
                        @click="resetForm">
                        Reset
                    </Button>
                    <DialogClose as-child>
                        <Button
                            type="submit"
                            class="self-end dark:text-accent-foreground"
                            @click="onSubmit">
                            Submit
                        </Button>
                    </DialogClose>
                </div>
            </form>
        </DialogContent>
    </Dialog>
</template>

<style scoped></style>
