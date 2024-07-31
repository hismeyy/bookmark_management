<template>
  <div class="view-box">
    <div class="high-level-box">
      <input type="text" v-model="condition_wen_info.name" class="input" placeholder="请输入名称..." @keyup.enter="search">
      <input type="text" v-model="condition_wen_info.typeName" class="input" placeholder="请输入类型..."
        @keyup.enter="search">
      <input type="text" v-model="condition_wen_info.address" class="input" placeholder="请输入网站..."
        @keyup.enter="search">
      <button class="button" @click="search">搜索</button>
      <button class="button" @click="reset">重置</button>
    </div>
    <div class="info-level-box">
      <div class="grid-container">
        <div class="grid-header">名称</div>
        <div class="grid-header">类型</div>
        <div class="grid-header">网站</div>
        <div class="grid-header">操作</div>

        <div class="grid-row" v-for="(item, index) in webInfo" :key="index">
          <div class="grid-item">{{ item.name }}</div>
          <div class="grid-item">{{ item.type_name }}</div>
          <div class="grid-item">{{ item.address }}</div>
          <div class="grid-item">
            <button class="button" @click="openUrl(item.address)">打开</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const webInfo = ref();

const condition_wen_info = ref(
  {
    name: "",
    address: "",
    typeName: ""
  }
)


// 打开URL
const openUrl = async (url) => {
  await invoke("open_url", { url: url });
};

// 搜索
const search = () => {
  getWebInfoList()
}

// 重置
const reset = () => {
  condition_wen_info.value.name = "";
  condition_wen_info.value.address = "";
  condition_wen_info.value.typeName = "";
  getWebInfoList()
}

// 获取列表
const getWebInfoList = () => {
  invoke("get_web_info_ofent_list", {
    name: condition_wen_info.value.name,
    address: condition_wen_info.value.address,
    typeName: condition_wen_info.value.typeName
  }).then(
    (data) => {
      console.log(data);
      webInfo.value = data
    }
  );
}

// 加载页面是执行
getWebInfoList()

</script>

<style></style>