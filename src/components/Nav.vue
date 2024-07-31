<template>
    <div data-tauri-drag-region class="titlebar">
        <div class="titlebar-button" id="titlebar-minimize">
            <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
        </div>
        <div class="titlebar-button" id="titlebar-maximize">
            <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
        </div>
        <div class="titlebar-button" id="titlebar-close">
            <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </div>
    </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { appWindow } from '@tauri-apps/api/window'


onMounted(() => {
    document.getElementById('titlebar-minimize').addEventListener('click', () => appWindow.minimize())
    document.getElementById('titlebar-maximize').addEventListener('click', () => appWindow.toggleMaximize())
    document.getElementById('titlebar-close').addEventListener('click', () => appWindow.close())
})
</script>

<style>
.titlebar {
    width: 100%;
    height: 30px;
    background: #fff;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    border-bottom: 0.1px solid #71c9ce;
    
}

#title {
    width: 100%;
    display: flex;

}

#title h1 {
    font-size: 16px;
    margin-left: 30px;
    margin-top: 60px;
}

.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    
}

.titlebar-button:hover {
    background: #e4e4e4;
}
</style>