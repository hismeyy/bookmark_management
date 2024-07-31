<template>
  <div id="modal" @click="closeInfo">
    <div id="modal-content" @click.stop>
      <button @click="closeInfo" id="close-btn">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </button>
      <h1 id="title">编辑</h1>
      <div>
        <p style="display: inline-block;">类型：</p>
        <input type="text" class="input" v-model="type_name" placeholder="请输入类型..."
          style="width: 300px; margin-right: 0;">
      </div>
      <div class="button-box">
        <p style="float: left; line-height:0; color: #fc5185;">{{ tip }}</p>
        <button @click="remove" style="float: right; margin-right: 0; " class="button">删除</button>
        <button @click="update" style="float: right;; " class="button">修改</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  type_info_id: Number,
});

const emit = defineEmits(['close']);
const type_name = ref('');
const tip = ref('');

onMounted (() => {
  // 通过id加载数据
  invoke("get_type_info_by_id", { id: props.type_info_id }).then(
    (data) => {
      type_name.value = data.type_name;
    }
  )
})

function closeInfo() {
  emit('close');
  type_name.value = '';
  tip.value = '';
}

const update = () => {
  console.log(props.type_info_id);
  if (type_name.value == '') {
    tip.value = '请输入类型！';
    return;
  }
  else {
    tip.value = '';
  }

  invoke("update_type_info_by_id", { id: props.type_info_id, typeName: type_name.value })

  emit('close');
}

const remove = () => {
  invoke("delete_type_info_by_id", { id: props.type_info_id })
  emit('close');
}


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
  height: 140px;
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