<script setup lang="ts">
    import { NodeService } from './service/RegisterMapService.ts';
    import { ref, onMounted } from "vue";

    const nodes = ref();
    onMounted(() => {
        NodeService.update();
        NodeService.getTreeTableNodes().then((data) => { nodes.value = data; });
    })
</script>

<template>
    <div class="card">
        <TreeTable :value="nodes" size="small">
            <Column field="name" header="Name" expander filter></Column>
            <Column field="value" header="Value">
                <template #body="{ node }">
                    <InputNumber v-if="node.data.value !== undefined" v-model="node.data.value" :disabled="node.data.readonly" autofocus fluid />
                </template>
            </Column>
        </TreeTable>
    </div>
</template>

