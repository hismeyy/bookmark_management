<template>
  <div id="modal" @click="closeInfo">
    <div id="modal-content" @click.stop>
      <button @click="closeInfo" id="close-btn">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </button>
      <h1 id="title">{{ title }}</h1>
      <div>
        <p style="display: inline-block;">名称：</p>
        <input type="text" class="input" v-model="name" placeholder="请输入名称..." style="width: 300px; margin-right: 0;">
      </div>
      <div>
        <p style="display: inline-block;">地址：</p>
        <input type="text" class="input" v-model="address" placeholder="请输入地址..."
          style="width: 300px; margin-right: 0;">
      </div>
      <div>
        <p style="display: inline-block;">类型：</p>
        <select class="select" v-model="typeId" style="width: 300px; margin-right: 0;">
          <option v-for="(type, index) in types" :key="index" :value="type.id">{{ type.type_name }}</option>
        </select>
      </div>
      <div class="button-box">
        <p style="float: left; line-height:0; color: #fc5185;">{{ tip }}</p>
        <button @click="save" style="float: right; margin-right: 0; " class="button">保存</button>
        <button style="float: right; " class="button" v-if="is_not_add" @click="changePinned">{{ isPinned ? "取消置顶" :
          "置顶" }}</button>
        <button style="float: right; " class="button" v-if="is_not_add" @click="changeFrequent">{{ isFrequent ? "取消常用" :
          "常用" }}</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  id: Number,
  title: String,
  is_not_add: Boolean
});

const emit = defineEmits(['close']);

function closeInfo() {
  emit('close');
  name.value = '';
  address.value = '';
  typeId.value = 0;
  tip.value = '';
}

// 数据
const name = ref('');
const address = ref('');
const typeId = ref(0);
const tip = ref('');
const isFrequent = ref("");
const isPinned = ref("");
const types = ref();

// 保存
const save = () => {
  if (name.value == '') {
    tip.value = '请输入名称！';
    return;
  }
  else if (address.value == '') {
    tip.value = '请输入网址！';
    return;
  } else {
    tip.value = '';
  }
  if (props.is_not_add) {
    invoke("update_web_info_by_id", { id: props.id, name: name.value, address: address.value, typeId: typeId.value });
  } else {
    invoke("set_web_info", { name: name.value, address: address.value, typeId: typeId.value })
  }
  closeInfo();
}

// 回显列表
const getTypeList = () => {
  invoke("get_type_info_list").then(
    (data) => {
      types.value = data
    }
  )
}

// 加载时执行
getTypeList()




const getInfo = () => {
  if (props.id === 0) {
    return
  }
  invoke("get_web_info_by_id", { id: props.id }).then(
    (data) => {
      name.value = data.name;
      address.value = data.address;
      typeId.value = data.type_id;
      isFrequent.value = data.is_frequent;
      isPinned.value = data.is_pinned;
    }
  )
}

const changePinned = () => {
  invoke("update_web_info_is_pinned_by_id", { id: props.id }).then(
    isPinned.value = !isPinned.value
  )
}

const changeFrequent = () => {
  invoke("update_web_info_is_frequent_by_id", { id: props.id }).then(
    isFrequent.value = !isFrequent.value
  )
}

getInfo()
</script>

<style scoped>
#modal {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  font-size: 14px;
}

#modal-content {
  position: relative;
  padding: 10px 30px;
  border-radius: 4px;
  z-index: 1001;
  background-color: #fff;
  width: 345px;
  height: 225px;
  box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.2);
}

#close-btn {
  position: absolute;
  top: 10px;
  right: 10px;
  border: none;
  background: transparent;
  cursor: pointer;
}

#title {
  margin-top: 10px;
  font-size: 18px;
  font-weight: bold;
}

.button-box {
  margin-top: 10px;
  width: 100%;
}
</style>