<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm } from 'vee-validate';
import { toast } from '~/components/ui/toast';
import { h } from 'vue';
import { useFormCheckStore } from '~/stores/formCheck';
import { OwnerService } from '~/api/owner';

const ownerService = new OwnerService();
const formStore = useFormCheckStore();
const ownerSchema = toTypedSchema(
    z.object({
        owner_name: z.string().min(5).max(80),
        owner_email: z.string().email(),
        owner_phone_number: z.string().min(10).max(15),
        owner_address: z.string().min(10).max(255)
    })
);

const props = defineProps();
const emits = defineEmits(['update:modelValue', 'owner-added']);
const ownerData = ref({});

function updateOwnerData(key, value) {
    ownerData.value = { ...ownerData.value, [key]: value };
    emits('update:modelValue', ownerData.value);
}

const { isFieldDirty, handleSubmit, validate } = useForm({
    validationSchema: ownerSchema
});

const onSubmit = handleSubmit((values) => {
    ownerService
        .addOwner(values)
        .then((response) => {
            const ownerId = response.data.owner_id;
            toast({
                title: '✅ Owner added successfully',
                description: 'Owner has been added successfully'
            });
            resetForm();
            emits('owner-added', ownerId);
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description: 'Failed to add owner' + error.message
            });
        });
});

const resetForm = () => {};

watch(
    () => formStore.triggerValidation,
    async () => {
        const result = await validate();
        if (!result) {
            toast({
                title: '❌ Error',
                description: 'Please fill in all required fields'
            });
        } else {
            await onSubmit();
        }
    }
);
</script>

<template>
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
                        v-bind="componentField"
                        @input="
                            updateOwnerData('owner_name', $event.target.value)
                        " />
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
                        v-bind="componentField"
                        @input="
                            updateOwnerData('owner_email', $event.target.value)
                        " />
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
                        v-bind="componentField"
                        @input="
                            updateOwnerData(
                                'owner_phone_number',
                                $event.target.value
                            )
                        " />
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
                        v-bind="componentField"
                        @input="
                            updateOwnerData(
                                'owner_address',
                                $event.target.value
                            )
                        " />
                </FormControl>
                <FormMessage />
            </FormItem>
        </FormField>
    </form>
</template>

<style scoped></style>
