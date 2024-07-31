<template>
  <div class="view-box">
    <!-- <UploadExcel/> -->
    <TypeInfo v-if="showType" :type_info_id="typeInfoId" @close="handleClose" />
    <div class="info-level-box">
      <div class="website-type-m-box">
        <div id="title">
          网站类型管理
        </div>
        <div>
          <input type="text" style="width: 200px;" class="input" placeholder="请输入需要添加的类型..." v-model="typeText"
            @keyup.enter="addType">
          <button class="button" @click="addType">添加</button>
        </div>
        <div id="web-type-box">
          <button v-for="(type, index) in types" :key="index"
            class="type-button" @click="openTypeInfo(type.id)">
            {{ type.type_name }}
          </button>
        </div>
      </div>
    </div>
  </div>

</template>

<script setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";
import TypeInfo from '../components/TypeInfo.vue';
import UploadExcel from '../components/UploadExcel.vue';


const typeText = ref('');
const types = ref([]);

const addType = () => {
  if (typeText.value === "") return;

  // 新增数据
  invoke("set_type_info", { typeName: typeText.value }).then(
    (id) => {
      console.log(id);
      types.value.push({
        id: id,
        type_name: typeText.value
      });
      typeText.value = '';
    }
  );
}

// 回显列表
const getTypeList = () => {
  invoke("get_type_info_list").then(
    (data) => {
      types.value = data
    }
  )
}
getTypeList()

// 修改框
const showType = ref(false);
const typeInfoId = ref(null);

const handleClose = () => {
  showType.value = false;
  getTypeList();
};

const openTypeInfo = (id) => {
  typeInfoId.value = id
  showType.value = true
}
</script>

<style scoped>
.website-type-m-box {
  height: 300px;
  padding: 10px;

}

#title {
  font-weight: bold;
  font-size: 18px;
  margin-bottom: 10px;
}

#web-type-box {
  margin-top: 10px;
  height: 225px;
  border: 0.1px solid #71c9ce;
  overflow-y: auto;
  border-radius: 4px;
}


#web-type-box::-webkit-scrollbar {
  width: 3px;
  border-radius: 4px;
}

#web-type-box::-webkit-scrollbar-track {
  background: #EDEDED;
  border-radius: 4px;
}

#web-type-box::-webkit-scrollbar-thumb {
  background: #95e1d3;
  border-radius: 4px;
}

#web-type-box::-webkit-scrollbar-thumb:hover {
  background: #EDEDED;
  border-radius: 4px;
}

.type-button {
  margin-top: 10px;
  margin-left: 10px;
  padding: 5px 10px;
  font-size: 16px;
  border: none;
  border-radius: 4px;
  color: white;
  cursor: pointer;
  background-color: #4ec588;
}
</style>
