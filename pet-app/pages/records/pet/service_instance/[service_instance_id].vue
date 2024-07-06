<script setup lang="ts">
const { service_instance_id } = useRoute().params;
</script>

<template>
    <div class="main container">
        <Card>
            <div class="h-[85vh] p-10">
                {{ service_instance_id }}
            </div>
        </Card>
    </div>
</template>

<style scoped></style>
