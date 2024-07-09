<script setup lang="ts">
import { PenIcon } from 'lucide-vue-next';
import { serviceTypes } from '~/utils/checkBoxItems';
import { cn } from '~/lib/utils';
import {
    CalendarDate,
    DateFormatter,
    parseDate
} from '@internationalized/date';
import { toDate } from 'radix-vue/date';
import { CalendarIcon } from '@radix-icons/vue';
import { Textarea } from '~/components/ui/textarea';
import { useForm } from 'vee-validate';
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { ServiceInstanceService } from '~/api/service-instance';
import { DialogClose } from 'radix-vue';
import { toast } from '~/components/ui/toast';

const { service_instance_id } = useRoute().params;
const serviceInstanceService = new ServiceInstanceService();
const placeholder = ref();
const petId = ref('');
const fetchServiceInstance = async () => {
    const response =
        await serviceInstanceService.getServiceInstance(service_instance_id);
    petId.value = response.data.pet_id;
    setValues({
        pet_id: response.data.pet_id,
        service_type: response.data.service_type,
        service_reason: response.data.service_reason,
        general_diagnosis: response.data.general_diagnosis,
        requires_followup: response.data.requires_followup,
        followup_date: response.data.followup_date
    });
};
const df = new DateFormatter('en-US', {
    dateStyle: 'long'
});
const value = computed({
    get: () =>
        values.followup_date ? parseDate(values.followup_date) : undefined,
    set: (val) => val
});

const updateService = toTypedSchema(
    z.object({
        pet_id: z.string().nonempty(),
        service_type: z.array(z.string().nonempty()),
        service_reason: z.string().nonempty(),
        general_diagnosis: z.string().nonempty(),
        requires_followup: z.boolean(),
        followup_date: z.string().nullable().optional()
    })
);

const { handleSubmit, isFieldDirty, values, setFieldValue, setValues } =
    useForm({
        validationSchema: updateService,
        initialValues: {
            service_type: []
        }
    });

const onSubmit = handleSubmit(async (values) => {
    await serviceInstanceService
        .updateServiceInstance(service_instance_id, values)
        .then(() => {
            toast({
                title: '✅ Service Instance updated successfully',
                description: 'Record has been updated successfully'
            });
            const fetch = useRefetchStore();
            fetch.triggerRefetch();
        })
        .catch((error) => {
            toast({
                title: '❌ Error updating Service Instance',
                description: error.message
            });
        });
});
</script>

<template>
    <Dialog>
        <DialogTrigger>
            <Button
                variant="ghost"
                @click="fetchServiceInstance">
                <PenIcon class="h-5 w-5" />
            </Button>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>Edit Service Instance: </DialogHeader>
            <DialogDescription>
                <form
                    @submit.prevent="onSubmit"
                    class="flex flex-col gap-5">
                    <FormField
                        name="pet_id"
                        :validate-on-blur="!isFieldDirty('pet_id')">
                        <FormItem>
                            <FormLabel>Pet ID</FormLabel>
                            <FormControl>
                                <Input
                                    v-model="petId"
                                    type="text"
                                    disabled />
                            </FormControl>
                        </FormItem>
                    </FormField>
                    <FormField name="service_type">
                        <FormItem>
                            <FormLabel> Service Type: </FormLabel>
                            <FormField
                                v-for="item in serviceTypes"
                                v-slot="{ value, handleChange }"
                                :key="item.id"
                                v-model="values.service_type"
                                type="checkbox"
                                :value="item.id"
                                :unchecked-value="false"
                                name="service_type">
                                <FormItem
                                    class="flex flex-row items-start space-x-3 space-y-0">
                                    <FormControl>
                                        <Checkbox
                                            :checked="value.includes(item.id)"
                                            @update:checked="handleChange" />
                                    </FormControl>
                                    <FormLabel>
                                        {{ item.label }}
                                    </FormLabel>
                                </FormItem>
                            </FormField>
                            <FormMessage />
                        </FormItem>
                    </FormField>
                    <FormField
                        v-slot="{ componentField }"
                        name="service_reason"
                        :validate-on-blur="!isFieldDirty('service_reason')">
                        <FormItem class="w-full">
                            <FormLabel>Service Reason: </FormLabel>
                            <FormControl>
                                <Textarea
                                    placeholder="Service Reason"
                                    class="resize-none"
                                    v-bind="componentField" />
                            </FormControl>
                        </FormItem>
                    </FormField>

                    <FormField
                        v-slot="{ componentField }"
                        name="general_diagnosis"
                        :validate-on-blur="!isFieldDirty('general_diagnosis')">
                        <FormItem class="w-full">
                            <FormLabel>General Diagnosis: </FormLabel>
                            <FormControl>
                                <Textarea
                                    placeholder="General Diagnosis"
                                    class="resize-none"
                                    v-bind="componentField" />
                            </FormControl>
                        </FormItem>
                    </FormField>

                    <FormField
                        v-slot="{ value, handleChange }"
                        name="requires_followup"
                        :validate-on-blur="!isFieldDirty('requires_followup')">
                        <FormItem
                            class="flex w-full items-center gap-3 space-y-0">
                            <FormLabel>Requires Follow Up: </FormLabel>
                            <FormControl>
                                <Switch
                                    v-model="values.requires_followup"
                                    :checked="value"
                                    @update:checked="handleChange" />
                            </FormControl>
                        </FormItem>
                    </FormField>

                    <FormField
                        v-if="values.requires_followup"
                        name="followup_date">
                        <FormItem class="flex items-center space-x-3 space-y-0">
                            <FormLabel>Follow Up Date: </FormLabel>
                            <Popover>
                                <PopoverTrigger as-child>
                                    <FormControl>
                                        <Button
                                            variant="outline"
                                            :class="
                                                cn(
                                                    'w-[240px] ps-3 text-start font-normal',
                                                    !value &&
                                                        'text-muted-foreground'
                                                )
                                            ">
                                            <span>{{
                                                value
                                                    ? df.format(toDate(value))
                                                    : 'Pick a date'
                                            }}</span>
                                            <CalendarIcon
                                                class="ms-auto h-4 w-4 opacity-50" />
                                        </Button>
                                        <input hidden />
                                    </FormControl>
                                </PopoverTrigger>
                                <PopoverContent class="w-auto p-0">
                                    <Calendar
                                        v-model:placeholder="placeholder"
                                        v-model="value"
                                        calendar-label="Date of birth"
                                        initial-focus
                                        :min-value="
                                            new CalendarDate(1900, 1, 1)
                                        "
                                        @update:model-value="
                                            (v) => {
                                                if (v) {
                                                    setFieldValue(
                                                        'followup_date',
                                                        v.toString()
                                                    );
                                                } else {
                                                    setFieldValue(
                                                        'followup_date',
                                                        undefined
                                                    );
                                                }
                                            }
                                        " />
                                </PopoverContent>
                            </Popover>
                        </FormItem>
                    </FormField>
                    <div class="flex gap-4 self-end py-4">
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
            </DialogDescription>
        </DialogContent>
    </Dialog>
</template>

<style scoped></style>
