<template>
    <div>
        <input type="file" @change="handleFileUpload" />
    </div>
</template>

<script setup>
import * as XLSX from 'xlsx';

const handleFileUpload = (event) => {
    const file = event.target.files[0];
    if (file) {
        const reader = new FileReader();
        reader.onload = (e) => {
            const data = new Uint8Array(e.target.result);
            const workbook = XLSX.read(data, { type: 'array' });

            // 获取第一个工作表
            const sheetName = workbook.SheetNames[0];
            const worksheet = workbook.Sheets[sheetName];

            // 将工作表数据转为 JSON
            const jsonData = XLSX.utils.sheet_to_json(worksheet);

            // 打印数据到控制台
            console.log(jsonData);
        };
        reader.readAsArrayBuffer(file);
    }
};
</script>