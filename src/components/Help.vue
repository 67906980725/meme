<template>
  <n-h1>使用方法</n-h1>
  <n-p> 在 .asset 目录下存放你的按风格(模式)文件夹/分类文件夹分好类的表情图片 </n-p>
  <n-p> 每个风格文件夹下放一个utf-8编码的文本文件(如sort.txt), 里边写个序号以用来排序 </n-p>
  <n-p>
    每个分类文件夹下存放一个utf-8编码的文本文件(如keywork.txt),
    里边写上对于这个类目你所能联想到的尽可能多的关键词并以逗号分隔</n-p
  >
  <!-- <a href="./.asset.zip">示例文件</a><br /> -->

  <n-p>
    准备好资源文件后就可以初始化数据库了
    <n-button text tag="a" target="_blank" type="primary" @click="re_init_db"> 点击开始 </n-button>
  </n-p>
</template>

<script lang="js">
import { FolderService } from '@/stores/api'
import { notify } from '@/stores/notification'

export default {
  setup() {
    return { }
  },
  methods: {
    re_init_db() {
      FolderService.re_init_db()
      notify('开始初始化数据库', `初始化数据库时间与你的资源数量和开发者的优化程度有关, 通常不会太久. 处理结果不会返回界面通知, 请留意服务端日志. `, '我知道了')
    }
  }
}
</script>

<style></style>
