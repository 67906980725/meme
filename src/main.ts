import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import LongPress from './directives/longPress'
const app = createApp(App)
app.use(createPinia())
app.directive('longPress', LongPress)

app.mount('#app')
