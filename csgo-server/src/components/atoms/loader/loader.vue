<template>
    <div class="loader" />
</template>

<style scoped>
@keyframes spin {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}

.loader {
    box-sizing: border-box;
    height: 32px;
    width: 32px;
    border-width: 6px;
    border-style: solid;
    border-color: #e6e6e6;
    border-radius: 50%;
    border-top-color: #2f81f7;
    animation: spin 1.25s ease-in-out infinite;
}
</style>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "loader"
});
</script>
