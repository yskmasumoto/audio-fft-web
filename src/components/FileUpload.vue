<template>
    <div class="wrapper">
        <div class="file-upload">
            <label for="upload" class="file-upload__label">
                <span class="file-upload__icon">📂</span>
                <span class="file-upload__text">{{ fileName || 'Choose a file...' }}</span>
            </label>
            <input type="file" id="upload" class="file-upload__input" @change="parentUpload" />
        </div>
    </div>
</template>

<script setup>
    import { ref } from 'vue';

    const emit = defineEmits(['file-uploaded']);
    const fileName = ref('');

    const parentUpload = async (event) => {
        const file = await event.target.files[0];
        fileName.value = file.name;
        emit('file-uploaded', file);
    };
</script>

<style scoped>
.wrapper {
    height: 100%;
}

.file-upload {
    height: 100%;
    display: inline-block;
    position: relative;
    overflow: hidden;
    cursor: pointer;
}

.file-upload__label {
    display: flex;
    align-items: center;
    /* padding: 8px 16px; */
    height: 100%;
    padding-left: 16px;
    padding-right: 16px;
    background-color: #646464;
    border-radius: 8px;
    transition: background-color 0.3s ease;
}

.file-upload__label:hover {
    background-color: #e0e0e0;
}

.file-upload__icon {
    margin-right: 8px;
    font-size: 20px;
}

.file-upload__text {
    font-size: 16px;
}

.file-upload__input {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
}
</style>