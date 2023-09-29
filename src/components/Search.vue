<template>
  <!-- 允许拖动图片到页放上传 -->
  <div class="drop-zone" @dragover="allowDrop" @drop="handleDrop" style="height: 100%">
    <n-layout>
      <n-tabs class="style-tab" :type="tabs_type" animated @update:value="handle_styles_update">
        <n-tab
          v-for="style in state.styles"
          :key="style.id"
          :name="style.id"
          :tab="style.name"
        ></n-tab>
      </n-tabs>

      <n-input-group>
        <!-- 安卓只有input和blur事件, 使用接口防抖, 和pc区分开 -->
        <n-input
          v-if="isMobile"
          placeholder="请输入关键词"
          @change="handle_search_input_change_mobile"
          ref="searchInput"
        />
        <n-input
          v-else
          placeholder="请输入关键词"
          @blur="handle_search_input_blur"
          @change="handle_search_input_change"
          ref="searchInput"
        />
        <n-button @click="showModal = true">F</n-button>
        <n-button v-if="isMobile" @click="selectFile">+</n-button>
        <n-button v-if="!isMobile" @click="open_folder">O</n-button>
      </n-input-group>

      <n-space>
        <n-tabs
          class="folder-tab"
          :type="tabs_type"
          size="small"
          animated
          @change="handle_folders_change"
          @update:value="handle_folders_update"
        >
          <n-tab
            v-for="folder in state.folders"
            :key="folder.id"
            :name="folder.id"
            :tab="folder.name"
          ></n-tab>
        </n-tabs>
      </n-space>
    </n-layout>

    <!-- 手机上放个悬浮按钮放便唤起键盘 -->
    <div v-if="isMobile" class="floating-button" @click="focus_input">
      <i class="icon">/</i>
    </div>

    <n-layout>
      <n-image-group>
        <n-image
          v-for="img in state.images"
          :key="img.id"
          width="88"
          preview-disabled
          @click="copy_img(img, 0)"
          v-longPress="() => long_press_img(img)"
          :src="path_to_url(img.path)"
        />
      </n-image-group>
      <input
        type="file"
        accept="image/*"
        style="display: none"
        ref="fileInput"
        @change="handleFileChange"
      />
    </n-layout>
  </div>

  <n-modal v-model:show="showModal">
    <n-card style="width: 600px" :bordered="false" size="huge" role="dialog" aria-modal="true">
      <template #header-extra> </template>
      <n-input v-model:value="add_folder_name" type="text" placeholder="表情" />
      <n-input v-model:value="add_folder_keyword" type="textarea" placeholder="关键词(逗号分隔)" />
      <n-button @click="add_folder">提交</n-button>
      <template #footer> </template>
    </n-card>
  </n-modal>
</template>

<script lang="js">
import { ref } from 'vue'
import { reactive, watch, computed } from 'vue'
import { FolderService } from '@/stores/api'
import { NetUtil, DeviceUtil } from '@/stores/util'
import { Config } from '@/stores/config'

const isMobile = DeviceUtil.isMobile
const debounce = isMobile ? NetUtil.dyn_debounce(370) : null

export default {
  setup() {
    const state = reactive({
      styles: [],
      style_id: 0,
      search_str: '',
      folders: [],
      folder_id: 0,
      images: [],
    })
    const showModal = ref(false)
    const add_folder_name = ref('')
    const add_folder_keyword = ref('')

    const tabs_type = isMobile ? 'line' : 'card'

    return { state, isMobile, Config, debounce, tabs_type, showModal, add_folder_name, add_folder_keyword }
  },
  mounted() {
    // 页面渲染后
    // 加载style列表
    FolderService.styles().then(r => {
      this.state.styles = r
      if (r && r.length) {
        this.handle_styles_update(r[0].id)
      }
    })
    // 聚焦搜索框
    this.focus_input()
  },
  methods: {
    handle_styles_update(val) {
      this.state.style_id = val;
      // 选中style标签后用搜索框内容搜索folder
      this.search_folder(this.state.search_str)
    },
    handle_search_input_change(val) {
      this.state.search_str = val
      // 搜索框内容变更后搜索folder
      this.search_folder(val)
    },
    handle_search_input_change_mobile(val) {
      this.blur_input()
      this.state.search_str = val
      // 搜索框内容变更后搜索folder
      this.search_folder(val)
    },
    handle_search_input_blur() {
      // 搜索框失焦1秒后自动聚焦
      setTimeout(() => {
        this.focus_input()
      }, 1000)
    },
    handle_folders_change() {
      // 刷新folders后默认选中第一个
      const folders = this.state.folders
      if (folders && folders.length) {
        this.handle_folders_update(folders[0].id, 1)
      } else {
        this.state.images = []
      }
    },
    handle_folders_update(val, auto = 0) {
      this.state.folder_id = val
      // 选择folder后加载folder下图片
      this.get_imgs(val, auto).then(r => {
        if (r) {
          r.then(this.handle_imgs_change)
        } else {
          this.handle_imgs_change()
        }
      })
    },
    handle_imgs_change() {
      // img group 刷新后默认复制第一个
      const imgs = this.state.images
      if (imgs && imgs.length && this.state.search_str != '') {
        this.copy_img(imgs[0], isMobile ? 0 : 1)
      }
    },
    focus_input() {
      // 使用$refs来获取输入框元素，并调用focus()方法聚焦
      // if (!this.showModal) {
        this.$refs.searchInput.focus()
        this.$refs.searchInput.select()
      // }
    },
    blur_input() {
      this.$refs.searchInput.blur()
    },
    search_folder(str) {
      // 搜索框内容改变后: 搜索folder 渲染 自动选中第一个标签
      if (str == '') { str = ' ' }
      const styleId = this.state.style_id
      let by_key = () => {
        FolderService.by_key(str, styleId).then(r => this.state.folders = r).then(this.handle_folders_change)
      }
      if (isMobile) { debounce(by_key) } else { by_key() }
    },
    open_folder() {
      const folder_id = this.state.folder_id
      if (folder_id != 0) {
        FolderService.open(folder_id)
      }
    },
    get_imgs(id, auto = 0) {
      return FolderService.get_imgs(id).then(res => {
        this.state.images = res
        if (this.state.search_str != '') {
          return FolderService.click(id, auto)
        }
        return null;
      })
    },
    path_to_url: FolderService.path_to_url,
    long_press_img(img) {
      if (isMobile) {
        FolderService.img_click(img.id, 0)
      }
    },
    copy_img(img, auto = 0) {
      if (!isMobile) {
        FolderService.img_click(img.id, auto)
        FolderService.copy_img(img.path)
        return
      }

      // 调用浏览器分享图片 (不太行)
      // const url = this.path_to_url(img.path)
      // this.urlToFileList(url)
      // .then(fileList => { navigator.share({ files: fileList }).then(() => { }) })

      // 发送长按图片事件

    },
    add_folder() {
      FolderService.add(this.add_folder_name, this.add_folder_keyword, this.state.style_id)
        .then(r => {
          this.showModal = false
          this.search_folder(this.add_folder_name)
        })
    },
    allowDrop(event) {
      event.preventDefault()
    },
    handleDrop(event) {
      event.preventDefault();
      const file = event.dataTransfer.files[0]
      this.uploadFile(file)
    },
    selectFile() {
      this.$refs.fileInput.click();
    },
    handleFileChange(event) {
      const file = event.target.files[0]
      this.uploadFile(file)
    },
    uploadFile(file) {
      const data = new FormData()
      data.append('files', file)
      const folder_id = this.state.folder_id
      FolderService.add_img(folder_id, data).then(() => this.get_imgs(folder_id, 1))
    },
    urlToFileList(url) {
      return fetch(url, {
            method: 'get',
            responseType: 'blob'
        }).then(res => {
            return res.blob()
        }).then(blob => {
            // 创建一个虚拟的FileList对象，并将blob添加到该对象中
            // var fileList = new DataTransfer().files;
            var fileList = [];
            fileList.push(new File([blob], 'meme', { type: blob.type }))
            return fileList
        })
    },
  }
}
</script>

<style scoped>
.floating-button {
  position: fixed;
  z-index: 9999;
  bottom: 40%;
  right: 20px;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background-color: rgba(64, 64, 64, 0.5);
  color: rgba(192, 192, 192, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}
</style>