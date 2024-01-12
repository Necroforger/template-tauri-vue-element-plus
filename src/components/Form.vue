<script setup>
    import {
        ElCard,
        ElTabs,
        ElTabPane,
        ElButton,
        ElContainer,
        ElAside,
        ElMain,
        ElDivider,
        ElRow,
        ElSpace,
    } from 'element-plus'
    import { useStateStore } from '../global_state'

    const state = useStateStore()

    function toggle_theme() {
        state.theme = state.theme == 'dark' ? 'light' : 'dark'
        document.documentElement.classList.toggle('dark')
    }
</script>

<template> 
    <ElTabs type="card">
        <ElTabPane label="Home">
            <ElSpace size="30px" direction="horizontal">
                <ElCard>
                    One
                </ElCard>
                <ElCard>Two</ElCard>
            </ElSpace>
        </ElTabPane>
        <ElTabPane label="About" name="about">
            What is a man but a miserable pile of secrets
        </ElTabPane>
        <ElTabPane label="Settings">
            <ElButton @click="toggle_theme">
                Toggle Theme: {{  state.theme }}
            </ElButton>
        </ElTabPane>
    </ElTabs>
</template>

<style scoped>
</style>