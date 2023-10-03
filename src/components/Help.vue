<template>
  <n-h1>使用方法</n-h1>
  <n-p> [启动] 在命令行(windows可以双击)启动后在浏览器访问 `127.0.0.1:8899` </n-p>
  <n-p> [添加模式/风格] 点击 `S` 按钮添加模式/风格 </n-p>
  <n-p> [添加表情分类] 点击 `F` 按钮添加表情分类, 在 `关键词` 栏目输入你所能联想到的尽可能多的关键词并以逗号分隔 </n-p>
  <n-p> [添加表情图片 (`pc:chrome`)] 拖动/复制图片到当前分类下空白处 </n-p>
  <n-p> [添加表情图片 (`android:termux`)] 点击 `+` 按钮选择图片 </n-p>
  <n-p> [搜索表情图片] 在搜索框输入 关键词 / 关键词拼音 / 关键词拼音缩写 / 多个关键词 后回车 </n-p>
  <n-p> [复制表情图片 (`pc`)] 点击表情图片 </n-p>
  <n-p> [修改关键词] 添加相同名称的表情分类 </n-p>
  <n-p> [删除风格/分类/表情] 可以用文件管理器删除, 会延时(5分钟)自动同步, 或手动同步 
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
