
<template>
    <div class="card">
        <TreeTable :value="nodes" >
            <Column field="name" header="Name" expander filter></Column>
            <Column field="value" header="Value">
                <template #body="{ node, column}">
                    <Checkbox v-if="!node.children && node.data.mask && node.data.mask.toString(2).split('1').length === 2" v-model="node.data.value" :trueValue=1 :falseValue=0 binary />
                    <InputNumber v-else-if="!node.children" v-model="node.data.value" autofocus fluid />
                </template>
            </Column>
        </TreeTable>
    </div>
</template>

<script>
import { NodeService } from './service/RegisterMapService.ts';

export default {
    data() {
        return {
            nodes: null
        }
    },
    mounted() {
        NodeService.getTreeTableNodes().then((data) => (this.nodes = data));
    }
}
</script>