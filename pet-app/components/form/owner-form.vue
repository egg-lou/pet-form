<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm } from 'vee-validate';
import { toast } from '~/components/ui/toast';
import { h } from 'vue';

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

const onSubmit = handleSubmit((values) => {
    console.log(values);
    toast({
        title: 'You submitted the form',
        description: h('pre', JSON.stringify(values, null, 2))
    });
});

const resetForm = () => {
    console.log('resetting form');
};
</script>

<template>
    <div class="py-4">
        <h3 class="text-2xl font-semibold">Owner Information</h3>
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
                <Button
                    type="submit"
                    class="self-end"
                    @click="onSubmit">
                    Submit
                </Button>
            </div>
        </form>
    </div>
</template>

<style scoped></style>
