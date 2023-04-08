<template>
    <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
            <component class="page" :is="Component" />
        </transition>
    </router-view>
</template>

<style scoped>
#app {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    height: 100%;
    min-height: 100vh;
}

#app > .fade-enter-active,
#app > .fade-leave-active {
    transition: opacity 0.15s ease-in-out;
}

#app > .fade-enter-from,
#app > .fade-leave-to {
    opacity: 0;
}
</style>

<script lang="ts">
export default {
    name: "App"
};
</script>
