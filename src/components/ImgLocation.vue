<template>
    <div class="scroll-container" @paste="handlePaste">
      <div @dragover="allowDrop" @drop="handleDrop">
        <label>*视频抽取一帧: ffmpeg -i input.mp4 -vframes 1 -ss 00:00:01 output.jpg</label>
        <br>  

        <input type="file" @change="onFileChange" />(也可以拖动或粘贴图片<br>
        <label>宽度: {{ width }}, </label>
        <label>高度: {{ height }}. </label>

        <label>填充值到: </label>
        <input type="radio" id="l_r_1" value="l_r_1" v-model="l_r">  
        <label for="l_r_1">坐标1</label>
        <input type="radio" id="l_r_2" value="l_r_2" v-model="l_r">  
        <label for="l_r_2">坐标2</label>
        <br>  

        <label>坐标1:</label><input v-model="x1" type="number" /><input v-model="y1" type="number" />
        <label>, 坐标2:</label><input v-model="x2" type="number" /><input v-model="y2" type="number" />

        <br>  
        <label>视频区域剪裁: ffmpeg -i input.mp4 -vf crop={{ w }}:{{ h }}:{{ x }}:{{ y }} output.mp4</label>

        <div><img v-if="imageUrl" :src="imageUrl" @click="copyCoordinates" ref="image" /></div>
      </div>
    </div>
  </template>  

  <script lang="js">
  import { Config } from '@/stores/config'
  import { ref } from 'vue'
  import { reactive, watch, computed } from 'vue'

  export default {  
    setup() {  
      const imageUrl = ref('')
      const width = ref(0)
      const height = ref(0)

      const l_r = ref('l_r_1')

      const x1 = ref(0)
      const y1 = ref(0)
      const x2 = ref(0)
      const y2 = ref(0)

      const w = ref(0)
      const h = ref(0)
      const x = ref(0)
      const y = ref(0)
      const image = ref(null)
      
      return { imageUrl, l_r, x, y, w, h, x1, y1, x2, y2, width, height, image }
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
                page_this.width = width
                page_this.height = height

                page_this.w = width
                page_this.h = height


                page_this.x2 = width
                page_this.y2 = height
            }
        },
        copyCoordinates(e) {  
          // 缩放倍数
          // const a = width.value / originalWidth.value
          let el = e.target
          const rect = el.getBoundingClientRect()
          const t_x = parseInt(e.clientX - rect.left)
          const t_y = parseInt(e.clientY - rect.top)
          
          // 这里可以根据实际需求添加复制坐标的代码，例如使用 navigator.clipboard.writeText() 方法复制到剪贴板  
          // alert(`坐标：(${newX}, ${newY})`)
          if (this.l_r == 'l_r_2') {
            this.x2 = t_x
            this.y2 = t_y
          } else {
            this.x1 = t_x
            this.y1 = t_y
          }
          this.w = this.x2 - this.x1
          this.h = this.y2 - this.y1
          this.x = this.x1
          this.y = this.y1
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