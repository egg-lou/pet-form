<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm } from 'vee-validate';
import { OwnerService } from '~/api/owner';
import { Dialog, DialogTrigger, DialogClose } from '~/components/ui/dialog';
import { toast } from '~/components/ui/toast';

const props = defineProps({
    owner_id: String
});

const ownerService = new OwnerService();
const ownerSchema = toTypedSchema(
    z.object({
        owner_id: z.string().min(5).max(36),
        owner_name: z.string().min(5).max(80),
        owner_email: z.string().email(),
        owner_phone_number: z.string().min(10).max(15),
        owner_address: z.string().min(5).max(255)
    })
);

const fetchOwnerAndPets = async () => {
    const response = await ownerService.getOwnerAndPets(props.owner_id);
    setValues(response.data.owner);
};

const { isFieldDirty, handleSubmit, setValues } = useForm({
    validationSchema: ownerSchema
});

const onSubmit = handleSubmit((values) => {
    ownerService
        .updateOwner(values, props.owner_id)
        .then(() => {
            toast({
                title: '✅ Owner updated successfully',
                description: 'Owner has been updated successfully'
            });
            const refetch = useRefetchStore();
            refetch.triggerRefetch();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description: 'Failed to update owner' + error.message
            });
        });
});
</script>

<template>
    <Dialog>
        <DialogTrigger>
            <Button
                variant="default"
                @click="fetchOwnerAndPets"
                class="dark:text-accent-foreground"
                >Edit</Button
            >
        </DialogTrigger>
        <DialogContent>
            <h3 class="text-lg font-semibold">Update Owner</h3>
            <form
                class="flex w-full flex-col items-center justify-center space-y-6 py-4"
                @submit="onSubmit">
                <FormField
                    v-slot="{ componentField }"
                    name="owner_id"
                    :validate-on-blur="!isFieldDirty('owner_name')">
                    <FormItem class="w-full">
                        <FormLabel>ID: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Owner ID"
                                v-bind="componentField"
                                disabled />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
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
                <div class="flex w-full justify-end">
                    <DialogClose as-child>
                        <Button
                            type="submit"
                            variant="default"
                            class="dark:text-accent-foreground"
                            @click="onSubmit"
                            >Update</Button
                        >
                    </DialogClose>
                </div>
            </form>
        </DialogContent>
    </Dialog>
</template>

<style scoped></style>
