<script setup lang="ts">
import { groomingTypes } from '~/utils/checkBoxItems';
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import * as z from 'zod';
import { ServiceInstanceService } from '~/api/service-instance';
import { toast } from '~/components/ui/toast';
const serviceInstanceService = new ServiceInstanceService();
const { service_instance_id } = useRoute().params;
const isDialogOpen = ref(false);
const addGroomingSchema = toTypedSchema(
    z.object({
        grooming_type: z
            .array(z.string())
            .nonempty()
            .refine((value) => value !== null, {
                message: 'Please select a grooming type'
            })
    })
);

const { handleSubmit } = useForm({
    validationSchema: addGroomingSchema,
    initialValues: {
        grooming_type: []
    }
});

const onSubmit = handleSubmit(async (values) => {
    await serviceInstanceService
        .addGrooming(service_instance_id, values)
        .then(() => {
            toast({
                title: '✅ Grooming added successfully',
                description: 'Grooming has been added successfully'
            });
            isDialogOpen.value = false;
            const fetch = useRefetchStore();
            fetch.triggerRefetch();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description: error.response.data.message
            });
        });
});

const resetForm = () => {
    console.log('reset');
};
</script>

<template>
    <Dialog v-model:open="isDialogOpen">
        <DialogTrigger class="w-full border-2 border-dashed p-2">
            <p>Add</p>
        </DialogTrigger>
        <DialogContent class="flex flex-col p-10">
            <DialogHeader>Add Grooming: </DialogHeader>
            <DialogDescription>
                <form @submit.prevent="onSubmit">
                    <FormField name="grooming_type">
                        <FormItem class="flex flex-col gap-3">
                            <FormLabel>
                                <div class="types-container">
                                    <FormField
                                        v-for="item in groomingTypes"
                                        v-slot="{ value, handleChange }"
                                        :key="item.id"
                                        type="checkbox"
                                        :value="item.id"
                                        :unchecked-value="false"
                                        name="grooming_type">
                                        <FormItem
                                            class="flex flex-row items-start space-x-3 space-y-0">
                                            <FormControl>
                                                <Checkbox
                                                    :checked="
                                                        value.includes(item.id)
                                                    "
                                                    @update:checked="
                                                        handleChange
                                                    " />
                                            </FormControl>
                                            <FormLabel>{{
                                                item.label
                                            }}</FormLabel>
                                        </FormItem>
                                    </FormField>
                                </div>
                                <FormMessage />
                            </FormLabel>
                        </FormItem>
                    </FormField>
                    <div class="flex justify-end gap-4 self-end py-4">
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
            </DialogDescription>
        </DialogContent>
    </Dialog>
</template>
<style scoped>
.types-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
}
</style>
