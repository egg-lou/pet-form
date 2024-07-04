<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm } from 'vee-validate';
import { toast } from '~/components/ui/toast';
import { h } from 'vue';
import { useFormCheckStore } from '~/stores/formCheck';
import {
    CalendarDate,
    DateFormatter,
    getLocalTimeZone,
    parseDate,
    today
} from '@internationalized/date';
import { RadioGroup, RadioGroupItem } from '~/components/ui/radio-group';

import {
    Popover,
    PopoverContent,
    PopoverTrigger
} from '~/components/ui/popover';
import { CalendarIcon } from 'lucide-vue-next';
import { cn } from '~/lib/utils';
import { toDate } from 'radix-vue/date';
import { Calendar } from '~/components/ui/calendar';
import { placeholder } from '@babel/types';
import { PetService } from '~/api/pet';

const petService = new PetService();
const formStore = useFormCheckStore();

const props = defineProps({
    mode: String,
    pet_data: Object,
    owner_id: String
});

const df = new DateFormatter('en-US', {
    dateStyle: 'long'
});

const petSchema = toTypedSchema(
    z.object({
        owner_id: z.string().max(36).optional(),
        pet_name: z.string().max(80),
        pet_type: z.enum(['Dog', 'Cat'], {
            required_error: 'Please select a pet type.'
        }),
        pet_breed: z.string().max(40),
        pet_weight: z.number().min(0),
        pet_birth_date: z
            .string()
            .refine((v) => v, { message: 'A date of birth is required.' }),
        pet_color: z.string().max(20)
    })
);

const placeholder = ref();

const value = computed({
    get: () =>
        values.pet_birth_date ? parseDate(values.pet_birth_date) : undefined,
    set: (val) => val
});

const { isFieldDirty, handleSubmit, values, setFieldValue, validate } = useForm(
    {
        validationSchema: petSchema
    }
);

onMounted(() => {
    if (props.mode === 'update' && props.pet_data) {
        setValues(props.pet_data);
    }
});

watch(
    () => props.owner_id,
    (newOwnerId) => {
        setFieldValue('owner_id', newOwnerId);
    }
);

const onSubmit = handleSubmit((values) => {
    if (props.mode === 'add') {
        petService
            .addPet(values)
            .then((response) => {
                toast({
                    title: '✅ Pet added successfully',
                    description: 'Pet has been added successfully'
                });
                resetForm();
            })
            .catch((error) => {
                toast({
                    title: '❌ Error',
                    description: 'Failed to add pet' + error.message
                });
            });
    } else {
        petService
            .updatePet(values)
            .then((response) => {
                toast({
                    title: '✅ Pet updated successfully',
                    description: 'Pet has been updated successfully'
                });
                resetForm();
            })
            .catch((error) => {
                toast({
                    title: '❌ Error',
                    description: 'Failed to update pet' + error.message
                });
            });
    }
});

const resetForm = () => {
    console.log('resetting form');
};

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
    <h3
        v-if="props.mode !== 'add'"
        class="text-lg font-semibold">
        Pet Information
    </h3>
    <form
        class="flex w-full items-center justify-center gap-4 space-y-6 py-4"
        @submit="onSubmit">
        <div>
            <FormField
                v-if="props.mode !== 'add'"
                v-slot="{ componentField }"
                name="owner_id"
                :validate-on-blur="!isFieldDirty('owner_id')">
                <FormItem class="w-full">
                    <FormLabel>Owner ID: </FormLabel>
                    <FormControl>
                        <Input
                            type="text"
                            placeholder="Owner ID"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                name="pet_name"
                :validate-on-blur="!isFieldDirty('pet_name')">
                <FormItem class="w-full">
                    <FormLabel>Name: </FormLabel>
                    <FormControl>
                        <Input
                            type="text"
                            placeholder="Pet Name"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                name="pet_weight"
                :validate-on-blur="!isFieldDirty('pet_weight')">
                <FormItem class="w-full">
                    <FormLabel>Weight: </FormLabel>
                    <FormControl>
                        <Input
                            type="number"
                            placeholder="Pet Weight"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                name="pet_breed"
                :validate-on-blur="!isFieldDirty('pet_breed')">
                <FormItem class="w-full">
                    <FormLabel>Breed: </FormLabel>
                    <FormControl>
                        <Input
                            type="text"
                            placeholder="Pet Breed"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>
        </div>
        <div>
            <FormField
                name="pet_birth_date"
                :validate-on-blur="!isFieldDirty('pet_birth_date')">
                <FormItem class="w-full">
                    <FormLabel>Birth Date: </FormLabel>
                    <Popover>
                        <PopoverTrigger as-child>
                            <FormControl>
                                <Button
                                    variant="outline"
                                    :class="
                                        cn(
                                            'w-[240px] ps-3 text-start font-normal',
                                            !value && 'text-muted-foreground'
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
                                :min-value="new CalendarDate(1900, 1, 1)"
                                :max-value="today(getLocalTimeZone())"
                                @update:model-value="
                                    (v) => {
                                        if (v) {
                                            setFieldValue(
                                                'pet_birth_date',
                                                v.toString()
                                            );
                                        } else {
                                            setFieldValue(
                                                'pet_birth_date',
                                                undefined
                                            );
                                        }
                                    }
                                " />
                        </PopoverContent>
                    </Popover>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                name="pet_color"
                :validate-on-blur="!isFieldDirty('pet_color')">
                <FormItem class="w-full">
                    <FormLabel>Color: </FormLabel>
                    <FormControl>
                        <Input
                            type="text"
                            placeholder="Pet Color"
                            v-bind="componentField" />
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <FormField
                v-slot="{ componentField }"
                type="radio"
                name="pet_type">
                <FormItem class="space-y-3">
                    <FormLabel>Type: </FormLabel>

                    <FormControl>
                        <RadioGroup
                            class="flex flex-col space-y-1"
                            v-bind="componentField">
                            <FormItem
                                class="flex items-center gap-x-3 space-y-0">
                                <FormControl>
                                    <RadioGroupItem value="Dog" />
                                </FormControl>
                                <FormLabel class="font-normal"> Dog </FormLabel>
                            </FormItem>
                            <FormItem
                                class="flex items-center gap-x-3 space-y-0">
                                <FormControl>
                                    <RadioGroupItem value="Cat" />
                                </FormControl>
                                <FormLabel class="font-normal"> Cat </FormLabel>
                            </FormItem>
                        </RadioGroup>
                    </FormControl>
                    <FormMessage />
                </FormItem>
            </FormField>

            <div
                v-if="props.mode !== 'add'"
                class="flex gap-4 self-end py-4">
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
        </div>
    </form>
</template>

<style scoped></style>
