<template>
  <main>
    <n-layout has-sider style="height: 90vh">
      <n-layout-sider
        bordered
        collapse-mode="width"
        :collapsed-width="20"
        :width="120"
        :collapsed="collapsed"
        show-trigger
        @collapse="collapsed = true"
        @expand="collapsed = false"
      >
        <n-menu
          :collapsed="collapsed"
          :collapsed-width="64"
          :collapsed-icon-size="22"
          :options="menuOptions"
          :render-label="renderMenuLabel"
          :render-icon="renderMenuIcon"
          :expand-icon="expandIcon"
          :on-update:value="menu_handler"
        />
      </n-layout-sider>

      <n-layout-content>
        <!-- <n-space vertical> -->
        <Search v-if="page == 'search'" />
        <Help v-if="page == 'help'" />
        <!-- </n-space> -->
      </n-layout-content>
    </n-layout>
  </main>
</template>

<script lang="js">
import { h, ref, defineComponent } from 'vue'
import { useNotification, useMessage } from 'naive-ui'

import Search from '../components/Search.vue'
import Help from '../components/Help.vue'

import { NIcon } from "naive-ui";
// import { BookmarkOutline, CaretDownOutline } from "@vicons/ionicons5";
import { DeviceUtil } from '@/stores/util'

// import { DeviceUtil } from '@/stores/util'
const isMobile = DeviceUtil.isMobile

export default {
  setup() {
    const collapsed = ref(true)
    const page = ref('search')
    const menuOptions = [
      {
        label: "搜索",
        key: "search",
      },
      {
        label: "帮助",
        key: "help",
      }
    ];

    window.$notification = useNotification()
    window.$message = useMessage()

    return { page, collapsed, menuOptions }
  },
  methods: {
    menu_handler(key) {
      this.page = key
      // if (isMobile) {
        this.collapsed = true
      // }
    }
  }
}
</script>

<style scoped></style>
