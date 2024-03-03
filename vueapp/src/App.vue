<script setup>
import { onMounted } from 'vue';
import HelloWorld from './components/HelloWorld.vue';
import TheWelcome from './components/TheWelcome.vue';
import init, { greet, WebGLResources } from '../public/web_video_editor/web_video_editor.js';

// Use onMounted lifecycle hook with <script setup>
onMounted(async () => {
  await init();
  console.log('WASM module initialized');
});

// Define a method for <script setup>
const sayHello = () => {
  greet();
};

const startWebGL = () => {
  const webGLResources = new WebGLResources("canvas");
  webGLResources.setup_resources();
  webGLResources.draw();
};
</script>

<template>
  <header>

  </header>
  <main>
    <button @click="sayHello">Say Hello</button>
    <canvas id="canvas" width="640" height="480"></canvas>
    <button @click="startWebGL">Start WebGL</button>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
