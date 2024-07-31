<template>
  <div class="view-box">
    <Info :id="infolId" v-if="showInfo" :title="infoTitle" @close="handleClose" :is_not_add="isNotAdd" />
    <div class="high-level-box">
      <input type="text" v-model="condition_wen_info.name" class="input" placeholder="请输入名称..." @keyup.enter="search">
      <input type="text" v-model="condition_wen_info.typeName" class="input" placeholder="请输入类型..."
        @keyup.enter="search">
      <input type="text" v-model="condition_wen_info.address" class="input" placeholder="请输入网站..."
        @keyup.enter="search">
      <button class="button" @click="search">搜索</button>
      <button class="button" @click="reset">重置</button>
      <div style="float: right;">
        <!-- <button class="button">导入</button> -->
        <button class="button" @click="openInfo(0, '新增', false)">新增</button>
      </div>
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
            <button class="button" @click="openInfo(item.id, '编辑', true)">编辑</button>
            <button class="button" @click="remove(item.id)">删除</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import Info from '../components/Info.vue';
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const webInfo = ref();

const showInfo = ref(false);
const infolId = ref(0);
const infoTitle = ref(null);
const isNotAdd = ref(true);

const condition_wen_info = ref(
  {
    name: "",
    address: "",
    typeName: ""
  }
)

const openUrl = async (address) => {
  await invoke("open_url", { url: address });
};

// 打开Info
const openInfo = (id, title, is_not_add) => {
  infolId.value = id;
  showInfo.value = true;
  infoTitle.value = title;
  isNotAdd.value = is_not_add;
};

// 关闭Info
const handleClose = () => {
  showInfo.value = false;
  getWebInfoList()
};

// 获取列表
const getWebInfoList = () => {
  console.log({
    name: condition_wen_info.value.name,
    address: condition_wen_info.value.address,
    typeName: condition_wen_info.value.typeName
  });
  invoke("get_web_info_list", {
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

// 删除
const remove = (id) => {
  invoke("delete_web_info_by_id", { id: id }).then(
    getWebInfoList()
  );
}

// 加载页面执行
getWebInfoList()
</script>

<style></style>