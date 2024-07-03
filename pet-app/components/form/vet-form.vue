<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm, useSetFieldValue } from 'vee-validate';
import { toast } from '~/components/ui/toast';
import { VetService } from '~/api/vet';

const props = defineProps({
    mode: String,
    vet_data: Object
});

const vetService = new VetService();
const emit = defineEmits(['formSubmitted', 'formUpdated']);

const vetSchema = toTypedSchema(
    z.object({
        vet_name: z.string().min(5).max(80),
        vet_email: z.string().email(),
        vet_phone_number: z.string().min(10).max(15),
        vet_license_number: z.string().min(10).max(15)
    })
);

const { isFieldDirty, handleSubmit, setValues } = useForm({
    validationSchema: vetSchema
});
onMounted(() => {
    if (props.mode === 'update' && props.vet_data) {
        setValues(props.vet_data);
    }
});

const onSubmit = handleSubmit((values) => {
    if (props.mode === 'add') {
        vetService
            .addVet(values)
            .then(() => {
                toast({
                    title: 'Vet added successfully',
                    description: 'Vet has been added successfully'
                });
                emit('formSubmitted');
            })
            .catch((error) => {
                toast({
                    title: 'Error',
                    description: 'Failed to add veterinarian' + error.message
                });
            });
    } else if (props.mode === 'update') {
        const vetId = props.vet_data.vet_id;

        vetService
            .updateVet(values, vetId)
            .then(() => {
                toast({
                    title: 'Vet updated successfully',
                    description: 'Vet has been updated successfully'
                });
                emit('formUpdated');
            })
            .catch((error) => {
                toast({
                    title: 'Error',
                    description: 'Failed to update veterinarian' + error.message
                });
            });
    }
});

const resetForm = () => {
    console.log('resetting form');
};
</script>

<template>
    <div class="py-4">
        <h3 class="text-lg font-semibold">Veterinarian Information</h3>
        <form
            class="flex w-full flex-col items-center justify-center space-y-3 py-2"
            @submit="onSubmit">
            <FormField
                v-slot="{ componentField }"
                name="vet_name"
                :validate-on-blur="!isFieldDirty('vet_name')">
                <FormItem class="w-full">
                    <FormLabel>Name: </FormLabel>
                    <FormControl>
                        <Input
                            type="text"
                            placeholder="Vet Name"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                name="vet_email"
                :validate-on-blur="!isFieldDirty('vet_name')">
                <FormItem class="w-full">
                    <FormLabel>Email: </FormLabel>
                    <FormControl>
                        <Input
                            type="email"
                            placeholder="Vet Email"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                name="vet_phone_number"
                :validate-on-blur="!isFieldDirty('vet_name')">
                <FormItem class="w-full">
                    <FormLabel>Phone Number: </FormLabel>
                    <FormControl>
                        <Input
                            type="text"
                            placeholder="Vet Phone Number"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                name="vet_license_number"
                :validate-on-blur="!isFieldDirty('vet_name')">
                <FormItem class="w-full">
                    <FormLabel>License Number: </FormLabel>
                    <FormControl>
                        <Input
                            type="text"
                            placeholder="Vet License Number"
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
                <Button
                    type="submit"
                    class="self-end dark:text-accent-foreground"
                    @click="onSubmit">
                    Submit
                </Button>
            </div>
        </form>
    </div>
</template>

<style scoped></style>
