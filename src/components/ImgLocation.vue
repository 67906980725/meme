<template>
    <div class="scroll-container" @paste="handlePaste">
      <div @dragover="allowDrop" @drop="handleDrop">
        <input type="file" @change="onFileChange" />(也可以拖动或粘贴图片<br>
        <label>基准宽度:</label>  
        <input v-model="width" type="number" />
        <label>宽度: {{ originalWidth }}</label>
        <label>高度: {{ originalHeight }}</label>
        <label>(当修改宽度后, 将会以输入的宽度为基准缩放坐标</label>  
        <div><img v-if="imageUrl" :src="imageUrl" @click="copyCoordinates" ref="image" /></div>
      </div>
    </div>
  </template>  

  <script lang="js">
  import { Config } from '@/stores/config'
  import { ref } from 'vue';  
  import { reactive, watch, computed } from 'vue'

  export default {  
    setup() {  
      const imageUrl = ref('');  
      const originalWidth = ref('');  
      const originalHeight = ref('');  
      const width = ref('');  
      const height = ref('');  
      const image = ref(null);  
    
      const copyCoordinates = (e) => {  
        // 缩放倍数
        const a = width.value / originalWidth.value
        const rect = image.value.getBoundingClientRect()
        const x = e.clientX - rect.left
        const y = e.clientY - rect.top
        const newX = x * a
        const newY = y * a
        // 这里可以根据实际需求添加复制坐标的代码，例如使用 navigator.clipboard.writeText() 方法复制到剪贴板  
        alert(`坐标：(${newX}, ${newY})`)
      }
    
      return { imageUrl, originalWidth, originalHeight, width, height, copyCoordinates, image }
    },  
    mounted() {  
      this.$el.addEventListener('drop', this.handleDrop)
    },  
    methods: {
        allowDrop(event) {
            event.preventDefault()
        },
        handleDrop(e) {
            e.preventDefault()
            const file = e.dataTransfer.files[0]
            this.showImg(file)
        },
        handlePaste(event) {
            const items = event.clipboardData.items
            for (let i = 0; i < items.length; i++) {  
                const item = items[i]
                if (item.type.indexOf('image') !== -1) {  
                    const blob = item.getAsFile()
                    const url = URL.createObjectURL(blob)
                    this.imageUrl = url
                    this.img_size(url)
                }  
            }  
        },
        onFileChange(e) {
            const file = e.target.files[0]
            this.showImg(file)
        },
        showImg(file) {
            if (!file) { return }  
            const reader = new FileReader()
            reader.onload = (e) => {
                this.imageUrl = e.target.result
                this.img_size(e.target.result)
            }
            reader.readAsDataURL(file) 
        },
        img_size(url) {
            const page_this = this
            const img = new Image()
            img.src = url
            img.onload = function() {
                const width = img.width
                const height = img.height
                page_this.originalWidth = width
                page_this.originalHeight = height
                page_this.width = width
                page_this.height = height
            }
        },
    },
    unmounted() {  
      this.$el.removeEventListener('drop', this.handleDrop)
    },  
  }
  </script>  
    
  <style scoped>  
  .scroll-container {  
    overflow-x: auto; /* 或 overflow-x: scroll; */  
  }
  </style>