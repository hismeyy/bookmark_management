<template>
    <aside class="sidebar" data-tauri-drag-region>
        <div style="margin-top: 48px;">
            <div class="menu-button" id="menu-often">
                <router-link to="/ofent"><img src="../assets/ofent.svg" alt="常用" /></router-link>
            </div>
            <div class="menu-button" id="menu-collect">
                <router-link to="/collect"><img src="../assets/collect.svg" alt="收藏" /></router-link>
            </div>
            <div class="menu-button" id="menu-setting">
                <router-link to="/setting"><img src="../assets/setting.svg" alt="设置" /></router-link>
            </div>
        </div>
    </aside>
</template>

<script setup>

</script>

<style>
.sidebar {
    width: 50px;
    background-color: #222831;
    box-shadow: 2px 0 5px rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
}

.menu-button {
    width: 50px;
    height: 50px;
    padding: 0 12.5px;
}

.menu-button img {
    width: 25px;
    height: 25px;
    cursor: pointer;
}

.menu-button img:hover {
    filter: invert(50%) sepia(100%) hue-rotate(200deg) saturate(500%);
}
</style>