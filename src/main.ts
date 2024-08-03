import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import TreeTable from "primevue/treetable";
import Column from "primevue/column";
import InputNumber from "primevue/inputnumber";
import Checkbox from "primevue/checkbox";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import Select from "primevue/select";
import Aura from "@primevue/themes/aura";

const app = createApp(App);
app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
});
app.component('TreeTable', TreeTable);
app.component('Column', Column);
app.component('InputNumber', InputNumber);
app.component('Checkbox', Checkbox);
app.component('Splitter', Splitter);
app.component('SplitterPanel', SplitterPanel);
app.component('Select', Select);
app.mount("#app");
